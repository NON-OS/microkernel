// NONOS Operating System (AGPL-3.0-or-later)

#[path = "../../../../../../../src/arch/x86_64/uefi/tables/header.rs"]
pub mod header;

#[path = "../../../../../../../src/arch/x86_64/uefi/tables/runtime.rs"]
pub mod runtime;

#[path = "../../../../../../../src/arch/x86_64/uefi/tables/time.rs"]
pub mod time;

pub use header::TableHeader;
pub use runtime::RuntimeServices;
pub use time::{EfiTime, EfiTimeCapabilities};
