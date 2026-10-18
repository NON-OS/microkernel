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

//! Serving the error sources: each polled one at its own interval (never
//! faster than every 100 ms, whatever HEST says), and every SCI one when
//! the Error Device is notified (ACPI 6.5, 18.3.2.7.2).

use super::read::read_source;
use super::state::Apei;
use crate::sys::clock::now_ms;
use nonos_aml::apei::{POLLED, SCI};
use nonos_aml::namespace::Id;

const MIN_POLL_MS: u64 = 100;

fn prefix(id: u16) -> alloc::vec::Vec<u8> {
    let mut p = b"hw error source ".to_vec();
    p.extend_from_slice(alloc::format!("{id} ").as_bytes());
    p
}

pub fn tick(a: &mut Apei) {
    let now = now_ms();
    let Apei { sources, phys, .. } = a;
    for s in sources.iter_mut().filter(|s| s.ghes.notify == POLLED && now >= s.next_ms) {
        s.next_ms = now + u64::from(s.ghes.poll_ms).max(MIN_POLL_MS);
        read_source(phys, &s.ghes, &prefix(s.id));
    }
}

/// True when `id` is an Error Device, whatever the value.
pub fn notified(a: &mut Apei, id: Id) -> bool {
    if !a.error_devices.contains(&id) {
        return false;
    }
    let Apei { sources, phys, .. } = a;
    for s in sources.iter().filter(|s| s.ghes.notify == SCI) {
        read_source(phys, &s.ghes, &prefix(s.id));
    }
    true
}
