# The marketplace catalogue

How a curated Linux app becomes a schema-3 Marketplace listing that carries its own box, and how to add one.

## What a listing carries

A catalogue app is an Alpine v3.20 package. Its release pins the BLAKE3 of the `.apk`. It also carries a `guest_policy`, the 256-byte box of the plan, written for the exact program the installer will record (`install/record/program.rs`):

- `usr/bin/<package>` when the package has a regular file there;
- else its first ELF under `usr/bin`, in archive order.

The bytes hashed are the last copy of that name, which is the one that lands. A package whose `usr/bin/<package>` is not an ELF is refused, never passed over (`tools/nonos_market_catalogue/apk.py`, `program`).

The policy's listing and release ids are derived from the publisher key by `marketplace_abi`'s rule, so the market's policy gate answers Valid. A schema-3 Linux release without a policy answers Missing and can never be installed. Every catalogue listing is free (`settle_mode` 0): it needs no bond, binding or ReleaseAttest.


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
| `dests` | no | a file inside the tree of the plan destinations, one a line; needs NET_ANON |

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

Downloads are capped before they are read and written to the cache only whole (`fetch.py`).

A package that fails alone is left out with a line on stderr, never listed unready, and its cached copy is dropped. A missing or malformed recipes file, a bad publisher key, or an index that fails its signature stops the run with a message. A recipe inside another family's namespace (`kali.`, `blackarch.`, `qwen-`, `nonos-`) is refused. `python3 -m nonos_market_catalogue.documents OUT.json DIR`, run from `tools/`, splits the index into one unsigned document per listing, as committed under `nonos-data/market/catalogue/`. The publisher then signs each one with `marketplace-index sign-release`.


## Adding an app

1. Check that the package is in Alpine v3.20, that its licence is on the allowlist, and that it needs nothing the platform lacks: no pty, no program over 64 MiB, no inbound network.
2. Add its recipe with the narrowest mask it works in.
3. Run the generator and the documents step, and commit the new document.

## Proofs

`python3 -m unittest discover -s tools/nonos_market_catalogue -t tools` runs these:

- **A real package:** `vectors/` holds one real Alpine package (CC0-1.0) and its signed index record. RSA verification and the package chain are tested against Alpine's own signature, and a test key covers the padding and length rules.
- **Hostile inputs:** tests for every refusal: gzip, tar, pax, the chain, the index, names, recipes, documents, fetches and the run.
- **A seeded fuzz sweep:** `test_fuzz.py`, over every parser of untrusted input. Use NONOS_FUZZ_ITERS and NONOS_FUZZ_SEED for a long run.
- **Mutants:** `test_mutants.py` applies each of the 99 mutants in `tools/nonos_market_catalogue/mutants.json` to a copy of the tree. The suite must fail on every one and pass on the unmutated copy. A mutant that hangs the suite counts as killed after 120 s.
