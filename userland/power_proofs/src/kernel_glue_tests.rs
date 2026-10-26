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

//! The kernel's and the shell's side, read from the source: the kernel
//! registers `power.off` for capsule_power when it spawns it, on the port
//! Capsule.mk declares, and HELD lets the shell alone address it; nothing
//! else registers the name; AdminShutdown is zerostate's terminate; the
//! shell sends to `power.off` and says the power-off is not available only
//! when the name is not registered.

const POWER_OFF: &str = include_str!("../../../src/userspace/capsule_power/power_off.rs");
const SPAWN: &str = include_str!("../../../src/userspace/capsule_power/spawn.rs");
const PLAN: &str = include_str!("../../../src/userspace/init/spawn_plan/platform_acpi.rs");
const HELD: &str = include_str!("../../../src/services/registry/held_table.rs");
const ADMIN: &str = include_str!("../../../src/syscall/dispatch/router/admin/route.rs");
const SHUTDOWN: &str = include_str!("../../../src/syscall/dispatch/router/admin/shutdown.rs");
const SEND: &str = include_str!("../../capsule_desktop_shell/src/power_off_client/send.rs");
const ANSWER: &str = include_str!("../../capsule_desktop_shell/src/server/handlers/power_off.rs");

#[test]
fn the_kernel_registers_power_off_at_spawn_on_the_declared_port() {
    assert!(POWER_OFF.contains("const NAME: &str = \"power.off\";\n"));
    assert!(POWER_OFF.contains("const PORT: u32 = 4446;\n"));
    let spawned = SPAWN.find("let pid = capsule_spawn::spawn_verified(").expect("spawn");
    let registered = SPAWN.find("super::power_off::register(pid);").expect("register");
    assert!(spawned < registered);
    assert!(PLAN.contains("c::spawn_power_capsule"));
    // First, so the name is the power service's even with an ACPI capsule
    // built in, whose mirror would register it too.
    let order =
        "fn spawn() {\n    spawn_power();\n    crate::arch::acpi_first_pass_wait(spawn_acpi());";
    assert!(PLAN.contains(order));
}

#[test]
fn held_lets_the_shell_and_the_installers_address_power_off() {
    assert!(
        HELD.contains("(\"power.off\", &[\"desktop_shell\", \"app.install\", \"tool.install\"]),")
    );
    assert_eq!(HELD.matches("\"power.off\"").count(), 1);
}

#[test]
fn admin_shutdown_is_zerostates_terminate() {
    assert!(ADMIN.contains("SyscallNumber::AdminShutdown => shutdown(),"));
    assert!(SHUTDOWN.contains("terminate(PowerOff::Shutdown)"));
}

#[test]
fn the_shell_says_not_available_only_when_the_name_is_missing() {
    assert!(SEND.contains("if rc < 0 || port == 0 {\n        return Sent::Missing;"));
    assert!(ANSWER.contains("Sent::Missing => POWER_OFF_UNAVAILABLE,"));
    // One path: the shell asks power.off and nothing else.
    assert!(!SEND.contains("b\"power\"") && !ANSWER.contains("mk_admin_shutdown"));
}
