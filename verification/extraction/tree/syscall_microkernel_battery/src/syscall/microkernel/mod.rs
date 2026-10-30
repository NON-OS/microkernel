// NONOS Operating System (AGPL-3.0-or-later)
//
// The kernel's own directory is mounted, not its files one by one: battery.rs
// declares `mod report;`, and only a file reached through its directory finds
// that child at battery/report.rs as the kernel does.

#[path = "../../../../../../../src/syscall/microkernel"]
mod kernel {
    pub mod battery;
    pub mod errnos;
}

pub use kernel::{battery, errnos};
pub use battery::sys_battery_status;
