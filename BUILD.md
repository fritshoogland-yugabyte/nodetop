# build from source
In order to build nodetop from source using the rust compiler, the following RPMs need to be installed on Centos 7:
- cmake
- gcc-c++
- freetype-devel
- expat-devel
- open-sans-fonts
- fontconfig
- openssl-devel  

Yum command for quick install:  
`yum install -y cmake gcc-c++ freetype-devel expat-devel open-sans-fonts fontconfig openssl-devel`

# generate rpm on centos 7
In order to create an RPM file on Centos 7, when a rust environment with Cargo is installed:

Add cargo-generate-rpm:
```
cargo install cargo-generate-rpm
```

Steps to build an RPM:
1. Build an executable:
```
cargo build --release
```
2. Strip debug symbols:
```
strip -s target/release/nodetop
```
3. Generate RPM
```
cargo generate-rpm
```

The RPM will be available in the `target/generate-rpm` directory.