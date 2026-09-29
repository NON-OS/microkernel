// NONOS Operating System (AGPL-3.0-or-later)

#[path = "../../../../../../../../../../src/arch/x86_64/interrupt/apic/idle_timer/consts.rs"]
pub mod consts;

#[path = "../../../../../../../../../../src/arch/x86_64/interrupt/apic/idle_timer/halt_safe.rs"]
pub mod halt_safe;

pub use halt_safe::halt_safe;
