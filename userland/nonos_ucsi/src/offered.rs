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

//! What a charger offers, read when a connector sinks from a partner in
//! Power Delivery mode: its source PDOs, four to a command, at most seven
//! (USB Power Delivery's limit, recalled). A PPM that refuses GET_PDOS,
//! or a partner that offers fewer, ends the list where it ends.

use super::command::partner_source_pdos;
use super::exchange::{command, Fault};
use super::ppm::Ppm;
use super::status::Status;

pub const MAX_PDOS: usize = 7;
/// Power operation mode 3: Power Delivery.
const PD: u8 = 3;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Offered {
    pub count: u8,
    pub pdos: [u32; MAX_PDOS],
}

impl Offered {
    pub fn list(&self) -> &[u32] {
        &self.pdos[..usize::from(self.count)]
    }
}

/// The offers when `s` is a Power Delivery charger, else none.
pub fn read_offered(
    ppm: &mut impl Ppm,
    pending: &mut u128,
    n: u8,
    s: Option<Status>,
) -> Result<Offered, Fault> {
    let mut o = Offered::default();
    if !s.is_some_and(|s| s.charger() && s.power_mode == PD) {
        return Ok(o);
    }
    for (offset, count) in [(0u8, 4u8), (4, 3)] {
        let b = match command(ppm, pending, partner_source_pdos(n, offset, count), false) {
            Ok(b) => b,
            Err(Fault::Failed | Fault::NotSupported) => return Ok(o),
            Err(e) => return Err(e),
        };
        let words = b.chunks(4).map(|c| u32::from_le_bytes([c[0], c[1], c[2], c[3]]));
        for w in words.take(usize::from(count)) {
            if w == 0 {
                return Ok(o);
            }
            o.pdos[usize::from(o.count)] = w;
            o.count += 1;
        }
    }
    Ok(o)
}
