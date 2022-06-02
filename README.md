# nodetop

Nodetop is a utility to measure and print detailed data from prometheus (node exporter like) sources at high frequency, such as per 1 second.

The current data that nodetop measures is:
- CPU statistics (from node exporter)
- Disk statistics (from node exporter)
- Yugabyte IO statistics (from YugabyteDB tablet server or master)
 
Nodetop, when executed, will print out the statistics at a 5 seconds interval for all nodes involved to the screen. 
If the switch `--graph` is set, it will create PNG files for CPU, disk and YugabyteIO in the current working directory.

See `--help` for all the options.

# install

In order to install nodetop, perform the following tasks:
1. Install rust: https://www.rust-lang.org/tools/install
2. Clone this repository
3. `cd nodetop`
4. Build the nodetop utility: `cargo build --release`

The nodetop utility will be in the `target/release` directory.

Note: install on centos: when installing on centos, the following packages should be installed prior to building nodetop:
- cmake
- gcc-c++
- freetype-devel
- expat-devel
- open-sans-fonts
- fontconfig
- openssl-devel  
  
Yum command for quick install:  
`yum install -y cmake gcc-c++ freetype-devel expat-devel open-sans-fonts fontconfig openssl-devel`