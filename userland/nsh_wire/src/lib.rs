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

//! The nsh/1 wire format. Every parser here reads bytes from the network
//! before any check of who sent them, so each one takes a slice, reads it
//! exactly, and returns a typed view or a `WireError`. Nothing is copied
//! and nothing is allocated. The specification is docs/design/nsh-v1*.md.

#![no_std]
#![forbid(unsafe_code)]

mod auth_ok;
mod cap_names;
mod cap_names_write;
pub mod enrol;
mod enrol_parse;
mod error;
mod escalate;
mod escalate_result;
mod expect;
mod frame;
pub mod grant;
pub mod hello;
mod hello_preamble;
mod host_proof;
pub mod keys;
mod kind;
mod limits;
pub mod names;
mod reader;
mod reader_fields;
mod reason;
mod reason_sets;
pub mod record;
mod session;
mod session_codes;
mod user_auth;
mod writer;
mod writer_fields;

pub use auth_ok::{AuthOk, MAX_IDLE_S, SESSION_ID_LEN};
pub use cap_names::{CapNames, MAX_CAPS, MAX_CAP_NAME};
pub use enrol::{boot::*, Enrol, AK_PUBLIC_LEN, FIRMWARE_PCRS, FLEET_ID_LEN, PCR_LEN};
pub use error::WireError;
pub use escalate::escalate;
pub use escalate_result::EscalateResult;
pub use expect::Expect;
pub use frame::{len_field, split, wire_len};
pub use grant::{Approval, Grant, GrantKind, GRANT_ID_LEN, MAX_APPROVALS};
pub use hello::{ClientHello, ServerHello, CLIENT_HELLO_LEN, SERVER_HELLO_LEN};
pub use host_proof::{HostProof, ENTRY_LEN, MAX_DOC, MAX_ENROL, MAX_ENTRIES};
pub use keys::{before_sig, HybridPublic, HybridSig};
pub use kind::Kind;
pub use limits::{AUTH_CAP, HELLO_CAP, LEN_BYTES, MAX_DATA, SESSION_CAP, TAG_LEN};
pub use reader::Reader;
pub use reason::Reason;
pub use record::{BoxRun, Entry, EntryBody, Footer, Preset, Record, RecordHeader, RecordPart};
pub use session::{data, window, Window, MAX_CELLS};
pub use session_codes::{abort, auth_refused, empty, exit_status, plain_abort, signal, Signal};
pub use user_auth::{UserAuth, MAX_GRANT};
pub use writer::Writer;
