# foot's Wayland side, for foot.sh: libwayland and the protocols it speaks,
# and libxkbcommon. Needs lib/meson.sh sourced and meson_setup run first.
#
# A cross build of libwayland runs a wayland-scanner of the same release on
# the build host, so the pinned wayland is built twice: once for the host
# with only its scanner, on a host build of the pinned expat, then for the
# guest with only its libraries.

# An autotools source for the guest, static, installed under DEPS/usr.
# configure guesses the build machine itself.
autotools_target() {
	src=$1 && shift
	(cd "$src" &&
		CC="$CC" AR="$AR" RANLIB="$RANLIB" CFLAGS="$CFLAGS -ffunction-sections -fdata-sections" \
			./configure --host=x86_64-linux-musl --prefix=/usr --libdir=/usr/lib \
			--disable-shared --enable-static "$@" >/dev/null &&
		make -j"$jobs" >/dev/null && make install DESTDIR="$deps" >/dev/null)
	pc_fix
}

wayland_build() {
	ex=$(unpack expat)
	# The host's copy is built out of tree by the host's own compiler; CC
	# and the flags above are the guest's.
	mkdir "$ex.host"
	(cd "$ex.host" && unset CC CXX AR RANLIB CFLAGS LDFLAGS &&
		"$ex/configure" --prefix="$hostdeps" --libdir="$hostdeps/lib" --disable-shared \
			--without-docbook --without-examples --without-tests --without-xmlwf >/dev/null &&
		make -j"$jobs" >/dev/null && make install >/dev/null)
	autotools_target "$ex" --without-docbook --without-examples --without-tests --without-xmlwf
	autotools_target "$(unpack libffi)" --disable-docs --disable-multi-os-directory
	wl=$(unpack wayland)
	(unset CC CXX AR RANLIB CFLAGS LDFLAGS &&
		PKG_CONFIG_LIBDIR="$hostdeps/lib/pkgconfig" $meson setup "$wl.host" "$wl" \
			--prefix="$hostdeps" --libdir=lib --buildtype=plain --wrap-mode=nodownload \
			-Dlibraries=false -Dscanner=true -Ddocumentation=false -Ddtd_validation=false \
			-Dtests=false >/dev/null &&
		ninja -C "$wl.host" -j"$jobs" >/dev/null && $meson install -C "$wl.host" --no-rebuild >/dev/null)
	meson_target "$wl" -Dlibraries=true -Dscanner=false -Ddocumentation=false \
		-Ddtd_validation=false -Dtests=false
	meson_target "$(unpack wayland-protocols)" -Dtests=false
	# The keymap comes from the server as a file (capsule_linux
	# wayland/keymap.rs), so nothing reads xkeyboard-config's data.
	meson_target "$(unpack libxkbcommon)" -Denable-x11=false -Denable-wayland=false \
		-Denable-tools=false -Denable-docs=false -Denable-xkbregistry=false \
		-Denable-bash-completion=false
}
