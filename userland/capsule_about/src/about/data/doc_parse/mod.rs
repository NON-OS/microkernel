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

//! The attestation document's wire format, and nothing else.
//!
//! Split from the syscall that fetches one so it can be driven directly by the
//! host proofs in `attest_doc_proofs`. A parser that decides whether a signed
//! statement is well formed is exactly the code that should be tested against
//! malformed input rather than trusted, and it cannot be if reaching it needs a
//! trusted platform module.

mod layout;
mod parse;
mod types;

pub use layout::DOC_CAP;
pub use parse::parse;
pub use types::Doc;
