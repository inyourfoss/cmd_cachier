# Documentation

[Manpage](https://gitlab.com/inyourfoss/cmd_cachier/-/blob/main/docs/cmd_cachier.1.pdf?ref_type=heads)

# Install 
## Dependencies
- `cargo`
  - `colored` library (automatically pulled by cargo)
  - `redis-rs` library (automatically pulled by cargo)
  - `Command` library (automatically pulled by cargo)
- `groff` usually already installed
- `man` usually already installed
- `make` or `gnumake` usually already installed

## How to
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

