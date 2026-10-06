#!/bin/sh
# The host oracle for a proof guest: the program run as the personality runs
# it. Root with no capability (capget is refused to a guest), in a pid
# namespace of its own with its own /proc (a guest sees only its family),
# a tmpfs at /tmp (the family's private /tmp), no terminal, and the
# guest's environment. usage: oracle.sh <program> [args...]
set -e
prog=$(realpath "$1"); shift
h=$(mktemp -d /dev/shm/oracle.XXXXXX)
trap 'rm -rf "$h"' EXIT
mkdir -p "$h/bin" && cp "$prog" "$h/bin/"
name=$(basename "$prog")
exec setsid --wait unshare --pid --fork --mount-proc --mount sh -c '
	mount -t tmpfs tmpfs /tmp && cd / &&
	exec env -i PATH=/usr/local/bin:/usr/bin:/bin:/usr/local/sbin:/usr/sbin:/sbin \
		HOME=/root TERM=linux PWD=/ SHELL=/bin/sh LANG=C.UTF-8 \
		setpriv --bounding-set=-all --inh-caps=-all --ambient-caps=-all \
		--securebits=+noroot,+noroot_locked,+no_setuid_fixup,+no_setuid_fixup_locked \
		"$0" "$@" < /dev/null' "$h/bin/$name" "$@"
