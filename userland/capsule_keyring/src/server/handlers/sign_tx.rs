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

//! One EIP-1559 transaction the wallet has reviewed with its owner: any
//! recipient, value and calldata, on Ethereum mainnet or Sepolia only.
//!
//! The fixed templates beside this one each sign a single shape on mainnet.
//! A deposit into the shield pool, a settlement the owner lands, a token
//! transfer on Sepolia or a USDC payment is none of them, and a template per
//! contract would put every contract address the wallet will ever meet in
//! here. The calling wallet is the one that shows what is signed, and it is
//! the owner of the key (`resolve_caller`, `eth_secret`), so this signs what
//! that owner reviewed, bound to a chain id this keyring accepts, so a
//! signature cannot be replayed on a network the wallet never offered.

use alloc::vec;
use alloc::vec::Vec;

use super::super::eip1559::tx_fields;
use super::super::field32::field32;
use super::super::rlp::{rlp_list, rlp_uint_be};
use super::super::zeroize::zeroize32;
use crate::protocol::{encode_response, Request, EACCES, EINVAL};
use crate::store::{Store, StoreError};

/// Ethereum mainnet and Sepolia: the two networks the wallet offers.
const CHAINS: [u64; 2] = [1, 11_155_111];
/// pid, wallet, chain id, to, nonce, tip, fee cap, gas, value, data length.
const HDR: usize = 4 + 4 + 8 + 20 + 32 * 5 + 4;
/// A settlement carries about 99 KB of proof; nothing the wallet sends is larger.
pub const MAX_DATA: usize = 128 * 1024;

pub fn sign_tx(store: &mut Store, req: Request<'_>, sender_pid: u32) -> Vec<u8> {
    let p = req.payload;
    if p.len() < HDR {
        return encode_response(req.seq, EINVAL, &[]);
    }
    let data_len = u32::from_le_bytes([p[HDR - 4], p[HDR - 3], p[HDR - 2], p[HDR - 1]]) as usize;
    if data_len > MAX_DATA || p.len() != HDR + data_len {
        return encode_response(req.seq, EINVAL, &[]);
    }
    let mut chain = [0u8; 8];
    chain.copy_from_slice(&p[8..16]);
    let chain_id = u64::from_le_bytes(chain);
    if !CHAINS.contains(&chain_id) {
        return encode_response(req.seq, EINVAL, &[]);
    }
    let payload_pid = u32::from_le_bytes([p[0], p[1], p[2], p[3]]);
    let caller_pid = match crate::server::caller::resolve_caller(payload_pid, sender_pid) {
        Some(pid) => pid,
        None => return encode_response(req.seq, EACCES, &[]),
    };
    let id = u32::from_le_bytes([p[4], p[5], p[6], p[7]]);
    let mut to = [0u8; 20];
    to.copy_from_slice(&p[16..36]);
    let nonce = field32(p, 36);
    let max_priority = field32(p, 68);
    let max_fee = field32(p, 100);
    let gas = field32(p, 132);
    let value = field32(p, 164);
    let data = &p[HDR..];
    let chain_be = chain_id.to_be_bytes();
    let fields = || {
        tx_fields(&chain_be, &nonce, &max_priority, &max_fee, &gas, &to, &value, data)
    };
    let mut unsigned = vec![0x02u8];
    unsigned.extend_from_slice(&rlp_list(&fields()));
    let mut digest = [0u8; 32];
    if nonos_libc::crypto_keccak256(unsigned.as_ptr(), unsigned.len(), digest.as_mut_ptr(), 32)
        != 32
    {
        return encode_response(req.seq, EINVAL, &[]);
    }
    let mut secret = match store.eth_secret(id, caller_pid) {
        Ok(s) => s,
        Err(StoreError::AccessDenied) => return encode_response(req.seq, EACCES, &[]),
        Err(_) => return encode_response(req.seq, EINVAL, &[]),
    };
    let mut sig = [0u8; 65];
    let rc = crate::server::secp::sign(&secret, &digest, &mut sig);
    zeroize32(&mut secret);
    if rc != 65 || sig[64] < 27 {
        return encode_response(req.seq, EINVAL, &[]);
    }
    let mut f = fields();
    f.push(rlp_uint_be(&[sig[64] - 27]));
    f.push(rlp_uint_be(&sig[0..32]));
    f.push(rlp_uint_be(&sig[32..64]));
    let mut raw = vec![0x02u8];
    raw.extend_from_slice(&rlp_list(&f));
    encode_response(req.seq, 0, &raw)
}
