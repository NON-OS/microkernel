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

//! A connection's secrets are zeros where they lay once their block is
//! dropped.

use core::mem::ManuallyDrop;
use core::ptr;

use crate::traffic_keys::Secrets;

fn read(p: *const u8, n: usize) -> Vec<u8> {
    // SAFETY: each caller's pointer is into storage it still owns.
    (0..n).map(|i| unsafe { ptr::read_volatile(p.add(i)) }).collect()
}

#[test]
fn a_dropped_block_of_secrets_is_zeros_where_it_lay() {
    let (handshake_secret, client_secret, server_secret) = ([1; 32], [2; 32], [3; 32]);
    let (client_key, client_iv, server_key, server_iv) = ([4; 32], [5; 12], [6; 32], [7; 12]);
    let all = Secrets {
        handshake_secret,
        client_secret,
        server_secret,
        client_key,
        client_iv,
        server_key,
        server_iv,
    };
    let mut s = ManuallyDrop::new(all);
    let at = &*s as *const Secrets as *const u8;
    let size = core::mem::size_of::<Secrets>();
    assert!(size >= 184, "every secret is inside the block");
    // SAFETY: dropped once; the storage is only read afterwards.
    unsafe { ManuallyDrop::drop(&mut s) };
    assert!(read(at, size).iter().all(|b| *b == 0));
}
