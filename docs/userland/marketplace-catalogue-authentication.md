# Catalogue authentication

How `nonos-market-catalogue --catalogue` decides that Alpine published the bytes it lists. Part of [The marketplace catalogue](marketplace-catalogue.md).

Nothing is listed until the generator has checked it as the device checks it, against the three Alpine keys the device pins in `userland/capsule_linux/keys/alpine/`:

1. Each branch's APKINDEX must be exactly two gzip members, the second signed by the first under a pinned key (`alpine_index.py`, as `install/auth/signature.rs`). An index that fails is dropped from the cache and stops the run.
2. Each package is fetched to the size the signed index gives.
3. Its control member must hash (SHA-1) to the index's `C:`, and its data, from the first data member to the end, must hash (SHA-256) to the control's `datahash` (`alpine_auth.py`, as `install/auth/package.rs`).
4. Only the data members are read for the program.

The gzip and tar readers copy the installer's own:

- at most 64 members and 128 MiB inflated, with output bounded before it is made (`gz.py`);
- the tar walk of `install/tar.rs`, with `./` cut, pax and GNU long names honoured, and no checksum read (`tarwalk.py`).

RSA verification is PKCS#1 v1.5, in plain Python (`rsa.py`).

A package that fails any step is left out and its cached copy dropped, so the next run fetches it again. The test vector in `tools/nonos_market_catalogue/vectors/` is a real Alpine package, so the chain is tested against Alpine's own signature.
