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
 * A channel's opening followed (B8, v1.4.38): OPEN START's job answers the
 * approve and open transactions; CONFIRM is then asked every CONFIRM_MS
 * until the opening is mined and the meter records the channel.
 */

use alloc::boxed::Box;

use nonos_libc::mk_uptime_ms;
use nonos_payment_proto::{OpenConfirm, EAGAIN};

use super::intake::Step;
use super::opening::refused;
use crate::wallet::meter::order_calls::{open_confirm, order_job};
use crate::wallet::state::State;

const CONFIRM_MS: u64 = 5_000;

/// Whether the opening moved on.
pub fn poll(state: &mut State) -> bool {
    let Some(p) = state.meter.purchase.as_deref_mut() else { return false };
    p.step = match &p.step {
        Step::Opening(id, s) => match order_job(*id) {
            Err(EAGAIN) => return false,
            Ok(b) if b.len() == 64 => {
                let open_tx = b[32..].try_into().unwrap_or([0; 32]);
                Step::Confirming(Box::new(OpenConfirm { start: **s, open_tx }), 0)
            }
            Ok(_) => Step::Failed(refused(0)),
            Err(e) => Step::Failed(refused(e)),
        },
        Step::Confirming(c, at) => {
            let now = mk_uptime_ms().max(0) as u64;
            if now < *at {
                return false;
            }
            match open_confirm(&c.encode()) {
                Err(EAGAIN) => Step::Confirming(c.clone(), now + CONFIRM_MS),
                Ok(b) if b.len() == 32 => Step::Opened(b[..].try_into().unwrap_or([0; 32])),
                Ok(_) => Step::Failed(refused(0)),
                Err(e) => Step::Failed(refused(e)),
            }
        }
        _ => return false,
    };
    true
}
