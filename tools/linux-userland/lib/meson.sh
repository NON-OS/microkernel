# Building meson projects for the guest, for a recipe that sources this file
# (foot.sh). meson runs from its pinned release tarball on the build host's
# python3, and ninja, pkg-config and python3 come from the build host.
#
# Each target library is configured for /usr, so the paths compiled into it
# are the guest's, and installed under DEPS/usr. Its pkg-config files then
# say prefix=/usr, which pc_fix points at DEPS/usr so the next build finds
# the headers and archives there; nothing it compiles in changes.

# unpack NAME: the pinned source NAME, extracted on its own, as its one
# top-level directory.
unpack() {
	mkdir -p "$work/src/$1"
	tar -xf "$(fetch "$1")" -C "$work/src/$1"
	set -- "$1" "$work/src/$1"/*
	[ $# -eq 2 ] && [ -d "$2" ] || { echo "nonos-linux-userland-build: $1 is not one directory" >&3; exit 1; }
	echo "$2"
}

# words ARG...: each argument as a quoted meson array element.
words() {
	for w; do printf "'%s', " "$w"; done
}

# meson_setup: meson itself, and the cross file the target builds read
# (zig for static x86-64 musl, pkg-config limited to DEPS) and the native
# file the build-host passes read (pkg-config limited to HOSTDEPS).
meson_setup() {
	# meson takes an exported CC or CFLAGS for the build machine; the guest's
	# are in the cross file, and the host passes use the host's own.
	meson="env -u CC -u CXX -u AR -u RANLIB -u CFLAGS -u CPPFLAGS -u LDFLAGS python3 $(unpack meson)/meson.py"
	cat >"$work/cross.ini" <<-EOF
		[binaries]
		c = [$(words $zig cc -target x86_64-linux-musl)]
		ar = [$(words $zig ar)]
		ranlib = [$(words $zig ranlib)]
		pkg-config = 'pkg-config'
		[properties]
		needs_exe_wrapper = false
		pkg_config_libdir = [$(words "$deps/usr/lib/pkgconfig" "$deps/usr/share/pkgconfig")]
		[built-in options]
		c_args = [$(words $CFLAGS -ffunction-sections -fdata-sections)]
		c_link_args = [$(words $LDFLAGS -Wl,--gc-sections)]
		[host_machine]
		system = 'linux'
		cpu_family = 'x86_64'
		cpu = 'x86_64'
		endian = 'little'
	EOF
	cat >"$work/native.ini" <<-EOF
		[properties]
		pkg_config_libdir = [$(words "$hostdeps/lib/pkgconfig" "$hostdeps/share/pkgconfig")]
	EOF
}

# meson_target SRC [OPTION...]: SRC built static for the guest, installed
# under DEPS/usr. No meson subproject may download anything.
meson_target() {
	b="$1.target" && src=$1 && shift
	$meson setup "$b" "$src" --cross-file "$work/cross.ini" --native-file "$work/native.ini" \
		--prefix=/usr --libdir=lib --sysconfdir=/etc --localstatedir=/var --buildtype=plain \
		--default-library=static --prefer-static --wrap-mode=nodownload \
		-Db_pie=false -Db_staticpic=false "$@" >/dev/null
	ninja -C "$b" -j"$jobs" >/dev/null
	$meson install -C "$b" --no-rebuild --destdir "$deps" >/dev/null
	pc_fix
}

pc_fix() {
	for pc in "$deps"/usr/lib/pkgconfig/*.pc "$deps"/usr/share/pkgconfig/*.pc; do
		[ -f "$pc" ] || continue
		sed -i.orig "s|^prefix=/usr\$|prefix=$deps/usr|" "$pc" && rm -f "$pc.orig"
	done
}
