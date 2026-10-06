# jq 1.8.2, with the oniguruma regular expression library its release tarball
# bundles (vendor/oniguruma) built in, and decNumber for exact numbers.
#   OUT/jq
# The hash pinned is the one jqlang publishes in the release's sha256sum.txt.
tar -xzf "$(fetch jq)" -C "$work"
(cd "$work/jq-1.8.2" &&
	CC="$CC" AR="$AR" RANLIB="$RANLIB" CFLAGS="$CFLAGS" LDFLAGS="$LDFLAGS" \
		./configure --host=x86_64-linux-musl --build="$(sh config/config.guess)" --prefix=/usr \
		--with-oniguruma=builtin --enable-all-static --disable-shared --disable-docs \
		--disable-valgrind --disable-maintainer-mode >/dev/null &&
	# jq --build-configuration prints configure's command line, which names
	# this machine's compiler and work directory; it says what was chosen.
	echo '#define JQ_CONFIG "--host=x86_64-linux-musl --prefix=/usr --with-oniguruma=builtin --enable-all-static (NONOS, zig cc, static non-PIE musl)"' \
		>src/config_opts.inc &&
	make -j8 >/dev/null)
cp "$work/jq-1.8.2/jq" "$out/jq"
