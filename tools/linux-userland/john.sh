# John the Ripper 1.9.0 core, the password auditing tool NONOS ships, with
# its config and the bundled word list.
#   OUT/john, OUT/john.conf, OUT/password.lst
# Openwall publishes a detached PGP signature, not a sha256; the hash
# pinned here is of the .tar.xz whose signature verifies under the
# Openwall offline signing key, fingerprint 297A D21C F86C 9480 8152
# 0C18 05C0 27FD 4BDC 136E (www.openwall.com/signatures).
tarball=$(fetch john)
tar -xJf "$tarball" -C "$work"
j="$work/john-1.9.0"
# JOHN_SYSTEMWIDE puts its config and data under /usr/share/john, where
# the store places them; the home directory holds only a session's pot.
make -s -C "$j/src" linux-x86-64 CC="$CC" AS="$CC" LD="$CC" \
	OMPFLAGS="-g0 -fno-pie -DJOHN_SYSTEMWIDE=1 -ffile-prefix-map=$work=/build" \
	LDFLAGS="$LDFLAGS" >/dev/null
cp "$j/run/john" "$out/john"
cp "$j/run/john.conf" "$out/john.conf"
cp "$j/run/password.lst" "$out/password.lst"
