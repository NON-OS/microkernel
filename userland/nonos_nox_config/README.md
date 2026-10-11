# nonos_nox_config

Every chain constant the OS builds against, in one `no_std` crate with no
dependencies. It covers:
- chain ids;
- contract addresses;
- the EIP-712 domains;
- the type strings and typehashes of every struct the OS signs;
- the tags at the head of every derived id;
- the price units.

The owner is redeploying the marketplace contracts from scratch, so no other
crate may name an address or a typehash. A redeploy changes this crate and
nothing else.

## What it holds

- `MAINNET` (1) and `SEPOLIA` (11155111). These are the only chains a lookup
  resolves.
- `Deployment` per chain, with these contracts:
  - NOX token. Mainnet uses the live V2.1 token; Sepolia uses the wallet's test
    token.
  - Settlement, Escrow, PublisherBond, ListingRegistry, ProofAnchor.
- Every address is an `Option`. An undeployed contract is `None`, and
  `address()` answers `NotDeployed`, never a zero address. Today every new
  contract is `None`, so every caller fails closed until the owner deploys.
- Domains: `SETTLEMENT`, `ESCROW`, `BOND` and `LISTING`, all version "1".
  - `domain_words` gives the 160 bytes the domain separator hashes.
  - The crate holds no keccak. The keyring and the capsules hash those bytes
    with the kernel's keccak, and host tools with their own.
- Typed structs: `VOUCHER`, `PUBLISHER_BINDING` and `RELEASE_ATTEST`. They are
  listed in `TYPED` in the order of the keyring's table.
- Tags: `CHANNEL_TAG`, `ORDER_TAG`, `BOND_TAG`, `LISTING_TAG` and
  `RELEASE_TAG`, each the keccak256 of `NONOS.nox.<x>.v1`. Each is used as the
  first word inside `abi.encode` of its id.
- Units: install, proof, call, second and byte.

## Tests

13 host tests in `tests/`. `sha3` is a dev-dependency only, so every pinned
hash is recomputed independently of the constant it checks:
- typehashes from their type strings;
- name hashes from their names;
- tags and units from their labels;
- the domain encoding against the Ether Mail separator from the EIP-712
  specification.

```sh
cd userland/nonos_nox_config
cargo test
cargo build --release -Zbuild-std=core --target ../x86_64-nonos-user.json
```
