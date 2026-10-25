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

//! Step 5, the firmware: PCRs 0, 1, 2 and 7 are the values enrolled.

use nsh_attest::Refusal;

use super::refused;
use crate::build::Spec;

#[test]
fn a_firmware_pcr_other_than_enrolled_is_refused() {
    let edits: [fn(&mut crate::build::enrol::Cert); 4] = [
        |c| c.pcrs[0][0] ^= 1,
        |c| c.pcrs[1][0] ^= 1,
        |c| c.pcrs[2][0] ^= 1,
        |c| c.pcrs[3][0] ^= 1,
    ];
    for (edit, pcr) in edits.into_iter().zip([0, 1, 2, 7]) {
        let s = Spec { cert: edit, ..Spec::default() };
        assert_eq!(refused(s), Refusal::Firmware(pcr), "PCR {pcr}");
    }
}
