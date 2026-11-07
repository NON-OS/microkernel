// NONOS Operating System (AGPL-3.0-or-later)

#[path = "../../../../../../../src/syscall/microkernel/battery.rs"]
pub mod battery;

#[path = "../../../../../../../src/syscall/microkernel/errnos.rs"]
pub mod errnos;

pub use battery::sys_battery_status;
