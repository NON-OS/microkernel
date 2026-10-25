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

//! The kernel's attestation document, its read-back of the PCRs and its
//! verdict on a quote, mounted at their module path so the live tests reach
//! what `produce` keeps private.

#[allow(dead_code)]
#[path = "../../../../src/security/attest_doc/document.rs"]
pub mod document;
#[allow(dead_code)]
#[path = "../../../../src/security/attest_doc/error.rs"]
pub mod error;
#[path = "../../../../src/security/attest_doc/produce_judge.rs"]
pub mod produce_judge;
#[path = "../../../../src/security/attest_doc/produce_quote.rs"]
pub mod produce_quote;
#[path = "../../../../src/security/attest_doc/quote_shape.rs"]
pub mod quote_shape;

mod judge_tests;
mod tests;
