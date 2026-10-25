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
//! Advertising Report subevents built sound (Vol 4, Part E, 7.7.65.2).

use super::rng::Rng;

/// Up to four advertising reports: the event, and each one's fields.
/// One report as built: event type, address type, address, data, RSSI.
pub type Report = (u8, u8, [u8; 6], Vec<u8>, u8);

pub fn reports(rng: &mut Rng) -> (Vec<u8>, Vec<Report>) {
    let mut built = Vec::new();
    for _ in 0..rng.below(5) {
        let addr: [u8; 6] = rng.bytes(6).try_into().unwrap();
        built.push((rng.byte(), rng.below(4) as u8, addr, rng.span(0, 32), rng.byte()));
    }
    let mut ev = vec![built.len() as u8];
    for (t, a, addr, data, rssi) in &built {
        ev.extend([*t, *a]);
        ev.extend(addr);
        ev.push(data.len() as u8);
        ev.extend(data);
        ev.push(*rssi);
    }
    (ev, built)
}
