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


//! Admission soundness (`Formal.admission_soundness`, `admission_within_cert`,
//! `admission_within_image`) on the real bit rules `preflight::run` composes:
//! `within_ceiling`, `grant_within_manifest`, `install_caps` and
//! `image_ceiling::gate`, in the order the Rust runs them.

use super::caps_bits::{grant_within_manifest, install_caps, within_ceiling};
use super::ceiling_rule::gate;
use super::words::random_words;

/// What reaches the PCB, or None where a check refuses: `Formal.admission`
/// with the signature, structural and attestation checks passing.
fn admitted(req: u64, opt: u64, granted: u64, cert: u64, image: u64, keep: u64) -> Option<u64> {
    if !within_ceiling(req, opt, cert) || !grant_within_manifest(req, opt, granted) {
        return None;
    }
    let install = install_caps(req, opt, granted);
    gate(install, image).ok()?;
    Some(install & keep)
}

/// The conclusion of the Lean theorem, on one accepted admission.
fn sound(req: u64, opt: u64, granted: u64, cert: u64, image: u64, w: u64) -> bool {
    (req | opt) & !cert == 0
        && granted & !(req | opt) == 0
        && w & !cert == 0
        && w & !image == 0
        && w & !(req | opt) == 0
}

#[test]
fn every_small_admission_is_sound() {
    let mut accepted = 0u32;
    for req in 0..8u64 {
        for opt in 0..8u64 {
            for granted in 0..8u64 {
                for cert in 0..8u64 {
                    for image in 0..8u64 {
                        for keep in [0u64, 5, 7] {
                            if let Some(w) = admitted(req, opt, granted, cert, image, keep) {
                                accepted += 1;
                                assert!(sound(req, opt, granted, cert, image, w));
                            }
                        }
                    }
                }
            }
        }
    }
    assert!(accepted > 0);
}

#[test]
fn full_word_admissions_are_sound() {
    let w = random_words(0x5eed_ad31, 6 * 20_000);
    let mut accepted = 0u32;
    for c in w.chunks(6) {
        // Narrow the request into the certificate half the time, so both
        // outcomes are exercised.
        let cert = c[3];
        let (req, opt) = if c[0] & 1 == 0 { (c[0] & cert, c[1] & cert) } else { (c[0], c[1]) };
        let extra = if c[5] & 2 == 0 { 0 } else { c[5] };
        let granted = c[2] & (req | opt | extra);
        let image = c[4] | (c[5] & 1).wrapping_neg();
        if let Some(wd) = admitted(req, opt, granted, cert, image, c[5]) {
            accepted += 1;
            assert!(sound(req, opt, granted, cert, image, wd));
        }
    }
    assert!(accepted > 1000, "{accepted}");
}

#[test]
fn a_manifest_above_its_certificate_is_refused() {
    assert!(admitted(0b100, 0, 0, 0b011, !0, !0).is_none());
    assert!(admitted(0, 0b100, 0b100, 0b011, !0, !0).is_none());
}

#[test]
fn an_install_word_above_the_image_ceiling_is_refused() {
    assert!(admitted(0b10, 0, 0, !0, 0b01, !0).is_none());
}
