// NONOS Operating System (AGPL-3.0-or-later)
#[path = "../../../../src/arch/aarch64/gic/intid.rs"]
pub mod intid;
// The handler table, included unmodified: core atomics and `intid` only.
#[path = "../../../../src/arch/aarch64/gic/irq_handlers"]
pub mod irq_handlers {
    mod register;
    mod state;

    pub use register::{
        holds_kernel_handler, register, register_for_capsule, unregister, unregister_for_capsule,
        GicIrqError,
    };
}
pub mod its;
#[path = "../../../../src/arch/aarch64/gic/select.rs"]
pub mod select;
pub mod v2;
#[path = "../../../../src/arch/aarch64/gic/version.rs"]
pub mod version;
