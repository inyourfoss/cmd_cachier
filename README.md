# Documentation

[Manpage](https://gitlab.com/inyourfoss/cmd_cachier/-/blob/main/docs/cmd_cachier.1.pdf?ref_type=heads)

# Install 
## Dependencies
### Runtime
- `redis`
### Build
- `cargo`
  - `colored` library (automatically pulled by cargo)
  - `redis-rs` library (automatically pulled by cargo)
  - `Command` library (automatically pulled by cargo)
- `make` or `gnumake` usually already installed
### Optional
- `man` usually already installed
- `groff` usually already installed

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
## Usage
See manpage [here](https://gitlab.com/inyourfoss/cmd_cachier/-/blob/main/docs/cmd_cachier.1.pdf?ref_type=heads))

## Setting up Redis
### Note
As of Version 0.3.0 redis does need further setup apart from installing it.
`cmd_cachier` can now spawn its own redis process wich is user specific.
It will launch the redis instance in the background 
the first time you use `cmd_cachier <ANY_COMMAND>` if it is not already running.

**Important:** the cache is **not** persistent accross booting.

### Installation
#### **RHEL/Fedora**
```
sudo dnf install redis
```

#### **Debian/Ubuntu**
```
sudo apt install redis
```

#### **MacOS**
```
brew install redis
```

