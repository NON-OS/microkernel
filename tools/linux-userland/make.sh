# GNU make 4.4.1.
#   OUT/make
# GNU publishes a detached PGP signature, not a sha256; the hash pinned is of
# the tarball whose signature verifies under Paul D. Smith's key, fingerprint
# B250 8A90 102F 8AE3 B12A 0090 DEAC CAAE DB78 137A, in the GNU keyring.
tar -xzf "$(fetch make)" -C "$work"
# No Guile and no native language support; no load directive, since the
# personality maps no object it has not proved.
(cd "$work/make-4.4.1" &&
	# configure's AC_PROG_CXX probes the build host for a C++ compiler and bakes
	# the name it finds into make's built-in $(CXX) (default.c, MAKE_CXX): a Linux
	# host gives g++, a macOS host gives c++, the one byte by which the same make
	# built on the two hosts differed. make itself is C only and never calls it,
	# so pin the name to the Linux default the target expects, the same on either
	# host. config.guess feeds --build only, which configure does not bake.
	CC="$CC" CXX=g++ AR="$AR" RANLIB="$RANLIB" CFLAGS="$CFLAGS" LDFLAGS="$LDFLAGS" \
		./configure --host=x86_64-linux-musl --build="$(sh build-aux/config.guess)" --prefix=/usr \
		--disable-nls --without-guile --disable-load >/dev/null &&
	make -j8 >/dev/null)
cp "$work/make-4.4.1/make" "$out/make"
