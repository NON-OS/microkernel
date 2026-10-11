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

mod check;
mod reply;
mod request;
mod words;

pub use check::{believed, struct_hash};
pub use reply::{parse_reply, Signed, REPLY_LEN};
pub use request::{typed_request, ENTRY_ATTEST, ENTRY_BINDING, OP_SIGN_TYPED_DATA};
pub use words::{attest_words, binding_words, Wrong};
