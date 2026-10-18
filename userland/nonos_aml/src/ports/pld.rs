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

//! _PLD, Physical Location of Device (ACPI 6.5, 6.1.8): a package of
//! buffers, each a bit-packed description, 16 bytes in revision 1 and 20
//! in revision 2. Read here: the revision (bits 6:0), user visible (bit
//! 64), dock (65), lid (66) and panel (69:67). A buffer shorter than its
//! revision needs is refused, not read in part.

use super::panel::{Panel, PANELS};
use crate::object::Object;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Pld {
    pub revision: u8,
    pub user_visible: bool,
    pub dock: bool,
    pub lid: bool,
    pub panel: Panel,
}

fn read(b: &[u8]) -> Option<Pld> {
    let revision = b.first()? & 0x7F;
    let need = match revision {
        0 => return None,
        1 => 16,
        _ => 20,
    };
    if b.len() < need {
        return None;
    }
    let bit = |n: usize| b[n / 8] & (1 << (n % 8)) != 0;
    let panel = (b[8] >> 3) & 7;
    let panel = PANELS.get(usize::from(panel)).copied().unwrap_or(Panel::Unknown);
    Some(Pld { revision, user_visible: bit(64), dock: bit(65), lid: bit(66), panel })
}

/// The first buffer of the package; a bare buffer is taken as well, since
/// some firmware returns one.
pub fn pld(o: &Object) -> Option<Pld> {
    match o {
        Object::Buffer(b) => read(&b.borrow()),
        Object::Package(p) => match p.borrow().first()? {
            Object::Buffer(b) => read(&b.borrow()),
            _ => None,
        },
        _ => None,
    }
}
