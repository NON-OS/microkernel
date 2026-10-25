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

//! The largest log a machine boots with, which the loader takes up to
//! 1 MiB: a document carrying almost that much, written by the kernel's
//! encoder, passes the whole check in one HOST_PROOF frame. One byte over
//! the cap is refused (`numbers`).

use nsh_attest::consts::MAX_LOG;

use super::super::passes;
use super::by_the_kernel;
use crate::build::tcg::EV_ACTION;
use crate::build::Spec;

#[test]
fn a_document_with_a_log_of_almost_a_mebibyte_passes_end_to_end() {
    let mut s = Spec { bytes: |d| *d = by_the_kernel(d), ..Spec::default() };
    for i in 0..16u8 {
        s.log.note(5, EV_ACTION, &[i; 64_000]);
    }
    assert!(s.log.bytes.len() > 1_000_000 && s.log.bytes.len() <= MAX_LOG);
    let b = s.build();
    assert!(b.proof().len() < nsh_wire::AUTH_CAP, "one HOST_PROOF frame carries it");
    passes(s);
}
