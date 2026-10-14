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

/*
 * One op-30 signature (plan/31): the request to the keyring, and the reply
 * believed only under the Bond domain this wallet recomputes. The words are
 * the publisher plans', already checked against this account's own bond.
 */

use nonos_nox_config::{address, Contract, TypedData};

use super::super::ipc::{keyring_call, HDR_LEN};
use super::super::state::State;
use super::super::typed::{
    believed, struct_hash, typed_request, Asked, OP_SIGN_TYPED_DATA, REPLY_LEN,
};
use super::kec::Kec;

pub fn sign_typed(
    s: &State,
    entry: u16,
    t: &TypedData,
    words: &[[u8; 32]],
    chain_id: u64,
) -> Result<[u8; 65], &'static str> {
    let bond = address(chain_id, Contract::PublisherBond)
        .map_err(|_| "no Bond contract on this network yet")?;
    let struct_hash = struct_hash(&Kec, t, words).ok_or("the words do not fit the type")?;
    let req = typed_request(s.vault.owner_pid, s.vault.wallet_id, entry, chain_id, words);
    let rx = keyring_call(s.vault.keyring_port, OP_SIGN_TYPED_DATA, &req, REPLY_LEN)
        .map_err(|_| "the keyring did not sign")?;
    let asked = Asked { signer: s.address, struct_hash, chain_id, bond };
    believed(&Kec, rx.get(HDR_LEN..).unwrap_or(&[]), &asked)
        .ok_or("the keyring signed something else; nothing was kept")
}
