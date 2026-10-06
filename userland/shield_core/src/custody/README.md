# Custody

The recovery phrase, the seed and the sealed vault, with what this module trusts and what a hostile
neighbour can do. What is checked and what is not is in [20-security-status.md](../../../docs/20-security-status.md).

## What this owns

The recovery phrase, the 64-byte seed, and the vault the seed is sealed in at rest. It decides that
a wallet is 24 words with no passphrase (256 bits of entropy, `Phrase` in `phrase.rs`), the byte
layout of the vault file (`format.rs`: the magic `NOXSHLD1`, the length of the wrapped key, the
wrapped key, then the seed sealed with ChaCha20-Poly1305 under the associated data
`nox-shield/vault/v1`), and the rule that an existing vault is never overwritten (`vault/store.rs`).

It does not own the keys derived from the seed. `keys` derives the shield keys, `evm` derives the
public account key, and `store` derives the key its rows are sealed under. Custody decides how a
seed is made, stored and destroyed, and nothing about what it is used for.

## What it trusts

The platform CSPRNG, for the entropy of the phrase and for the file key of each vault. There is no
second source and no mixing: a device with a broken generator makes wallets that are broken, and no
arrangement here could repair that.

`nonos_hd` for BIP-39 and its word list, and `nonos_seal` for ChaCha20-Poly1305. Both are vendored
source under `vendor/`, and both are tested against their published vectors. `core/tests/phrase.rs`
pins the zero-entropy vector here as well, so a change in either crate that altered the encoding
fails in this suite and never silently derives other keys.

The platform keystore, through the `HardwareGuard` trait in `guard.rs`, for wrapping the file key:
the Secure Enclave on iOS and StrongBox or the TEE on Android. Custody trusts it to hold a key it
cannot export and to require the owner before it unwraps. The keystore never sees the seed.

## What it hands its neighbours

To `keys`, `evm` and `store`: a `Seed`, crate internal, zeroed on drop, with a `Debug` that prints
nothing (`secret.rs`).

To the apps: the words, once, when a wallet is created, and nothing else. No function exported to
the apps returns a seed or a derived secret.

## What an attacker who controls a neighbour can do

**An app that lies about the keystore.** A faulty or malicious `HardwareGuard` can refuse, which is
a denial of service and is reported as `CustodyError::Guard`. It can return a wrong key, and then
the seal fails to authenticate and the wallet reports a stored seed it cannot open. It cannot learn
the seed: the guard only ever sees the 32-byte file key, and only its wrapped form leaves this
module. A guard that recorded every key it wrapped would leak the file key, and the file key with
the vault file gives the seed, so the guard implementations in both apps are part of the trusted
set and are reviewed as such.

**A platform with a weak random source.** A predictable file key makes the vault openable without
the keystore, and a predictable phrase makes the whole wallet guessable. Nothing here can detect
either, so there is no fallback that pretends to improve on the platform.

**An attacker with the vault file alone.** They hold a ciphertext and a wrapped key. Both are
useless without the device. They learn that a wallet exists on that device, which the existence of
the file was never meant to hide.

**An attacker holding the phone unlocked.** They can open the wallet only if the keystore check of
the owner passes, which on both platforms means the device credential or a biometric. The answer
here is the lock: a dropped session is every derived key gone, and the next unlock asks the keystore
again.

**A request to destroy the wallet.** `vault/destroy.rs` overwrites the file before unlinking it.
That does not defeat a flash controller that copies blocks on write, which nothing an app does can
defeat, so the defence that holds is the wrapping key: each app deletes it from the keystore, and
without it any surviving copy of the ciphertext is noise.
