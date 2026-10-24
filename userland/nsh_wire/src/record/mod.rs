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

//! The session record, `nsh-record/1`: its header, entries and footer, and
//! the `RECORD` frame that carries it in parts.

mod body;
mod boxed;
pub(crate) mod entry;
pub(crate) mod footer;
pub(crate) mod header;
mod header_read;
mod parse;
mod part;

pub use boxed::{BoxRun, Preset, BOX_BODY, BOX_KINDS};
pub use entry::{Entry, EntryBody, MAX_COMMAND};
pub use footer::{Footer, FOOTER_LEN};
pub use header::{RecordHeader, MAX_RECORD, MAX_RECORD_ENTRIES};
pub use parse::Record;
pub use part::RecordPart;
