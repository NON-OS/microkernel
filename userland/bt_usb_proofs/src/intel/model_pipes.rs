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

//! The model's pipes: commands in on the default endpoint, Secure Send on
//! bulk OUT, events out on the interrupt endpoint in 16-octet transfers, or
//! on bulk IN while `bulk_events` is set and the firmware has not booted.

use std::collections::VecDeque;

use super::model::Intel;
use nonos_bt_hci::usb::{Setup, UsbPipes, COMMAND};

impl UsbPipes for Intel {
    fn control_out(&mut self, setup: Setup, data: &[u8]) -> Result<(), i32> {
        assert_eq!(setup, COMMAND, "a command not sent as Vol 4, Part B, 2.2 has it");
        self.command(data);
        Ok(())
    }
    fn interrupt_in_poll(&mut self, out: &mut [u8]) -> Result<Option<usize>, i32> {
        Ok(give(&mut self.interrupt, out))
    }
    fn bulk_out(&mut self, data: &[u8]) -> Result<usize, i32> {
        self.secure_send(data);
        Ok(data.len())
    }
    fn bulk_in_poll(&mut self, out: &mut [u8]) -> Result<Option<usize>, i32> {
        Ok(give(&mut self.bulk_in, out))
    }
    fn pause(&mut self) {
        self.pauses += 1;
        assert!(self.pauses < 100_000, "the loader never stopped waiting");
    }
}

fn give(q: &mut VecDeque<Vec<u8>>, out: &mut [u8]) -> Option<usize> {
    let t = q.pop_front()?;
    assert!(t.len() <= out.len(), "a transfer longer than the armed length is babble");
    out[..t.len()].copy_from_slice(&t);
    Some(t.len())
}

impl Intel {
    pub(super) fn complete(&mut self, op: u16, status: u8, ret: &[u8]) {
        let [lo, hi] = op.to_le_bytes();
        self.event(&[&[0x0e, (4 + ret.len()) as u8, 1, lo, hi, status][..], ret].concat());
    }

    pub(super) fn event(&mut self, e: &[u8]) {
        let bulk = self.bulk_events && !self.booted;
        let (q, cut) = if bulk { (&mut self.bulk_in, 64) } else { (&mut self.interrupt, 16) };
        q.extend(e.chunks(cut).map(<[u8]>::to_vec));
    }
}
