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

//! The nsh/1 client's check of a host, before any credential is sent
//! (docs/design/nsh-v1-attest.md, "What the client checks, in order"). The
//! document is `NONOSATT` version 5 (docs/design/k1-attest-doc-v4.md); an
//! older one quotes too little and is refused. Every step is its own
//! function and refuses with its own name, and the first refusal ends the
//! check: `ABORT(POLICY)`, shown to the user as `POLICY (<name>)`.

#![no_std]

extern crate alloc;

mod check;
pub mod consts;
pub mod doc;
mod evidence;
mod evidence_verify;
pub mod log;
pub mod p256;
pub mod policy;
pub mod quote;
mod read;
mod read_fields;
mod refusal;
mod refusal_kind;
mod refusal_name;
pub mod steps;

pub use check::{check, Verdict};
pub use doc::Doc;
pub use evidence::{Evidence, Session};
pub use evidence_verify::EvidenceError;
pub use policy::{Authority, Policy, PolicyError, Release};
pub use refusal::{CapsuleFault, FormatFault, LogFault, QuoteFault};
pub use refusal_kind::Refusal;
