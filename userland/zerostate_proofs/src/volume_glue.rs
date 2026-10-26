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

//! The kernel asks the volume rule where it matters: before the disk's
//! volume is keyed and before a sector reaches the disk.

const OPEN: &str = include_str!("../../../src/fs/blockfs_volume/open_machine.rs");
pub(crate) const SWITCH: &str = include_str!("../../../src/fs/blockfs_volume/keep_switch.rs");
const SESSION: &str = include_str!("../../../src/fs/blockfs_volume/session.rs");
const DEVICE: &str = include_str!("../../../src/fs/cryptoblock/device.rs");
const HEADER: &str = include_str!("../../../src/fs/blockfs_volume/key_header_io.rs");
const PASS: &str = include_str!("../../../src/fs/blockfs_volume/passphrase.rs");

pub(crate) fn flat(s: &str) -> String {
    s.lines()
        .map(|l| l.split("//").next().unwrap())
        .collect::<String>()
        .split_whitespace()
        .collect()
}

/// `first` comes before `then` in `src`, both present.
pub(crate) fn before(src: &str, first: &str, then: &str) {
    let s = flat(src);
    let a = s.find(first).unwrap_or_else(|| panic!("lost `{first}`"));
    let b = s.find(then).unwrap_or_else(|| panic!("lost `{then}`"));
    assert!(a < b, "`{first}` must come before `{then}`");
}

#[test]
fn the_disk_s_volume_is_keyed_only_after_the_rule_said_disk() {
    before(OPEN, "keep_switch::in_memory(plan.is_live())", "set_window(plan.volume_base");
    before(OPEN, "keep_switch::in_memory(plan.is_live())", "derive_for_kernel(KEY_LABEL)");
    before(
        SWITCH,
        "ifplace(!live,keeping())==Place::Disk{returnNone;}",
        "open_session_volume(!live)",
    );
    before(SWITCH, "open_session_volume(!live)", "if!live&&keeping(){keeping_on();returnNone;}");
    before(SESSION, "*volume=Some(VolumeState{key,mount});", "keep_switch::HELD.store(held,");
    before(PASS, "ifplace(true,crate::security::zerostate::keeping())==Place::Ram{", "set_window(");
}

#[test]
fn no_sector_reaches_the_disk_before_the_boot_keeps() {
    before(
        DEVICE,
        "if!may_write(ram,crate::security::zerostate::keeping()){",
        "block_device::write(lba,bytes)",
    );
    before(
        DEVICE,
        "returnErr(BlockDeviceError::AccessDenied);",
        "ifram{returnsuper::ram::write(lba,bytes);}",
    );
    before(
        HEADER,
        "if!may_write(false,crate::security::zerostate::keeping()){",
        "write_plan_sector(KEY_INDEX",
    );
}
