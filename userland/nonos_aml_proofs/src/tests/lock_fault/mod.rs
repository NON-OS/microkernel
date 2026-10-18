// NONOS Operating System (AGPL-3.0-or-later)
//! The Global Lock across fields whose LockRule is Lock (ACPI 6.5,
//! 5.2.10.1 and 19.6.48; recalled, unverified), when the access faults:
//! in a BankField's select or an IndexField's index write, in the data
//! access, for want of steps at any point, or for want of time while
//! firmware holds the lock. Whatever ends the evaluation, the capsule's
//! count is back at zero and the FACS word is not left the capsule's.

mod fixture;
mod gen;
mod model;
mod sweep_tests;
