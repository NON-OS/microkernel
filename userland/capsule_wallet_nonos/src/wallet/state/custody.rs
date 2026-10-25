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

//! The secrets the window holds while a person is looking at them or typing
//! them: a private key typed for import, recovery words typed to restore, a
//! key shown for export, and a new wallet's words on its one-time backup
//! screen. Every one sits in a Zeroizing buffer, wiped when dropped as well as
//! on each path that ends its use. No Clone, Copy or Debug: nothing copies
//! or prints it.

use zeroize::Zeroizing;

pub struct Custody {
    /// The typed private key, as hex. Never drawn, only counted.
    pub import_hex: Zeroizing<[u8; 64]>,
    pub import_len: usize,
    /// The typed recovery words, space separated, shown only while typing.
    pub recover_buf: Zeroizing<[u8; 240]>,
    pub recover_len: usize,
    /// The 0x-prefixed key while it is shown for export.
    pub export_hex: Zeroizing<[u8; 66]>,
    /// A new wallet's word indices, until the person confirms them written
    /// down. Kept across a lock and never drawn while locked (README).
    pub backup_words: Zeroizing<[u16; 24]>,
    pub backup_count: u8,
}

impl Custody {
    pub fn new() -> Custody {
        Custody {
            import_hex: Zeroizing::new([0; 64]),
            import_len: 0,
            recover_buf: Zeroizing::new([0; 240]),
            recover_len: 0,
            export_hex: Zeroizing::new([0; 66]),
            backup_words: Zeroizing::new([0; 24]),
            backup_count: 0,
        }
    }
}
