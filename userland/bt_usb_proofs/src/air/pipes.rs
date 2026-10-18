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

//! The air as the USB pipes the capsule's HCI transport drives. Each
//! transfer is no longer than the transport armed, or the proof fails as
//! babble would. Once the host has paused QUIET times in a row with
//! nothing to take, the next scripted act runs.

use super::act::{notification, Act};
use super::give::give;
use super::state::{Air, HANDLE};
use crate::hci_sim::link_events::disconnected;
use nonos_bt_hci::usb::{Setup, UsbPipes, COMMAND};
use nonos_bt_wire::l2cap::cid::{ATT, LE_SIGNALING};

const QUIET: u32 = 50;
/// A link the test forgot to end would spin here for good.
const PAUSES_MAX: u64 = 5_000_000;

impl UsbPipes for Air {
    fn control_out(&mut self, setup: Setup, data: &[u8]) -> Result<(), i32> {
        assert_eq!(setup, COMMAND, "a command not sent as Vol 4, Part B, 2.2 has it");
        self.command(data);
        Ok(())
    }
    fn interrupt_in_poll(&mut self, out: &mut [u8]) -> Result<Option<usize>, i32> {
        let given = give(&mut self.interrupt, &mut self.idle, out)?;
        given.inspect(|_| self.read_one());
        Ok(given)
    }
    fn bulk_out(&mut self, data: &[u8]) -> Result<usize, i32> {
        self.take_from_host(data);
        Ok(data.len())
    }
    fn bulk_in_poll(&mut self, out: &mut [u8]) -> Result<Option<usize>, i32> {
        give(&mut self.bulk_in, &mut self.idle, out)
    }
    fn pause(&mut self) {
        self.pauses += 1;
        assert!(self.pauses < PAUSES_MAX, "the link never ended");
        for done in std::mem::take(&mut self.held) {
            self.event(&done);
        }
        self.idle += 1;
        if self.idle < QUIET {
            return;
        }
        self.idle = 0;
        match self.script.pop_front() {
            Some(Act::Notify(h, v)) => self.send_to_host(ATT, &notification(h, &v)),
            Some(Act::Signal(frame)) => self.send_to_host(LE_SIGNALING, &frame),
            Some(Act::Drop(reason)) => self.event(&disconnected(HANDLE, reason)),
            Some(Act::Event(e)) => self.event(&e),
            Some(Act::Events(all)) => all.iter().for_each(|e| self.event(e)),
            Some(Act::Acl(packets)) => self.bulk_in.extend(packets),
            Some(Act::Lie(lie)) => lie(&mut self.lies),
            Some(Act::Together(e, f)) => (self.event(&e), self.send_to_host(LE_SIGNALING, &f)).1,
            None => {}
        }
    }
}
