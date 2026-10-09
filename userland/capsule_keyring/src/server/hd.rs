// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

// Shared HD-wallet derivation for the generate and recover handlers: mnemonic
// word indices to the m/44'/60'/0'/0/0 account key. The mnemonic, seed and
// every intermediate live only on this stack and are wiped before return; the
// public keys the non-hardened steps need come from the kernel's proven
// secp256k1 via the pubkey syscall.

use nonos_hd::bip39::seed_from_words;
use nonos_hd::{derive_eth_key_at, wipe};

/// Uncompressed SEC1 public key for a secret. `None` when the scalar is not
/// on the curve's order, which the derivation retries past.
pub(super) fn syscall_pubkey(sk: &[u8; 32]) -> Option<[u8; 65]> {
    let mut out = [0u8; 65];
    if crate::server::secp::pubkey(sk, &mut out) == 65 {
        Some(out)
    } else {
        None
    }
}

/// Derive the Ethereum account private key for a BIP39 mnemonic given as
/// wordlist indices, empty passphrase, path m/44'/60'/0'/0/0. The 64-byte
/// seed is wiped before returning. None when derivation fails; the caller
/// treats that as a failed request, never as a partial key.
pub(super) fn account_key_from_words(indices: &[u16]) -> Option<[u8; 32]> {
    account_key_at(indices, 0)
}

/// The key of account `index` of the phrase, m/44'/60'/0'/0/index.
pub(super) fn account_key_at(indices: &[u16], index: u32) -> Option<[u8; 32]> {
    let mut seed = [0u8; 64];
    if !seed_from_words(indices, b"", &mut seed) {
        return None;
    }
    let mut key = [0u8; 32];
    let ok = derive_eth_key_at(&seed, syscall_pubkey, index, &mut key);
    wipe(&mut seed);
    if ok {
        Some(key)
    } else {
        wipe(&mut key);
        None
    }
}

/// Keep the words beside the account key `id`, or take the key back out: a
/// wallet from words always has its words for the shield, never not.
pub(super) fn keep_seed(
    store: &mut crate::store::Store,
    id: u32,
    indices: &[u16],
    owner: u32,
    now: u64,
    expires_at: u64,
) -> bool {
    let kept = store.put_shield_words(id, indices, owner, now, expires_at).is_ok();
    if !kept {
        let _ = store.delete(id, owner);
    }
    kept
}
