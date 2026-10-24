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

//! A log that does not parse refuses the document as `POLICY (log)`.

use nsh_attest::{LogFault, Refusal};

use super::refused;
use crate::build::tcg::{header_with, spec_body, EV_NO_ACTION, SHA256};
use crate::build::Spec;

#[test]
fn a_document_without_a_log_is_refused() {
    let mut s = Spec::default();
    s.log.bytes.clear();
    assert_eq!(refused(s), Refusal::Log(LogFault::Short));
}

#[test]
fn a_document_whose_log_does_not_parse_is_refused() {
    let mut s = Spec::default();
    s.log.bytes = header_with(0, EV_NO_ACTION, [0; 20], &spec_body(&[(SHA256, 20)], [0, 2, 0, 2]));
    assert_eq!(refused(s), Refusal::Log(LogFault::Bank));
    let mut s = Spec::default();
    s.log.bytes.pop();
    assert_eq!(refused(s), Refusal::Log(LogFault::Short));
}
