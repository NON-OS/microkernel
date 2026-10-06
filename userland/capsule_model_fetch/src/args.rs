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

/* The words the Terminal started this with, as the kernel keeps them. */

use alloc::vec;
use alloc::vec::Vec;

use nonos_libc::mk_args;

pub fn words() -> Vec<Vec<u8>> {
    let need = mk_args(core::ptr::null_mut(), 0);
    if need <= 0 {
        return Vec::new();
    }
    let mut blob = vec![0u8; need as usize];
    let got = mk_args(blob.as_mut_ptr(), blob.len());
    if got < 0 {
        return Vec::new();
    }
    blob.truncate(got as usize);
    blob.split(|&b| b == 0).filter(|w| !w.is_empty()).map(<[u8]>::to_vec).collect()
}
