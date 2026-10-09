# Lua 5.4.9, the upstream posix target.
#   OUT/lua
tarball=$(fetch lua)
tar -xzf "$tarball" -C "$work"
make -s -C "$work/lua-5.4.9/src" posix CC="$CC" AR="$AR rc" RANLIB="$RANLIB" \
	MYCFLAGS="$CFLAGS" MYLDFLAGS="$LDFLAGS" >/dev/null
cp "$work/lua-5.4.9/src/lua" "$out/lua"
