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

//! load_store's accepted payloads: the request it hands mk_capsule_load.

use crate::frame::{frame, reply, store};
use crate::payloads::store_payload;
use crate::protocol::OP_LOAD_FROM_STORE;
use crate::syscall::{self, N_MK_CAPSULE_LOAD};

const HEAD_LEN: u64 = 28;

#[test]
fn head_only_with_zero_lengths_is_accepted() {
    syscall::reset(40);
    let f = frame(OP_LOAD_FROM_STORE, &store_payload(0x55, [0; 5]));
    assert_eq!(store(&f), reply(0, &40u32.to_le_bytes()));
    let loads = syscall::take();
    assert_eq!(loads.len(), 1);
    let (r, at) = (loads[0].req, f.as_ptr() as u64 + 8 + HEAD_LEN);
    assert_eq!(loads[0].num, N_MK_CAPSULE_LOAD);
    assert_eq!([r.elf_ptr, r.cert_ptr, r.manifest_ptr, r.trailer_ptr], [at; 4]);
    let lens = [r.elf_len, r.cert_len, r.manifest_len, r.trailer_len, r.args_len];
    assert_eq!(lens, [0; 5]);
    assert_eq!((r.args_ptr, r.requested_caps, r.on_behalf_of), (0, 0x55, 0));
}

#[test]
fn well_formed_payload_points_each_blob_at_its_slice() {
    syscall::reset(41);
    let caps = 0x0123_4567_89AB_CDEF;
    let f = frame(OP_LOAD_FROM_STORE, &store_payload(caps, [3, 5, 7, 11, 13]));
    assert_eq!(store(&f), reply(0, &41u32.to_le_bytes()));
    let loads = syscall::take();
    assert_eq!(loads.len(), 1);
    let (r, base) = (loads[0].req, f.as_ptr() as u64 + 8);
    assert_eq!(r.elf_ptr, base + HEAD_LEN);
    assert_eq!(r.cert_ptr, base + HEAD_LEN + 3);
    assert_eq!(r.manifest_ptr, base + HEAD_LEN + 3 + 5);
    assert_eq!(r.trailer_ptr, base + HEAD_LEN + 3 + 5 + 7);
    assert_eq!(r.args_ptr, base + HEAD_LEN + 3 + 5 + 7 + 11);
    let lens = [r.elf_len, r.cert_len, r.manifest_len, r.trailer_len, r.args_len];
    assert_eq!(lens, [3, 5, 7, 11, 13]);
    assert_eq!((r.requested_caps, r.on_behalf_of), (caps, 0));
    for (i, blob) in loads[0].blobs.iter().enumerate() {
        assert_eq!(*blob, vec![0xA0 + i as u8; lens[i] as usize], "blob {i}");
    }
}

#[test]
fn negative_rc_is_the_reply_status() {
    syscall::reset(-13);
    let f = frame(OP_LOAD_FROM_STORE, &store_payload(1, [1, 1, 1, 1, 0]));
    assert_eq!(store(&f), reply(-13, &[]));
    assert_eq!(syscall::take().len(), 1);
}
