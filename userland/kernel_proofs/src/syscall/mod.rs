// NONOS Operating System (AGPL-3.0-or-later)
// The real syscall id decode (ABI registry + number enum), self-contained.
#[path = "../../../../src/syscall/abi/mod.rs"]
pub mod abi;
#[path = "../../../../src/syscall/numbers/mod.rs"]
pub mod numbers;

// The capability checks and the syscall capability table (authorization gate).
pub mod caps;
pub mod contract;

pub mod microkernel;

// The CryptoRandom handler and the result type its file names, for the test
// that a capsule holding no capability draws random bytes.
#[cfg(test)]
#[path = "../../../../src/syscall/types/result.rs"]
mod result;
#[cfg(test)]
pub use result::SyscallResult;
#[cfg(test)]
pub mod dispatch;

#[cfg(test)]
mod index_tests;
#[cfg(test)]
mod router_tests;
#[cfg(test)]
mod tables_tests;
