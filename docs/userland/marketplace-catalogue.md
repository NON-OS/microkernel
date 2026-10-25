# The marketplace catalogue

How a curated Linux app becomes a schema-3 Marketplace listing that carries its own box, and how to add one.

## What a listing carries

A catalogue app is an Alpine v3.20 package. Its release pins the BLAKE3 of the `.apk`. It also carries a `guest_policy`, the 256-byte box of plan/30, written for the exact program the installer will record (`install/market/choose.rs`):

- `usr/bin/<package>` when the package has a regular file there;
- else its first ELF under `usr/bin`, in archive order;
- or, for a package of many programs, the one its recipe names as `program` (monero's `usr/bin/monero-wallet-cli`).

On the device a market install records the program whose bytes its policy names: of the names under `usr/bin` whose last copy is an ELF of those bytes, `usr/bin/<package>` if it is one, else the first in archive order. A policy naming bytes the package does not carry records nothing, and the install is refused. The generator holds a recipe's `program` to that same rule and refuses one that would lead the device to another name (`tools/nonos_market_catalogue/apk.py`, `chosen_by_bytes`).

The bytes hashed are the last copy of that name, which is the one that lands. A package whose `usr/bin/<package>` is not an ELF is refused, never passed over (`tools/nonos_market_catalogue/apk.py`, `program`).

The policy's listing and release ids are derived from the publisher key by `marketplace_abi`'s rule, so the market's policy gate answers Valid. A schema-3 Linux release without a policy answers Missing and can never be installed. Every catalogue listing is free (`settle_mode` 0): it needs no bond, binding or ReleaseAttest.

init asks the market for the release's policy (`OP_GET_LINUX_POLICY`, I-C5) and hands it to the installer, which binds it to the app's program. A policy the market does not vouch for refuses the install, and no listed package from an index installs without one (`userland/capsule_linux/src/linux/install/market/run.rs`, `install_from`). Each release's note says so (`tools/nonos_market_catalogue/catalogue.py:30`, `NOTE`).

A market release's policy names one program, the one its listing starts. Every other ELF of that package is held: the installer writes an unvouched policy beside each one before its bytes land, and the program gets its real policy only once that policy is bound. So no ELF of the listed package starts outside its box at its installed path. That holds if its install is cut off, and if an uninstall leaves some of it behind (`userland/capsule_linux/src/linux/install/market/minted.rs`). The package's other programs are refused at their installed paths until they have a policy of their own. In 0.9.3 those are git's HTTP and SSH helpers, age's `age-keygen` and flac's `metaflac`, and their listings say so.

The box is kept by path, so three ways around it are open today. Each ends in the owner's default box: the terminal, the person's home, exec, every Linux call and no network.

- **Dependencies:** a listed package's dependencies install unheld (tig pulls git, bat pulls less, vim pulls xxd), and each runs in the default box.
- **A copy:** a held ELF copied to another path along with its trailer.
- **The loader:** `/lib/ld-musl-x86_64.so.1 <program>`, because the loader's gate checks only the trailer.

The fix is to tie a policy to the program's hash, not its path, so that a copy or a loader exec keeps its box. It is filed with track A (HANDOFF/letters/m/10-to-a-policy-by-hash.md).

Nothing is listed until the generator has authenticated it as the device does, under the Alpine keys the device pins: [Catalogue authentication](marketplace-catalogue-authentication.md).

## The recipes file

`userland/capsule_market/linux-catalogue.json` is versioned. Its shape is `{"format": "nonos.linux-catalogue.v1", "apps": [...]}`, and a file of any other format, shape, encoding, or more than 1 MiB is refused whole (`tools/nonos_market_catalogue/recipes_file.py:29`, `load`). Each app is an object of strings:

| Key | Required | What it holds |
|---|---|---|
| `package` | yes | the Alpine package name |
| `licence` | yes | the licence expression the index must still give, exactly |
| `mask` | yes | the services, from WAYLAND NET_ANON NET_LISTEN_LO FILES_HOME MODELS_RO EXEC JIT TTY |
| `name`, `text` | yes | what the store shows |
| `calls` | no | the measured set, as `nonos-linux-pack syscalls --from-log` prints it |
| `dests` | no | a file inside the tree of plan/33 destinations, one a line; needs NET_ANON |
| `program` | no | the program the listing sells, as `usr/bin/<name>`, for a package of many programs |

Without `calls` the set is UNMEASURED: buckets 0 to 511 set and 512 clear (`tools/nonos_market_catalogue/recipes.py:31`). That is the market-ready form of "no set yet". A recipe is refused if:

- its licence is off the allowlist (`tools/nonos_market_catalogue/licences.py:26`);
- it names an unknown service, or FILES_USER;
- its set is not market ready;
- it is listed twice.

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

Downloads are capped before they are read and written to the cache only whole (`fetch.py`). Each package is cached under the branch that served it, so a warm cache never names another branch in a release's `package_url` (`sources.py`, `cached`).

Any fault stops the run with a message, so a seal lists every recipe or none. That covers a missing or malformed recipes file or lock, a bad publisher key, an index that fails its signature or has moved past the lock, and a package that fails any check. A failed package's cached copy is dropped, so the next run fetches it again (`tools/nonos_market_catalogue/run.py`, `catalogue_run`). A recipe inside another family's namespace (`kali.`, `blackarch.`, `qwen-`, `nonos-`) is refused. `python3 -m nonos_market_catalogue.documents OUT.json DIR`, run from `tools/`, splits the index into one unsigned document per listing, as committed under `nonos-data/market/catalogue/`. The publisher then signs each one with `marketplace-index sign-release`.

`mk/21-market.mk` and the seal (`tools/nonos_seal/market.py`) pass `--catalogue`, so the 17 apps are in 0.9.3's signed baseline. The owner chose that, and only the owner signs it.

## The lock

`userland/capsule_market/linux-catalogue.lock.json` pins the version, checksum (`C:`) and size of the package each recipe resolved to, and the BLAKE3 of its whole `.apk` (`B`), which its listing signs. `C:` covers only the control and data members, so `B` is what keeps a same-size package with another signature member out. Its shape is `{"format": "nonos.linux-catalogue-lock.v2", "packages": {...}}`, and any other format, shape or encoding is refused whole (`tools/nonos_market_catalogue/lock.py`, `read`).

Every run fetches the signed indexes fresh, then holds them to the lock. A run stops, naming every package, if the index:

- resolves a recipe to another version or checksum;
- lacks a recipe's package;
- has a recipe with no pin, or a pin with no recipe.

After the fetch, every listing's `.apk` must be the one `B` pins. So two seals of one tree sign the same bytes or the second one stops. Alpine replaces a package's version in place, so a security update stops the next seal by name. `--update-lock` writes the lock from a run instead of holding it to the lock. That is a maintainer's deliberate step, and its diff is reviewed and committed like any other change.

## Adding an app

1. Check that the package is in Alpine v3.20, that its licence is on the allowlist, and that it needs nothing the platform lacks: no pty, no program over 64 MiB, no inbound network.
2. Add its recipe with the narrowest mask it works in.
3. Run the generator and the documents step, and commit the new document.
4. Run once with `--update-lock` and commit the lock with the recipe.
5. After the boot session's runs (HANDOFF/catalogue/05-boot-plan.md), run `python3 -m nonos_market_catalogue.verdict --write APP=LOG ...`. It checks each log, one per run:
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
- **Mutants:** `test_mutants.py` applies each of the 127 mutants in `tools/nonos_market_catalogue/mutants.json` to a copy of the tree. The suite must fail on every one and pass on the unmutated copy. A mutant that hangs the suite counts as killed after 120 s.
