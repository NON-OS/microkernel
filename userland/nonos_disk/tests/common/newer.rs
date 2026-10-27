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

//! A newer release's image, for an update to write over a disk `files`
//! installed: every file a different length or different bytes, so what
//! the update writes is told apart from what it replaced.

pub fn newer() -> crate::files::Files {
    let mut f = crate::files::files();
    f.boot_efi.iter_mut().step_by(4096).for_each(|b| *b ^= 0xFF);
    f.kernel_bin.extend_from_slice(&[0x33; 300_007]);
    f.boot_cfg = b"timeout=0\ndefault=nonos-0.9.3\n".to_vec();
    f.kernel_approval.reverse();
    f
}
