# The terminal libraries a recipe links when its program draws a screen or
# edits a line: ncurses, and readline on top of it. A recipe sources this file
# and calls the functions it needs; each installs a static library and its
# headers under DEST/usr.
#
# ncurses and readline publish detached PGP signatures, not sha256s; the
# hashes pinned for them and for readline's six patches are of the files
# whose signatures verify under Thomas E. Dickey's key, fingerprint 1988 2D92
# DDA4 C400 C22C 0D56 CC2A F447 2167 BE03, and Chet Ramey's, 7C01 35FB 088A
# AF6C 66C6 50B9 BB58 69F0 64EA 74AB, both in the GNU keyring.

# ncurses_build CFLAGS DEST: ncurses with its terminal descriptions compiled
# in, so a program reads no terminfo files at run time. Writing them takes a
# tic and an infocmp of the same release that run on the build machine.
ncurses_build() {
	tar -xzf "$(fetch ncurses)" -C "$work"
	mkdir "$work/ncurses-host"
	(cd "$work/ncurses-host" &&
		../ncurses-6.6/configure --without-shared --without-debug --without-ada --without-cxx \
			--without-cxx-binding --without-manpages --without-tests --enable-widec >/dev/null &&
		make -C include >/dev/null && make -C ncurses -j8 >/dev/null &&
		make -C progs -j8 tic infocmp >/dev/null)
	(cd "$work/ncurses-6.6" &&
		CC="$CC" AR="$AR" RANLIB="$RANLIB" CFLAGS="$1" \
			./configure --host=x86_64-linux-musl --build="$(sh ./config.guess)" --prefix=/usr \
			--without-shared --without-debug --without-ada --without-cxx --without-cxx-binding \
			--without-manpages --without-progs --without-tests --without-gpm --enable-widec \
			--enable-overwrite --disable-database --with-fallbacks=xterm-256color,xterm,vt100,linux \
			--with-tic-path="$work/ncurses-host/progs/tic" \
			--with-infocmp-path="$work/ncurses-host/progs/infocmp" >/dev/null &&
		make -j8 >/dev/null && make install DESTDIR="$2" >/dev/null)
}

# readline_build CFLAGS DEST: readline with each patch GNU has issued for
# 8.3, applied in order, against the ncurses ncurses_build put in DEST.
readline_build() {
	tar -xzf "$(fetch readline)" -C "$work"
	for n in 001 002 003 004 005 006; do
		patch -s -p0 -d "$work/readline-8.3" <"$(fetch "readline83-$n")"
	done
	(cd "$work/readline-8.3" &&
		CC="$CC" AR="$AR" RANLIB="$RANLIB" CFLAGS="$1" \
			CPPFLAGS="-I$2/usr/include" LDFLAGS="-L$2/usr/lib" \
			./configure --host=x86_64-linux-musl --build="$(sh support/config.guess)" --prefix=/usr \
			--disable-shared --with-curses --disable-install-examples >/dev/null &&
		make -j8 >/dev/null && make install-static DESTDIR="$2" >/dev/null)
}
