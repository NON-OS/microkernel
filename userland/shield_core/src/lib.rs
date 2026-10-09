// NOX Shield wallet core.
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! The wallet core the Kotlin and Swift shells render. Proofs are made and verified on the
//! device, since a proving server learns the balance. Each hidden proof takes a fresh CSPRNG
//! blinding seed, refused if reused. Nothing leaves except through a proxy `net::policy` admits.

pub mod bench;
pub mod custody;
pub mod discovery;
pub mod entropy;
pub mod error;
pub mod evm;
pub mod ffi;
#[cfg(feature = "fuzzing")]
pub mod fuzz;
#[path = "shield_keys/mod.rs"]
pub mod keys;
pub mod net;
pub mod notes;
pub mod prover;
pub mod store;
pub mod wallet;

