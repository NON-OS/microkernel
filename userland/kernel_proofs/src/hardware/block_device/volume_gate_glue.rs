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

//! Every block client asks the one gate, the gate asks the rule with the
//! kernel's own facts, and only crate::fs opens a volume scope: the token's
//! field is private to fs::cryptoblock and the scope is fs-only.

const SCOPE: &str = include_str!("../../../../../src/hardware/block_device/volume_scope.rs");
const TOKEN: &str = include_str!("../../../../../src/fs/cryptoblock/volume_io.rs");
const GATES: [&str; 5] = [
    include_str!("../../../../../src/hardware/nvme_capsule/capability.rs"),
    include_str!("../../../../../src/hardware/ahci_capsule/capability.rs"),
    include_str!("../../../../../src/hardware/usb_msc_capsule/capability.rs"),
    include_str!("../../../../../src/hardware/virtio_blk_capsule/capability.rs"),
    include_str!("../../../../../src/hardware/block_device/wire/gate.rs"),
];

fn flat(s: &str) -> String {
    s.lines()
        .map(|l| l.split("//").next().unwrap())
        .collect::<String>()
        .split_whitespace()
        .collect()
}

#[test]
fn every_block_client_asks_the_one_gate() {
    for g in GATES {
        let f = flat(g);
        assert!(f.contains("if!crate::hardware::block_device::disk_caller_admitted(pid){"));
        assert!(!f.contains("has_capability("), "no gate of its own beside the rule");
    }
}

#[test]
fn the_gate_asks_the_rule_for_the_thread_and_the_guard_lets_go() {
    let s = flat(SCOPE);
    assert!(s.contains("admits(inside(&seen(),tid),has_capability(tid,CAP_DRIVER),has_capability(tid,CAP_STORAGE))"));
    assert!(
        s.contains("lettid=crate::process::current_pid()?;"),
        "the thread's own, not its process's"
    );
    assert!(
        s.contains("for_in0..SLOTS{letslot=free_slot(&seen())?;"),
        "bounded, none past the last"
    );
    assert!(s.contains("fndrop(&mutself){INSIDE[self.slot].store(0,Ordering::Release);}"));
    assert_eq!(s.matches(".store(0,").count(), 1, "released by the guard's drop alone");
}

#[test]
fn only_crate_fs_opens_a_scope_and_a_full_table_is_refused() {
    assert!(flat(SCOPE).contains("pubfnvolume_scope(_:&VolumeIo)->Option<VolumeScope>{"));
    let t = flat(TOKEN);
    assert!(t.contains("pubstructVolumeIo(());"), "a private field: made in volume_io.rs alone");
    assert!(t.contains("pub(incrate::fs)fnvolume_scope()->Result<VolumeScope,BlockDeviceError>{"));
    assert!(t.contains(".ok_or_else(||{") && t.contains("BlockDeviceError::NotReady"), "EAGAIN");
    assert!(t.contains("[DATA]volumebusy:16callersinside;tryagain"));
}
