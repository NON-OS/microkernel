# Catalogue authentication

How `nonos-market-catalogue --catalogue` decides that Alpine published the bytes it lists. Part of [The marketplace catalogue](marketplace-catalogue.md).

Nothing is listed until the generator has checked it as the device checks it, against the three Alpine keys the device pins in `userland/capsule_linux/keys/alpine/`:

1. Each branch's APKINDEX is fetched fresh every run. It must be exactly two gzip members, the second signed by the first under a pinned key (`alpine_index.py`, as `install/auth/signature.rs`); an index that fails stops the run. The release and arch must be the device's (`RELEASE`, `ARCH` in `install/run.rs`, pinned by a test).
2. The records are read as the device reads them (`apkindex.py`, as `install/index.rs`): every APKINDEX entry of main, then community, joined; lines as Rust's `str::lines`; a record only with a name and a version. The listed package is the first of its own name, never a package that only provides it (`Index::listed`).
3. Each package is fetched from the first branch that serves it, as `install/get/package.rs` does, held to its signed size, with a deadline on the whole download (`sources.py`, `fetch.py`).
4. Its control member must hash (SHA-1) to the index's `C:`, and its data, from the first data member to the end, must hash (SHA-256) to the control's `datahash` (`alpine_auth.py`, as `install/auth/package.rs`).
5. Only the data members are read for the program, which must be at most the 64 MiB the device binds (`MAX_IMAGE`, pinned).

The gzip and tar readers copy the installer's own:

- at most 64 members and 128 MiB inflated, with output bounded before it is made (`gz.py`);
- the tar walk of `install/tar.rs`, with `./` cut, pax and GNU long names honoured, and no checksum read (`tarwalk.py`).

RSA verification is PKCS#1 v1.5, in plain Python (`rsa.py`), on strict DER (`rsa_key.py`), with the bounds the device's RSA takes: 2048 to 4096 bits, an odd exponent from 3 to 2^33. `.PKGINFO` is read with Rust's `lines` and `trim` (`rust_text.py`).

A package that fails any step stops the run by name, so a seal never signs a catalogue with an app silently missing. Its cached copy is dropped, so the next run fetches it again. The test vector in `tools/nonos_market_catalogue/vectors/` is a real Alpine package, so the chain is tested against Alpine's own signature.
