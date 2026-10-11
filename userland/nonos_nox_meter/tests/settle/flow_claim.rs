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

//! CLAIM's flow (v1.4.17): what is claimable, then GAS, then claim(NOX,
//! meter) to the pool, signed and broadcast; nothing claimable is ENOENT.

use nonos_nox_config::{Contract, CLAIM_GAS};
use nonos_nox_meter::flow::{Ask, Contracts, Flow, Next, Work};
use nonos_nox_meter::settle::{claim_data, CLAIM_SELECTOR, NOX_ASSET};
use nonos_nox_meter::MeterError;
use nonos_settle_proto::Gas;
use sha3::{Digest, Keccak256};

use super::fixture::{gas, state, CHAIN, METER, NOW};
use super::ret::{ret, word};

const POOL: [u8; 20] = [0x9c; 20];
const AT: Contracts = Contracts { nox: [0; 20], settlement: [0; 20], pool: POOL };

#[test]
fn a_claim_reads_claimable_then_signs_one_claim_to_the_meter() {
    let job = Work::Claim { chain_id: CHAIN, meter: METER };
    assert_eq!(job.needs(), [Contract::ShieldPool]);
    let (mut f, first) = Flow::begin(job.clone());
    assert_eq!(first, Next::Ask(Ask::View { view: 60, args: vec![ret(1)[0], word(&METER)] }));
    assert_eq!(f.on_view(ret(5_000)), Ok(Next::Ask(Ask::Gas)));
    let g = Gas { balance: u128::from(CLAIM_GAS) * 10, ..gas() };
    let Ok(Next::Sign(txs)) = f.on_gas(&g, &state(), (&AT, NOW)) else { panic!("no claim") };
    let tx = &txs[0];
    assert_eq!((txs.len(), tx.to, tx.gas, tx.value, tx.nonce), (1, POOL, CLAIM_GAS, 0, g.tx_nonce));
    assert_eq!(tx.data, claim_data(NOX_ASSET, &METER));
    assert_eq!(f.on_signed(vec![vec![1; 3]]), Next::Ask(Ask::Broadcast(vec![1; 3])));
    assert_eq!(f.on_hash(&[0xc1; 32]), Next::Done(vec![0xc1; 32]));
    let (mut empty, _) = Flow::begin(job);
    assert_eq!(empty.on_view(ret(0)), Err(MeterError::NothingOwed));
}

#[test]
fn claims_calldata_is_claim_uint64_address() {
    assert_eq!(CLAIM_SELECTOR[..], Keccak256::digest(b"claim(uint64,address)")[..4]);
    let d = claim_data(1, &METER);
    assert_eq!((d.len(), d[35], &d[48..]), (68, 1, &METER[..]));
    assert!(d[4..35].iter().chain(&d[36..48]).all(|b| *b == 0));
}
