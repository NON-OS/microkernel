# Test vectors

`nitrokey-udev-rules-1.0.0-r2.apk` is Alpine v3.20's community package, byte
for byte, licensed CC0-1.0. `nitrokey-udev-rules-1.0.0-r2.json` holds its
record (P, V, C, S, L) as read from the community APKINDEX on 2026-10-24,
after that index's signature verified under a key pinned in
userland/capsule_linux/keys/alpine/. The package's own first member is an
Alpine RSA signature over its control member, which test_auth.py checks
with rsa.py, so the verifier is tested against a real signature, not one
made here.
