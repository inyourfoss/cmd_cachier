#!/bin/sh

socketfile="$XDG_RUNTIME_DIR/cmd_cachier.sock"
pidfile="$XDG_RUNTIME_DIR/cmd_cachier.pid"

[ ! -e $socketfile ] && memcached --unix-socket=$XDG_RUNTIME_DIR/cmd_cachier.sock --daemon --pidfile=$XDG_RUNTIME_DIR/cmd_cachier.pid || echo "Server already running."
