# linux_userland

The Linux programs a production NONOS image carries in its store, for the Linux personality
(`userland/capsule_linux`) to run beside its built-in BusyBox. `Userland.mk` builds, signs and
enrolls them; `mk/20-build.mk` includes it in every image. The handbook page is
[`docs/handbook/linux/userland-tools.md`](../../docs/handbook/linux/userland-tools.md).

| Program | Source | In the Linux tree |
| --- | --- | --- |
| CPython 3.12.15 | python.org, sha256 c2c43219… | `/usr/bin/python3`, standard library at `/usr/lib/python312.zip` |
| Lua 5.4.9 | lua.org, sha256 2335b6c5… | `/usr/bin/lua` |
| Zstandard 1.5.7 | GitHub release, sha256 eb33e51f… | `/usr/bin/zstd` |
| qwenchat, the Qwen conversation | llama.cpp at commit 00af6356…, built with cmake 4.4.3 (`tools/nonos-cmake`) | `/bin/qwenchat` (x86-64-v3), `/bin/qwenchat-x86_64_v2`, `/bin/qwenchat-x86_64` |
| John the Ripper 1.9.0 | openwall.com, PGP-signed release (offline key 297A D21C…) | `/usr/bin/john`, config and word list under `/usr/share/john` |

## Build

`tools/nonos-linux-userland-build PROGRAM OUT` builds one program. Every source it takes is a
line in `tools/nix/sources.txt`: a name, a URL and a sha256. Its `fetch` looks the name up,
downloads into `NONOS_SRC_CACHE` (default `target/src-cache`) when the file is absent, and
refuses the file unless its sha256 is the pinned one. Where a project publishes no sha256, the
script's comment beside the `fetch` says what the pinned hash was checked against (a PGP
signature, SHA3-256 or SHA-512). It cross compiles with zig 0.16.0 to a static, non-PIE x86-64
musl executable, the shape the personality runs. The zig comes from the flake:
`tools/nonos-zig` prints `NONOS_ZIG` and refuses outside `nix develop`, and `tools/nonos-cmake`
does the same for cmake. Build time and build paths are fixed and mapped out, so the same
sources give the same bytes.

CPython links every standard library C module it can build into the one binary, since a shared
object would be a second file to prove; `tools/nonos-python-stdlib-zip` packs the standard
library as sources with unchecked hash-based bytecode beside them. qwenchat is built once per
instruction set the personality chooses between, each against its own llama.cpp build, and the
App Store lists the Qwen tiers by the BLAKE3 of the one in the store
(`userland/capsule_market/linux-guests.json`, `mk/21-market.mk`).

`nix build` makes the same programs through `tools/nix/userland.nix`: each source in
`sources.txt` becomes a fixed output derivation keyed by its sha256, the script runs with that
cache, the flake's zig and cmake, and no network.

## Trust

Each program is signed under the one Linux userland publisher pair the key ceremony makes
(`linux_userland_publisher`, in `tools/nonos_keys/catalog.py`), its NØNOS-ID certificate signed
by the trust anchor, and its STARK trailer enrolled under the same policy root as every capsule.
The personality checks all three before any page of a program runs. A program holds no
capabilities (`CAPSULE_REQUIRED_CAPS := 0x0`); its endpoints, 5150 to 5163, are declared and
never registered.

## Data beside the programs

The standard library zip, the `lib-dynload` note, the CA bundle at `/etc/ssl/cert.pem`, John's
`john.conf` and `password.lst`, the tour under `/usr/share/nonos/tour/` and the link table at
`/etc/nonos-links` are data: nothing in them is run, so nothing proves them. The link table maps
`/usr/bin/python` and `/usr/bin/python3.12` to `/usr/bin/python3`.

## Not done yet

- John's incremental-mode charset files are not shipped (`Userland.mk` leaves them to a later
  install).
- With `NONOS_LINUX_GUESTS=1` the store lists are emptied, so the programs are enrolled but not
  stored in the guest test image.
