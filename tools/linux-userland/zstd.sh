# Zstandard 1.5.7, the zstd program alone, with no zlib, LZMA or LZ4.
#   OUT/zstd
tarball=$(fetch zstd)
tar -xzf "$tarball" -C "$work"
make -s -C "$work/zstd-1.5.7/programs" zstd CC="$CC" AR="$AR" CFLAGS="$CFLAGS" \
	LDFLAGS="$LDFLAGS" HAVE_ZLIB=0 HAVE_LZMA=0 HAVE_LZ4=0 >/dev/null
cp "$work/zstd-1.5.7/programs/zstd" "$out/zstd"
