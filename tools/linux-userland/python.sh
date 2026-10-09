# CPython 3.12.15, every stdlib C module it can build linked in (with
# OpenSSL, SQLite, readline, ncurses, libffi, bzip2, XZ, zlib and libuuid),
# the stdlib as a zip of sources and bytecode, and the CA bundle its ssl
# module trusts.
#   OUT/python3, OUT/python312.zip, OUT/cacert.pem
py=$(fetch python)
zl=$(fetch zlib)
# The libraries behind the stdlib modules that need one. OpenSSL 3.5, the
# long-term support series, publishes a .sha256 beside each release, and
# util-linux a signed list of them; libffi and XZ Utils release on
# GitHub, which lists each file's sha256 (XZ's tarball also verifies
# under Lasse Collin's key, 3690 C240 CE51 B467 0D30 AD1C 38EE 757D 6918
# 4620).
ssl=$(fetch openssl)
uu=$(fetch util-linux)
ffi=$(fetch libffi)
xz=$(fetch xz)
# SQLite publishes a SHA3-256, not a sha256; the hash pinned here is of
# the tarball whose SHA3-256 is the published
# 454e45f61c6bd75b7420e7190732dea03ce6639c63ada47bbc592f67fc340338.
sq=$(fetch sqlite)
# bzip2 publishes SHA-512 sums (sha512.sum beside the release) and a PGP
# signature; the hash pinned here is of the tarball that matches its
# SHA-512 and verifies under Mark Wielaard's key, fingerprint EC3C FE88
# F6CA 0788 774F 5C1D 1AA4 4BE6 49DE 760A.
bz=$(fetch bzip2)
# Build parallelism, bounded by the cores nix allocated and by memory. The
# interpreter's frozen-module source (Python/deepfreeze/deepfreeze.c, ~144k
# generated lines) and the module compiles are memory-heavy, so a forced -j"$jobs"
# makes eight clang processes overrun a small builder (a 7 GB macOS CI runner)
# and the kernel kills one mid-compile, which reads only as "builder failed".
# Respect NIX_BUILD_CORES, fall back to the CPU count, and cap so each
# concurrent compile has roughly 2 GB. Parallelism changes scheduling, never
# output, so the reproducibility check stays byte-identical.
jobs="${NIX_BUILD_CORES:-0}"
[ "$jobs" -gt 0 ] 2>/dev/null || jobs="$(nproc 2>/dev/null || sysctl -n hw.logicalcpu 2>/dev/null || echo 4)"
# The `|| true` matters: under `set -e` a command substitution that fails aborts
# the script, and sed exits non-zero when /proc/meminfo is absent (every macOS).
_memkb="$(sed -n 's/^MemTotal:[[:space:]]*\([0-9]*\).*/\1/p' /proc/meminfo 2>/dev/null || true)"
[ -n "$_memkb" ] || _memkb=$(( $(sysctl -n hw.memsize 2>/dev/null || echo 0) / 1024 ))
if [ "${_memkb:-0}" -gt 0 ]; then
	_memcap=$(( _memkb / 1024 / 1024 / 2 ))
	[ "$_memcap" -lt 1 ] && _memcap=1
	[ "$jobs" -gt "$_memcap" ] && jobs="$_memcap"
fi
[ "${jobs:-0}" -ge 1 ] 2>/dev/null || jobs=1

# A cross build runs a Python of the same version on the build machine.
host="$root/target/toolchains/python-3.12.15-host"
if [ ! -x "$host/bin/python3.12" ]; then
	mkdir -p "$work/host"
	tar -xJf "$py" -C "$work/host"
	# It runs here, so it is built by the build machine's own compiler: CC,
	# CFLAGS and LDFLAGS above are the cross build's (zig for musl) and,
	# exported by a development shell, would reach this configure too, which
	# then finds headers that compiler cannot see. It only freezes modules for
	# the cross build and zips the standard library, which needs zlib alone,
	# so the modules that need other system libraries are left out rather
	# than taken from whatever the build machine has.
	(cd "$work/host/Python-3.12.15" && unset CC CXX AR RANLIB CFLAGS LDFLAGS &&
		for m in _bz2 _lzma readline _curses _curses_panel _dbm _gdbm _sqlite3 _ssl _hashlib _tkinter _uuid _ctypes; do
			export "py_cv_module_$m=n/a"
		done &&
		./configure --prefix="$host" --disable-test-modules --without-ensurepip >/dev/null &&
		make -j"$jobs" >/dev/null && make install >/dev/null)
fi
# The kernel proves the one binary whole before any page of it runs, and
# takes at most 16 MiB: each function and datum gets a section of its
# own, so the link keeps only what Python reaches, and no unwind tables
# are made, since nothing in a static interpreter walks them.
lean="-ffunction-sections -fdata-sections -fno-asynchronous-unwind-tables -fno-unwind-tables"
tar -xzf "$zl" -C "$work"
(cd "$work/zlib-1.3.2" &&
	CC="$CC" AR="$AR" RANLIB="$RANLIB" CFLAGS="$CFLAGS $lean" CHOST=x86_64-linux-musl \
		./configure --static --prefix="$work/zlib" >/dev/null &&
	make -j"$jobs" libz.a >/dev/null && make install >/dev/null)
# The other libraries are compiled for size too, which keeps the binary
# under 14 MB; the interpreter itself stays at -O2. Each is configured
# for /usr, so no path of this build is compiled in, and installed in one
# tree.
libopt="-Os -g0 -fno-pie $lean"
libcflags="$libopt -ffile-prefix-map=$work=/build"
deps="$work/deps"
mkdir -p "$deps/usr/include" "$deps/usr/lib"
for t in "$bz" "$xz" "$ffi" "$sq" "$ssl"; do tar -xzf "$t" -C "$work"; done
tar -xJf "$uu" -C "$work"
# bzip2's Makefile builds its programs and runs its tests with the
# library; the library's seven sources are all the module needs.
(cd "$work/bzip2-1.0.8" &&
	for f in blocksort huffman crctable randtable compress decompress bzlib; do
		$CC $libcflags -D_FILE_OFFSET_BITS=64 -c $f.c || exit 1
	done &&
	$AR rcs libbz2.a blocksort.o huffman.o crctable.o randtable.o compress.o decompress.o bzlib.o &&
	cp bzlib.h "$deps/usr/include" && cp libbz2.a "$deps/usr/lib")
(cd "$work/xz-5.8.4" &&
	CC="$CC" AR="$AR" RANLIB="$RANLIB" CFLAGS="$libcflags" \
		./configure --host=x86_64-linux-musl --build="$(sh build-aux/config.guess)" --prefix=/usr \
		--disable-shared --disable-xz --disable-xzdec --disable-lzmadec --disable-lzmainfo \
		--disable-lzma-links --disable-scripts --disable-doc --disable-nls >/dev/null &&
	make -C src/liblzma -j"$jobs" >/dev/null && make -C src/liblzma install DESTDIR="$deps" >/dev/null)
(cd "$work/libffi-3.8.0" &&
	CC="$CC" AR="$AR" RANLIB="$RANLIB" CFLAGS="$libcflags" \
		./configure --host=x86_64-linux-musl --build="$(sh ./config.guess)" --prefix=/usr \
		--disable-shared --disable-docs --disable-multi-os-directory >/dev/null &&
	make -j"$jobs" >/dev/null && make install DESTDIR="$deps" >/dev/null)
# Of util-linux, libuuid alone, keeping its clock under /var/lib/libuuid.
(cd "$work/util-linux-2.42.4" &&
	CC="$CC" AR="$AR" RANLIB="$RANLIB" CFLAGS="$libcflags" \
		./configure --host=x86_64-linux-musl --build="$(sh config/config.guess)" --prefix=/usr \
		--localstatedir=/var --disable-shared --disable-all-programs --enable-libuuid \
		--disable-nls >/dev/null &&
	make -j"$jobs" libuuid.la >/dev/null && mkdir -p "$deps/usr/include/uuid" &&
	cp libuuid/src/uuid.h "$deps/usr/include/uuid" && cp .libs/libuuid.a "$deps/usr/lib")
# Full-text search (FTS4, FTS5), R*Tree and dbstat beside the JSON and
# math functions SQLite has by default; no extension loading, since the
# personality maps no file it has not proved.
(cd "$work/sqlite-autoconf-3530400" &&
	CC="$CC" AR="$AR" RANLIB="$RANLIB" CFLAGS="$libcflags" \
		./configure --host=x86_64-linux-musl --prefix=/usr --disable-shared \
		--fts4 --fts5 --rtree --dbstat --disable-load-extension --disable-readline >/dev/null &&
	make libsqlite3.a >/dev/null && make install-lib install-headers DESTDIR="$deps" >/dev/null)
# ncurses with its terminal descriptions compiled in, so the REPL and
# curses read no terminfo files at run time, and readline on it.
. "$root/tools/linux-userland/lib/terminal.sh"
ncurses_build "$libcflags" "$deps"
readline_build "$libcflags" "$deps"
# OpenSSL compiles the command that built it and the day into itself
# (OpenSSL_version), so zig is named from PATH, the same on every
# machine, the day is the release day (1790726400 is 30 September 2026),
# and the in-tree build has no path to map. Its directory is /etc/ssl, so
# the trust store ssl.create_default_context() loads is
# /etc/ssl/cert.pem. For size, left out is what Python's ssl and hashlib
# never reach (QUIC, DTLS, CMS, CMP, OCSP, CT, SRP, time stamps, engines,
# compression) and the algorithms no TLS suite Python offers uses: SM2,
# SM3, SM4, ARIA, Camellia, SEED, IDEA, RC2, RC4, RC5, Blowfish, CAST,
# MD2, MDC2, Whirlpool, binary-field curves and SLH-DSA.
(cd "$work/openssl-3.5.9" &&
	PATH="$(dirname "$zig"):$PATH" SOURCE_DATE_EPOCH=1790726400 &&
	export PATH SOURCE_DATE_EPOCH &&
	CC="zig cc -target x86_64-linux-musl" AR="zig ar" RANLIB="zig ranlib" CFLAGS="$libopt" \
		perl ./Configure linux-x86_64 --prefix=/usr --openssldir=/etc/ssl --libdir=lib \
		no-shared no-module no-tests no-docs no-quic no-dtls no-srtp no-cms no-cmp \
		no-ocsp no-ct no-srp no-ts no-engine no-comp no-sm2 no-sm3 no-sm4 no-aria \
		no-camellia no-seed no-idea no-rc2 no-rc4 no-rc5 no-md2 no-mdc2 no-whirlpool \
		no-bf no-cast no-ec2m no-slh-dsa >/dev/null &&
	make -j"$jobs" build_libs >/dev/null && make install_dev DESTDIR="$deps" >/dev/null)
tar -xJf "$py" -C "$work"
src="$work/Python-3.12.15"
# What a cross configure cannot run a program to find out, and the panel
# library, whose check links it without the curses library a static one
# needs after it (the module's own link names both).
printf 'ac_cv_file__dev_ptmx=yes\nac_cv_file__dev_ptc=no\nac_cv_buggy_getaddrinfo=no\nac_cv_lib_panelw_update_panels=yes\n' >"$src/config.site"
# Every module statically into the one binary: a shared object is a
# second file to prove, and the personality maps nothing unproven.
# readline's link names ncurses after it, as a static link needs.
# Configure's account of which modules it builds goes to the log.
# configure records the pkg-config path of the environment, the build
# machine's, though it reads none (PKG_CONFIG=false).
build=$(sh "$src/config.guess")
(cd "$src" && unset PKG_CONFIG_PATH &&
	CONFIG_SITE="$src/config.site" MODULE_BUILDTYPE=static PKG_CONFIG=false \
	CC="$CC" CXX="$CXX" AR="$AR" RANLIB="$RANLIB" READELF=true \
	CFLAGS="$CFLAGS $lean" CPPFLAGS="-I$deps/usr/include" \
	LDFLAGS="$LDFLAGS -Wl,--gc-sections -L$deps/usr/lib" \
	ZLIB_CFLAGS="-I$work/zlib/include" ZLIB_LIBS="$work/zlib/lib/libz.a" \
	LIBREADLINE_LIBS="-lreadline -lncursesw" \
	./configure --host=x86_64-pc-linux-musl --build="$build" \
		--with-build-python="$host/bin/python3.12" --prefix=/usr \
		--disable-shared --disable-test-modules --without-ensurepip \
		--with-openssl="$deps/usr" --with-openssl-rpath=no >&2)
# A library configure did not take would leave its module out without a
# word, so the build stops instead.
for m in zlib _ssl _hashlib readline _curses _curses_panel _sqlite3 _ctypes _bz2 _lzma _uuid; do
	if ! grep -q "^MODULE_$(echo "$m" | tr a-z A-Z)_STATE=yes" "$src/Makefile"; then
		echo "nonos-linux-userland-build: CPython's configure leaves out $m" >&3
		exit 1
	fi
done
# The binary is python.exe on a disk that folds case, where python would
# be the Python/ directory, and python elsewhere: configure says which.
exe=python$(sed -n 's/^BUILDEXE=[[:space:]]*//p' "$src/Makefile")
# zig refuses __DATE__ and __TIME__; the build info names the release day,
# 30 September 2026.
make -s -C "$src" -j"$jobs" "$exe" pybuilddir.txt \
	CPPFLAGS="-DDATE='\"Sep 30 2026\"' -DTIME='\"00:00:00\"'" >/dev/null
cp "$src/$exe" "$out/python3"
# The sysconfig module records how this interpreter was configured, and with
# it two things of the build machine's: its triplet, and the store path of
# each tool configure found by PATH (zig, install, mkdir). Neither says
# anything of the interpreter, and both differ from one machine to the next,
# a Mac's most of all, so the triplet is recorded as an x86-64 Linux
# machine's and each tool by its name.
for f in "$src/$(cat "$src/pybuilddir.txt")"/_sysconfigdata_*.py; do
	sed -e "s|$build|x86_64-pc-linux-gnu|g" \
		-e "s|/nix/store/[0-9a-z]\{32\}-[^/ '\"]*/bin/||g" \
		-e "s|/nix/store/[0-9a-z]\{32\}-[^/ '\"]*/||g" "$f" >"$f.new"
	mv "$f.new" "$f"
done
"$host/bin/python3.12" "$root/tools/nonos-python-stdlib-zip" "$src" "$work" "$root" "$out/python312.zip"
# The CA bundle the ssl module verifies peers against, for the store to
# place where OpenSSL looks: /etc/ssl/cert.pem.
cp "$root/nonos-data/cacert.pem" "$out/cacert.pem"
