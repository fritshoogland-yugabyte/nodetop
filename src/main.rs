use std::collections::BTreeMap;
use structopt::StructOpt;
use std::{thread, time};
use std::process;
use chrono::{DateTime, Utc};
use ctrlc;
use std::sync::{Arc, Mutex};
use plotters::prelude::*;
use itertools::Itertools;

use nodetop::{read_node_exporter_into_map, cpu_details, diff_cpu_details, disk_details, CpuPresentation, DiskPresentation, diff_disk_details};

#[derive(Debug)]
struct CpuGraph {
    hostname: String,
    timestamp: DateTime<Utc>,
    user: f64,
    system: f64,
    iowait: f64,
    nice: f64,
    irq: f64,
    softirq: f64,
    steal: f64,
    idle: f64,
    scheduler_runtime: f64,
    scheduler_wait: f64,
}

const DEFAULT_HOSTNAMES: &str = "192.168.66.80";
const DEFAULT_PORTS: &str = "9300";
const INTERVAL: &str = "5";

#[derive(Debug, StructOpt)]
struct Opts {
    /// hostnames (comma separated)
    #[structopt(short, long, default_value = DEFAULT_HOSTNAMES)]
    hosts: String,
    /// port numbers (comma separated)
    #[structopt(short, long, default_value = DEFAULT_PORTS)]
    ports: String,
    /// cpu statistics
    #[structopt(short, long)]
    cpu: bool,
    /// disk statistics
    #[structopt(short, long)]
    disk: bool,
    /// interval in seconds
    #[structopt(short, long, default_value = INTERVAL)]
    interval: u64,
    /// headers every number of lines
    #[structopt(short, long, default_value = "60")]
    lines_for_header: u64,
}

fn main() {
    let options = Opts::from_args();
    let hosts_string = &options.hosts as &str;
    let hosts = &hosts_string.split(",").collect();
    let ports_string = &options.ports as &str;
    let ports = &ports_string.split(",").collect();
    let cpu = options.cpu as bool;
    let disk = options.disk as bool;
    let interval = options.interval as u64;
    let lines_for_header = options.lines_for_header as u64;

    if !cpu && !disk {
        Opts::clap().print_help().unwrap();
        process::exit(0);
    }
    let mut host_presentation: BTreeMap<String, CpuPresentation> = BTreeMap::new();
    let mut disk_presentation: BTreeMap<String, DiskPresentation> = BTreeMap::new();
    let mut row_counter = 0;
    let cpu_history: Vec<CpuGraph> = Vec::new();
    let cpu_history_ref: Arc<Mutex<Vec<CpuGraph>>> = Arc::new(Mutex::new(cpu_history));

    let cpu_history_ctrlc_clone = cpu_history_ref.clone();
    ctrlc::set_handler(move || {

        /*
        let cpu_history = cpu_history_ctrlc_clone.lock().unwrap();
        //println!("{:?}", cpu_history);

        let start_time = cpu_history.iter().map(|x| x.timestamp).min().unwrap();
        let end_time = cpu_history.iter().map(|x| x.timestamp).max().unwrap();
        let low_value: f64 = 0.0;
        let high_value_cpu = cpu_history.iter().map(|x| x.idle).fold(0./0., f64::max);
        let high_value_scheduler = cpu_history.iter().map(|x| x.scheduler_wait).fold(0./0., f64::max);
        let high_value = if high_value_cpu > high_value_scheduler {
            high_value_cpu
        } else {
            high_value_scheduler
        };
        println!("{}, {}, {}. {}", start_time, end_time, low_value, high_value);
        let root = BitMapBackend::new("xplot.png", (600,400))
            .into_drawing_area();
        root.fill(&WHITE).unwrap();
        let mut context = ChartBuilder::on(&root)
            .set_label_area_size(LabelAreaPosition::Left, 60)
            .set_label_area_size(LabelAreaPosition::Bottom, 50)
            .caption("heading", ("sans-serif", 20))
            .build_cartesian_2d(start_time..end_time, low_value..high_value)
            .unwrap();
        context.configure_mesh()
            .x_labels(4)
            .x_label_formatter(&|x| x.naive_local().to_string())
            .x_desc("Time")
            .y_desc("Seconds per second")
            .draw()
            .unwrap();
        //context.draw_series( cpu_history.iter().map(|x| Circle::new((x.timestamp, x.user), 5, &GREEN ) ) ).unwrap();
        //context.draw_series( AreaSeries::new( cpu_history.iter().map(|x| (x.timestamp, x.user)), 0.0, GREEN ) ).unwrap();
        context.draw_series( cpu_history.iter().map(|x| Circle::new((x.timestamp, x.idle), 6, GREEN ) ) ).unwrap();
        //context.draw_series( AreaSeries::new( cpu_history.iter().map(|x| x.timestamp).zip(cpu_history.iter().map(|x| x.steal)), 0.0, &BLACK ) ).unwrap();
        //context.draw_series( AreaSeries::new( cpu_history.iter().map(|x| x.timestamp).zip(cpu_history.iter().map(|x| x.softirq)), 0.0, &YELLOW ) ).unwrap();
        //context.draw_series( AreaSeries::new( cpu_history.iter().map(|x| x.timestamp).zip(cpu_history.iter().map(|x| x.irq)), 0.0, &WHITE ) ).unwrap();


         */
        draw_cpu(&cpu_history_ctrlc_clone);

        //println!("{:#?}", cpu_history);


        process::exit(0);
    }).unwrap();

    let cpu_history_loop_clone = cpu_history_ref.clone();
    loop {
        if row_counter == 0 && lines_for_header != 0 {
                print_header(cpu, disk);
        }
        let start_time = time::Instant::now();
        let values = read_node_exporter_into_map(hosts, ports, 1);
        if cpu {
            let cpu_details = cpu_details(&values);
            diff_cpu_details(cpu_details, &mut host_presentation);
            for (hostname_port, row) in &host_presentation {

                if ! ( row.user_diff == 0. && row.system_diff == 0. && row.iowait_diff == 0. && row.nice_diff == 0. && row.irq_diff == 0. && row.softirq_diff == 0. && row.steal_diff == 0. ) {
                    let mut cpu_history = cpu_history_loop_clone.lock().unwrap();
                    cpu_history.push(CpuGraph {
                        hostname: hostname_port.to_string(),
                        timestamp: row.timestamp,
                        user: row.user_diff,
                        system: row.user_diff + row.system_diff,
                        iowait: row.user_diff + row.system_diff + row.iowait_diff,
                        nice: row.user_diff + row.system_diff + row.iowait_diff + row.nice_diff,
                        irq: row.user_diff + row.system_diff + row.iowait_diff + row.nice_diff + row.irq_diff,
                        softirq: row.user_diff + row.system_diff + row.iowait_diff + row.nice_diff + row.irq_diff + row.softirq_diff,
                        steal: row.user_diff + row.system_diff + row.iowait_diff + row.nice_diff + row.irq_diff + row.softirq_diff + row.steal_diff,
                        idle: row.user_diff + row.system_diff + row.iowait_diff + row.nice_diff + row.irq_diff + row.softirq_diff + row.steal_diff + row.idle_diff,
                        scheduler_runtime: row.schedstat_running_diff,
                        scheduler_wait: row.schedstat_running_diff + row.schedstat_waiting_diff,
                    });
                };
                //let cpu_details = CpuGraph { timestamp: row.timestamp, user: row.user_diff, system: row.system_diff, iowait: row.iowait_diff, nice: row.nice_diff, irq: row.irq_diff, softirq: row.softirq_diff, steal: row.steal_diff, idle: row.idle_diff, scheduler_runtime: row.schedstat_running_diff, scheduler_wait: row.schedstat_waiting_diff };
                //cpu_history.push(HostnameCpu{ hostname: hostname_port.to_string(), cpugraph: Vec { buf: (), cpu_details, len: 0 } });
                //cpu_history.push( row.timestamp, hostname_cpu);

                //cpu_history.push( row.timestamp, HostnameCpu { hostname: hostname_port.to_string(), cpugraph: CpuGraph, {  } } );
                //let total_time = row.idle_diff + row.user_diff + row.system_diff + row.irq_diff + row.softirq_diff + row.iowait_diff + row.nice_diff + row.steal_diff + row.guest_user_diff + row.guest_nice_diff;
                println!("{:30} {:5.0} {:5.0} | {:7.3} {:7.3} {:7.3} {:7.3} {:7.3} {:7.3} {:7.3} {:7.3} | {:7.3} {:7.3} | {:7.3} {:7.3} | {:7.0} {:7.0} | {:6.3} {:6.3} {:6.3}",
                         hostname_port,
                         row.procs_running,
                         row.procs_blocked,
                         row.idle_diff,
                         row.user_diff,
                    row.system_diff,
                    row.iowait_diff,
                    row.nice_diff,
                    row.irq_diff,
                    row.softirq_diff,
                    row.steal_diff,
                    row.guest_user_diff,
                    row.guest_nice_diff,
                    /*
                         if (row.idle_diff / total_time * 100.0).is_nan() {
                             0.0
                         } else {
                             row.idle_diff / total_time * 100.0
                         },
                         if (row.user_diff / total_time * 100.0).is_nan() {
                             0.0
                         } else {
                             row.user_diff / total_time * 100.0
                         },
                         if (row.system_diff / total_time * 100.0).is_nan() {
                             0.0
                         } else {
                             row.system_diff / total_time * 100.0
                         },
                         if (row.iowait_diff / total_time * 100.0).is_nan() {
                             0.0
                         } else {
                             row.iowait_diff / total_time * 100.0
                         },
                         if (row.nice_diff / total_time * 100.0).is_nan() {
                             0.0
                         } else {
                             row.nice_diff / total_time * 100.0
                         },
                         if (row.irq_diff / total_time * 100.0).is_nan() {
                             0.0
                         } else {
                             row.irq_diff / total_time * 100.0
                         },
                         if (row.softirq_diff / total_time * 100.0).is_nan() {
                             0.0
                         } else {
                             row.softirq_diff / total_time * 100.0
                         },
                         if (row.steal_diff / total_time * 100.0).is_nan() {
                             0.0
                         } else {
                             row.steal_diff / total_time * 100.0
                         },
                         if (row.guest_user_diff / total_time * 100.0).is_nan() {
                             0.0
                         } else {
                             row.guest_user_diff / total_time * 100.0
                         },
                         if (row.guest_nice_diff / total_time * 100.0).is_nan() {
                             0.0
                         } else {
                             row.guest_nice_diff / total_time * 100.0
                         },
                     */
                         row.schedstat_running_diff,
                         row.schedstat_waiting_diff,
                         row.interrupts_diff,
                         row.context_switches_diff,
                         row.load_1,
                         row.load_5,
                         row.load_15,
                );
                row_counter+=1;
            }
        }
        if disk {
            let disk_details = disk_details(&values);
            diff_disk_details(disk_details, &mut disk_presentation);
            for (host_disk, row) in &disk_presentation {
                println!("{:30} {:5.0} {:5.0} {:5.0} {:8.6} | {:5.0} {:5.0} {:5.0} {:8.6} | {:5.0} {:5.0} {:5.0} {:8.6} | {:8.3} | {:5.0} {:5.0}",
                         host_disk,
                         row.reads_merged_diff.round(),
                         row.reads_completed_diff.round(),
                         (row.reads_bytes_diff / (1024 * 1024) as f64).round(),
                         if (row.reads_time_diff / row.reads_completed_diff).is_nan() {
                             0.0
                         } else {
                             row.reads_time_diff / row.reads_completed_diff
                         },
                         row.writes_merged_diff.round(),
                         row.writes_completed_diff.round(),
                         (row.writes_bytes_diff / (1024 * 1024) as f64).round(),
                         if (row.writes_time_diff / row.writes_completed_diff).is_nan() {
                             0.0
                         } else {
                             row.writes_time_diff / row.writes_completed_diff
                         },
                         row.discards_merged_diff.round(),
                         row.discards_completed_diff.round(),
                         row.discards_sectors_diff.round(),
                         if (row.discards_time_diff / row.discards_completed_diff).is_nan() {
                             0.0
                         } else {
                             row.discards_time_diff / row.discards_completed_diff
                         },
                         row.queue_diff,
                         (row.reads_completed_diff + row.writes_completed_diff).round(),
                         (row.reads_bytes_diff / (1024 * 1024) as f64 + row.writes_bytes_diff / (1024 * 1024) as f64).round(),
                );
                row_counter+=1;
            }
        }
        if row_counter > lines_for_header {
            row_counter = 0;
        }
        if start_time.elapsed() < time::Duration::from_secs(interval) {
            thread::sleep(time::Duration::from_secs(interval) - start_time.elapsed());
        }
    }
}

fn print_header(cpu: bool, disk: bool) {
    if cpu {
        println!("{:30} {:>5} {:>5} | {:>7} {:>7} {:>7} {:>7} {:>7} {:>7} {:>7} {:>7} | {:>7} {:>7} | {:>7} {:>7} | {:>7} {:>7} | {:>6} {:>6} {:>6}",
                 "hostname",
                 "r",
                 "b",
                 "id",
                 "us",
                 "sy",
                 "io",
                 "ni",
                 "ir",
                 "si",
                 "st",
                 "gu",
                 "gn",
                 "scd_rt",
                 "scd_wt",
                 "in",
                 "cs",
                 "l_1",
                 "l_5",
                 "l_15",
        );
    };
    if disk {
        println!("{:30} {:26} | {:26} | {:26} | {:8} | {:11}",
                 "",
                 "reads per second",
                 "writes per second",
                 "discards per second",
                 "",
                 "totals per second" ,
        );
        println!("{:30} {:>5} {:>5} {:>5} {:>8} | {:>5} {:>5} {:>5} {:>8} | {:>5} {:>5} {:>5} {:>8} | {:>8} | {:>5} {:>5}",
                 "hostname",
                 "merge",
                 "io",
                 "mb",
                 "avg",
                 "merge",
                 "io",
                 "mb",
                 "avg",
                 "merge",
                 "io",
                 "sect",
                 "avg",
                 "queue",
                 "IOPS",
                 "MBPS",
        );
    };
}

fn draw_cpu(data: &Arc<Mutex<Vec<CpuGraph>>>) {

    let cpu_data = data.lock().unwrap();

    let start_time = cpu_data.iter().map(|x| x.timestamp).min().unwrap();
    let end_time = cpu_data.iter().map(|x| x.timestamp).max().unwrap();
    let low_value: f64 = 0.0;
    let high_value_cpu = cpu_data.iter().map(|x| x.idle).fold(0./0., f64::max);
    let high_value_scheduler = cpu_data.iter().map(|x| x.scheduler_wait).fold(0./0., f64::max);
    let high_value = if high_value_cpu > high_value_scheduler {
        high_value_cpu
    } else {
        high_value_scheduler
    };
    let root = BitMapBackend::new("xplot.png", (1200,800))
        .into_drawing_area();
    let nr_servers = cpu_data.iter().map(|x| x.hostname.clone()).unique().count();
    let multiroot = root.split_evenly((nr_servers,1));

    for (multiroot_nr, server) in (0..nr_servers).zip(cpu_data.iter().map(|x| x.hostname.clone()).unique()) {
        multiroot[multiroot_nr].fill(&WHITE).unwrap();
        let mut context = ChartBuilder::on(&multiroot[multiroot_nr])
            .set_label_area_size(LabelAreaPosition::Left, 60)
            .set_label_area_size(LabelAreaPosition::Bottom, 50)
            .caption(server, ("sans-serif", 20))
            .build_cartesian_2d(start_time..end_time, low_value..high_value)
            .unwrap();
        context.configure_mesh()
            .x_labels(4)
            .x_label_formatter(&|x| x.naive_local().to_string())
            .x_desc("Time")
            .y_desc("Seconds per second")
            .draw()
            .unwrap();
        context.draw_series(AreaSeries::new(cpu_data.iter().map(|x| (x.timestamp, x.scheduler_wait)), 0.0, Palette99::pick(1))).unwrap().label("scheduler wait").legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], Palette99::pick(1)));
        context.draw_series(AreaSeries::new(cpu_data.iter().map(|x| (x.timestamp, x.scheduler_runtime)), 0.0, Palette99::pick(2))).unwrap().label("scheduler run").legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], Palette99::pick(2)));
        context.draw_series(AreaSeries::new(cpu_data.iter().map(|x| (x.timestamp, x.idle)), 0.0, TRANSPARENT).border_style(RED)).unwrap().label("Total CPU").legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED));
        context.draw_series(AreaSeries::new(cpu_data.iter().map(|x| (x.timestamp, x.steal)), 0.0, Palette99::pick(3))).unwrap().label("steal").legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], Palette99::pick(3)));
        context.draw_series(AreaSeries::new(cpu_data.iter().map(|x| (x.timestamp, x.softirq)), 0.0, Palette99::pick(4))).unwrap().label("soft irq").legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], Palette99::pick(4)));
        context.draw_series(AreaSeries::new(cpu_data.iter().map(|x| (x.timestamp, x.irq)), 0.0, Palette99::pick(5))).unwrap().label("irq").legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], Palette99::pick(5)));
        context.draw_series(AreaSeries::new(cpu_data.iter().map(|x| (x.timestamp, x.nice)), 0.0, Palette99::pick(6))).unwrap().label("nice").legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], Palette99::pick(6)));
        context.draw_series(AreaSeries::new(cpu_data.iter().map(|x| (x.timestamp, x.iowait)), 0.0, Palette99::pick(7))).unwrap().label("iowait").legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], Palette99::pick(7)));
        context.draw_series(AreaSeries::new(cpu_data.iter().map(|x| (x.timestamp, x.system)), 0.0, Palette99::pick(8))).unwrap().label("system").legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], Palette99::pick(8)));
        context.draw_series(AreaSeries::new(cpu_data.iter().map(|x| (x.timestamp, x.user)), 0.0, GREEN)).unwrap().label("user").legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], GREEN));
        context.configure_series_labels().border_style(BLACK).background_style(WHITE).draw().unwrap();
    }
}