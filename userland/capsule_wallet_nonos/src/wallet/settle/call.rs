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

//! The wallet's one ASYNC request to settle (v1.4.53), held until its
//! answer or STEP_MS ends it; the answer waits for the step that asked.

use alloc::vec::Vec;

use nonos_payment_proto::{EBUSY, ETIME};

use super::heard::{expired, heard, Asked, Heard};
use super::send::send;

#[derive(Default)]
pub struct SettleCall {
    asked: Option<Asked>,
    next_id: u32,
    /// The answer to the last request, by its op, until the step takes it.
    answer: Option<(u16, Result<Vec<u8>, i32>)>,
}

impl SettleCall {
    /// Send `op` with `body`; EBUSY while another is outstanding or unread.
    pub fn ask(&mut self, op: u16, body: &[u8], now_ms: u64) -> Result<(), i32> {
        if self.asked.is_some() || self.answer.is_some() {
            return Err(EBUSY);
        }
        self.next_id = self.next_id.wrapping_add(1);
        let settle_pid = send(op, self.next_id, body)?;
        self.asked = Some(Asked { op, id: self.next_id, settle_pid, at_ms: now_ms });
        Ok(())
    }

    /// A frame from a process other than the router. True when it was the
    /// answer, so the screen repaints.
    pub fn frame(&mut self, sender: u32, frame: &[u8]) -> bool {
        let Heard::Answer(r) = heard(self.asked.as_ref(), sender, frame) else { return false };
        let op = self.asked.take().map_or(0, |a| a.op);
        self.answer = Some((op, r));
        true
    }

    /// End the request ETIME once it has waited STEP_MS. True when it did.
    pub fn tick(&mut self, now_ms: u64) -> bool {
        let Some(a) = self.asked.filter(|a| expired(a, now_ms)) else { return false };
        self.asked = None;
        self.answer = Some((a.op, Err(ETIME)));
        true
    }

    /// The answer to `op`, once, for the step that asked it.
    pub fn take(&mut self, op: u16) -> Option<Result<Vec<u8>, i32>> {
        match self.answer.take() {
            Some((o, r)) if o == op => Some(r),
            other => {
                self.answer = other;
                None
            }
        }
    }
}
