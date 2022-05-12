use std::collections::BTreeMap;
use structopt::StructOpt;
use std::{thread, time};

use nodetop::{read_node_exporter_into_map, cpu_details, diff_cpu_details, CpuPresentation};

const DEFAULT_HOSTNAMES: &str = "192.168.66.80";
const DEFAULT_PORTS: &str = "9300";
const INTERVAL: u64  = 5;

#[derive(Debug, StructOpt)]
struct Opts {
    /// hostnames (comma separated)
    #[structopt(short, long, default_value = DEFAULT_HOSTNAMES)]
    hosts: String,
    /// port numbers (comma separated)
    #[structopt(short, long, default_value = DEFAULT_PORTS)]
    ports: String,
}

fn main() {
    let options = Opts::from_args();
    let hosts_string = &options.hosts as &str;
    let hosts = &hosts_string.split(",").collect();
    let ports_string = &options.ports as &str;
    let ports = &ports_string.split(",").collect();

    let mut host_presentation: BTreeMap<String, CpuPresentation> = BTreeMap::new();

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
    loop {
        let start_time = time::Instant::now();
        let values = read_node_exporter_into_map(hosts, ports, 1);
        let details = cpu_details(values);
        diff_cpu_details(details, &mut host_presentation);
        for (hostname_port, row) in &host_presentation {
            let total_time = row.idle_diff+row.user_diff+row.system_diff+row.irq_diff+row.softirq_diff+row.iowait_diff+row.nice_diff+row.steal_diff+row.guest_user_diff+row.guest_nice_diff;
            println!("{:30} {:5.0} {:5.0} | {:3.0} {:3.0} {:3.0} {:3.0} {:3.0} {:3.0} {:3.0} {:3.0} | {:3.0} {:3.0} | {:7.3} {:7.3} | {:6.3} {:6.3} {:6.3}",
                     hostname_port,
                     row.procs_running,
                     row.procs_blocked,
                     row.idle_diff/total_time*100.0,
                     row.user_diff/total_time*100.0,
                     row.system_diff/total_time*100.0,
                     row.iowait_diff/total_time*100.0,
                     row.nice_diff/total_time*100.0,
                     row.irq_diff/total_time*100.0,
                     row.softirq_diff/total_time*100.0,
                     row.steal_diff/total_time*100.0,
                     row.guest_user_diff/total_time*100.0,
                     row.guest_nice_diff/total_time*100.0,
                     row.schedstat_running_diff,
                     row.schedstat_waiting_diff,
                     row.load_1,
                     row.load_5,
                     row.load_15,
            );
        }
        thread::sleep(time::Duration::from_secs(INTERVAL)-start_time.elapsed());
    }
}
