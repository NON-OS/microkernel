// NONOS Operating System (AGPL-3.0-or-later)

#[path = "../../../../../../../src/drivers/security/error.rs"]
pub mod error;

#[path = "../../../../../../../src/drivers/security/lba.rs"]
pub mod lba;

pub use error::DriverError;
pub use lba::{is_lba_in_partition, validate_lba_in_partition, validate_lba_range, validate_lba_range_with_size};
