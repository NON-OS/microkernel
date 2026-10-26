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

//! S-14: the installers restart through capsule_power and hold no Admin.
//! The request they send is the `power` port's reboot (who_tests.rs says
//! how the capsule tells them from the shell).

const OPS: &str = include_str!("../../capsule_power/src/protocol/ops.rs");
const HEADER: &str = include_str!("../../capsule_power/src/protocol/header.rs");
const RESTART: &str = include_str!("../../libc/src/admin/restart.rs");
const WINDOW: &str = include_str!("../../capsule_install/src/install/event/after.rs");
const CLI: &str = include_str!("../../tool_install/src/cli/receipt.rs");
const SPAWN: &str = include_str!("../../../src/userspace/capsule_install/spawn.rs");
const WINDOW_MK: &str = include_str!("../../capsule_install/Capsule.mk");
const CLI_MK: &str = include_str!("../../tool_install/Capsule.mk");

fn flat(s: &str) -> String {
    s.lines()
        .map(|l| l.split("//").next().unwrap())
        .collect::<String>()
        .split_whitespace()
        .collect()
}

#[test]
fn the_installers_send_the_power_ports_reboot() {
    let r = flat(RESTART);
    assert!(r.contains("constREQUEST:[u8;20]=[b'R',b'W',b'O',b'P',1,0,2,0,"));
    assert!(flat(HEADER).contains("pubconstMAGIC:u32=0x504F_5752;pubconstVERSION:u16=1;"));
    assert!(flat(OPS).contains("pubconstOP_REBOOT:u16=0x0002;"));
    assert_eq!(u32::from_le_bytes(*b"RWOP"), 0x504F_5752);
    assert!(r.contains("mk_service_lookup(name.as_ptr()") && r.contains("(b\"power\",0u32,0u32)"));
    for src in [WINDOW, CLI] {
        assert!(flat(src).contains("power_restart()") && !src.contains("mk_admin_reboot"));
    }
    // A refusal is said, never dropped: the window's Done line, the CLI's error.
    assert!(flat(WINDOW).contains("letrc=power_restart();state.notice=Some(alloc::format!("));
    assert!(flat(CLI).contains("letrc=nonos_libc::power_restart();eprintln!("));
}

#[test]
fn neither_installer_holds_admin() {
    let caps = &SPAWN[SPAWN.find("pub const CLI_CAPS").expect("CLI_CAPS")..];
    let caps = &caps[..caps.find(';').unwrap()];
    assert!(!caps.contains("Admin"), "CLI_CAPS, the grant of both installers");
    assert!(WINDOW_MK.contains("CAPSULE_REQUIRED_CAPS    := 0x84009879\n"));
    assert!(CLI_MK.contains("CAPSULE_REQUIRED_CAPS      := 0x04008079\n"));
    assert_eq!(0x84009A79u64 & !0x200, 0x84009879);
}
