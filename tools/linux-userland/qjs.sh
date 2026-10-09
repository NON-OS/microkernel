# QuickJS 2026-06-04, Fabrice Bellard's JavaScript engine: the qjs
# interpreter with its REPL and the std and os modules.
#   OUT/qjs
# Bellard publishes the tarball with no digest or signature; the hash pinned
# is of the bytes bellard.org served, whose sources are byte for byte those of
# commit 3d5e064e9dd67c70f7962836505a7fa067bf0a4e ("new release") in
# github.com/bellard/quickjs; the tarball adds only the built manual.
tar -xJf "$(fetch quickjs)" -C "$work"
q="$work/quickjs-2026-06-04"
# The REPL is JavaScript compiled to C by qjsc, so a qjsc runs on the build
# machine first, built by the build machine's compiler. qjsc records the
# file name it is given, so it is given the bare name, as upstream's
# Makefile does.
make -s -C "$q" CROSS_PREFIX= CONFIG_LTO= qjsc >/dev/null
(cd "$q" && ./qjsc -s -c -o repl.c -m repl.js)
defs="-D_GNU_SOURCE -DCONFIG_VERSION=\"$(cat "$q/VERSION")\""
(cd "$q" && $CC $CFLAGS -fwrapv -funsigned-char $defs -o "$out/qjs" \
	qjs.c repl.c quickjs.c dtoa.c libregexp.c libunicode.c cutils.c quickjs-libc.c \
	$LDFLAGS -lm -lpthread)
