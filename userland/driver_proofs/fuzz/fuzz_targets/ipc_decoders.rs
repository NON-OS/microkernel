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

//! Any bytes as an IPC request to a driver capsule or the vfs: every decoder
//! is total, and decodes the same bytes the same way twice.

#![no_main]

use libfuzzer_sys::fuzz_target;

macro_rules! twice {
    ($f:path, $b:expr) => {
        assert!($f($b).is_some() == $f($b).is_some())
    };
}

fuzz_target!(|data: &[u8]| {
    let Some((&which, b)) = data.split_first() else {
        return;
    };
    match which % 8 {
        0 => twice!(driver_proofs::protocol::decode_request, b),
        1 => twice!(nvme_proofs::protocol::decode_request, b),
        2 => twice!(virtio_blk_proofs::protocol::decode_request, b),
        3 => twice!(e1000_proofs::protocol::decode_request, b),
        4 => twice!(rtl8139_proofs::protocol::decode_request, b),
        5 => twice!(xhci_proofs::protocol::decode_request, b),
        6 => twice!(virtio_net_proofs::protocol::decode_request, b),
        _ => {
            let first = fs_proofs::vfs_protocol::decode_request(b).is_ok();
            assert!(fs_proofs::vfs_protocol::decode_request(b).is_ok() == first);
        }
    }
});
