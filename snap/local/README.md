# Snapcraft

## Tooling

```bash
sudo apt update
sudo apt install snapcraft
sudo iptables -I DOCKER-USER  -j ACCEPT
```

## Packaging

```bash
# Ensure snapcraft.yaml looks good and is updated

# Build snap in root of project
snapcraft --verbose

# Test locally by installing
sudo snap install "bieye_<VERSION>_<ARCH>.snap" --devmode --dangerous
snap info bieye
bieye --help

# Remove locally installed snap
sudo snap remove bieye
```

## Publishing

```bash
# Ensure snapcraft has build a .snap file

# Login snapcraft
snapcraft login

# Upload the snap to the "edge" channel
snapcraft upload --verbose --release=edge bieye_*.snap

# Check on the snap
snapcraft list-revisions bieye
snapcraft status bieye

# Try the "edge" version
sudo snap install --edge bieye
bieye --help
sudo snap remove bieye

# If all looks good, promote "edge" to "stable" channel
snapcraft promote bieye --from-channel=edge --to-channel=stable
```
