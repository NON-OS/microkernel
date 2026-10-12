// NONOS Operating System (AGPL-3.0-or-later)
//! The driver's engine: the command wait's verdicts, the spans and PRD
//! counts a command may carry, and the command path itself, issue to
//! recovery, which the tests run against a model port and drive.

mod bare_port;
#[path = "../../../capsule_driver_ahci/src/engine/build.rs"]
mod build;
#[path = "../../../capsule_driver_ahci/src/engine/cause.rs"]
pub(crate) mod cause;
mod cause_tests;
#[path = "../../../capsule_driver_ahci/src/engine/cmd_header.rs"]
pub(crate) mod cmd_header;
#[path = "../../../capsule_driver_ahci/src/engine/cmd_table.rs"]
pub(crate) mod cmd_table;
#[path = "../../../capsule_driver_ahci/src/engine/completion.rs"]
pub mod completion;
mod completion_tests;
mod comreset_tests;
#[path = "../../../capsule_driver_ahci/src/engine/data.rs"]
mod data;
#[path = "../../../capsule_driver_ahci/src/engine/fis.rs"]
pub(crate) mod fis;
#[path = "../../../capsule_driver_ahci/src/engine/flush.rs"]
pub(crate) mod flush;
#[path = "../../../capsule_driver_ahci/src/engine/holdup.rs"]
mod holdup;
#[path = "../../../capsule_driver_ahci/src/engine/identify.rs"]
pub(crate) mod identify;
#[path = "../../../capsule_driver_ahci/src/engine/issue.rs"]
mod issue;
#[path = "../../../capsule_driver_ahci/src/engine/kick.rs"]
mod kick;
pub mod link;
#[path = "../../../capsule_driver_ahci/src/engine/port.rs"]
pub(crate) mod port;
#[path = "../../../capsule_driver_ahci/src/engine/prd_count.rs"]
pub mod prd_count;
#[path = "../../../capsule_driver_ahci/src/engine/prdt.rs"]
pub(crate) mod prdt;
#[path = "../../../capsule_driver_ahci/src/engine/prdt_write.rs"]
mod prdt_write;
#[path = "../../../capsule_driver_ahci/src/engine/prepare.rs"]
mod prepare;
#[path = "../../../capsule_driver_ahci/src/engine/ready.rs"]
pub mod ready;
#[path = "../../../capsule_driver_ahci/src/engine/recover.rs"]
pub(crate) mod recover;
mod recover_cause_tests;
mod recover_stuck_tests;
mod recover_tests;
pub(crate) mod region;
#[path = "../../../capsule_driver_ahci/src/engine/region_io.rs"]
mod region_io;
#[path = "../../../capsule_driver_ahci/src/engine/span.rs"]
pub mod span;
#[path = "../../../capsule_driver_ahci/src/engine/start.rs"]
mod start;
#[path = "../../../capsule_driver_ahci/src/engine/stop.rs"]
mod stop;
mod stop_tests;
#[path = "../../../capsule_driver_ahci/src/engine/transfer.rs"]
pub(crate) mod transfer;
#[path = "../../../capsule_driver_ahci/src/engine/verdict.rs"]
pub mod verdict;

pub(crate) use port::Port;
