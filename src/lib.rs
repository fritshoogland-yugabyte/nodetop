use chrono::{DateTime, Local, Utc};
use prometheus_parse::Value;
use std::collections::{HashMap, BTreeMap};
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

#[derive(Debug)]
pub struct CpuDetails {
    pub hostname_port: String,
    pub timestamp: DateTime<Utc>,
    pub load_1: f64,
    pub load_5: f64,
    pub load_15: f64,
    pub cpu_idle: f64,
    pub cpu_irq: f64,
    pub cpu_softirq: f64,
    pub cpu_system: f64,
    pub cpu_user: f64,
    pub cpu_iowait: f64,
    pub cpu_nice: f64,
    pub cpu_steal: f64,
    pub cpu_guest_nice: f64,
    pub cpu_guest_user: f64,
    pub schedstat_running: f64,
    pub schedstat_waiting: f64,
    pub procs_running: f64,
    pub procs_blocked: f64,
    pub context_switches: f64,
    pub interrupts: f64,
}

#[derive(Debug)]
pub struct DiskHost {
    pub hostname_port: String,
    pub timestamp: DateTime<Utc>,
    pub diskdetail: Vec<DiskDetail>,
}

#[derive(Debug)]
pub struct YugabyteIODetails {
    pub hostname_port: String,
    pub timestamp: DateTime<Utc>,
    pub glog_messages_info: f64,
    pub glog_messages_prio: f64,
    pub log_bytes_logged: f64,
    pub log_reader_bytes_read: f64,
    pub log_sync_latency_count: f64,
    pub log_sync_latency_sum: f64,
    pub log_append_latency_count: f64,
    pub log_append_latency_sum: f64,
    pub log_cache_disk_reads: f64,
    pub rocksdb_flush_write_bytes: f64,
    pub rocksdb_compact_read_bytes: f64,
    pub rocksdb_compact_write_bytes: f64,
    pub rocksdb_write_raw_block_micros_count: f64,
    pub rocksdb_write_raw_block_micros_sum: f64,
    pub rocksdb_sst_read_micros_count: f64,
    pub rocksdb_sst_read_micros_sum: f64,
}

#[derive(Debug)]
pub struct YBIOPresentation {
    pub timestamp: DateTime<Utc>,
    pub glog_messages_info_diff: f64,
    pub glog_messages_info_counter: f64,
    pub glog_messages_prio_diff: f64,
    pub glog_messages_prio_counter: f64,
    pub log_bytes_logged_diff: f64,
    pub log_bytes_logged_counter: f64,
    pub log_reader_bytes_read_diff: f64,
    pub log_reader_bytes_read_counter: f64,
    pub log_sync_latency_count_diff: f64,
    pub log_sync_latency_count_counter: f64,
    pub log_sync_latency_sum_diff: f64,
    pub log_sync_latency_sum_counter: f64,
    pub log_append_latency_count_diff: f64,
    pub log_append_latency_count_counter: f64,
    pub log_append_latency_sum_diff: f64,
    pub log_append_latency_sum_counter: f64,
    pub log_cache_disk_reads_diff: f64,
    pub log_cache_disk_reads_counter: f64,
    pub rocksdb_flush_write_bytes_diff: f64,
    pub rocksdb_flush_write_bytes_counter: f64,
    pub rocksdb_compact_read_bytes_diff: f64,
    pub rocksdb_compact_read_bytes_counter: f64,
    pub rocksdb_compact_write_bytes_diff: f64,
    pub rocksdb_compact_write_bytes_counter: f64,
    pub rocksdb_write_raw_block_micros_count_diff: f64,
    pub rocksdb_write_raw_block_micros_count_counter: f64,
    pub rocksdb_write_raw_block_micros_sum_diff: f64,
    pub rocksdb_write_raw_block_micros_sum_counter: f64,
    pub rocksdb_sst_read_micros_count_diff: f64,
    pub rocksdb_sst_read_micros_count_counter: f64,
    pub rocksdb_sst_read_micros_sum_diff: f64,
    pub rocksdb_sst_read_micros_sum_counter: f64,
}

#[derive(Debug)]
pub struct DiskDetail {
    pub disk_name: String,
    pub reads_completed: f64,
    pub writes_completed: f64,
    pub discards_completed: f64,
    pub reads_merged: f64,
    pub writes_merged: f64,
    pub discards_merged: f64,
    pub reads_bytes: f64,
    pub writes_bytes: f64,
    pub discards_sectors: f64,
    pub reads_time: f64,
    pub writes_time: f64,
    pub discards_time: f64,
    pub total_time: f64,
    pub queue: f64,
}

#[derive(Debug)]
pub struct CpuPresentation {
    pub timestamp: DateTime<Utc>,
    pub idle_diff: f64,
    pub idle_counter: f64,
    pub irq_diff: f64,
    pub irq_counter: f64,
    pub softirq_diff: f64,
    pub softirq_counter: f64,
    pub system_diff: f64,
    pub system_counter: f64,
    pub user_diff: f64,
    pub user_counter: f64,
    pub iowait_diff: f64,
    pub iowait_counter: f64,
    pub nice_diff: f64,
    pub nice_counter: f64,
    pub steal_diff: f64,
    pub steal_counter: f64,
    pub guest_nice_diff: f64,
    pub guest_nice_counter: f64,
    pub guest_user_diff: f64,
    pub guest_user_counter: f64,
    pub schedstat_running_diff: f64,
    pub schedstat_running_counter: f64,
    pub schedstat_waiting_diff: f64,
    pub schedstat_waiting_counter: f64,
    pub load_1: f64,
    pub load_5: f64,
    pub load_15: f64,
    pub procs_running: f64,
    pub procs_blocked: f64,
    pub context_switches_diff: f64,
    pub context_switches_counter: f64,
    pub interrupts_diff: f64,
    pub interrupts_counter: f64,
}

#[derive(Debug)]
pub struct DiskPresentation {
    pub timestamp: DateTime<Utc>,
    pub reads_completed_diff: f64,
    pub reads_completed_counter: f64,
    pub writes_completed_diff: f64,
    pub writes_completed_counter: f64,
    pub discards_completed_diff: f64,
    pub discards_completed_counter: f64,
    pub reads_merged_diff: f64,
    pub reads_merged_counter: f64,
    pub writes_merged_diff: f64,
    pub writes_merged_counter: f64,
    pub discards_merged_diff: f64,
    pub discards_merged_counter: f64,
    pub reads_bytes_diff: f64,
    pub reads_bytes_counter: f64,
    pub writes_bytes_diff: f64,
    pub writes_bytes_counter: f64,
    pub discards_sectors_diff: f64,
    pub discards_sectors_counter: f64,
    pub reads_time_diff: f64,
    pub reads_time_counter: f64,
    pub writes_time_diff: f64,
    pub writes_time_counter: f64,
    pub discards_time_diff: f64,
    pub discards_time_counter: f64,
    pub disk_total_time_diff: f64,
    pub disk_total_time_counter: f64,
    pub queue_diff: f64,
    pub queue_counter: f64,
}

pub fn read_node_exporter_into_map(
    hosts: &Vec<&str>,
    ports: &Vec<&str>,
    parallel: usize,
) -> HashMap<String, Vec<NodeExporterValues>> {
    let pool = rayon::ThreadPoolBuilder::new().num_threads(parallel).build().unwrap();
    let (tx, rx) = channel();
    pool.scope(move |s| {
        for host in hosts {
            for port in ports {
                let endpoints = vec!["metrics", "prometheus-metrics"];
                for endpoint in endpoints {
                    let tx = tx.clone();
                    s.spawn(move |_| {
                        let detail_snapshot_time = Local::now();
                        let node_exporter_values = read_node_exporter(&host, &port, &endpoint);
                        tx.send((format!("{}:{}:{}", host, port, endpoint), detail_snapshot_time, node_exporter_values)).expect("error sending data via tx (node_exporter)");
                    });
                }
            }
        }
    });
    let mut map_exporter_values: HashMap<String, Vec<NodeExporterValues>> = HashMap::new();
    for (hostname_port, _detail_snapshot_time, node_exporter_values) in rx {
        map_exporter_values.insert( hostname_port, node_exporter_values);
    }
    map_exporter_values
}

pub fn read_node_exporter(
    host: &str,
    port: &str,
    endpoint: &str,
) -> Vec<NodeExporterValues> {
    if !scan_port_addr(format!("{}:{}", host, port)) {
        println!("Warning! hostname:port {}:{} cannot be reached, skipping (node_exporter)", host, port);
        return parse_node_exporter(String::from(""));
    };
    let data_from_http = reqwest::blocking::get(format!("http://{}:{}/{}", host, port, endpoint))
        .unwrap_or_else(|e| {
            eprintln!("Fatal: error reading from URL: {}", e);
            process::exit(1);
        })
        .text().unwrap();
    parse_node_exporter(data_from_http)
}

fn parse_node_exporter(node_exporter_data: String) -> Vec<NodeExporterValues> {
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
                }
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
                }
                Value::Untyped(val) => {
                    // it turns out summary type _sum and _count values are untyped values.
                    // so I remove them here.
                    //if sample.metric.ends_with("_sum") || sample.metric.ends_with("_count") { continue; };
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
                }
                Value::Summary(_val) => {}
                Value::Histogram(_val) => {}
            }
        }
        // glog_(info|warning|error)_messages => glog_messages_total
        // counter, in written messages (write()) (IOPS).
        // this is the number of logging messages written as part of the logging framework, done via synchronous write() call.
        if nodeexportervalues.iter().filter(|r| r.node_exporter_name == "glog_info_messages").count() > 0 {
            for record in nodeexportervalues.iter_mut().filter(|r| r.node_exporter_name == "glog_info_messages" ) {
                record.node_exporter_category = "detail".to_string();
            }
            nodeexportervalues.push( NodeExporterValues {
                node_exporter_name: "glog_messages_info".to_string(),
                node_exporter_type: "counter".to_string(),
                node_exporter_labels: "".to_string(),
                node_exporter_category: "summary".to_string(),
                node_exporter_value: nodeexportervalues.iter().filter(|r| r.node_exporter_name == "glog_info_messages").map(|x| x.node_exporter_value).sum(),
                node_exporter_timestamp: nodeexportervalues.iter().filter(|r| r.node_exporter_name == "glog_info_messages").map(|x| x.node_exporter_timestamp).min().unwrap(),
            });
            nodeexportervalues.push( NodeExporterValues {
                node_exporter_name: "glog_messages_prio".to_string(),
                node_exporter_type: "counter".to_string(),
                node_exporter_labels: "".to_string(),
                node_exporter_category: "summary".to_string(),
                node_exporter_value: nodeexportervalues.iter().filter(|r| r.node_exporter_name == "glog_warning_messages" || r.node_exporter_name == "glog_error_messages").map(|x| x.node_exporter_value).sum(),
                node_exporter_timestamp: nodeexportervalues.iter().filter(|r| r.node_exporter_name == "glog_warning_messages" || r.node_exporter_name == "glog_error_messages").map(|x| x.node_exporter_timestamp).min().unwrap(),
            });
        };
        // log_bytes_logged
        // counter, in bytes (MBPS).
        // this is the number of bytes written as part of Yugabyte WAL logging.
        if nodeexportervalues.iter().filter(|r| r.node_exporter_name == "log_bytes_logged").count() > 0 {
            for record in nodeexportervalues.iter_mut().filter(|r| r.node_exporter_name == "log_bytes_logged") {
                record.node_exporter_category = "detail".to_string();
            }
            nodeexportervalues.push( NodeExporterValues {
                node_exporter_name: "log_bytes_logged".to_string(),
                node_exporter_type: "counter".to_string(),
                node_exporter_labels: "".to_string(),
                node_exporter_category: "summary".to_string(),
                node_exporter_value: nodeexportervalues.iter().filter(|r| r.node_exporter_name == "log_bytes_logged").map(|x| x.node_exporter_value).sum(),
                node_exporter_timestamp: nodeexportervalues.iter().filter(|r| r.node_exporter_name == "log_bytes_logged").map(|x| x.node_exporter_timestamp).min().unwrap(),
            });
        };
        // log_reader_bytes_read
        // counter, in bytes (MBPS).
        // this is the number of bytes read from WAL. Is this all reads?
        if nodeexportervalues.iter().filter(|r| r.node_exporter_name == "log_reader_bytes_read").count() > 0 {
            for record in nodeexportervalues.iter_mut().filter(|r| r.node_exporter_name == "log_reader_bytes_read") {
                record.node_exporter_category = "detail".to_string();
            }
            nodeexportervalues.push( NodeExporterValues {
                node_exporter_name: "log_reader_bytes_read".to_string(),
                node_exporter_type: "counter".to_string(),
                node_exporter_labels: "".to_string(),
                node_exporter_category: "summary".to_string(),
                node_exporter_value: nodeexportervalues.iter().filter(|r| r.node_exporter_name == "log_reader_bytes_read").map(|x| x.node_exporter_value).sum(),
                node_exporter_timestamp: nodeexportervalues.iter().filter(|r| r.node_exporter_name == "log_reader_bytes_read").map(|x| x.node_exporter_timestamp).min().unwrap(),
            });
        };
        // log_cache_disk_reads
        // counter, in # of reads (IOPS).
        // this is the number of bytes written as part of Yugabyte WAL logging.
        if nodeexportervalues.iter().filter(|r| r.node_exporter_name == "log_cache_disk_reads").count() > 0 {
            for record in nodeexportervalues.iter_mut().filter(|r| r.node_exporter_name == "log_cache_disk_reads") {
                record.node_exporter_category = "detail".to_string();
            }
            nodeexportervalues.push( NodeExporterValues {
                node_exporter_name: "log_cache_disk_reads".to_string(),
                node_exporter_type: "counter".to_string(),
                node_exporter_labels: "".to_string(),
                node_exporter_category: "summary".to_string(),
                node_exporter_value: nodeexportervalues.iter().filter(|r| r.node_exporter_name == "log_cache_disk_reads").map(|x| x.node_exporter_value).sum(),
                node_exporter_timestamp: nodeexportervalues.iter().filter(|r| r.node_exporter_name == "log_cache_disk_reads").map(|x| x.node_exporter_timestamp).min().unwrap(),
            });
        };
        // log_sync_latency_(count|sum)
        // both counters,
        // count = number of calls to Log::Sync() in log.cc NOT number of calls to fsync() !!
        // sum = total time spent in Log::Sync(), SOMETIMES with fsync() happening !!
        // https://github.com/yugabyte/yugabyte-db/issues/11039
        if nodeexportervalues.iter().filter(|r| r.node_exporter_name == "log_sync_latency_count").count() > 0 {
            for record in nodeexportervalues.iter_mut().filter(|r| r.node_exporter_name == "log_sync_latency_count") {
                record.node_exporter_category = "detail".to_string();
            }
            nodeexportervalues.push( NodeExporterValues {
                node_exporter_name: "log_sync_latency_count".to_string(),
                node_exporter_type: "counter".to_string(),
                node_exporter_labels: "".to_string(),
                node_exporter_category: "summary".to_string(),
                node_exporter_value: nodeexportervalues.iter().filter(|r| r.node_exporter_name == "log_sync_latency_count").map(|x| x.node_exporter_value).sum(),
                node_exporter_timestamp: nodeexportervalues.iter().filter(|r| r.node_exporter_name == "log_sync_latency_count").map(|x| x.node_exporter_timestamp).min().unwrap(),
            });
            for record in nodeexportervalues.iter_mut().filter(|r| r.node_exporter_name == "log_sync_latency_sum") {
                record.node_exporter_category = "detail".to_string();
            }
            nodeexportervalues.push( NodeExporterValues {
                node_exporter_name: "log_sync_latency_sum".to_string(),
                node_exporter_type: "counter".to_string(),
                node_exporter_labels: "".to_string(),
                node_exporter_category: "summary".to_string(),
                node_exporter_value: nodeexportervalues.iter().filter(|r| r.node_exporter_name == "log_sync_latency_sum").map(|x| x.node_exporter_value).sum(),
                node_exporter_timestamp: nodeexportervalues.iter().filter(|r| r.node_exporter_name == "log_sync_latency_sum").map(|x| x.node_exporter_timestamp).min().unwrap(),
            });
        }
        // log_append_latency_(count|sum)
        // both counters,
        // count = number of log append (write: writev()) occasions. (IOPS)
        // sum = total time spent writing in microseconds (us).
        if nodeexportervalues.iter().filter(|r| r.node_exporter_name == "log_append_latency_count").count() > 0 {
            for record in nodeexportervalues.iter_mut().filter(|r| r.node_exporter_name == "log_append_latency_count") {
                record.node_exporter_category = "detail".to_string();
            }
            nodeexportervalues.push( NodeExporterValues {
                node_exporter_name: "log_append_latency_count".to_string(),
                node_exporter_type: "counter".to_string(),
                node_exporter_labels: "".to_string(),
                node_exporter_category: "summary".to_string(),
                node_exporter_value: nodeexportervalues.iter().filter(|r| r.node_exporter_name == "log_append_latency_count").map(|x| x.node_exporter_value).sum(),
                node_exporter_timestamp: nodeexportervalues.iter().filter(|r| r.node_exporter_name == "log_append_latency_count").map(|x| x.node_exporter_timestamp).min().unwrap(),
            });
            for record in nodeexportervalues.iter_mut().filter(|r| r.node_exporter_name == "log_append_latency_sum") {
                record.node_exporter_category = "detail".to_string();
            }
            nodeexportervalues.push( NodeExporterValues {
                node_exporter_name: "log_append_latency_sum".to_string(),
                node_exporter_type: "counter".to_string(),
                node_exporter_labels: "".to_string(),
                node_exporter_category: "summary".to_string(),
                node_exporter_value: nodeexportervalues.iter().filter(|r| r.node_exporter_name == "log_append_latency_sum").map(|x| x.node_exporter_value).sum(),
                node_exporter_timestamp: nodeexportervalues.iter().filter(|r| r.node_exporter_name == "log_append_latency_sum").map(|x| x.node_exporter_timestamp).min().unwrap(),
            });
        }
        // rocksdb_flush_write_bytes
        // counter, number in bytes. (MBPS)
        // this statistic does not seem to be registering the write bytes as they happen, but rather after a transaction?
        if nodeexportervalues.iter().filter(|r| r.node_exporter_name == "rocksdb_flush_write_bytes").count() > 0 {
            for record in nodeexportervalues.iter_mut().filter(|r| r.node_exporter_name == "rocksdb_flush_write_bytes") {
                record.node_exporter_category = "detail".to_string();
            }
            nodeexportervalues.push( NodeExporterValues {
                node_exporter_name: "rocksdb_flush_write_bytes".to_string(),
                node_exporter_type: "counter".to_string(),
                node_exporter_labels: "".to_string(),
                node_exporter_category: "summary".to_string(),
                node_exporter_value: nodeexportervalues.iter().filter(|r| r.node_exporter_name == "rocksdb_flush_write_bytes").map(|x| x.node_exporter_value).sum(),
                node_exporter_timestamp: nodeexportervalues.iter().filter(|r| r.node_exporter_name == "rocksdb_flush_write_bytes").map(|x| x.node_exporter_timestamp).min().unwrap(),
            });
        }
        // rocksdb_compact_read_bytes
        // counter, number in bytes. (MBPS)
        // this statistic does not seem to be registering the read bytes as they happen, but rather after a transaction?
        if nodeexportervalues.iter().filter(|r| r.node_exporter_name == "rocksdb_compact_read_bytes").count() > 0 {
            for record in nodeexportervalues.iter_mut().filter(|r| r.node_exporter_name == "rocksdb_compact_read_bytes") {
                record.node_exporter_category = "detail".to_string();
            }
            nodeexportervalues.push( NodeExporterValues {
                node_exporter_name: "rocksdb_compact_read_bytes".to_string(),
                node_exporter_type: "counter".to_string(),
                node_exporter_labels: "".to_string(),
                node_exporter_category: "summary".to_string(),
                node_exporter_value: nodeexportervalues.iter().filter(|r| r.node_exporter_name == "rocksdb_compact_read_bytes").map(|x| x.node_exporter_value).sum(),
                node_exporter_timestamp: nodeexportervalues.iter().filter(|r| r.node_exporter_name == "rocksdb_compact_read_bytes").map(|x| x.node_exporter_timestamp).min().unwrap(),
            });
        }
        // rocksdb_compact_write_bytes
        // counter, number in bytes. (MBPS)
        // this statistic does not seem to be registering the write bytes as they happen, but rather after a transaction?
        if nodeexportervalues.iter().filter(|r| r.node_exporter_name == "rocksdb_compact_write_bytes").count() > 0 {
            for record in nodeexportervalues.iter_mut().filter(|r| r.node_exporter_name == "rocksdb_compact_write_bytes") {
                record.node_exporter_category = "detail".to_string();
            }
            nodeexportervalues.push( NodeExporterValues {
                node_exporter_name: "rocksdb_compact_write_bytes".to_string(),
                node_exporter_type: "counter".to_string(),
                node_exporter_labels: "".to_string(),
                node_exporter_category: "summary".to_string(),
                node_exporter_value: nodeexportervalues.iter().filter(|r| r.node_exporter_name == "rocksdb_compact_write_bytes").map(|x| x.node_exporter_value).sum(),
                node_exporter_timestamp: nodeexportervalues.iter().filter(|r| r.node_exporter_name == "rocksdb_compact_write_bytes").map(|x| x.node_exporter_timestamp).min().unwrap(),
            });
        }
        // rocksdb_write_raw_block_micros_(count|sum)
        // count = number in write IOs (write()). (IOPS)
        // sum = total time spent writing in microseconds (us).
        if nodeexportervalues.iter().filter(|r| r.node_exporter_name == "rocksdb_write_raw_block_micros_count").count() > 0 {
            for record in nodeexportervalues.iter_mut().filter(|r| r.node_exporter_name == "rocksdb_write_raw_block_micros_count") {
                record.node_exporter_category = "detail".to_string();
            }
            nodeexportervalues.push( NodeExporterValues {
                node_exporter_name: "rocksdb_write_raw_block_micros_count".to_string(),
                node_exporter_type: "counter".to_string(),
                node_exporter_labels: "".to_string(),
                node_exporter_category: "summary".to_string(),
                node_exporter_value: nodeexportervalues.iter().filter(|r| r.node_exporter_name == "rocksdb_write_raw_block_micros_count").map(|x| x.node_exporter_value).sum(),
                node_exporter_timestamp: nodeexportervalues.iter().filter(|r| r.node_exporter_name == "rocksdb_write_raw_block_micros_count").map(|x| x.node_exporter_timestamp).min().unwrap(),
            });
            for record in nodeexportervalues.iter_mut().filter(|r| r.node_exporter_name == "rocksdb_write_raw_block_micros_sum") {
                record.node_exporter_category = "detail".to_string();
            }
            nodeexportervalues.push( NodeExporterValues {
                node_exporter_name: "rocksdb_write_raw_block_micros_sum".to_string(),
                node_exporter_type: "counter".to_string(),
                node_exporter_labels: "".to_string(),
                node_exporter_category: "summary".to_string(),
                node_exporter_value: nodeexportervalues.iter().filter(|r| r.node_exporter_name == "rocksdb_write_raw_block_micros_sum").map(|x| x.node_exporter_value).sum(),
                node_exporter_timestamp: nodeexportervalues.iter().filter(|r| r.node_exporter_name == "rocksdb_write_raw_block_micros_sum").map(|x| x.node_exporter_timestamp).min().unwrap(),
            });
        }
        // rocksdb_sst_read_micros_(count|sum)
        // count = number in read IOs (pread64()). (IOPS)
        // sum = total time spent writing in microseconds (us).
        if nodeexportervalues.iter().filter(|r| r.node_exporter_name == "rocksdb_sst_read_micros_count").count() > 0 {
            for record in nodeexportervalues.iter_mut().filter(|r| r.node_exporter_name == "rocksdb_sst_read_micros_count") {
                record.node_exporter_category = "detail".to_string();
            }
            nodeexportervalues.push( NodeExporterValues {
                node_exporter_name: "rocksdb_sst_read_micros_count".to_string(),
                node_exporter_type: "counter".to_string(),
                node_exporter_labels: "".to_string(),
                node_exporter_category: "summary".to_string(),
                node_exporter_value: nodeexportervalues.iter().filter(|r| r.node_exporter_name == "rocksdb_sst_read_micros_count").map(|x| x.node_exporter_value).sum(),
                node_exporter_timestamp: nodeexportervalues.iter().filter(|r| r.node_exporter_name == "rocksdb_sst_read_micros_count").map(|x| x.node_exporter_timestamp).min().unwrap(),
            });
            for record in nodeexportervalues.iter_mut().filter(|r| r.node_exporter_name == "rocksdb_sst_read_micros_sum") {
                record.node_exporter_category = "detail".to_string();
            }
            nodeexportervalues.push( NodeExporterValues {
                node_exporter_name: "rocksdb_sst_read_micros_sum".to_string(),
                node_exporter_type: "counter".to_string(),
                node_exporter_labels: "".to_string(),
                node_exporter_category: "summary".to_string(),
                node_exporter_value: nodeexportervalues.iter().filter(|r| r.node_exporter_name == "rocksdb_sst_read_micros_sum").map(|x| x.node_exporter_value).sum(),
                node_exporter_timestamp: nodeexportervalues.iter().filter(|r| r.node_exporter_name == "rocksdb_sst_read_micros_sum").map(|x| x.node_exporter_timestamp).min().unwrap(),
            });
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
        // node_cpu_guest_seconds_total:
        // I only see 'nice' and 'user', not sure why currently?
        if nodeexportervalues.iter().filter(|r| r.node_exporter_name == "node_cpu_guest_seconds_total").count() > 0 {
            for record in nodeexportervalues.iter_mut().filter(|r| r.node_exporter_name == "node_cpu_guest_seconds_total") {
                record.node_exporter_category = "detail".to_string();
            }
            // user
            nodeexportervalues.push( NodeExporterValues {
                node_exporter_name: "node_cpu_guest_seconds_total".to_string(),
                node_exporter_type: "counter".to_string(),
                node_exporter_labels: "_user".to_string(),
                node_exporter_category: "summary".to_string(),
                node_exporter_timestamp: nodeexportervalues.iter().filter(|r| r.node_exporter_name == "node_cpu_guest_seconds_total").filter(|r| r.node_exporter_labels.contains("user")).map(|x| x.node_exporter_timestamp).min().unwrap(),
                node_exporter_value: nodeexportervalues.iter().filter(|r| r.node_exporter_name == "node_cpu_guest_seconds_total").filter(|r| r.node_exporter_labels.contains("user")).map(|x| x.node_exporter_value).sum(),
            });
            // nice
            nodeexportervalues.push( NodeExporterValues {
                node_exporter_name: "node_cpu_guest_seconds_total".to_string(),
                node_exporter_type: "counter".to_string(),
                node_exporter_labels: "_nice".to_string(),
                node_exporter_category: "summary".to_string(),
                node_exporter_timestamp: nodeexportervalues.iter().filter(|r| r.node_exporter_name == "node_cpu_guest_seconds_total").filter(|r| r.node_exporter_labels.contains("nice")).map(|x| x.node_exporter_timestamp).min().unwrap(),
                node_exporter_value: nodeexportervalues.iter().filter(|r| r.node_exporter_name == "node_cpu_guest_seconds_total").filter(|r| r.node_exporter_labels.contains("nice")).map(|x| x.node_exporter_value).sum(),
            });
        }
        // node_cpu_seconds_total:
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

pub fn cpu_details(
    values: &HashMap<String, Vec<NodeExporterValues>>
) -> Vec<CpuDetails>
{
    let mut details: Vec<CpuDetails> = Vec::new();
    for (hostname_port, node_exporter_vector) in values {
        if node_exporter_vector.iter().filter(|r| r.node_exporter_name == "node_load1").count() > 0 {
            details.push(CpuDetails {
                hostname_port: hostname_port.to_string(),
                timestamp: node_exporter_vector.iter().filter(|r| r.node_exporter_name == "node_load1").map(|x| x.node_exporter_timestamp).nth(0).unwrap(),
                load_1: node_exporter_vector.iter().filter(|r| r.node_exporter_name == "node_load1").map(|x| x.node_exporter_value).nth(0).unwrap(),
                load_5: node_exporter_vector.iter().filter(|r| r.node_exporter_name == "node_load5").map(|x| x.node_exporter_value).nth(0).unwrap(),
                load_15: node_exporter_vector.iter().filter(|r| r.node_exporter_name == "node_load15").map(|x| x.node_exporter_value).nth(0).unwrap(),
                cpu_idle: node_exporter_vector.iter().filter(|r| r.node_exporter_name == "node_cpu_seconds_total" && r.node_exporter_labels == "_idle" && r.node_exporter_category == "summary").map(|x| x.node_exporter_value).nth(0).unwrap(),
                cpu_irq: node_exporter_vector.iter().filter(|r| r.node_exporter_name == "node_cpu_seconds_total" && r.node_exporter_labels == "_irq" && r.node_exporter_category == "summary").map(|x| x.node_exporter_value).nth(0).unwrap(),
                cpu_softirq: node_exporter_vector.iter().filter(|r| r.node_exporter_name == "node_cpu_seconds_total" && r.node_exporter_labels == "_softirq" && r.node_exporter_category == "summary").map(|x| x.node_exporter_value).nth(0).unwrap(),
                cpu_system: node_exporter_vector.iter().filter(|r| r.node_exporter_name == "node_cpu_seconds_total" && r.node_exporter_labels == "_system" && r.node_exporter_category == "summary").map(|x| x.node_exporter_value).nth(0).unwrap(),
                cpu_user: node_exporter_vector.iter().filter(|r| r.node_exporter_name == "node_cpu_seconds_total" && r.node_exporter_labels == "_user" && r.node_exporter_category == "summary").map(|x| x.node_exporter_value).nth(0).unwrap(),
                cpu_iowait: node_exporter_vector.iter().filter(|r| r.node_exporter_name == "node_cpu_seconds_total" && r.node_exporter_labels == "_iowait" && r.node_exporter_category == "summary").map(|x| x.node_exporter_value).nth(0).unwrap(),
                cpu_nice: node_exporter_vector.iter().filter(|r| r.node_exporter_name == "node_cpu_seconds_total" && r.node_exporter_labels == "_nice" && r.node_exporter_category == "summary").map(|x| x.node_exporter_value).nth(0).unwrap(),
                cpu_steal: node_exporter_vector.iter().filter(|r| r.node_exporter_name == "node_cpu_seconds_total" && r.node_exporter_labels == "_steal" && r.node_exporter_category == "summary").map(|x| x.node_exporter_value).nth(0).unwrap(),
                cpu_guest_user: node_exporter_vector.iter().filter(|r| r.node_exporter_name == "node_cpu_guest_seconds_total" && r.node_exporter_labels == "_user" && r.node_exporter_category == "summary").map(|x| x.node_exporter_value).nth(0).unwrap(),
                cpu_guest_nice: node_exporter_vector.iter().filter(|r| r.node_exporter_name == "node_cpu_guest_seconds_total" && r.node_exporter_labels == "_nice" && r.node_exporter_category == "summary").map(|x| x.node_exporter_value).nth(0).unwrap(),
                schedstat_running: node_exporter_vector.iter().filter(|r| r.node_exporter_name == "node_schedstat_running_seconds_total" && r.node_exporter_category == "summary").map(|x| x.node_exporter_value).nth(0).unwrap(),
                schedstat_waiting: node_exporter_vector.iter().filter(|r| r.node_exporter_name == "node_schedstat_waiting_seconds_total" && r.node_exporter_category == "summary").map(|x| x.node_exporter_value).nth(0).unwrap(),
                procs_running: node_exporter_vector.iter().filter(|r| r.node_exporter_name == "node_procs_running").map(|x| x.node_exporter_value).nth(0).unwrap(),
                procs_blocked: node_exporter_vector.iter().filter(|r| r.node_exporter_name == "node_procs_blocked").map(|x| x.node_exporter_value).nth(0).unwrap(),
                context_switches: node_exporter_vector.iter().filter(|r| r.node_exporter_name == "node_context_switches_total").map(|x| x.node_exporter_value).nth(0).unwrap(),
                interrupts: node_exporter_vector.iter().filter(|r| r.node_exporter_name == "node_intr_total").map(|x| x.node_exporter_value).nth(0).unwrap(),
            });
        };
    }
    details
}

pub fn yugabyte_details(
    values: &HashMap<String, Vec<NodeExporterValues>>
) -> Vec<YugabyteIODetails>
{
    let mut details: Vec<YugabyteIODetails> = Vec::new();
    for (hostname_port, node_exporter_vector) in values {
        if node_exporter_vector.iter().filter(|r| r.node_exporter_name == "log_bytes_logged").count() > 0 {
            details.push(YugabyteIODetails {
                hostname_port: hostname_port.to_string(),
                timestamp: node_exporter_vector.iter().filter(|r| r.node_exporter_name == "log_bytes_logged").map(|x| x.node_exporter_timestamp).nth(0).unwrap(),
                glog_messages_info: node_exporter_vector.iter().filter(|r| r.node_exporter_name == "glog_messages_info" && r.node_exporter_category == "summary").map(|x| x.node_exporter_value).nth(0).unwrap(),
                glog_messages_prio: node_exporter_vector.iter().filter(|r| r.node_exporter_name == "glog_messages_prio" && r.node_exporter_category == "summary").map(|x| x.node_exporter_value).nth(0).unwrap(),
                log_bytes_logged: node_exporter_vector.iter().filter(|r| r.node_exporter_name == "log_bytes_logged" && r.node_exporter_category == "summary").map(|x| x.node_exporter_value).nth(0).unwrap(),
                log_reader_bytes_read: node_exporter_vector.iter().filter(|r| r.node_exporter_name == "log_reader_bytes_read" && r.node_exporter_category == "summary").map(|x| x.node_exporter_value).nth(0).unwrap(),
                log_sync_latency_count: node_exporter_vector.iter().filter(|r| r.node_exporter_name == "log_sync_latency_count" && r.node_exporter_category == "summary").map(|x| x.node_exporter_value).nth(0).unwrap(),
                log_sync_latency_sum: node_exporter_vector.iter().filter(|r| r.node_exporter_name == "log_sync_latency_sum" && r.node_exporter_category == "summary").map(|x| x.node_exporter_value).nth(0).unwrap(),
                log_append_latency_count: node_exporter_vector.iter().filter(|r| r.node_exporter_name == "log_append_latency_count" && r.node_exporter_category == "summary").map(|x| x.node_exporter_value).nth(0).unwrap(),
                log_append_latency_sum: node_exporter_vector.iter().filter(|r| r.node_exporter_name == "log_append_latency_sum" && r.node_exporter_category == "summary").map(|x| x.node_exporter_value).nth(0).unwrap(),
                log_cache_disk_reads: node_exporter_vector.iter().filter(|r| r.node_exporter_name == "log_cache_disk_reads" && r.node_exporter_category == "summary").map(|x| x.node_exporter_value).nth(0).unwrap(),
                rocksdb_flush_write_bytes: node_exporter_vector.iter().filter(|r| r.node_exporter_name == "rocksdb_flush_write_bytes" && r.node_exporter_category == "summary").map(|x| x.node_exporter_value).nth(0).unwrap(),
                rocksdb_compact_read_bytes: node_exporter_vector.iter().filter(|r| r.node_exporter_name == "rocksdb_compact_read_bytes" && r.node_exporter_category == "summary").map(|x| x.node_exporter_value).nth(0).unwrap(),
                rocksdb_compact_write_bytes: node_exporter_vector.iter().filter(|r| r.node_exporter_name == "rocksdb_compact_write_bytes" && r.node_exporter_category == "summary").map(|x| x.node_exporter_value).nth(0).unwrap(),
                rocksdb_write_raw_block_micros_count: node_exporter_vector.iter().filter(|r| r.node_exporter_name == "rocksdb_write_raw_block_micros_count" && r.node_exporter_category == "summary").map(|x| x.node_exporter_value).nth(0).unwrap(),
                rocksdb_write_raw_block_micros_sum: node_exporter_vector.iter().filter(|r| r.node_exporter_name == "rocksdb_write_raw_block_micros_sum" && r.node_exporter_category == "summary").map(|x| x.node_exporter_value).nth(0).unwrap(),
                rocksdb_sst_read_micros_count: node_exporter_vector.iter().filter(|r| r.node_exporter_name == "rocksdb_sst_read_micros_count" && r.node_exporter_category == "summary").map(|x| x.node_exporter_value).nth(0).unwrap(),
                rocksdb_sst_read_micros_sum: node_exporter_vector.iter().filter(|r| r.node_exporter_name == "rocksdb_sst_read_micros_sum" && r.node_exporter_category == "summary").map(|x| x.node_exporter_value).nth(0).unwrap(),
            });
            //println!("{}", node_exporter_vector.iter().filter(|r| r.node_exporter_name == "rocksdb_flush_write_bytes" && r.node_exporter_category == "summary").map(|x| x.node_exporter_value).nth(0).unwrap());
        };
    }
    details
}

pub fn disk_details(
    values: &HashMap<String, Vec<NodeExporterValues>>
) -> Vec<DiskHost>
{
    let mut details: Vec<DiskHost> = Vec::new();
    for (hostname_port, node_exporter_vector) in values {
        if node_exporter_vector.iter().filter(|r| r.node_exporter_name == "node_disk_reads_completed_total").count() > 0 {
            let mut diskstats: Vec<DiskDetail> = Vec::new();
            for row in node_exporter_vector.iter().filter(|r| r.node_exporter_name == "node_disk_reads_completed_total").filter(|r| !r.node_exporter_labels.contains("dm-")).map(|x| x.node_exporter_labels.clone()) {
                diskstats.push(DiskDetail {
                    disk_name: row[1..].to_string(),
                    reads_completed: node_exporter_vector.iter().filter(|r| r.node_exporter_name == "node_disk_reads_completed_total" && r.node_exporter_labels == row).map(|x| x.node_exporter_value).nth(0).unwrap(),
                    writes_completed: node_exporter_vector.iter().filter(|r| r.node_exporter_name == "node_disk_writes_completed_total" && r.node_exporter_labels == row).map(|x| x.node_exporter_value).nth(0).unwrap(),
                    discards_completed: node_exporter_vector.iter().filter(|r| r.node_exporter_name == "node_disk_discards_completed_total" && r.node_exporter_labels == row).map(|x| x.node_exporter_value).nth(0).unwrap_or_default(),
                    reads_merged: node_exporter_vector.iter().filter(|r| r.node_exporter_name == "node_disk_reads_merged_total" && r.node_exporter_labels == row).map(|x| x.node_exporter_value).nth(0).unwrap(),
                    writes_merged: node_exporter_vector.iter().filter(|r| r.node_exporter_name == "node_disk_writes_merged_total" && r.node_exporter_labels == row).map(|x| x.node_exporter_value).nth(0).unwrap(),
                    discards_merged: node_exporter_vector.iter().filter(|r| r.node_exporter_name == "node_disk_discards_merged_total" && r.node_exporter_labels == row).map(|x| x.node_exporter_value).nth(0).unwrap_or_default(),
                    reads_bytes: node_exporter_vector.iter().filter(|r| r.node_exporter_name == "node_disk_read_bytes_total" && r.node_exporter_labels == row).map(|x| x.node_exporter_value).nth(0).unwrap(),
                    writes_bytes: node_exporter_vector.iter().filter(|r| r.node_exporter_name == "node_disk_written_bytes_total" && r.node_exporter_labels == row).map(|x| x.node_exporter_value).nth(0).unwrap(),
                    discards_sectors: node_exporter_vector.iter().filter(|r| r.node_exporter_name == "node_disk_discarded_sectors_total" && r.node_exporter_labels == row).map(|x| x.node_exporter_value).nth(0).unwrap_or_default(),
                    reads_time: node_exporter_vector.iter().filter(|r| r.node_exporter_name == "node_disk_read_time_seconds_total" && r.node_exporter_labels == row).map(|x| x.node_exporter_value).nth(0).unwrap(),
                    writes_time: node_exporter_vector.iter().filter(|r| r.node_exporter_name == "node_disk_write_time_seconds_total" && r.node_exporter_labels == row).map(|x| x.node_exporter_value).nth(0).unwrap(),
                    discards_time: node_exporter_vector.iter().filter(|r| r.node_exporter_name == "node_disk_discard_time_seconds_total" && r.node_exporter_labels == row).map(|x| x.node_exporter_value).nth(0).unwrap_or_default(),
                    total_time: node_exporter_vector.iter().filter(|r| r.node_exporter_name == "node_disk_io_time_seconds_total" && r.node_exporter_labels == row).map(|x| x.node_exporter_value).nth(0).unwrap(),
                    queue: node_exporter_vector.iter().filter(|r| r.node_exporter_name == "node_disk_io_time_weighted_seconds_total" && r.node_exporter_labels == row).map(|x| x.node_exporter_value).nth(0).unwrap(),
                });
            }
            details.push(
                DiskHost {
                    hostname_port: hostname_port.to_string(),
                    timestamp: node_exporter_vector.iter().map(|x| x.node_exporter_timestamp).nth(0).unwrap(),
                    diskdetail: diskstats,
                }
            );
        }
    }
    details
}

pub fn diff_yugabyte_details(
    values: Vec<YugabyteIODetails>,
    yugabyte_presentation: &mut BTreeMap<String, YBIOPresentation>,
) {
    for yugabyte_details in values {
        match yugabyte_presentation.get_mut( &yugabyte_details.hostname_port) {
            Some(row) => {
               let time_difference = yugabyte_details.timestamp.signed_duration_since(row.timestamp).num_milliseconds() as f64 / 1000.;
                *row = YBIOPresentation {
                    timestamp: yugabyte_details.timestamp,
                    glog_messages_info_diff: (yugabyte_details.glog_messages_info - row.glog_messages_info_counter)/time_difference,
                    glog_messages_info_counter: yugabyte_details.glog_messages_info,
                    glog_messages_prio_diff: (yugabyte_details.glog_messages_prio - row.glog_messages_prio_counter)/time_difference,
                    glog_messages_prio_counter: yugabyte_details.glog_messages_prio,
                    log_bytes_logged_diff: (yugabyte_details.log_bytes_logged - row.log_bytes_logged_counter)/time_difference,
                    log_bytes_logged_counter: yugabyte_details.log_bytes_logged,
                    log_reader_bytes_read_diff: (yugabyte_details.log_reader_bytes_read - row.log_reader_bytes_read_counter)/time_difference,
                    log_reader_bytes_read_counter: yugabyte_details.log_reader_bytes_read,
                    log_sync_latency_count_diff: (yugabyte_details.log_sync_latency_count - row.log_sync_latency_count_counter)/time_difference,
                    log_sync_latency_count_counter: yugabyte_details.log_sync_latency_count,
                    log_sync_latency_sum_diff: (yugabyte_details.log_sync_latency_sum - row.log_sync_latency_sum_counter)/time_difference,
                    log_sync_latency_sum_counter: yugabyte_details.log_sync_latency_sum,
                    log_append_latency_count_diff: (yugabyte_details.log_append_latency_count - row.log_append_latency_count_counter)/time_difference,
                    log_append_latency_count_counter: yugabyte_details.log_append_latency_count,
                    log_append_latency_sum_diff: (yugabyte_details.log_append_latency_sum - row.log_append_latency_sum_counter)/time_difference,
                    log_append_latency_sum_counter: yugabyte_details.log_append_latency_sum,
                    log_cache_disk_reads_diff: (yugabyte_details.log_cache_disk_reads - row.log_cache_disk_reads_counter)/time_difference,
                    log_cache_disk_reads_counter: yugabyte_details.log_cache_disk_reads,
                    rocksdb_flush_write_bytes_diff: (yugabyte_details.rocksdb_flush_write_bytes - row.rocksdb_flush_write_bytes_counter)/time_difference,
                    rocksdb_flush_write_bytes_counter: yugabyte_details.rocksdb_flush_write_bytes,
                    rocksdb_compact_read_bytes_diff: (yugabyte_details.rocksdb_compact_read_bytes - row.rocksdb_compact_read_bytes_counter)/time_difference,
                    rocksdb_compact_read_bytes_counter: yugabyte_details.rocksdb_compact_read_bytes,
                    rocksdb_compact_write_bytes_diff: (yugabyte_details.rocksdb_compact_write_bytes - row.rocksdb_compact_write_bytes_counter)/time_difference,
                    rocksdb_compact_write_bytes_counter: yugabyte_details.rocksdb_compact_write_bytes,
                    rocksdb_write_raw_block_micros_count_diff: (yugabyte_details.rocksdb_write_raw_block_micros_count - row.rocksdb_write_raw_block_micros_count_counter)/time_difference,
                    rocksdb_write_raw_block_micros_count_counter: yugabyte_details.rocksdb_write_raw_block_micros_count,
                    rocksdb_write_raw_block_micros_sum_diff: (yugabyte_details.rocksdb_write_raw_block_micros_sum - row.rocksdb_write_raw_block_micros_sum_counter)/time_difference,
                    rocksdb_write_raw_block_micros_sum_counter: yugabyte_details.rocksdb_write_raw_block_micros_sum,
                    rocksdb_sst_read_micros_count_diff: (yugabyte_details.rocksdb_sst_read_micros_count - row.rocksdb_sst_read_micros_count_counter)/time_difference,
                    rocksdb_sst_read_micros_count_counter: yugabyte_details.rocksdb_sst_read_micros_count,
                    rocksdb_sst_read_micros_sum_diff: (yugabyte_details.rocksdb_sst_read_micros_sum - row.rocksdb_sst_read_micros_sum_counter)/time_difference,
                    rocksdb_sst_read_micros_sum_counter:yugabyte_details.rocksdb_sst_read_micros_sum,
                }
            },
            None => {
                yugabyte_presentation.insert( yugabyte_details.hostname_port, YBIOPresentation {
                    timestamp: yugabyte_details.timestamp,
                    glog_messages_info_diff: 0.,
                    glog_messages_info_counter: yugabyte_details.glog_messages_info,
                    glog_messages_prio_diff: 0.,
                    glog_messages_prio_counter: yugabyte_details.glog_messages_prio,
                    log_bytes_logged_diff: 0.,
                    log_bytes_logged_counter: yugabyte_details.log_bytes_logged,
                    log_reader_bytes_read_diff: 0.0,
                    log_reader_bytes_read_counter: yugabyte_details.log_reader_bytes_read,
                    log_sync_latency_count_diff: 0.0,
                    log_sync_latency_count_counter: yugabyte_details.log_sync_latency_count,
                    log_sync_latency_sum_diff: 0.0,
                    log_sync_latency_sum_counter: yugabyte_details.log_sync_latency_sum,
                    log_append_latency_count_diff: 0.,
                    log_append_latency_count_counter: yugabyte_details.log_append_latency_count,
                    log_append_latency_sum_diff: 0.,
                    log_append_latency_sum_counter: yugabyte_details.log_append_latency_sum,
                    log_cache_disk_reads_diff: 0.,
                    log_cache_disk_reads_counter: yugabyte_details.log_cache_disk_reads,
                    rocksdb_flush_write_bytes_diff: 0.,
                    rocksdb_flush_write_bytes_counter: yugabyte_details.rocksdb_flush_write_bytes,
                    rocksdb_compact_read_bytes_diff: 0.,
                    rocksdb_compact_read_bytes_counter: yugabyte_details.rocksdb_compact_read_bytes,
                    rocksdb_compact_write_bytes_diff: 0.,
                    rocksdb_compact_write_bytes_counter: yugabyte_details.rocksdb_compact_write_bytes,
                    rocksdb_write_raw_block_micros_count_diff: 0.,
                    rocksdb_write_raw_block_micros_count_counter: yugabyte_details.rocksdb_write_raw_block_micros_count,
                    rocksdb_write_raw_block_micros_sum_diff: 0.,
                    rocksdb_write_raw_block_micros_sum_counter: yugabyte_details.rocksdb_write_raw_block_micros_sum,
                    rocksdb_sst_read_micros_count_diff:  0.,
                    rocksdb_sst_read_micros_count_counter: yugabyte_details.rocksdb_sst_read_micros_count,
                    rocksdb_sst_read_micros_sum_diff: 0.,
                    rocksdb_sst_read_micros_sum_counter:yugabyte_details.rocksdb_sst_read_micros_sum,
                });
            },
        }
    }
}

pub fn diff_cpu_details(
    values: Vec<CpuDetails>,
    host_presentation: &mut BTreeMap<String, CpuPresentation>,
) {
    for host_details in values {
        match host_presentation.get_mut(&host_details.hostname_port) {
            Some(row) => {
                let time_difference = host_details.timestamp.signed_duration_since(row.timestamp).num_milliseconds() as f64 / 1000.0;
                *row = CpuPresentation {
                    timestamp: host_details.timestamp,
                    idle_diff: (host_details.cpu_idle - row.idle_counter)/time_difference,
                    idle_counter: host_details.cpu_idle,
                    irq_diff: (host_details.cpu_irq - row.irq_counter)/time_difference,
                    irq_counter: host_details.cpu_irq,
                    softirq_diff: (host_details.cpu_softirq - row.softirq_counter)/time_difference,
                    softirq_counter: host_details.cpu_softirq,
                    system_diff: (host_details.cpu_system - row.system_counter)/time_difference,
                    system_counter: host_details.cpu_system,
                    user_diff: (host_details.cpu_user - row.user_counter)/time_difference,
                    user_counter: host_details.cpu_user,
                    iowait_diff: (host_details.cpu_iowait - row.iowait_counter)/time_difference,
                    iowait_counter: host_details.cpu_iowait,
                    nice_diff: (host_details.cpu_nice - row.nice_counter)/time_difference,
                    nice_counter: host_details.cpu_nice,
                    steal_diff: (host_details.cpu_steal - row.steal_counter)/time_difference,
                    steal_counter: host_details.cpu_steal,
                    guest_nice_diff: (host_details.cpu_guest_nice - row.guest_nice_counter)/time_difference,
                    guest_nice_counter: host_details.cpu_guest_nice,
                    guest_user_diff: (host_details.cpu_guest_user - row.guest_user_counter)/time_difference,
                    guest_user_counter: host_details.cpu_guest_user,
                    schedstat_running_diff: (host_details.schedstat_running - row.schedstat_running_counter)/time_difference,
                    schedstat_running_counter: host_details.schedstat_running,
                    schedstat_waiting_diff: (host_details.schedstat_waiting - row.schedstat_waiting_counter)/time_difference,
                    schedstat_waiting_counter: host_details.schedstat_waiting,
                    load_1: host_details.load_1,
                    load_5: host_details.load_5,
                    load_15: host_details.load_15,
                    procs_running: host_details.procs_running,
                    procs_blocked: host_details.procs_blocked,
                    context_switches_diff: (host_details.context_switches - row.context_switches_counter)/time_difference,
                    context_switches_counter: host_details.context_switches,
                    interrupts_diff: (host_details.interrupts - row.interrupts_counter)/time_difference,
                    interrupts_counter: host_details.interrupts,

                }
            },
            None => {
                host_presentation.insert( host_details.hostname_port, CpuPresentation {
                    timestamp: host_details.timestamp,
                    idle_diff: 0.0,
                    idle_counter: host_details.cpu_idle,
                    irq_diff: 0.0,
                    irq_counter: host_details.cpu_irq,
                    softirq_diff: 0.0,
                    softirq_counter: host_details.cpu_softirq,
                    system_diff: 0.0,
                    system_counter: host_details.cpu_system,
                    user_diff: 0.0,
                    user_counter: host_details.cpu_user,
                    iowait_diff: 0.0,
                    iowait_counter: host_details.cpu_iowait,
                    nice_diff: 0.0,
                    nice_counter: host_details.cpu_nice,
                    steal_diff: 0.0,
                    steal_counter: host_details.cpu_steal,
                    guest_nice_diff: 0.0,
                    guest_nice_counter: host_details.cpu_guest_nice,
                    guest_user_diff: 0.0,
                    guest_user_counter: host_details.cpu_guest_user,
                    schedstat_running_diff: 0.0,
                    schedstat_running_counter: host_details.schedstat_running,
                    schedstat_waiting_diff: 0.0,
                    schedstat_waiting_counter: host_details.schedstat_waiting,
                    load_1: host_details.load_1,
                    load_5: host_details.load_5,
                    load_15: host_details.load_15,
                    procs_running: host_details.procs_running,
                    procs_blocked: host_details.procs_blocked,
                    context_switches_diff: 0.0,
                    context_switches_counter: host_details.context_switches,
                    interrupts_diff: 0.0,
                    interrupts_counter: host_details.interrupts,
                });
            },
        }
    }
}

pub fn diff_disk_details(
    values: Vec<DiskHost>,
    disk_presentation: &mut BTreeMap<String, DiskPresentation>,
) {
    for disk_details in values {
        for disk in disk_details.diskdetail {
            match disk_presentation.get_mut(format!("{} {}", &disk_details.hostname_port.clone(), disk.disk_name).as_str()) {
               Some(row) => {
                    let time_difference = disk_details.timestamp.signed_duration_since(row.timestamp).num_milliseconds() as f64 / 1000.0;
                    *row = DiskPresentation {
                        timestamp: disk_details.timestamp,
                        reads_completed_diff: (disk.reads_completed - row.reads_completed_counter)/time_difference,
                        reads_completed_counter: disk.reads_completed,
                        writes_completed_diff: (disk.writes_completed - row.writes_completed_counter)/time_difference,
                        writes_completed_counter: disk.writes_completed,
                        discards_completed_diff: (disk.discards_completed - row.discards_completed_counter)/time_difference,
                        discards_completed_counter: disk.discards_completed,
                        reads_merged_diff: (disk.reads_merged - row.reads_merged_counter)/time_difference,
                        reads_merged_counter: disk.discards_merged,
                        writes_merged_diff: (disk.writes_merged - row.writes_merged_counter)/time_difference,
                        writes_merged_counter: disk.writes_merged,
                        discards_merged_diff: (disk.discards_merged - row.discards_merged_counter)/time_difference,
                        discards_merged_counter: disk.discards_merged,
                        reads_bytes_diff: (disk.reads_bytes - row.reads_bytes_counter)/time_difference,
                        reads_bytes_counter: disk.reads_bytes,
                        writes_bytes_diff: (disk.writes_bytes - row.writes_bytes_counter)/time_difference,
                        writes_bytes_counter: disk.writes_bytes,
                        discards_sectors_diff: (disk.discards_sectors - row.discards_sectors_counter)/time_difference,
                        discards_sectors_counter: disk.discards_sectors,
                        reads_time_diff: (disk.reads_time - row.reads_time_counter)/time_difference,
                        reads_time_counter: disk.reads_time,
                        writes_time_diff: (disk.writes_time - row.writes_time_counter)/time_difference,
                        writes_time_counter: disk.writes_time,
                        discards_time_diff: (disk.discards_time - row.discards_time_counter)/time_difference,
                        discards_time_counter: disk.discards_time,
                        disk_total_time_diff: (disk.total_time - row.disk_total_time_counter)/time_difference,
                        disk_total_time_counter: disk.total_time,
                        queue_diff: (disk.queue - row.queue_counter)/time_difference,
                        queue_counter: disk.queue,
                    }
                },
                None => {
                    disk_presentation.insert( format!("{} {}", disk_details.hostname_port, disk.disk_name), DiskPresentation {
                        timestamp: disk_details.timestamp,
                        reads_completed_diff: 0.0,
                        reads_completed_counter: disk.reads_completed,
                        writes_completed_diff: 0.0,
                        writes_completed_counter: disk.writes_completed,
                        discards_completed_diff: 0.0,
                        discards_completed_counter: disk.discards_completed,
                        reads_merged_diff: 0.0,
                        reads_merged_counter: disk.discards_merged,
                        writes_merged_diff: 0.0,
                        writes_merged_counter: disk.writes_merged,
                        discards_merged_diff: 0.0,
                        discards_merged_counter: disk.discards_merged,
                        reads_bytes_diff: 0.0,
                        reads_bytes_counter: disk.reads_bytes,
                        writes_bytes_diff: 0.0,
                        writes_bytes_counter: disk.writes_bytes,
                        discards_sectors_diff: 0.0,
                        discards_sectors_counter: disk.discards_sectors,
                        reads_time_diff: 0.0,
                        reads_time_counter: disk.reads_time,
                        writes_time_diff: 0.0,
                        writes_time_counter: disk.writes_time,
                        discards_time_diff: 0.0,
                        discards_time_counter: disk.discards_time,
                        disk_total_time_diff: 0.0,
                        disk_total_time_counter: disk.total_time,
                        queue_diff: 0.0,
                        queue_counter: disk.queue,
                    });
                },
            }
        }
    }
}

/*
let yugabytestats = YugabyteIO {
log_bytes_logged: node_exporter_vector.iter().filter(|r| r.node_exporter_name == "log_bytes_logged" && r.node_exporter_category == "summary").map(|x| x.node_exporter_value).nth(0).unwrap_or_default(),
rocksdb_flush_write_bytes: 0.0,
rocskdb_compact_read_bytes: 0.0,
rocksdb_compact_write_bytes: 0.0,
};

 */
/*
match yugabyte_presentation.get_mut(&disk_details.hostname_port.clone()) {
Some(row) => {
let time_difference = disk_details.timestamp.signed_duration_since(row.timestamp).num_milliseconds() as f64 / 1000.;
*row = YBIOPresentation {
timestamp: disk_details.timestamp,
log_bytes_logged_diff: (disk_details.yugabyte_io.log_bytes_logged - row.log_bytes_logged_counter)/time_difference,
log_bytes_logged_counter: disk_details.yugabyte_io.log_bytes_logged,
}
},
None => {
yugabyte_presentation.insert(disk_details.hostname_port.clone(), YBIOPresentation {
timestamp: disk_details.timestamp,
log_bytes_logged_diff: 0.,
log_bytes_logged_counter: disk_details.yugabyte_io.log_bytes_logged,
});
},
}

 */