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
// The real enable and disable wait: the decision and the polling loop.
#[path = "../../../capsule_driver_nvme/src/admin/ready_step.rs"]
mod ready_step;
#[path = "../../../capsule_driver_nvme/src/admin/ready_wait.rs"]
mod ready_wait;
// The real completion entry, its read, the cursor and the shared wait.
mod completions;
// The real namespace choice from the active namespace list, and the real
// step before the reset clears CC.EN.
#[path = "../../../capsule_driver_nvme/src/admin/active_ns.rs"]
mod active_ns;
#[path = "../../../capsule_driver_nvme/src/admin/disable_step.rs"]
mod disable_step;
// The real shutdown budget and wait the power-off makes.
#[path = "../../../capsule_driver_nvme/src/admin/shutdown_step.rs"]
mod shutdown_step;
#[path = "../../../capsule_driver_nvme/src/admin/shutdown_wait.rs"]
mod shutdown_wait;
// The real CC written at enable, from CAP alone.
#[path = "../../../capsule_driver_nvme/src/admin/cc_value.rs"]
mod cc_value;
// The real submission entries both queues send, the admin queue that sends
// them, and the controller's enable and disable.
#[path = "../../../capsule_driver_nvme/src/admin/command/mod.rs"]
mod command;
#[path = "../../../capsule_driver_nvme/src/admin/controller.rs"]
mod controller;
mod doorbells;
// With the admin doorbell offsets, which admin_doorbells names.
#[path = "../../../capsule_driver_nvme/src/admin/queue/mod.rs"]
mod queue;

pub use active_ns::{first_active_nsid, lists_active_namespaces, FALLBACK_NSID};
pub use cc_value::cc_value;
pub use command::{set_features, Submission};
pub use completions::{classify, read_entry, read_slot, wait_for_completion, Completion};
pub use completions::{CqCursor, CqStep, DEADLINE_CHECK_SPINS};
pub use controller::{enable, reset_to_disabled, shut_down};
pub use disable_step::{
    await_ready_before_disable, disable_start, pre_disable_step, DisableStart, PreDisableStep,
};
pub use doorbells::admin_doorbells;
pub use health::SmartHealth;
pub use identity::ControllerIdentity;
pub use namespace::NamespaceIdentity;
pub use queue::AdminQueue;
pub use ready_step::{ready_step, ready_timeout_ms, ReadyStep, READY_FLOOR_MS};
pub use ready_wait::wait_ready;
pub use shutdown_step::{
    shutdown_budget_ms, shutdown_step, ShutdownStep, SHUTDOWN_CAP_MS, SHUTDOWN_DEFAULT_MS,
};
pub use shutdown_wait::await_shutdown;
