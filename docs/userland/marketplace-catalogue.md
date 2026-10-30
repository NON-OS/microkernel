# The marketplace catalogue

How a curated Linux app becomes a schema-3 Marketplace listing that carries its own box, and how to add one.

## What a listing carries

A catalogue app is an Alpine v3.20 package. Its release pins the BLAKE3 of the `.apk`. It also carries a `guest_policy`, the 256-byte box of the plan, written for the exact program the installer will record (`install/market/choose.rs`):

- `usr/bin/<package>` when the package has a regular file there;
- else its first ELF under `usr/bin`, in archive order;
- or, for a package of many programs, the one its recipe names as `program` (monero's `usr/bin/monero-wallet-cli`).

On the device a market install records the program whose bytes its policy names: of the names under `usr/bin` whose last copy is an ELF of those bytes, `usr/bin/<package>` if it is one, else the first in archive order. A policy naming bytes the package does not carry records nothing, and the install is refused. The generator holds a recipe's `program` to that same rule and refuses one that would lead the device to another name (`tools/nonos_market_catalogue/apk.py`, `chosen_by_bytes`).

The bytes hashed are the last copy of that name, which is the one that lands. A package whose `usr/bin/<package>` is not an ELF is refused, never passed over (`tools/nonos_market_catalogue/apk.py`, `program`).

The policy's listing and release ids are derived from the publisher key by `marketplace_abi`'s rule, so the market's policy gate answers Valid. A schema-3 Linux release without a policy answers Missing and can never be installed. Every catalogue listing is free (`settle_mode` 0): it needs no bond, binding or ReleaseAttest.

init asks the market for the release's policy (`OP_GET_LINUX_POLICY`, I-C5) and hands it to the installer, which binds it to the app's program. A policy the market does not vouch for refuses the install, and no listed package from an index installs without one (`userland/capsule_linux/src/linux/install/market/run.rs`, `install_from`). Each release's note says so (`tools/nonos_market_catalogue/catalogue.py:30`, `NOTE`).

A market release's policy names one program, the one its listing starts. Every other ELF of that package is held: the installer writes an unvouched policy beside each one before its bytes land, and the program gets its real policy only once that policy is bound. So no ELF of the listed package starts outside its box at its installed path. That holds if its install is cut off, and if an uninstall leaves some of it behind (`userland/capsule_linux/src/linux/install/market/minted.rs`). The package's other programs are refused at their installed paths until they have a policy of their own. In 0.9.3 those are git's HTTP and SSH helpers, age's `age-keygen` and flac's `metaflac`, and their listings say so.


- **Dependencies:** a listed package's dependencies install unheld (tig pulls git, bat pulls less, vim pulls xxd), and each runs in the default box.
- **A copy:** a held ELF copied to another path along with its trailer.
- **The loader:** `/lib/ld-musl-x86_64.so.1 <program>`, because the loader's gate checks only the trailer.

The fix is to tie a policy to the program's hash, not its path, so that a copy or a loader exec keeps its box.

Nothing is listed until the generator has authenticated it as the device does, under the Alpine keys the device pins: [Catalogue authentication](marketplace-catalogue-authentication.md).

## The recipes file

`userland/capsule_market/linux-catalogue.json` is versioned. Its shape is `{"format": "nonos.linux-catalogue.v1", "apps": [...]}`, and a file of any other format, shape, encoding, or more than 1 MiB is refused whole (`tools/nonos_market_catalogue/recipes_file.py:29`, `load`). Each app is an object of strings, and `args` and `with` lists of them:

| Key | Required | What it holds |
|---|---|---|
| `package` | yes | the Alpine package name |
| `licence` | yes | the licence expression the index must still give, exactly |
| `mask` | yes | the services, from WAYLAND NET_ANON NET_LISTEN_LO FILES_HOME MODELS_RO EXEC JIT TTY |
| `name`, `text` | yes | what the store shows |
| `calls` | no | the measured set, as `nonos-linux-pack syscalls --from-log` prints it |
| `dests` | no | a file inside the tree of the plan destinations, one a line; needs NET_ANON |
| `program` | no | the program the listing sells, as `usr/bin/<name>`, for a package of many programs |
| `args` | no | the long options the program starts with, before the person's own (below) |
| `with` | no | Alpine packages the install adds to the closure, which nothing the package needs names (below) |

Without `calls` the set is UNMEASURED: buckets 0 to 511 set and 512 clear (`tools/nonos_market_catalogue/recipes.py:31`). That is the market-ready form of "no set yet". A recipe is refused if:

- its licence is off the allowlist (`tools/nonos_market_catalogue/licences.py:26`);
- it names an unknown service, or FILES_USER;
- its set is not market ready;
- it is listed twice.

## Listed arguments

A recipe's `args` are the words its program starts with, before the person's own: `monero`'s are `--daemon-address=<a public node>` and `--untrusted-daemon`. They are for a default that the program has no other way to take, not for anything the box grants: they add no service and no mask bit.

- **The rule.** 1 to 16 words, each `--name` or `--name=value`: a name of lowercase letters, digits and hyphens, a value of printable ASCII with no space, no option twice. The block they make is at most 1024 bytes. No shell reads them: each is one argv word as written (`tools/nonos_market_catalogue/listed_args.py`; the device's copy of the rule is `userland/nonos_linux_policy/src/args/`, and both pin the same bytes).
- **Where they ride.** After the 256 policy bytes in the release's `guest_policy`, as `"NLXARG01" | count | (length | word)...`. The publisher's release signature and the operator's index signature cover them with the policy. marketplace_abi bounds the field at 256 + 1024 bytes and refuses arguments after a vendor tier's policy (`verdict/policy.rs`).
- **To the device.** The market hands them on after the policy and its keys (`OP_GET_LINUX_POLICY`, `nonos_market_proto::linux_policy`). init appends them to the install word `m` (`src/userspace/capsule_linux/install.rs`). The install role reads them whole before it fetches anything, refusing the install otherwise. It then writes them beside the program as `.nonos_linux_policy.args.bin`, before the policy, and vouches for them over the policy and the arguments together (`install/market/bind_args.rs`). An uninstall takes them out with the box (`take.rs`), and a package may not ship either name (`proof_names.rs`).
- **At start.** The personality reads them beside the program and holds them to that trailer and to the policy the program was bound to (`app/listed.rs`). It puts each one before the person's words, except one whose option the person names themself, as `--name` or `--name=...` before any `--` (`app/args_listed.rs`). A file that is there but not vouched for refuses the start, and does not start the program without them. A program another program starts with `execve` gets only the words it is given.

*host-tested* (capsule_linux_proofs, market_proofs, market_proto, nonos_linux_policy, the catalogue suite). Not booted yet.

## Extra packages

A recipe's `with` names Alpine packages its install adds to the closure: a plugin the program needs to be useful but no package depends on. zathura opens no PDF without `zathura-pdf-mupdf`, for one. They add no service and no mask bit. They install as dependencies do, so a program among them runs in the default box, not the listing's.

- **The rule.** 1 to 8 names, each a package name by the generator's rule (`names.py`, `PACKAGE`), none twice and not the recipe's own. Each must be in the fresh signed index, or the run stops by name (`tools/nonos_market_catalogue/listed_with.py`; the device's copy of the rule is `userland/nonos_linux_policy/src/with/`, and both pin the same bytes).
- **The lock.** Each extra is pinned like a recipe's package, by its index record's version, checksum and size, but with no `B`, `G` or `R`, since no listing signs its `.apk`. An index that moves an extra stops the seal by name, and `--update-lock` pins a new one (`lock.py`, `lock_file.py`).
- **Where they ride.** In the same tail as the listed arguments, after them or straight after the policy, as `"NLXWITH1" | count | (length | name)...`. The publisher's release signature and the operator's index signature cover them. Arguments and extras together stay within marketplace_abi's 1024 bytes after the policy. A listing with no `with` carries the bytes it always has.
- **To the device.** The market and init hand the tail on unchanged. The install role splits it where the arguments' own lengths end and reads both whole before it fetches anything, refusing the install otherwise (`install/market/tail.rs`). An extra is wanted after the listed package and its needs, and is resolved, verified by the index's checksum and placed as any need is (`install/get/closure.rs`). The market's pin holds the listed package alone. An uninstall takes the extras out with the closure. A tier, a tool or a Debian or pacman install refuses extras (`install/family.rs`).

*host-tested* (capsule_linux_proofs, market_proofs, nonos_linux_policy, the catalogue suite). No listing carries extras yet. Not booted.

## Generating listings

```sh
python3 tools/nonos-market-catalogue --out OUT.json --no-nonos \
  --operator-pubkey <64 hex> --catalogue userland/capsule_market/linux-catalogue.json \
  --serial N
```

For each recipe, the generator:

1. authenticates the package as above;
2. holds every index field to Alpine's spelling (`tools/nonos_market_catalogue/names.py`);
3. checks the licence against the recipe;
4. writes the release.

Downloads are capped before they are read and written to the cache only whole (`fetch.py`). A failed download is tried again up to three times in all, after 2 and then 8 seconds, so one dropped connection does not stop a seal. A 4xx answer other than 408 or 429 is not tried again (`fetch.py`, `transient`). Each package is cached under the branch that served it, so a warm cache never names another branch in a release's `package_url` (`sources.py`, `cached`).

Any fault stops the run with a message, so a seal lists every recipe or none. That covers a missing or malformed recipes file or lock, a bad publisher key, an index that fails its signature or has moved past the lock, and a package that fails any check. A failed package's cached copy is dropped, so the next run fetches it again (`tools/nonos_market_catalogue/run.py`, `catalogue_run`). A recipe inside another family's namespace (`kali.`, `blackarch.`, `qwen-`, `nonos-`) is refused. `python3 -m nonos_market_catalogue.documents OUT.json DIR`, run from `tools/`, splits the index into one unsigned document per listing, as committed under `nonos-data/market/catalogue/`. The publisher then signs each one with `marketplace-index sign-release`.


## The lock

`userland/capsule_market/linux-catalogue.lock.json` pins the version, checksum (`C:`) and size of the package each recipe resolved to, and the BLAKE3 of its whole `.apk` (`B`), which its listing signs. `C:` covers only the control and data members, so `B` is what keeps a same-size package with another signature member out. Its shape is `{"format": "nonos.linux-catalogue-lock.v3", "packages": {...}}`, and any other format, shape or encoding is refused whole (`tools/nonos_market_catalogue/lock.py`, `read`).

Every run fetches the signed indexes fresh, then holds them to the lock. A run stops, naming every package, if the index:

- resolves a recipe to another version or checksum;
- lacks a recipe's package;
- has a recipe with no pin, or a pin with no recipe.

After the fetch, every listing's `.apk` must be the one `B` pins. So two seals of one tree sign the same bytes or the second one stops. Alpine replaces a package's version in place, so a security update stops the next seal by name. `--update-lock` writes the lock from a run instead of holding it to the lock. That is a developer's deliberate step, and its diff is reviewed and committed like any other change.

## Apps built from source

Some apps are packaged by no distribution: DarkFi's chat app and command-line tools, and SimpleX's simplex-chat. A recipe of kind `source` lists one of these from a build of its upstream at a single commit, or from the binary its upstream publishes (below) (`tools/nonos_market_catalogue/source_recipe.py`, `checked`). DarkFi's chat app, `linux.nonos-darkfi-app`, is in the catalogue, built from v0.5.0, and drawn by mesa's softpipe on the CPU (tools/nix/sources/darkfi-app-*.sh). So is DarkIRC, `linux.nonos-darkfi-darkirc`, as darkirc and the IRC client sic in one box (tools/nix/sources/darkirc-box.sh). So are its wallet, `linux.nonos-darkfi-drk`, a prompt over drk, and a local dev chain, `linux.nonos-darkfi-darkfid`: darkfid on localnet, minerd and drk in one box with no network (tools/nix/sources/darkfi-wallet-box.sh). A testnet or mainnet node is not listed ([DarkFi](../using/darkfi.md)). SimpleX's terminal client is in it as `linux.kali.nonos-simplex-chat`, from SimpleX's own v7.0.3 release binary. The same binary is in it a second time as `linux.kali.nonos-simplex-core`, the engine NØNOS's SimpleX window drives over its WebSocket API: its own package and program name (`usr/bin/simplex-core`), so its policy and listed arguments sit beside a program of their own, in the box FILES_HOME NET_ANON, with `--chat-server-port=5225 --yes-migrate`.

| Key | Required | What it holds |
|---|---|---|
| `kind` | yes | `"source"` |
| `package` | yes | the name, of lower-case letters, digits, `-` and `.`, at most 58 long |
| `source` | yes | the upstream's git URL: `https://`, a host by name and a path, with no user, port, query or fragment |
| `commit` | yes | the full 40-hex commit it is built from |
| `build` | yes | the name of the nix recipe under `tools/nix/sources/` that builds it offline |
| `program` | yes | the program the listing sells, as `usr/bin/<name>` |
| `licence`, `mask`, `name`, `text` | yes | as for an Alpine recipe |
| `calls`, `dests`, `logo`, `args`; `serve` | no | as for an Alpine recipe; `serve` is `name:port`, the one capsule (1 to 32 of `[a-z0-9._-]` from a letter) the kernel lets reach the program's one listener on 127.0.0.1, at a port from 1 to 65535. It sets SERVE_ONE and writes a version 2 policy with the port and name at byte 217 (`tools/nonos_linux_pack/serve.py`). It is refused with WAYLAND, named in `mask`, or in an Alpine recipe |

The rest of the recipe is checked as an Alpine one is: the allowlist, the services, the set, the dests file and the logo record. `index_licence` and `licence_source` describe an index, so a source recipe may not carry them. Its listing is `linux.nonos-<package>`. That is the namespace the device installs from the NONOS package mirror (`userland/capsule_linux/src/linux/install/tools.rs`). The generator adds the prefix, so a recipe that names a reserved namespace itself (`nonos-`, `qwen-`, `kali.`, `blackarch.`) is refused.


**The listing.** For each source recipe the generator (`tools/nonos_market_catalogue/source_run.py`, `source_entries`):

1. checks that the content makes one pack (`pack_size.py`);
2. finds the program by `choose.rs`'s rule under the install's own name, `nonos-<package>`. It must be an ELF at the recipe's path, no larger than 128 MiB (`apk.py`, `MAX_IMAGE`), and no earlier name under `usr/bin` may carry the same bytes;
3. writes the content to `--package-out` as `<blake3>.tar`, for the mirror;
4. writes the release, `nonos-<package>@<commit>`:
   - `package_url` is `http://<mirror>/linux/<blake3>.tar`, and `package_hash` is that BLAKE3;
   - its `guest_policy` is built as an Alpine listing's is: the program's BLAKE3, the mask, the ids, the dests hash, and the set, UNMEASURED unless measured.

On the device, `install/tools_install.rs` fetches the content from `NONOS_PACKAGE_MIRROR` and holds it to the index's pin before it is unpacked. `install/market/run.rs` then binds the policy to the program, as for any market listing. So the device needs no change for these apps.

**No mirror, none listed.** Without `--package-mirror`, the source recipes are left out, and each is named on stderr. They are not checked, and their lock is neither read nor written. The release's `nonos.toml` names the mirror `mirror.nonos.software:80` and the content `target/linux-sources` (11003f2356, 04f14c7ddb), so the release image lists them; a tree whose `linux_packages` is empty lists none. With a mirror, every source recipe must be listed or the run stops and names it. That covers:

- no `--sources`;
- missing or doubled content;
- a link;
- no program at the recipe's path;
- a program that is not an ELF, or is past 128 MiB;
- a pack past its bounds;
- the lock not holding.

**The sources lock.** `userland/capsule_market/linux-catalogue.sources.lock.json` sits beside the Alpine lock. Its shape is `{"format": "nonos.linux-catalogue-sources-lock.v1", "sources": {...}}`. Each package is pinned by:

- `commit`;
- `B`, the BLAKE3 of the bundled content tar, which its listing signs;
- `S`, the tar's size;
- `G` and `R`, its logo's BLAKE3s.

A missing file pins nothing, and any other format, shape or encoding is refused whole (`tools/nonos_market_catalogue/source_lock.py`, `read`). A run stops, naming the package and the field, when a rebuild's content, its size, the recipe's commit or a logo is not the one pinned. It also stops for a recipe with no pin, or a pin with no recipe. `--update-lock` writes the sources lock as well, but only on a run with a mirror. The design also pins the nix narHash of the source; that waits for `tools/nix/sources/`.

```sh
python3 tools/nonos-market-catalogue --out OUT.json --no-nonos --operator-pubkey <64 hex> \
  --catalogue userland/capsule_market/linux-catalogue.json --serial N \
  --package-mirror 10.0.2.2:8080 --package-out target/linux-packages --sources DIR
```

For a development boot, `tools/nonos-mirror-relay --content target/linux-packages` serves that directory as the mirror ([the development mirror](../build/dev-mirror.md)). A listing id may appear only once in an index, so a source recipe whose name an in-tree tool already has stops the generator.

### Upstream's own release binary

A source recipe may list the binary its upstream publishes instead of a build (`tools/nonos_market_catalogue/asset_recipe.py`). In place of `commit` and `build` it names:

| Key | Required | What it holds |
|---|---|---|
| `asset` | yes | the https URL of the one release file |
| `sha256` | yes | its sha256, 64 lower-case hex digits |
| `sums` | yes | upstream's checksum file, another file of the same release directory |
| `family` | no | `"debian"`: listed as `linux.kali.nonos-<package>`, which the kernel passes as `deb:nonos-<package>`, so it installs and runs in the Debian family's tree; the name is then at most 53 long |
| `needs` | with `family` | the Debian packages that give its shared libraries, space separated |

`source` stays required: it says whose binary it is. The release is `nonos-<package>@<tag>`, the release directory the asset's URL is in.

**Getting the content.** `tools/nonos-source-asset RECIPE.json --package-out target/linux-packages` runs on a machine that reaches the release. It stops at the first fault, and nothing it fetches is run:

1. upstream's checksum file and its `.asc` must verify under the release key pinned at `userland/capsule_market/keys/simplex-build.asc`, fingerprint `BBDF7BDAD1548B16836AF5B9D53BDFD153C366BA` (`asset_sums.py`). The key is SimpleX's; another upstream gets its own key beside it;
2. the sha256 that file gives the asset, and the downloaded bytes' sha256, must both be the recipe's;
3. with a family, Kali's index is verified under the pinned Kali key, as the catalogue verifies it. The needs are resolved to their closure by the device's rule (`deb_closure.py`), and each .deb is held to the index. Every DT_NEEDED name and the interpreter of the binary must be a library one of those packages places (`elf_needed.py`, `deb_files.py`);
4. it writes `target/linux-sources/<package>.tar`, which holds the program and, for a family, `usr/share/nonos-debs/<package>`: one line per closure package, `<name> <version> <pool file> <sha256>` (`asset_needs.py`). The seal takes that directory as `--sources`. With `--package-out` it also writes the bundled `<blake3>.tar` that `nonos-mirror-relay --content` serves, and it prints the lock row.

With `--learn`, the sha256 is taken from the signed file instead of the recipe, and the filled-in recipe is written beside the tar.

**The listing and its lock.** The generator holds the content as for any source recipe. It also refuses a program whose sha256 is not the recipe's, and a content whose Debian pins are malformed, missing for a family, present without one, or leave out a need (`source_asset.py`, `row`). The sources lock pins an asset by `asset`, `sha256`, `A` (the program's BLAKE3) and `sums`, in place of `commit`, beside `B`, `S`, `G` and `R` (`source_pins.py`). A run stops by name when any of them moves.

**On the device.** `install/tools_install.rs` holds the content to the market's pin as before. Then `install/deb/needs.rs` reads the pins, by the same rules (`install/deb/pinned.rs`), before the program is placed. Each package is fetched from the Debian mirror by its pool file, held to its pinned SHA-256, and placed as a package of its own. The pins are part of the BLAKE3-pinned content, so no Debian index is read for them. Pins in a family other than Debian are refused. When Kali moves a pinned version out of its pool, the install fails by name until the recipe is pinned again.

## Adding an app

1. Check that the package is in Alpine v3.20, that its licence is on the allowlist, and that it needs nothing the platform lacks: no program over 64 MiB, no inbound network. A pseudo-terminal is there (`/dev/ptmx` and `/dev/pts/N`, see [the Linux personality](linux-personality.md#pseudo-terminals)), but no app has yet run on one.
2. Add its recipe with the narrowest mask it works in.
3. Run the generator and the documents step, and commit the new document.
4. Run once with `--update-lock` and commit the lock with the recipe.
5. After the boot session's runs (docs/market/catalogue-boot-plan.md), run `python3 -m nonos_market_catalogue.verdict --write APP=LOG ...`. It checks each log, one per run:
   - the app installed in its box;
   - no policy refusal;
   - the workload ended well, with a process exiting status 0 and none exiting otherwise or on a signal;
   - exactly one measured set, the app's own.

   It then writes the union of the runs' sets into the recipe (`tools/nonos_market_catalogue/verdict_run.py`, `run_verdict`).

## Proofs

`python3 -m unittest discover -s tools/nonos_market_catalogue -t tools` runs these. It needs `b3sum` (BLAKE3) on the PATH, as the generator does, because the listings carry its digests. Without it the suite stops at once with one line saying so, and skips nothing (`tools/nonos_market_catalogue/__init__.py`, `load_tests`).

- **A real package:** `vectors/` holds one real Alpine package (CC0-1.0) and its signed index record. RSA verification and the package chain are tested against Alpine's own signature, and a test key covers the padding and length rules.
- **Hostile inputs:** tests for every refusal: gzip, tar, pax, the chain, the index, names, recipes, documents, fetches and the run.
- **A seeded fuzz sweep:** `test_fuzz.py`, over every parser of untrusted input. Use NONOS_FUZZ_ITERS and NONOS_FUZZ_SEED for a long run.
- **Mutants:** `test_mutants.py` applies each of the 175 mutants in `tools/nonos_market_catalogue/mutants.json` to a copy of the tree. The suite must fail on every one and pass on the unmutated copy. A mutant that hangs the suite counts as killed after 120 s.
