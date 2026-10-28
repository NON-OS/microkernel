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

//! The loader's side of the pipes. In its bootloader an Intel controller
//! may send events on bulk IN as well as the interrupt endpoint, and btusb
//! takes both as events (`btusb_recv_bulk_intel`); so does this. A wait
//! counts looks, and each empty one pauses about a millisecond.

use alloc::vec;
use alloc::vec::Vec;

use nonos_bt_hci::usb::{Joiner, UsbHci, UsbPipes};
use nonos_bt_hci::{HciError, Transport};

/// A whole line said on the capsule's log.
pub type Say = fn(&[&[u8]]);
type Polled<R> = Result<Option<R>, HciError>;

/// Bulk IN is read a full-speed packet at a time.
const BULK_READ: usize = 64;

pub struct Wire<'a, P> {
    pub(super) hci: &'a mut UsbHci<P>,
    bulk: (Joiner, Vec<u8>),
    /// The last Secure Send Result met, which may beat a Command Complete.
    pub result: Option<u8>,
    pub say: Say,
}

impl<'a, P: UsbPipes> Wire<'a, P> {
    pub fn new(hci: &'a mut UsbHci<P>, say: Say) -> Self {
        Self { hci, bulk: (Joiner::events(), vec![0; BULK_READ]), result: None, say }
    }

    /// The first event `pick` takes within `looks` looks.
    pub fn wait<R>(&mut self, looks: u32, mut pick: impl FnMut(&[u8]) -> Option<R>) -> Polled<R> {
        for _ in 0..looks {
            match self.event()?.map(|e| pick(&e)) {
                Some(Some(r)) => return Ok(Some(r)),
                Some(None) => {}
                None => self.hci.pause(),
            }
        }
        Ok(None)
    }

    fn event(&mut self) -> Polled<Vec<u8>> {
        let e = match self.hci.poll_event()? {
            Some(e) => Some(e),
            None => {
                let (j, buf) = &mut self.bulk;
                let got = self.hci.pipes().bulk_in_poll(buf).map_err(HciError::Transport)?;
                got.inspect(|&n| j.push(&buf[..n.min(BULK_READ)]));
                j.take()?
            }
        };
        if let Some([0xff, 5, 0x06, r, ..]) = e.as_deref() {
            self.result = Some(*r);
        }
        Ok(e)
    }
}
