# The OpenSSL 3.5.9 command line tool, openssl, from the release the python
# recipe builds its ssl module against.
#   OUT/openssl
# OpenSSL publishes a .sha256 beside each release; the hash pinned is that
# one.
tar -xzf "$(fetch openssl)" -C "$work"
# As in the python recipe: OpenSSL compiles the command that built it and
# the day into itself (openssl version -a), so zig is named from PATH, the
# same on every machine, and the day is the release day (1790726400 is 30
# September 2026). Its directory is /etc/ssl, where the store keeps the CA
# bundle, so s_client verifies against /etc/ssl/cert.pem. Static, with no
# loadable modules or engines, since the personality maps no object it has
# not proved; every command and algorithm of the default build stays.
(cd "$work/openssl-3.5.9" &&
	PATH="$(dirname "$zig"):$PATH" SOURCE_DATE_EPOCH=1790726400 &&
	export PATH SOURCE_DATE_EPOCH &&
	CC="zig cc -target x86_64-linux-musl" AR="zig ar" RANLIB="zig ranlib" \
		CFLAGS="-O2 -g0 -fno-pie -ffunction-sections -fdata-sections" \
		LDFLAGS="-static -no-pie -s -Wl,--gc-sections" \
		perl ./Configure linux-x86_64 --prefix=/usr --openssldir=/etc/ssl --libdir=lib \
		no-shared no-module no-tests no-docs no-engine no-dso >/dev/null &&
	make -j8 build_programs >/dev/null)
cp "$work/openssl-3.5.9/apps/openssl" "$out/openssl"
