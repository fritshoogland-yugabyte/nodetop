use std::collections::BTreeMap;
use structopt::StructOpt;
use std::{thread, time};
use std::process;

use nodetop::{read_node_exporter_into_map, cpu_details, diff_cpu_details, disk_details, CpuPresentation, DiskPresentation, diff_disk_details};

const DEFAULT_HOSTNAMES: &str = "192.168.66.80";
const DEFAULT_PORTS: &str = "9300";
const INTERVAL: u64 = 5;

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
}

fn main() {
    let options = Opts::from_args();
    let hosts_string = &options.hosts as &str;
    let hosts = &hosts_string.split(",").collect();
    let ports_string = &options.ports as &str;
    let ports = &ports_string.split(",").collect();
    let cpu = options.cpu as bool;
    let disk = options.disk as bool;

    if !cpu && !disk {
        Opts::clap().print_help().unwrap();
        process::exit(0);
    }
    let mut host_presentation: BTreeMap<String, CpuPresentation> = BTreeMap::new();
    let mut disk_presentation: BTreeMap<String, DiskPresentation> = BTreeMap::new();

    if cpu {
        println!("{:30} {:>5} {:>5} | {:3} {:3} {:3} {:3} {:3} {:3} {:3} {:3} | {:3} {:3} | {:>7} {:>7} | {:>6} {:>6} {:>6}",
                 "hostname",
                 "r",
                 "b",
                 "id%",
                 "us%",
                 "sy%",
                 "io%",
                 "ni%",
                 "ir%",
                 "si%",
                 "st%",
                 "gu%",
                 "gn%",
                 "scd rt",
                 "scd wt",
                 "l 1",
                 "l 5",
                 "l 15",
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
    }
    //
    loop {
        let start_time = time::Instant::now();
        let values = read_node_exporter_into_map(hosts, ports, 1);
        if cpu {
            let cpu_details = cpu_details(&values);
            diff_cpu_details(cpu_details, &mut host_presentation);
            for (hostname_port, row) in &host_presentation {
                let total_time = row.idle_diff + row.user_diff + row.system_diff + row.irq_diff + row.softirq_diff + row.iowait_diff + row.nice_diff + row.steal_diff + row.guest_user_diff + row.guest_nice_diff;
                println!("{:30} {:5.0} {:5.0} | {:3.0} {:3.0} {:3.0} {:3.0} {:3.0} {:3.0} {:3.0} {:3.0} | {:3.0} {:3.0} | {:7.3} {:7.3} | {:6.3} {:6.3} {:6.3}",
                         hostname_port,
                         row.procs_running,
                         row.procs_blocked,
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
                         row.schedstat_running_diff,
                         row.schedstat_waiting_diff,
                         row.load_1,
                         row.load_5,
                         row.load_15,
                );
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
            }
        }
        if start_time.elapsed() < time::Duration::from_secs(INTERVAL) {
            thread::sleep(time::Duration::from_secs(INTERVAL) - start_time.elapsed());
        }
    }
}
