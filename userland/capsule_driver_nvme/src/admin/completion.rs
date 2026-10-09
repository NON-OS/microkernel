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

#[repr(C, align(16))]
#[derive(Clone, Copy)]
pub struct Completion {
    pub dw0: u32,
    pub dw1: u32,
    pub sq_head: u16,
    pub sq_id: u16,
    pub cid: u16,
    pub status: u16,
}

impl Completion {
    pub const fn phase(self) -> bool {
        (self.status & 1) != 0
    }

    pub const fn successful(self) -> bool {
        (self.status >> 1) == 0
    }

    /// Status Code (SC), status field bits 8:1.
    pub const fn status_code(self) -> u8 {
        ((self.status >> 1) & 0xff) as u8
    }

    /// Status Code Type (SCT), bits 11:9: 0 generic, 1 command specific,
    /// 2 media and data integrity, 3 path related, 7 vendor specific.
    pub const fn status_code_type(self) -> u8 {
        ((self.status >> 9) & 0x7) as u8
    }

    /// Do Not Retry (DNR), bit 15: the same command would fail again.
    pub const fn do_not_retry(self) -> bool {
        (self.status & 0x8000) != 0
    }
}

const _: () = assert!(core::mem::size_of::<Completion>() == 16);
