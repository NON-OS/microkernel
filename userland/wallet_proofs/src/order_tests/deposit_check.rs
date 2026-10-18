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

//! orderId and the deposit address the wallet recomputes,
//! by nonos_nox_chunks, against abi.encode, CREATE2 and the EIP-1167 clone's
//! creation code written out by hand.

use nonos_nox_chunks::clone_initcode_hash;
use nonos_nox_config::{address, Contract, DISPUTE_WINDOW, ORDER_TAG, REFUND_TAG};
use sha3::{Digest, Keccak256};

use super::deposit_fixture::{ids, made_at, AT, K};
use super::fixture::{buy, offer, NOW, REFUND_PAIR};
use crate::order::deposit_check::{deposit_check, Pinned};

fn kec(parts: &[&[u8]]) -> [u8; 32] {
    Keccak256::digest(parts.concat()).into()
}

fn w(tail: &[u8]) -> [u8; 32] {
    let mut x = [0u8; 32];
    x[32 - tail.len()..].copy_from_slice(tail);
    x
}

#[test]
fn the_id_and_deposit_are_abi_encode_and_create2_written_out() {
    let n = buy().order_new(&offer(), REFUND_PAIR, NOW);
    let m = made_at(&n);
    let nox = address(n.chain_id, Contract::NoxToken).unwrap();
    assert_eq!(ORDER_TAG, kec(&[b"NONOS.nox.order.v1"]));
    let (amount, deadline) = (w(&n.amount.to_be_bytes()), w(&n.deadline.to_be_bytes()));
    let (window, len) = (w(&DISPUTE_WINDOW.to_be_bytes()), w(&n.content_len.to_be_bytes()));
    let head = [ORDER_TAG, w(&n.chain_id.to_be_bytes()), w(&AT.escrow)];
    let ids3 = [n.listing_id, n.release_id, n.bond_id, w(&n.payee), w(&nox)];
    let rest = [amount, deadline, window, n.chunk_root, len, n.cipher_root, n.key_commit];
    let refund = kec(&[&REFUND_TAG, &w(&n.refund_to), &n.refund_salt]);
    let enc = [&head[..], &ids3, &rest, &[refund, [0xBB; 32]]].concat();
    assert_eq!(enc.len(), 17);
    let id = kec(&[enc.concat().as_slice()]);
    let deposit = kec(&[&[0xff], &AT.escrow, &id, &AT.forwarder]);
    assert_eq!((m.order_id, &m.deposit[..]), (id, &deposit[12..]));
    assert_eq!(deposit_check(&K, &n, &m, Some(AT)), Ok(()));
}

#[test]
fn the_forwarder_hash_is_the_clones_creation_code_hashed() {
    let n = buy().order_new(&offer(), REFUND_PAIR, NOW);
    let m = made_at(&n);
    let fwd = [0x3F; 20];
    let head = hex("3d602d80600a3d3981f3363d3d373d3d3d363d73");
    let code = [&head[..], &fwd, &hex("5af43d82803e903d91602b57fd5bf3")].concat();
    assert_eq!(clone_initcode_hash(&K, &fwd), kec(&[&code]));
    let at = Pinned { forwarder: kec(&[&code]), ..AT };
    let mut on = m;
    (on.order_id, on.deposit) = ids(n.chain_id, &m.terms, &at);
    assert_eq!(deposit_check(&K, &n, &on, Some(at)), Ok(()));
}

fn hex(s: &str) -> Vec<u8> {
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap()).collect()
}
