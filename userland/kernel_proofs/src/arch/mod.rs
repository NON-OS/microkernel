// NONOS Operating System (AGPL-3.0-or-later)
pub mod paging;
/* The UEFI manager, included for the variable cache test. The allowed lints are
 * style findings in the included kernel files, which this crate does not edit. */
#[cfg(test)]
#[allow(
    clippy::manual_is_multiple_of,
    clippy::module_inception,
    clippy::derivable_impls,
    clippy::missing_safety_doc
)]
pub mod x86_64;
