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
//! Every head the driver published since the last yield, served the way a
//! device does: request from the first descriptor, answer into the second,
//! completion as a used entry naming the head. A dead host takes the head
//! and writes nothing back, so the driver's wait runs out. A ring the driver
//! wiped after a reset (`init::restart`) has the host drop what a reset
//! drops (`fresh::marked`).

use std::sync::atomic::{fence, Ordering};

use nonos_devmodel::FakeBar;

use super::state::{Cmd, Host};
use super::words::at;
use super::words::display_info;
use crate::constants::{
    VG_CMD_GET_DISPLAY_INFO, VG_RESP_OK_NODATA, VQ_AVAIL_OFFSET, VQ_DESC_OFFSET, VQ_USED_OFFSET,
};
use crate::tests::model::QUEUE_SIZE;
use crate::tests::part::answers::{header, RESP_ERR_UNSPEC};
use crate::tests::part::ring::{bytes, desc, put, u16_at};

impl Host {
    /// True when the driver is to be killed now, at `die_at`.
    pub fn serve(&mut self, bar: &FakeBar) -> bool {
        let mut killed = false;
        self.marked(bar);
        while self.seen != u16_at(bar, VQ_AVAIL_OFFSET + 2) {
            fence(Ordering::Acquire);
            let head = u16_at(bar, VQ_AVAIL_OFFSET + 4 + (self.seen % QUEUE_SIZE) as usize * 2);
            self.seen = self.seen.wrapping_add(1);
            let first = desc(bar, head, VQ_DESC_OFFSET);
            let second = desc(bar, first.next, VQ_DESC_OFFSET);
            let request = bytes(bar, at(bar, first.addr, first.len), first.len as usize);
            let kind = u32::from_le_bytes(request[..4].try_into().unwrap());
            let step = self.steps() + 1;
            killed |= self.die_at == Some(step);
            if self.dead(step) {
                self.log.push(Cmd { kind, landed: false });
                continue;
            }
            let ok = self.refuse_at != Some(step) && self.apply(kind, &request);
            self.log.push(Cmd { kind, landed: ok });
            let mut answer = match (ok, kind) {
                (true, VG_CMD_GET_DISPLAY_INFO) => display_info(&self.displays),
                (true, _) => header(VG_RESP_OK_NODATA, 0, 0),
                (false, _) => header(RESP_ERR_UNSPEC, 0, 0),
            };
            answer.truncate(second.len as usize);
            put(bar, at(bar, second.addr, second.len), &answer);
            let entry = VQ_USED_OFFSET + 4 + (self.used % QUEUE_SIZE) as usize * 8;
            put(bar, entry, &(head as u32).to_le_bytes());
            put(bar, entry + 4, &(answer.len() as u32).to_le_bytes());
            self.used = self.used.wrapping_add(1);
            fence(Ordering::Release);
            put(bar, VQ_USED_OFFSET + 2, &self.used.to_le_bytes());
        }
        killed
    }
}
