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

//! Each PDU from the peripheral. Notifications and indications are
//! handled in any phase; a request to this host is answered from its own
//! small database (`server`); a response must answer the request
//! outstanding.

use super::phase::Phase;
use super::state::Host;
use super::step::Step;
use super::timer::response_to;
use crate::error::HogpError;
use crate::server::serve;
use nonos_bt_wire::att::{self, opcode::*, response};

impl Host {
    pub fn on_pdu(&mut self, pdu: &[u8], now_ms: u64) -> Step {
        // After the timeout no more ATT PDUs may be sent on this bearer
        // (Vol 3, Part F, 3.3.3); after any other failure the caller is
        // starting over, and an answer would only confuse that.
        if self.phase == Phase::Failed {
            return Step::default();
        }
        if self.expired(now_ms) {
            return self.fail(HogpError::Timeout);
        }
        let Some((&op, p)) = pdu.split_first() else { return self.fail(HogpError::Protocol) };
        match op {
            HANDLE_VALUE_NTF | HANDLE_VALUE_IND => match response::handle_value(p) {
                Ok(hv) => {
                    let mut step = Step::input(self.report(hv.handle, hv.value));
                    if op == HANDLE_VALUE_IND {
                        step.send.push(att::confirmation());
                    }
                    step
                }
                Err(e) => self.fail(e.into()),
            },
            op if is_request(op) => Step::send(serve(op, p)),
            op if op & COMMAND_FLAG != 0 => Step::default(),
            op if Some(response_to(op)) == self.pending => {
                self.pending = None;
                self.answer(op, p, now_ms)
            }
            ERROR_RSP => self.answer(op, p, now_ms),
            _ => self.fail(HogpError::Protocol),
        }
    }
}
