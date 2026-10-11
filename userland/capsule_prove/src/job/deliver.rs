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

//! One DELIVER (plan/15): the live family the caller supervises, held to the
//! policy it names and refused OUTSIDE as PROVE refuses it; the body from
//! what it handed over; a device proof under e2 bound to ctx2; a kind-2
//! container for the caller alone. No isolation proof is made.

use alloc::vec;
use alloc::vec::Vec;

use nonos_libc::mk_seal_read_live;
use nonos_proof_abi::wire::{DeliverRequest, Stage, PROVER};
use nonos_proof_abi::RECORD_BYTES;

use crate::assemble::wipe_bytes;
use crate::market::{body_of, check, container2, delivery_bound, live_check, split, witness};
use crate::market::{Sealed, WITNESS_BYTES};

const LIST_MAX: usize = 4096;

pub fn deliver(
    req: &DeliverRequest,
    sender: u32,
    stage: &mut dyn FnMut(Stage),
) -> Result<Vec<u8>, i32> {
    stage(Stage::Reading);
    let s = live(sender)?;
    live_check(&s.record)?;
    check(req.policy, req.listing_id, &s.record)?;
    let (mut w, _) = witness(&s, req.policy)?;
    super::isolation::clear(&mut w);
    let day = super::run::today()?;
    let body = body_of(&s, req);
    stage(Stage::Device);
    let device = super::device::proof(day, delivery_bound(req.listing_id, day, &s, &body))?;
    container2(day, &s, &device, &body).ok_or(PROVER)
}

/// The live record, witness and list of `pid`'s family: ESRCH when it
/// supervises none.
fn live(pid: u32) -> Result<Sealed, i32> {
    let mut buf = vec![0u8; RECORD_BYTES + WITNESS_BYTES + 32 * LIST_MAX];
    let n = mk_seal_read_live(pid, &mut buf);
    let s = usize::try_from(n).ok().and_then(|n| split(&buf[..n]));
    wipe_bytes(&mut buf);
    match s {
        Some(s) => Ok(s),
        None if n < 0 => Err(super::status::failed(n)),
        None => Err(PROVER),
    }
}
