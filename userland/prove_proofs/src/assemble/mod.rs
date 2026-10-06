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

//! The capsule's pure half, file by file, in the module shape its `super::`
//! imports expect, all public so every item is held by a test.

#[path = "../../../capsule_prove/src/assemble/enrolled.rs"]
pub mod enrolled;
#[path = "../../../capsule_prove/src/assemble/error.rs"]
pub mod error;
#[path = "../../../capsule_prove/src/assemble/error_text.rs"]
pub mod error_text;
#[path = "../../../capsule_prove/src/assemble/output.rs"]
pub mod output;
#[path = "../../../capsule_prove/src/assemble/output_read.rs"]
pub mod output_read;
#[path = "../../../capsule_prove/src/assemble/request.rs"]
pub mod request;
#[path = "../../../capsule_prove/src/assemble/slots.rs"]
pub mod slots;
#[path = "../../../capsule_prove/src/assemble/statement.rs"]
pub mod statement;
#[path = "../../../capsule_prove/src/assemble/transcript_line.rs"]
pub mod transcript_line;
#[path = "../../../capsule_prove/src/assemble/wipe.rs"]
pub mod wipe;
#[path = "../../../capsule_prove/src/assemble/words.rs"]
pub mod words;
