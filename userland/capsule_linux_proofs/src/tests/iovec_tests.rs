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

//! An iovec array is checked as Linux's import_iovec checks it, after the
//! descriptor: a closed one is EBADF even with an empty array, which is
//! what keeps pwritev2 with RWF_SYNC from reaching a descriptor that is
//! not there.

use super::fake_memory::Fake;
use super::random::Regs;
use crate::linux::abi::errno::{EBADF, EFAULT, EINVAL};
use crate::linux::call::iovec::{iovecs, vector, IOVEC, IOV_MAX};

fn table(mem: &Fake, at: u64, pieces: &[(u64, u64)]) {
    for (i, (base, len)) in pieces.iter().enumerate() {
        mem.put(at + (i * IOVEC) as u64, &base.to_le_bytes());
        mem.put(at + (i * IOVEC) as u64 + 8, &len.to_le_bytes());
    }
}

#[test]
fn a_closed_descriptor_is_ebadf_before_the_array_even_an_empty_one() {
    let mem = Fake::new(0x4000, 0x100);
    assert_eq!(vector(&mem, false, 0, 0), Err(EBADF));
    assert_eq!(vector(&mem, false, 0x4000, IOV_MAX + 1), Err(EBADF));
    assert_eq!(vector(&mem, true, 0, 0), Ok(vec![]));
}

#[test]
fn the_array_is_refused_as_import_iovec_refuses_it() {
    let mem = Fake::new(0x4000, IOVEC * IOV_MAX as usize);
    assert_eq!(iovecs(&mem, 0x4000, IOV_MAX + 1), Err(EINVAL));
    assert_eq!(iovecs(&mem, 0x4000, IOV_MAX).map(|v| v.len()), Ok(IOV_MAX as usize));
    assert_eq!(iovecs(&mem, 0x1000, 1), Err(EFAULT));
    table(&mem, 0x4000, &[(0x7000, 5), (0x8000, 1 << 63)]);
    assert_eq!(iovecs(&mem, 0x4000, 2), Err(EINVAL), "a negative ssize_t");
    table(&mem, 0x4000, &[(0x7000, 5), (0x8000, i64::MAX as u64)]);
    assert_eq!(iovecs(&mem, 0x4000, 2), Ok(vec![(0x7000, 5), (0x8000, i64::MAX as u64)]));
}

#[test]
fn random_arrays_never_panic_and_never_hand_on_a_negative_length() {
    let mut r = Regs::new(0x696f_7665_635f_7465);
    let mem = Fake::new(0x4000, 0x400);
    for _ in 0..20_000 {
        let at = 0x4000 + r.small(0x400);
        if at + 8 <= 0x4400 {
            mem.put(at & !7, &r.arg().to_le_bytes());
        }
        let (count, open) = (if r.small(4) == 0 { r.arg() } else { r.small(80) }, r.small(8) != 0);
        if let Ok(pieces) = vector(&mem, open, if r.small(8) == 0 { r.arg() } else { at }, count) {
            assert!(open && count <= IOV_MAX && pieces.len() as u64 == count);
            assert!(pieces.iter().all(|&(_, len)| (len as i64) >= 0));
        }
    }
}
