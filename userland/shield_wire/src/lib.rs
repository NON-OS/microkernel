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

//! What the wallet says to `nonos.shield` and what it answers.
//!
//! A request is a sequence number, an operation and its fields; a reply is
//! the sequence number, a status and named values. Fields and values are
//! UTF-8 text, one to a line, so the wallet, which has no std, and the shield,
//! which does, read them with the same few lines of code, and nothing in a
//! reply is a number the other side has to guess the width of.
//!
//! Long work (a sync, a review that reads the chain, a proof) runs on the
//! shield's worker: the call answers `STARTED` at once and the wallet asks
//! for `RESULT` until the job is done, so neither side ever blocks on the
//! network or the prover.

#![no_std]

extern crate alloc;

mod frame;
mod ops;
mod values;

pub use frame::{
    decode_reply, decode_request, encode_reply, encode_request, Reply, Request, BODY_MAX,
    MESSAGE_MAX,
};
pub use ops::*;
pub use values::{field, fields, Values};

#[cfg(test)]
mod tests;
