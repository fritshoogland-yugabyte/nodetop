use chrono::{DateTime, Local, Utc};
use prometheus_parse::Value;
//use serde_derive::{Serialize,Deserialize};
use port_scanner::scan_port_addr;
use std::process;
use rayon;
use std::sync::mpsc::channel;
//use std::fs;
//use regex::Regex;

#[derive(Debug)]
pub struct NodeExporterValues {
    pub node_exporter_name: String,
    pub node_exporter_type: String,
    pub node_exporter_labels: String,
    pub node_exporter_category: String,
    pub node_exporter_value: f64,
    pub node_exporter_timestamp: DateTime<Utc>,
}

#[derive(Debug)]
pub struct StoredNodeExporterValues {
    pub hostname_port: String,
    pub timestamp: DateTime<Local>,
    pub node_exporter_name: String,
    pub node_exporter_type: String,
    pub node_exporter_labels: String,
    pub node_exporter_category: String,
    pub node_exporter_value: f64,
}

pub fn read_node_exporter_into_vectors(
    hosts: &Vec<&str>,
    ports: &Vec<&str>,
    parallel: usize
) -> Vec<StoredNodeExporterValues> {
    let pool = rayon::ThreadPoolBuilder::new().num_threads(parallel).build().unwrap();
    let (tx, rx) = channel();
    pool.scope(move |s| {
        for host in hosts {
            for port in ports {
                let tx = tx.clone();
                s.spawn(move |_| {
                    let detail_snapshot_time = Local::now();
                    let node_exporter_values = read_node_exporter(&host, &port);
                    tx.send((format!("{}:{}", host, port), detail_snapshot_time, node_exporter_values)).expect("error sending data via tx (node_exporter)");
                });
            }
        }
    });
    let mut stored_node_exporter_values: Vec<StoredNodeExporterValues> = Vec::new();
    for (hostname_port, _detail_snapshot_time, node_exporter_values) in rx {
        add_to_node_exporter_vectors(node_exporter_values, &hostname_port, &mut stored_node_exporter_values);
    }
    stored_node_exporter_values
}

pub fn read_node_exporter(
    host: &str,
    port: &str,
) -> Vec<NodeExporterValues> {
    if ! scan_port_addr(format!("{}:{}", host, port)) {
        println!("Warning! hostname:port {}:{} cannot be reached, skipping (node_exporter)", host, port);
        return parse_node_exporter(String::from(""))
    };
    let data_from_http = reqwest::blocking::get(format!("http://{}:{}/metrics", host, port))
        .unwrap_or_else(|e| {
            eprintln!("Fatal: error reading from URL: {}", e);
            process::exit(1);
        })
        .text().unwrap();
    parse_node_exporter(data_from_http)
}

fn parse_node_exporter( node_exporter_data: String ) -> Vec<NodeExporterValues> {
    let lines: Vec<_> = node_exporter_data.lines().map(|s| Ok(s.to_owned())).collect();
    let node_exporter_rows = prometheus_parse::Scrape::parse(lines.into_iter()).unwrap();
    let mut nodeexportervalues = Vec::new();

    if node_exporter_rows.samples.len() > 0 {
        for sample in node_exporter_rows.samples {
            let mut label_temp = sample.labels.values().cloned().collect::<Vec<String>>();
            label_temp.sort();
            let mut label = label_temp.join("_");
            label = if label.len() > 0 {
                format!("_{}", label)
            } else {
                label
            };

            match sample.value {
                Value::Counter(val) => {
                    nodeexportervalues.push(
                        NodeExporterValues {
                            node_exporter_name: sample.metric.to_string(),
                            node_exporter_type: "counter".to_string(),
                            node_exporter_labels: label,
                            node_exporter_category: "all".to_string(),
                            node_exporter_timestamp: sample.timestamp,
                            node_exporter_value: val,
                        }
                    )
                },
                Value::Gauge(val) => {
                    nodeexportervalues.push(
                        NodeExporterValues {
                            node_exporter_name: sample.metric.to_string(),
                            node_exporter_type: "gauge".to_string(),
                            node_exporter_labels: label,
                            node_exporter_category: "all".to_string(),
                            node_exporter_timestamp: sample.timestamp,
                            node_exporter_value: val,
                        }
                    )
                },
                Value::Untyped(val) => {
                    // it turns out summary type _sum and _count values are untyped values.
                    // so I remove them here.
                    if sample.metric.ends_with("_sum") || sample.metric.ends_with("_count") { continue };
                    // untyped: not sure what it is.
                    // I would say: probably a counter.
                    nodeexportervalues.push(
                        NodeExporterValues {
                            node_exporter_name: sample.metric.to_string(),
                            node_exporter_type: "counter".to_string(),
                            node_exporter_labels: label,
                            node_exporter_category: "all".to_string(),
                            node_exporter_timestamp: sample.timestamp,
                            node_exporter_value: val,

                        }
                    )
                },
                Value::Histogram(_val) => {},
                Value::Summary(_val) => {},
            }
        }
        // post processing.
        // anything that starts with process_ is node_exporter process
        for record in nodeexportervalues.iter_mut().filter(|r| r.node_exporter_name.starts_with("process_")) {
            record.node_exporter_category = "detail".to_string();
        }
        // anything that start with promhttp_ is the node_exporter http server
        for record in nodeexportervalues.iter_mut().filter(|r| r.node_exporter_name.starts_with("promhttp_")) {
            record.node_exporter_category = "detail".to_string();
        }
        // anything that starts with go_ are statistics about the node_exporter process
        for record in nodeexportervalues.iter_mut().filter(|r| r.node_exporter_name.starts_with("go_")) {
            record.node_exporter_category = "detail".to_string();
        }
        // anything that starts with node_scrape_collector is about the node_exporter scraper
        for record in nodeexportervalues.iter_mut().filter(|r| r.node_exporter_name.starts_with("node_scrape_collector_")) {
            record.node_exporter_category = "detail".to_string();
        }
        // any record that contains a label that contains 'dm-' is a specification of a block device, and not the block device itself
        for record in nodeexportervalues.iter_mut().filter(|r| r.node_exporter_labels.contains("dm-")) {
            record.node_exporter_category = "detail".to_string();
        }
        // softnet: node_softnet_processed_total
        if nodeexportervalues.iter().filter(|r| r.node_exporter_name == "node_softnet_processed_total").count() > 0 {
            // make current records detail records
            for record in nodeexportervalues.iter_mut().filter(|r| r.node_exporter_name == "node_softnet_processed_total") {
                record.node_exporter_category = "detail".to_string();
            }
            // add a summary record
            nodeexportervalues.push(NodeExporterValues {
                node_exporter_name: "node_softnet_processed_total".to_string(),
                node_exporter_type: "counter".to_string(),
                node_exporter_labels: "".to_string(),
                node_exporter_category: "summary".to_string(),
                node_exporter_timestamp: nodeexportervalues.iter().filter(|r| r.node_exporter_name == "node_softnet_processed_total").map(|x| x.node_exporter_timestamp).min().unwrap(),
                node_exporter_value: nodeexportervalues.iter().filter(|r| r.node_exporter_name == "node_softnet_processed_total").map(|x| x.node_exporter_value).sum(),
            });
        }
        // softnet: node_softnet_dropped_total
        if nodeexportervalues.iter().filter(|r| r.node_exporter_name == "node_softnet_dropped_total").count() > 0 {
            // make current records detail records
            for record in nodeexportervalues.iter_mut().filter(|r| r.node_exporter_name == "node_softnet_dropped_total") {
                record.node_exporter_category = "detail".to_string();
            }
            // add a summary record
            nodeexportervalues.push(NodeExporterValues {
                node_exporter_name: "node_softnet_dropped_total".to_string(),
                node_exporter_type: "counter".to_string(),
                node_exporter_labels: "".to_string(),
                node_exporter_category: "summary".to_string(),
                node_exporter_timestamp: nodeexportervalues.iter().filter(|r| r.node_exporter_name == "node_softnet_dropped_total").map(|x| x.node_exporter_timestamp).min().unwrap(),
                node_exporter_value: nodeexportervalues.iter().filter(|r| r.node_exporter_name == "node_softnet_dropped_total").map(|x| x.node_exporter_value).sum(),
            });
        }
        // softnet: node_softnet_times_squeezed_total
        if nodeexportervalues.iter().filter(|r| r.node_exporter_name == "node_softnet_times_squeezed_total").count() > 0 {
            for record in nodeexportervalues.iter_mut().filter(|r| r.node_exporter_name == "node_softnet_times_squeezed_total") {
                record.node_exporter_category = "detail".to_string();
            }
            nodeexportervalues.push(NodeExporterValues {
                node_exporter_name: "node_softnet_times_squeezed_total".to_string(),
                node_exporter_type: "counter".to_string(),
                node_exporter_labels: "".to_string(),
                node_exporter_category: "summary".to_string(),
                node_exporter_timestamp: nodeexportervalues.iter().filter(|r| r.node_exporter_name == "node_softnet_times_squeezed_total").map(|x| x.node_exporter_timestamp).min().unwrap(),
                node_exporter_value: nodeexportervalues.iter().filter(|r| r.node_exporter_name == "node_softnet_times_squeezed_total").map(|x| x.node_exporter_value).sum(),
            });
        }
        // schedstat: node_schedstat_waiting_seconds
        if nodeexportervalues.iter().filter(|r| r.node_exporter_name == "node_schedstat_waiting_seconds_total").count() > 0 {
            for record in nodeexportervalues.iter_mut().filter(|r| r.node_exporter_name == "node_schedstat_waiting_seconds_total") {
                record.node_exporter_category = "detail".to_string();
            }
            nodeexportervalues.push(NodeExporterValues {
                node_exporter_name: "node_schedstat_waiting_seconds_total".to_string(),
                node_exporter_type: "counter".to_string(),
                node_exporter_labels: "".to_string(),
                node_exporter_category: "summary".to_string(),
                node_exporter_timestamp: nodeexportervalues.iter().filter(|r| r.node_exporter_name == "node_schedstat_waiting_seconds_total").map(|x| x.node_exporter_timestamp).min().unwrap(),
                node_exporter_value: nodeexportervalues.iter().filter(|r| r.node_exporter_name == "node_schedstat_waiting_seconds_total").map(|x| x.node_exporter_value).sum(),
            });
        }
        // schedstat: node_schedstat_timeslices
        if nodeexportervalues.iter().filter(|r| r.node_exporter_name == "node_schedstat_timeslices_total").count() > 0 {
            for record in nodeexportervalues.iter_mut().filter(|r| r.node_exporter_name == "node_schedstat_timeslices_total") {
                record.node_exporter_category = "detail".to_string();
            }
            nodeexportervalues.push(NodeExporterValues {
                node_exporter_name: "node_schedstat_timeslices_total".to_string(),
                node_exporter_type: "counter".to_string(),
                node_exporter_labels: "".to_string(),
                node_exporter_category: "summary".to_string(),
                node_exporter_timestamp: nodeexportervalues.iter().filter(|r| r.node_exporter_name == "node_schedstat_timeslices_total").map(|x| x.node_exporter_timestamp).min().unwrap(),
                node_exporter_value: nodeexportervalues.iter().filter(|r| r.node_exporter_name == "node_schedstat_timeslices_total").map(|x| x.node_exporter_value).sum(),
            });
        }
        // schedstat: node_schedstat_running_seconds
        if nodeexportervalues.iter().filter(|r| r.node_exporter_name == "node_schedstat_running_seconds_total").count() > 0 {
            for record in nodeexportervalues.iter_mut().filter(|r| r.node_exporter_name == "node_schedstat_running_seconds_total") {
                record.node_exporter_category = "detail".to_string();
            }
            nodeexportervalues.push(NodeExporterValues {
                node_exporter_name: "node_schedstat_running_seconds_total".to_string(),
                node_exporter_type: "counter".to_string(),
                node_exporter_labels: "".to_string(),
                node_exporter_category: "summary".to_string(),
                node_exporter_timestamp: nodeexportervalues.iter().filter(|r| r.node_exporter_name == "node_schedstat_running_seconds_total").map(|x| x.node_exporter_timestamp).min().unwrap(),
                node_exporter_value: nodeexportervalues.iter().filter(|r| r.node_exporter_name == "node_schedstat_running_seconds_total").map(|x| x.node_exporter_value).sum(),
            });
        }
        // cpu_seconds_total:
        if nodeexportervalues.iter().filter(|r| r.node_exporter_name == "node_cpu_seconds_total").count() > 0 {
            for record in nodeexportervalues.iter_mut().filter(|r| r.node_exporter_name == "node_cpu_seconds_total") {
                record.node_exporter_category = "detail".to_string();
            }
            // idle
            nodeexportervalues.push(NodeExporterValues {
                node_exporter_name: "node_cpu_seconds_total".to_string(),
                node_exporter_type: "counter".to_string(),
                node_exporter_labels: "_idle".to_string(),
                node_exporter_category: "summary".to_string(),
                node_exporter_timestamp: nodeexportervalues.iter().filter(|r| r.node_exporter_name == "node_cpu_seconds_total").filter(|r| r.node_exporter_labels.contains("idle")).map(|x| x.node_exporter_timestamp).min().unwrap(),
                node_exporter_value: nodeexportervalues.iter().filter(|r| r.node_exporter_name == "node_cpu_seconds_total").filter(|r| r.node_exporter_labels.contains("idle")).map(|x| x.node_exporter_value).sum(),
            });
            nodeexportervalues.push(NodeExporterValues {
                node_exporter_name: "node_cpu_seconds_total".to_string(),
                node_exporter_type: "counter".to_string(),
                node_exporter_labels: "_irq".to_string(),
                node_exporter_category: "summary".to_string(),
                node_exporter_timestamp: nodeexportervalues.iter().filter(|r| r.node_exporter_name == "node_cpu_seconds_total").filter(|r| r.node_exporter_labels.contains("_irq")).map(|x| x.node_exporter_timestamp).min().unwrap(),
                node_exporter_value: nodeexportervalues.iter().filter(|r| r.node_exporter_name == "node_cpu_seconds_total").filter(|r| r.node_exporter_labels.contains("_irq")).map(|x| x.node_exporter_value).sum(),
            });
            nodeexportervalues.push(NodeExporterValues {
                node_exporter_name: "node_cpu_seconds_total".to_string(),
                node_exporter_type: "counter".to_string(),
                node_exporter_labels: "_softirq".to_string(),
                node_exporter_category: "summary".to_string(),
                node_exporter_timestamp: nodeexportervalues.iter().filter(|r| r.node_exporter_name == "node_cpu_seconds_total").filter(|r| r.node_exporter_labels.contains("_softirq")).map(|x| x.node_exporter_timestamp).min().unwrap(),
                node_exporter_value: nodeexportervalues.iter().filter(|r| r.node_exporter_name == "node_cpu_seconds_total").filter(|r| r.node_exporter_labels.contains("_softirq")).map(|x| x.node_exporter_value).sum(),
            });
            nodeexportervalues.push(NodeExporterValues {
                node_exporter_name: "node_cpu_seconds_total".to_string(),
                node_exporter_type: "counter".to_string(),
                node_exporter_labels: "_system".to_string(),
                node_exporter_category: "summary".to_string(),
                node_exporter_timestamp: nodeexportervalues.iter().filter(|r| r.node_exporter_name == "node_cpu_seconds_total").filter(|r| r.node_exporter_labels.contains("system")).map(|x| x.node_exporter_timestamp).min().unwrap(),
                node_exporter_value: nodeexportervalues.iter().filter(|r| r.node_exporter_name == "node_cpu_seconds_total").filter(|r| r.node_exporter_labels.contains("system")).map(|x| x.node_exporter_value).sum(),
            });
            nodeexportervalues.push(NodeExporterValues {
                node_exporter_name: "node_cpu_seconds_total".to_string(),
                node_exporter_type: "counter".to_string(),
                node_exporter_labels: "_user".to_string(),
                node_exporter_category: "summary".to_string(),
                node_exporter_timestamp: nodeexportervalues.iter().filter(|r| r.node_exporter_name == "node_cpu_seconds_total").filter(|r| r.node_exporter_labels.contains("user")).map(|x| x.node_exporter_timestamp).min().unwrap(),
                node_exporter_value: nodeexportervalues.iter().filter(|r| r.node_exporter_name == "node_cpu_seconds_total").filter(|r| r.node_exporter_labels.contains("user")).map(|x| x.node_exporter_value).sum(),
            });
            nodeexportervalues.push(NodeExporterValues {
                node_exporter_name: "node_cpu_seconds_total".to_string(),
                node_exporter_type: "counter".to_string(),
                node_exporter_labels: "_iowait".to_string(),
                node_exporter_category: "summary".to_string(),
                node_exporter_timestamp: nodeexportervalues.iter().filter(|r| r.node_exporter_name == "node_cpu_seconds_total").filter(|r| r.node_exporter_labels.contains("iowait")).map(|x| x.node_exporter_timestamp).min().unwrap(),
                node_exporter_value: nodeexportervalues.iter().filter(|r| r.node_exporter_name == "node_cpu_seconds_total").filter(|r| r.node_exporter_labels.contains("iowait")).map(|x| x.node_exporter_value).sum(),
            });
            nodeexportervalues.push(NodeExporterValues {
                node_exporter_name: "node_cpu_seconds_total".to_string(),
                node_exporter_type: "counter".to_string(),
                node_exporter_labels: "_nice".to_string(),
                node_exporter_category: "summary".to_string(),
                node_exporter_timestamp: nodeexportervalues.iter().filter(|r| r.node_exporter_name == "node_cpu_seconds_total").filter(|r| r.node_exporter_labels.contains("nice")).map(|x| x.node_exporter_timestamp).min().unwrap(),
                node_exporter_value: nodeexportervalues.iter().filter(|r| r.node_exporter_name == "node_cpu_seconds_total").filter(|r| r.node_exporter_labels.contains("nice")).map(|x| x.node_exporter_value).sum(),
            });
            nodeexportervalues.push(NodeExporterValues {
                node_exporter_name: "node_cpu_seconds_total".to_string(),
                node_exporter_type: "counter".to_string(),
                node_exporter_labels: "_steal".to_string(),
                node_exporter_category: "summary".to_string(),
                node_exporter_timestamp: nodeexportervalues.iter().filter(|r| r.node_exporter_name == "node_cpu_seconds_total").filter(|r| r.node_exporter_labels.contains("steal")).map(|x| x.node_exporter_timestamp).min().unwrap(),
                node_exporter_value: nodeexportervalues.iter().filter(|r| r.node_exporter_name == "node_cpu_seconds_total").filter(|r| r.node_exporter_labels.contains("steal")).map(|x| x.node_exporter_value).sum(),
            });
        }
    }
    nodeexportervalues
}

pub fn add_to_node_exporter_vectors(
    node_exporter_values: Vec<NodeExporterValues>,
    hostname: &str,
    stored_node_exporter_values: &mut Vec<StoredNodeExporterValues>,
) {
    for row in node_exporter_values {
        if row.node_exporter_value > 0.0 {
            stored_node_exporter_values.push(
                StoredNodeExporterValues {
                    hostname_port: hostname.to_string(),
                    timestamp: DateTime::from(row.node_exporter_timestamp),
                    node_exporter_name: row.node_exporter_name.to_string(),
                    node_exporter_type: row.node_exporter_type.to_string(),
                    node_exporter_labels: row.node_exporter_labels.to_string(),
                    node_exporter_category: row.node_exporter_category.to_string(),
                    node_exporter_value: row.node_exporter_value,
                }
            );
        }
    }
}