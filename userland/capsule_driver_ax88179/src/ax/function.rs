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

//! The chip's interface in one configuration: vendor class FFh, subclass
//! FFh, protocol 0, at alternate 0, as Linux matches it, with a bulk IN and
//! a bulk OUT; and the interrupt IN its link reports come on, when the
//! interface has one whose packet size and interval USB allows.

use nonos_usbnet::desc::{interfaces, notify_ep, CLASS_VENDOR};
use nonos_usbnet::{Notify, Pipes};

const SUBCLASS_VENDOR: u8 = 0xFF;
const PROTOCOL: u8 = 0x00;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct AxFunction {
    pub interface: u8,
    pub pipes: Pipes,
    pub status: Option<Notify>,
}

pub fn find_ax(raw: &[u8]) -> Option<AxFunction> {
    let ours = |i: &&nonos_usbnet::desc::Interface| {
        (i.class, i.subclass, i.protocol, i.alt) == (CLASS_VENDOR, SUBCLASS_VENDOR, PROTOCOL, 0)
            && i.has_bulk_pair()
    };
    let ifs = interfaces(raw);
    let i = ifs.iter().find(ours)?;
    Some(AxFunction { interface: i.number, pipes: i.pipes, status: notify_ep(raw, i.number) })
}
