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
//! The gateway key's slot: the connection it belongs to, and its wipe.

/// The connection a key was registered on.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Link {
    pub ip: [u8; 4],
    pub port: u16,
    pub stream: u32,
}

pub(crate) struct Slot {
    pub(crate) key: [u8; 32],
    pub(crate) owner: Option<Link>,
}

pub(crate) fn wipe_slot(slot: &mut Slot) {
    crate::crypto::wipe::wipe(&mut slot.key);
    slot.owner = None;
}
