// NONOS Operating System (AGPL-3.0-or-later)
// The second implementation, from src/security/crypto/constant_time. Only the
// three files the comparison primitives need are mirrored: ops.rs reads core.rs
// through `super`, and core.rs reads types.rs the same way.
#[path = "../../../../../../../src/security/crypto/constant_time/types.rs"]
pub mod types;

#[path = "../../../../../../../src/security/crypto/constant_time/core.rs"]
pub mod core;

#[path = "../../../../../../../src/security/crypto/constant_time/ops.rs"]
pub mod ops;
