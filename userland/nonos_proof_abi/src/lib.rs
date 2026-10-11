// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! What a NONOS proof is made of, byte for byte, and how it is checked.
//!
//! A family's run is sealed by the kernel into a 240-byte record. The prover
//! proves the isolation statement over the record's private histogram and
//! binds the record into the anonymous device proof as its context. The
//! container carries both, with the measurements and the app's policy, and
//! `verify` checks them in the one order every verifier uses. Only a 32-byte
//! anchor goes on chain.

#![no_std]

extern crate alloc;

mod anchor;
mod container;
mod delivery;
mod device;
mod hist;
pub mod isolation;
mod meas;
mod poseidon;
mod record;
mod scope;
mod syscall_root;
pub mod trust;
mod verify;
pub mod wire;
mod words;

pub use anchor::{anchor, anchor_of, anchor_tag, ANCHOR_LABEL};
pub use container::{encode, parse, tag, Container, Kind, MAGIC, MAX_BYTES, VERSION};
pub use delivery::{ctx2, nonce_ref, req_digest, rsp_digest, Delivery, DELIVERY_BYTES};
pub use device::{decode_device, encode_device, DEVICE_HEAD, DEVICE_MAGIC, DEVICE_MAX};
pub use hist::{bucket, hist_commit, side, BUCKETS, HIST_WORDS, ISO_DOMAIN, SIDES};
pub use meas::{meas_chain, meas_step, MEAS_DOMAIN};
pub use poseidon::chain;
pub use record::{
    ctx, decode_record, record_id, Record, FLAG_FINAL, FLAG_OVERFLOW, FLAG_SATURATED, RECORD_BYTES,
};
pub use scope::{day, market_scope};
pub use syscall_root::{allows, syscall_root, POL_DOMAIN, SET_BYTES};
pub use verify::{app_id, verify, Expected, PolicyDecode, PolicyView, Refusal, Verdict, MAX_DAYS};
