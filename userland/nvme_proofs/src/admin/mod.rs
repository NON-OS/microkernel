// NONOS Operating System (AGPL-3.0-or-later)
// The real NVMe identify and SMART/health parsers, which run on
// device-controlled bytes.
#[path = "../../../capsule_driver_nvme/src/admin/health/mod.rs"]
mod health;
#[path = "../../../capsule_driver_nvme/src/admin/hmb/mod.rs"]
pub mod hmb;
#[path = "../../../capsule_driver_nvme/src/admin/identity.rs"]
mod identity;
#[path = "../../../capsule_driver_nvme/src/admin/namespace/mod.rs"]
mod namespace;
// The real admin queue doorbell offsets (pub(super); wrapped below).
#[path = "../../../capsule_driver_nvme/src/admin/queue/cq0_head.rs"]
mod cq0_head;
#[path = "../../../capsule_driver_nvme/src/admin/queue/sq0_tail.rs"]
mod sq0_tail;
// The real enable and disable wait: the decision and the polling loop.
#[path = "../../../capsule_driver_nvme/src/admin/ready_step.rs"]
mod ready_step;
#[path = "../../../capsule_driver_nvme/src/admin/ready_wait.rs"]
mod ready_wait;
// The real completion entry, cursor and the wait both queues share.
#[path = "../../../capsule_driver_nvme/src/admin/completion.rs"]
mod completion;
#[path = "../../../capsule_driver_nvme/src/admin/completion_step.rs"]
mod completion_step;
#[path = "../../../capsule_driver_nvme/src/admin/completion_wait.rs"]
mod completion_wait;
#[path = "../../../capsule_driver_nvme/src/admin/cq_cursor.rs"]
mod cq_cursor;
// The real namespace choice from the active namespace list, and the real
// step before the reset clears CC.EN.
#[path = "../../../capsule_driver_nvme/src/admin/active_ns.rs"]
mod active_ns;
#[path = "../../../capsule_driver_nvme/src/admin/disable_step.rs"]
mod disable_step;

pub use active_ns::{first_active_nsid, lists_active_namespaces, FALLBACK_NSID};
pub use completion::Completion;
pub use completion_step::{classify, CqStep};
pub use completion_wait::{wait_for_completion, wait_noting_foreign, DEADLINE_CHECK_SPINS};
pub use cq_cursor::CqCursor;
pub use disable_step::{
    await_ready_before_disable, disable_start, pre_disable_step, DisableStart, PreDisableStep,
};
pub use health::SmartHealth;
pub use identity::ControllerIdentity;
pub use namespace::NamespaceIdentity;
pub use ready_step::{ready_step, ready_timeout_ms, ReadyStep, READY_FLOOR_MS};
pub use ready_wait::wait_ready;

/// The admin submission tail and completion head doorbells for a stride.
pub fn admin_doorbells(stride: u8) -> (u32, u32) {
    (sq0_tail::sq0_tail(stride), cq0_head::cq0_head(stride))
}
