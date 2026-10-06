# GNU nano 9.2, the small text editor, on ncurses with its terminal
# descriptions compiled in (tools/linux-userland/lib/terminal.sh).
#   OUT/nano
# GNU publishes a detached PGP signature, not a sha256; the hash pinned is of
# the tarball whose signature verifies under Benno Schulenberg's key,
# fingerprint 168E 6F42 97BF D7A7 9AFD 4496 514B BE2E B8E1 961F, in the GNU
# keyring.
tar -xJf "$(fetch nano)" -C "$work"
deps="$work/deps"
. "$root/tools/linux-userland/lib/terminal.sh"
ncurses_build "-Os -g0 -fno-pie -ffunction-sections -fdata-sections -ffile-prefix-map=$work=/build" "$deps"
# No spell checker, file type guessing or native language support: each would
# be another program or library to run or prove.
(cd "$work/nano-9.2" &&
	CC="$CC" AR="$AR" RANLIB="$RANLIB" CFLAGS="$CFLAGS -ffunction-sections -fdata-sections" \
		CPPFLAGS="-I$deps/usr/include" LDFLAGS="$LDFLAGS -Wl,--gc-sections -L$deps/usr/lib" \
		NCURSESW_CFLAGS="-I$deps/usr/include" NCURSESW_LIBS="-lncursesw" \
		./configure --host=x86_64-linux-musl --build="$(sh config.guess)" --prefix=/usr \
		--sysconfdir=/etc --disable-nls --disable-libmagic --disable-speller --enable-utf8 >/dev/null &&
	make -j8 >/dev/null)
cp "$work/nano-9.2/src/nano" "$out/nano"
