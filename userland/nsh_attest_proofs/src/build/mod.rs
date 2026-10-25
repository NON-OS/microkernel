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

//! Builders: a P-256 attestation key, TCG logs, quotes, documents,
//! enrolment certificates, and the `Spec` that puts them together.

pub mod ak;
pub mod attest;
pub mod doc;
pub mod enrol;
pub mod fixture;
pub mod hybrid;
pub mod keys;
mod make;
mod make_built;
mod make_doc;
pub mod parts;
pub mod proof;
pub mod sig;
pub mod spec;
mod spec_default;
pub mod tcg;
pub mod tcg_capsule;
pub mod tcg_event;
pub mod tcg_log;

pub use fixture::Built;
pub use spec::Spec;
