#!/bin/sh
# The host oracle for bbsuite: the same busybox, reached through the same
# links the guest's tree has, with the guest's environment and nothing of
# the host's own tools on the path. Prints the output bbsuite must match.
# usage: bbsuite-host.sh <busybox> <body.sh>
set -e
bb=$(realpath "$1"); body=$(realpath "$2")
h=$(mktemp -d /dev/shm/bbsuite-host.XXXXXX)
trap 'rm -rf "$h"' EXIT
mkdir -p "$h/bin"
cp "$bb" "$h/bin/busybox"
"$bb" --list-full | grep -v '^bin/busybox$' | while read -r p; do
	mkdir -p "$h/$(dirname "$p")"
	ln -s "$h/bin/busybox" "$h/$p"
done
cd /
env -i PATH="$h/usr/local/bin:$h/usr/bin:$h/bin:$h/usr/local/sbin:$h/usr/sbin:$h/sbin" \
	HOME=/root TERM=linux SHELL=/bin/sh LANG=C.UTF-8 BBSUITE_DIR="$h/work" \
	"$h/bin/sh" "$body"
