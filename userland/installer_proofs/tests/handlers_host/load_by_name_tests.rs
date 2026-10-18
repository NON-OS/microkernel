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

//! load_by_name: what it reads, and who the spawned capsule is attributed to.

use crate::fixtures::{artifact, put_artifacts, NAME};
use crate::frame::{by_name, by_name_payload as payload, frame, reply};
use crate::protocol::OP_LOAD_BY_NAME;
use crate::syscall;

#[test]
fn on_behalf_of_is_the_sender_pid_and_never_the_payload() {
    // Every pid-like value the payload and the artifacts carry differs from
    // the sender pids passed in.
    let args = [0xDEAD_BEEFu32.to_le_bytes(), 9u32.to_le_bytes()].concat();
    let p = payload(u64::from(0xDEAD_BEEFu32), &args);
    let f = frame(OP_LOAD_BY_NAME, &p);
    for sender in [4242, 1, u32::MAX] {
        put_artifacts(NAME, None);
        syscall::reset(55);
        assert_eq!(by_name(&f, sender), reply(0, &55u32.to_le_bytes()));
        let loads = syscall::take();
        assert_eq!(loads.len(), 1);
        let r = loads[0].req;
        assert_eq!(r.on_behalf_of, sender);
        assert_eq!(r.requested_caps, 0xDEAD_BEEF);
        let args_at = f.as_ptr() as u64 + 8 + 9 + NAME.len() as u64;
        assert_eq!((r.args_ptr, r.args_len), (args_at, 8));
        let lens = [r.elf_len, r.cert_len, r.manifest_len, r.trailer_len];
        assert_eq!(lens, [4, 8, 12, 16]);
        for i in 0..4 {
            assert_eq!(loads[0].blobs[i], artifact(i), "artifact {i}");
        }
        assert_eq!(loads[0].blobs[4], args);
    }
}

#[test]
fn empty_args_give_args_ptr_zero() {
    put_artifacts(NAME, None);
    syscall::reset(3);
    by_name(&frame(OP_LOAD_BY_NAME, &payload(0, &[])), 77);
    let r = syscall::take()[0].req;
    assert_eq!((r.args_ptr, r.args_len, r.on_behalf_of), (0, 0, 77));
}
