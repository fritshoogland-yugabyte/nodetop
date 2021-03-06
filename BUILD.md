# build from source
In order to build nodetop from source using the rust compiler, the following RPMs need to be installed:
(validated with Centos 7 and Alma 8)
- git
- cmake
- gcc-c++
- freetype-devel
- expat-devel
- open-sans-fonts
- fontconfig-devel
- openssl-devel  

Yum command for quick install of rpm packages:  
`yum install -y git cmake gcc-c++ freetype-devel expat-devel open-sans-fonts fontconfig-devel openssl-devel`

Install rust:
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
Clone this repository:
```
git clone https://github.com/fritshoogland-yugabyte/nodetop.git
```
Build nodetop in release mode:
```
cd nodetop
cargo build --release
```
The executable is in target/release.

# generate rpm
In order to create an RPM file on Centos 7, when a rust environment with Cargo is installed:

Add cargo-generate-rpm:
```
cargo install cargo-generate-rpm
```

Steps to build an RPM:
1. Build a release executable (see instructions above)
2. Strip debug symbols:
```
strip -s target/release/nodetop
```
3. Generate RPM  
Centos 7
```
cargo generate-rpm --auto-req auto --payload-compress gzip
```
Alma 8
The release flag must be set to reflect EL8 in Cargo.toml.
```
cargo generate-rpm --auto-req auto
```

The RPM will be available in the `target/generate-rpm` directory.