# Debian Package

## Tooling

```bash
sudo apt-get install devscripts debhelper
sudo apt-get install rustc cargo dh-cargo

# To create .deb package in target/debian
cargo install cargo-deb
```

## Packaging

```bash
# Update changelog
dch -i

# Build the debian package
cd debian
debuild -S
```

## Publishin

```bash
# Uploade to debian PPA repository
dput ppa:ismet55555/ppa ../bieye_<VERSION>_source.changes
```
