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

mod attest;
mod attest_press;
mod attest_review;
mod binding;
mod path;

pub use attest::{attest_of, attest_rows};
pub use attest_press::{attest_act, may_sign, AttestAct};
pub use attest_review::{attest_review, AttestReview};
pub use binding::{binding_plan, Refused};
pub use path::{attest_sig_path, binding_sig_path, doc_path_ok};
