// NONOS Operating System (AGPL-3.0-or-later)
//! The kernel module's own shape, minus `derive`, which needs the CRB
//! transport and the kernel RNG. Everything it calls is here.

#[allow(dead_code)]
#[path = "../../../../../../src/security/tpm/machine_key/consts.rs"]
pub mod consts;
#[allow(dead_code)]
#[path = "../../../../../../src/security/tpm/machine_key/create.rs"]
pub mod create;
#[allow(dead_code)]
#[path = "../../../../../../src/security/tpm/machine_key/error.rs"]
pub mod error;
#[allow(dead_code)]
#[path = "../../../../../../src/security/tpm/machine_key/flush.rs"]
pub mod flush;
#[allow(dead_code)]
#[path = "../../../../../../src/security/tpm/machine_key/hmac.rs"]
pub mod hmac;
#[allow(dead_code)]
#[path = "../../../../../../src/security/tpm/machine_key/pcrs.rs"]
pub mod pcrs;
#[allow(dead_code)]
#[path = "../../../../../../src/security/tpm/machine_key/policy.rs"]
pub mod policy;
#[allow(dead_code)]
#[path = "../../../../../../src/security/tpm/machine_key/session.rs"]
pub mod session;
#[allow(dead_code)]
#[path = "../../../../../../src/security/tpm/machine_key/wire.rs"]
pub mod wire;

pub use error::KeyError;
pub use pcrs::BOUND_PCRS;

#[cfg(test)]
mod command_tests;
#[cfg(test)]
mod live_tests;
#[cfg(test)]
mod parse_tests;
#[cfg(test)]
mod swtpm;
#[cfg(test)]
mod wire_tests;
