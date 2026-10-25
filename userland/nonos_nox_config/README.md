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
crate may name their addresses or a typehash, and a redeploy of them changes
this crate and nothing else. The keyring's typed-data table
(`server/typed/`, op 30) reads each struct, its domain and its verifying
contract on each chain from here, so a deployed address reaches its signatures
with no change there. The copies listed under Known copies are of contracts
the redeploy does not touch: NOX, the staking proxy, Shield and the retired
receipt settlement.

## What it holds

- `MAINNET` (1) and `SEPOLIA` (11155111). These are the only chains a lookup
  resolves.
- `Deployment` per chain, with these contracts:
  - NOX token. Mainnet uses the live V3_1 token (since block 25,960,151):
    fees up to 300 bps that the Safe can change, skipped when either side is
    fee exempt, a blacklist, EIP-2612 permit under domain "NONOS" (no op-30
    entry uses it), and a revert on any zero-amount transfer between two
    non-exempt addresses. Sepolia uses the wallet's test token.
  - Settlement, Escrow, PublisherBond, ListingRegistry, ProofAnchor. The
    PublisherBond label names the new NoxListingBond (plan/24), not the
    retired suite's NoxPublisherBond.
  - ShieldPool: the Sepolia pool shield_core's ACTIVE pool names; `None` on
    mainnet until Shield L1 deploys there (O3).
- Each contract is `Deployed { address, block }`: `block` is where a log read
  starts (plan/26). Mainnet NOX's is V3_1's block. Sepolia NOX's is not
  recorded, so it is 0: unknown, and settle refuses log reads of it.
  `deployed()` answers both; `CONTRACTS` lists them in the order settle checks.
- Every address is an `Option`. An undeployed contract is `None`, and
  `address()` answers `NotDeployed`, never a zero address. Today every new
  contract is `None`, so every caller fails closed until the owner deploys.
- Domains: `SETTLEMENT`, `ESCROW`, `BOND` and `LISTING`, all version "1".
  - `domain_words` gives the 160 bytes the domain separator hashes.
  - `domain_answer_matches` says whether a contract's ERC-5267
    eip712Domain() return is exactly that domain, at that chain and address.
  - The crate holds no keccak. The keyring and the capsules hash those bytes
    with the kernel's keccak, and host tools with their own.
- Typed structs: `VOUCHER`, `PUBLISHER_BINDING` and `RELEASE_ATTEST`. They are
  listed in `TYPED` in the order of the keyring's table (plan/31).
- Tags: `CHANNEL_TAG`, `ORDER_TAG`, `BOND_TAG`, `LISTING_TAG`, `RELEASE_TAG`
  and `REFUND_TAG`, each the keccak256 of `NONOS.nox.<x>.v1`. Each is used as
  the first word inside `abi.encode` of its id or commitment.
- Units: install, proof, call, second and byte.
- Gas limits for what the OS encodes itself: `APPROVE_GAS` 60000,
  `OPEN_GAS` 200000, `TOPUP_GAS` and `CLOSE_GAS` 120000, and
  `SETTLE_GAS_MAX` 3000000, the most one settle() is signed for.
- The fee caps the OS signs under: `MAX_FEE_PER_GAS` 1000 gwei and
  `MAX_PRIORITY_FEE_PER_GAS` 10 gwei.

## Known copies

New code reads its chain constants from here. These older copies stay where
they are, and each must match this crate where they overlap:
- the keyring: `server/wallet_rail/addresses.rs` (NOX) and
  `server/eip1559/consts.rs` (NOX, the retired receipt settlement address, the
  approve, stake and transfer selectors, and the staking proxy);
- the wallet: `wallet/nox/constants.rs` (NOX, the staking proxy),
  `wallet/chain.rs` (each chain's NOX), and
  `wallet/screen/shield/consts.rs` (Sepolia's Shield pool, verifier,
  registry, policy and NOX). The wallet's NOX address moves here in B7;
- shield_core: its pool and asset constants (`net/asset.rs`,
  `evm/network.rs`), which are upstream's and kept verbatim.

## Tests

22 host tests in `tests/`. `sha3` is a dev-dependency only, so every pinned
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
