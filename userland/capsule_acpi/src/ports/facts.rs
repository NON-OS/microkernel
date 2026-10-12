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

//! One namespace port as the kernel's wire carries it.

use super::wire::{PortFact, CONNECTABLE, DOCK, LID, PLD, REACH_SHIFT, UPC, VISIBLE};
use nonos_aml::ports::{Reach, UsbPort};

pub fn fact(p: &UsbPort) -> PortFact {
    let mut flags = 0;
    let (mut connector, mut panel) = (0, 0);
    if let Some(u) = p.upc {
        flags |= UPC | if u.connectable { CONNECTABLE } else { 0 };
        connector = u.connector;
    }
    if let Some(l) = p.pld {
        flags |= PLD;
        flags |= if l.user_visible { VISIBLE } else { 0 };
        flags |= if l.lid { LID } else { 0 };
        flags |= if l.dock { DOCK } else { 0 };
        // Panel's variants run in the field's value order, 0 to 6.
        panel = l.panel as u8;
    }
    let reach = match p.reach() {
        Reach::Unknown => 0,
        Reach::Internal => 1,
        Reach::External => 2,
    };
    flags |= reach << REACH_SHIFT;
    PortFact { parent: p.parent, port: p.port, flags, connector, panel }
}
