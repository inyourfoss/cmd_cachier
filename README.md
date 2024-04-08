
`cmd_cachier` is a very fast caching tool for your cli commands.
It is intended to save output of cli commands like find or fd.
My goal was to make it feel instant once you pair it with a fuzzy finder like `fzf`.


It is basically a light wrapper around redis (an in-memory database) where the command itself is the key and the output is the value.

# Documentation

[Manpage](https://cmd-cachier-inyourfoss-7e6ece76159a67277db29221cf93ca6439e1ff91.gitlab.io/)

# Announcements
## About: Redis license change
As redis has changed it's license and will not be included in most popular linux distros anymore.
I have been testing out KeyDB and ValKey which are forks and/or are redis compatible.
I have found that redis-rs still works well with either of these backends so once these packages start hitting official repos I will start supporting one or both of these backends.

## 0.3 Release

This release features a configureless usage. You only have to make sure redis is installed.
Before it would run redis on the default redis port and it would be the system wide redis process. 
Which could cause conflicts if your redis configuration differs from what cmd_cachier expects.

Now every user has their own redis process. 

That process is spawned once you use the cmd_cachier command the first time since reboot. From then on it keeps running in the background.

Also now the redis instance listens on a unix domain socket instead of TCP(I think?). 
The socket is in a user owned directory and is only readable by the user.
Which should increase security quite a bit. 
But still, please don't put your passwords in the cache!

I am happy how this release has turned out and hope you like it too!
Make sure to see the examples in the [docs](https://cmd-cachier-inyourfoss-7e6ece76159a67277db29221cf93ca6439e1ff91.gitlab.io/)

My thanks go out to the redis team and the contributors behind [redis-rs](https://crates.io/teams/github:redis-rs:release-team)!

# Install 
## Dependencies
### Runtime
- `redis`
### Build (cmd_cachier)
- `cargo` with rustc (oldest tested version is rustc 1.68)
  - `colored` 2.0.4 library (automatically pulled by cargo)
  - `redis-rs` library (automatically pulled by cargo)
  - `Command` library (automatically pulled by cargo)
- `make` or `gnumake` usually already installed
### Build (docs)
- `asciidoctor` - the manpage is authored in asciidoc
- `python` 3.11 or greater - for some templating (versioning, dates, etc.)
### Optional
- `man` usually already installed

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
See manpage [here](https://cmd-cachier-inyourfoss-7e6ece76159a67277db29221cf93ca6439e1ff91.gitlab.io/)

## Setting up Redis
### Note
As of Version 0.3.0 redis does **not** need further setup apart from installing it.
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

