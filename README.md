# Documentation

[Manpage](https://gitlab.com/inyourfoss/cmd_cachier/-/blob/main/docs/cmd_cachier.1.pdf?ref_type=heads)

# Install 
## Dependencies
- `redis`
- `cargo`
  - `colored` library (automatically pulled by cargo)
  - `redis-rs` library (automatically pulled by cargo)
  - `Command` library (automatically pulled by cargo)
- `groff` usually already installed
- `man` usually already installed
- `make` or `gnumake` usually already installed

## Via GitLab
Clone repo:
```
git clone https://gitlab.com/inyourfoss/cmd_cachier.git
```
Move into repo directory:
```
cd cmd_cachier 
```
Build project:
```
make build
```
Execute installation:
```
sudo make systeminstall # Or
```
Or:
```
make install # manpage won't work on non-systeminstall
```

## Via crates.io
As of now the man page will not work as it is only a user space install. 
```
cargo install cmd_cachier
```

## Setting up Redis (for more see [manpage](https://gitlab.com/inyourfoss/cmd_cachier/-/blob/main/docs/cmd_cachier.1.pdf?ref_type=heads))
### Example: Fedora
Install:
```
sudo dnf install redis
```
Start and enable on boot
```
systemctl enable --now redis
```
### Example: MacOS
Install:
```
brew install redis
```
Start and enable on boot
```
brew services start redis
```
