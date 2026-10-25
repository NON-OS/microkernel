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

//! A host that does everything right passes, the verdict names its release
//! and PCR 9, and a refusal is shown as `POLICY (<name>)`.

use nsh_attest::Refusal;

use super::refused;
use crate::build::Spec;

#[test]
fn a_host_that_does_everything_right_passes() {
    let (version, pcr9) = Spec::default().build().check().expect("passes");
    assert_eq!(version, "0.9.4");
    assert_eq!(pcr9, [0x09; 32]);
}

#[test]
fn a_policy_naming_another_pcr9_is_shown_as_policy_pcr9() {
    let r = refused(Spec { pcr: vec![(9, [0x99; 32])], ..Spec::default() });
    assert_eq!(r, Refusal::Pcr9);
    assert_eq!(format!("{r}"), "POLICY (pcr9)");
}

#[test]
fn the_first_failed_check_is_the_one_named() {
    // Bad PCR 9 (step 5) and bad DMA (step 6): step 5 is named.
    let s = Spec { pcr: vec![(9, [0x99; 32])], dma: (1, true, 3, 0), ..Spec::default() };
    assert_eq!(refused(s), Refusal::Pcr9);
    // A bad challenge (step 2) before a bad certificate (step 4).
    let s = Spec { challenge: Some([0x5A; 32]), cert: |c| c.fleet = [0xF2; 16], ..Spec::default() };
    assert_eq!(refused(s), Refusal::Challenge);
}
