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

//! The PPM as `nonos_ucsi` reaches it: the mailbox at the _CRS window, the
//! _DSM's write and read, and a millisecond between polls.

use super::dsm::{call, READ, WRITE};
use super::state::TypeC;
use crate::host::{read_shared, write_shared, CapsuleHost};
use nonos_aml::load::Aml;
use nonos_aml::namespace::Id;
use nonos_ucsi::layout::{CCI, CONTROL, MESSAGE_IN};
use nonos_ucsi::ppm::Ppm;

pub struct AcpiPpm<'a> {
    pub aml: &'a mut Aml,
    pub host: &'a mut CapsuleHost,
    pub dev: Id,
    pub mailbox: (u64, u64),
    pub revision: u64,
}

impl<'a> AcpiPpm<'a> {
    pub fn new(aml: &'a mut Aml, host: &'a mut CapsuleHost, t: &TypeC) -> Self {
        AcpiPpm { aml, host, dev: t.dev, mailbox: t.mailbox, revision: t.revision }
    }
}

impl Ppm for AcpiPpm<'_> {
    fn write_control(&mut self, control: u64) -> bool {
        write_shared(self.host, self.mailbox.0 + CONTROL, 8, self.mailbox, control)
    }
    fn read_cci(&mut self) -> Option<u32> {
        read_shared(self.host, self.mailbox.0 + CCI, 4, self.mailbox).map(|v| v as u32)
    }
    fn read_message(&mut self, out: &mut [u8; 16]) -> bool {
        let at = self.mailbox.0 + MESSAGE_IN;
        out.chunks_exact_mut(8).enumerate().all(|(i, c)| {
            let v = read_shared(self.host, at + 8 * i as u64, 8, self.mailbox);
            v.map(|v| c.copy_from_slice(&v.to_le_bytes())).is_some()
        })
    }
    fn signal_write(&mut self) -> bool {
        call(self.aml, self.host, self.dev, self.revision, WRITE).is_some()
    }
    fn signal_read(&mut self) -> bool {
        call(self.aml, self.host, self.dev, self.revision, READ).is_some()
    }
    fn pause(&mut self) {
        crate::sys::clock::sleep_ms(1);
    }
}
