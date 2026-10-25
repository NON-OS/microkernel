// NONOS Operating System (AGPL-3.0-or-later)
// The driver's completion entry, the order it is read in, the cursor over
// the ring and the wait both queues share, mounted together so each file's
// `super::` names its neighbours as in the driver.
#[path = "../../../../capsule_driver_nvme/src/admin/completion.rs"]
mod completion;
#[path = "../../../../capsule_driver_nvme/src/admin/completion_step.rs"]
mod completion_step;
#[path = "../../../../capsule_driver_nvme/src/admin/completion_wait.rs"]
mod completion_wait;
#[path = "../../../../capsule_driver_nvme/src/admin/cq_cursor.rs"]
mod cq_cursor;
#[path = "../../../../capsule_driver_nvme/src/admin/entry_read.rs"]
mod entry_read;
#[path = "../../../../capsule_driver_nvme/src/admin/slot_read.rs"]
mod slot_read;

pub use completion::Completion;
pub use completion_step::{classify, CqStep};
pub use completion_wait::{wait_for_completion, DEADLINE_CHECK_SPINS};
pub use cq_cursor::CqCursor;
pub use entry_read::read_entry;
pub use slot_read::read_slot;
