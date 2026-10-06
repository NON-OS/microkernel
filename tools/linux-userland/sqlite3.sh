# The SQLite 3.53.4 command line shell: shell.c and the amalgamation from the
# release tarball the python recipe builds its library from, with the same
# full-text search (FTS4, FTS5), R*Tree and dbstat, and line editing through
# readline.
#   OUT/sqlite3
# SQLite publishes a SHA3-256, not a sha256; the hash pinned is of the tarball
# whose SHA3-256 is the published
# 454e45f61c6bd75b7420e7190732dea03ce6639c63ada47bbc592f67fc340338.
tar -xzf "$(fetch sqlite)" -C "$work"
deps="$work/deps"
. "$root/tools/linux-userland/lib/terminal.sh"
libcflags="-Os -g0 -fno-pie -ffunction-sections -fdata-sections -ffile-prefix-map=$work=/build"
ncurses_build "$libcflags" "$deps"
readline_build "$libcflags" "$deps"
# No extension loading: the personality maps no file it has not proved.
s="$work/sqlite-autoconf-3530400"
$CC $CFLAGS -ffunction-sections -fdata-sections -I"$s" -I"$deps/usr/include" \
	-DSQLITE_ENABLE_FTS4 -DSQLITE_ENABLE_FTS5 -DSQLITE_ENABLE_RTREE \
	-DSQLITE_ENABLE_DBSTAT_VTAB -DSQLITE_ENABLE_MATH_FUNCTIONS -DSQLITE_OMIT_LOAD_EXTENSION \
	-DHAVE_READLINE=1 \
	"$s/shell.c" "$s/sqlite3.c" -o "$out/sqlite3" \
	$LDFLAGS -Wl,--gc-sections -L"$deps/usr/lib" -lreadline -lncursesw
