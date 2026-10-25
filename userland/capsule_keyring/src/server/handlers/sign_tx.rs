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

use alloc::vec::Vec;

use super::super::zeroize::zeroize32;
use super::sign_tx_parse::parse;
use super::sign_tx_rlp::{digest, signed};
use crate::protocol::{encode_response, Request, EACCES, EINVAL};
use crate::store::{Store, StoreError};

pub fn sign_tx(store: &mut Store, req: Request<'_>, sender_pid: u32) -> Vec<u8> {
    let Some(tx) = parse(req.payload) else {
        return encode_response(req.seq, EINVAL, &[]);
    };
    let caller_pid = match crate::server::caller::resolve_caller(tx.payload_pid, sender_pid) {
        Some(pid) => pid,
        None => return encode_response(req.seq, EACCES, &[]),
    };
    let Some(digest) = digest(&tx) else {
        return encode_response(req.seq, EINVAL, &[]);
    };
    let mut secret = match store.eth_secret(tx.id, caller_pid) {
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
    encode_response(req.seq, 0, &signed(&tx, &sig))
}
