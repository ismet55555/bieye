# Debian Package

## Install Tooling

- `sudo apt-get install devscripts debhelper`
- `sudo apt-get install rustc cargo dh-cargo`
- `cargo install cargo-deb`

## Packaging Work

```bash
# Update changelog
dch -i

# Build the debian package
cd debian
debuild -S 

# Uploade to debian PPA repository
dput ppa:ismet55555/ppa ../bieye_<VERSION>_source.changes
```
