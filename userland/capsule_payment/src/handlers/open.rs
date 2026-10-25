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

//! OPEN (op 12), from the wallet, in two calls (plan/21):
//!
//! - START is a job (channel_ops): the meter checks its gas, encodes
//!   approve(settlement, amount) and open(payee, listingId, amount, expiry,
//!   reclaimTo) itself, signs both with its own key and has settle broadcast
//!   them; JOB then answers approveTx | openTx.
//! - CONFIRM: by openTx, the meter records the channel only on a
//!   ChannelOpened it asked for (`nonos_nox_meter::open::confirmed`). EAGAIN
//!   while unmined; sent again, it answers the same.

use alloc::vec::Vec;

use nonos_nox_meter::flow::Work as Job;
use nonos_payment_proto::{parse_open, EINVAL};

use super::channel_ops::open_start;
use super::open_flow::confirm;
use crate::jobs::Work;
use crate::meter::Meter;
use crate::server::Call;

pub fn open(meter: &mut Meter, work: &mut Work, req: &Call<'_>) -> Vec<u8> {
    match parse_open(req.body) {
        Some(Ok(s)) => open_start(meter, work, Job::Open(s), req),
        Some(Err(c)) => req.result(confirm(meter, &c)),
        None => req.answer(EINVAL, &[]),
    }
}
