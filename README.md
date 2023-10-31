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
```
$ git clone https://gitlab.com/inyourfoss/cmd_cachier.git
$ cd cmd_cachier 
$ make build
# make systeminstall # Or
$ make install # manpage won't work on non-systeminstall
```

