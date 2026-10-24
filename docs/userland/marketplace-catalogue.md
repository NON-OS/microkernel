# The marketplace catalogue

How a curated Linux app becomes a schema-3 Marketplace listing that carries its own box, and how to add one.

## What a listing carries

A catalogue app is an Alpine v3.20 package. Its release pins the BLAKE3 of the `.apk`. It also carries a `guest_policy`, the 256-byte box of the plan, written for the exact program the installer will record: `usr/bin/<package>` when the package has one, else its first ELF under `usr/bin` (`tools/nonos_market_catalogue/apk.py:57`, `program`).

The policy's listing and release ids are derived from the publisher key by `marketplace_abi`'s rule, so the market's policy gate answers Valid. A schema-3 Linux release without a policy answers Missing and can never be installed. Every catalogue listing is free (`settle_mode` 0): it needs no bond, binding or ReleaseAttest.


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

1. fetches the package index and the apk;
2. holds every index field to Alpine's spelling (`tools/nonos_market_catalogue/names.py`);
3. checks the licence against the recipe;
4. writes the release.

A package that is missing, or fails a check, is left out with a line on stderr and is never listed unready. A malformed recipes file or publisher key stops the run with a message. `python3 -m nonos_market_catalogue.documents OUT.json DIR`, run from `tools/`, splits the index into one unsigned document per listing, as committed under `nonos-data/market/catalogue/`. The publisher then signs each one with `marketplace-index sign-release`.


## Adding an app

1. Check that the package is in Alpine v3.20, that its licence is on the allowlist, and that it needs nothing the platform lacks: no pty, no program over 64 MiB, no inbound network.
2. Add its recipe with the narrowest mask it works in.
3. Run the generator and the documents step, and commit the new document.
4. After the boot session's run, add `calls=` from `nonos-linux-pack syscalls --from-log`.

## Proofs

`python3 -m unittest discover -s tools/nonos_market_catalogue -t tools` runs these:

- **Hostile inputs:** tests for packages cut short, oversized or not gzip; index records with a traversal or a bad type; recipes, licences and document names.
- **A seeded fuzz sweep:** `test_fuzz.py`, over every parser of untrusted input. Use NONOS_FUZZ_ITERS and NONOS_FUZZ_SEED for a long run.
- **Mutants:** `test_mutants.py` applies each of the 23 mutants in `tools/nonos_market_catalogue/mutants.json` to a copy of the tree. The suite must fail on every one and pass on the unmutated copy.
