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

//! What the kernel names a capsule's admission (`capsule_measure/kind.rs`):
//! only a STARK proof under the vendor's root is kind 1, the path-only tag
//! is the verifier of a path-only capsule alone, and the client refuses each
//! other kind for a release by its own name.

use nsh_attest::{CapsuleFault, Refusal};

use super::super::super::{passes, refused};
use crate::build::tcg_event::{digest, extend};
use crate::build::Spec;
use crate::kernel_measure::event::{body, event, ALG_SHA1, ALG_SHA256};
use crate::kernel_measure::kind::{kind, verifier, PATH_ONLY_TAG};
use crate::kernel_measure::kind::{DEVELOPER, LOCAL_BUILD, PATH_ONLY, VENDOR};

#[test]
fn only_a_proof_under_the_vendor_s_root_is_kind_1() {
    for bits in 0..8u8 {
        let (path, local, vendor) = (bits & 4 != 0, bits & 2 != 0, bits & 1 != 0);
        let want = match (path, local, vendor) {
            (true, _, _) => PATH_ONLY,
            (_, true, _) => LOCAL_BUILD,
            (_, _, true) => VENDOR,
            _ => DEVELOPER,
        };
        assert_eq!(kind(path, local, vendor), want, "{path} {local} {vendor}");
    }
    assert_eq!([VENDOR, PATH_ONLY, DEVELOPER, LOCAL_BUILD], [1, 2, 3, 4]);
}

#[test]
fn the_verifier_is_the_trailer_s_but_for_a_path_alone() {
    let trailer = b"a proof's trailer";
    for k in [VENDOR, DEVELOPER, LOCAL_BUILD] {
        assert_eq!(verifier(trailer, k), digest(trailer));
    }
    assert_eq!(verifier(trailer, PATH_ONLY), digest(PATH_ONLY_TAG));
    assert_eq!(verifier(b"another", PATH_ONLY), verifier(trailer, PATH_ONLY));
}

/// The default host, after the kernel admits one more capsule as `k`.
fn with_kernel_capsule(k: u8) -> Spec {
    let mut s = Spec::default();
    let b = body(&[9; 32], &verifier(&[9; 8], k), k);
    let (bytes, digests) = event(&[ALG_SHA1, ALG_SHA256], &b).expect("two banks");
    s.log.bytes.extend(bytes);
    s.log.pcrs[14] = extend(&s.log.pcrs[14], &digests[1].1.clone().try_into().expect("32"));
    s
}

#[test]
fn every_kind_but_the_vendor_s_is_refused_for_a_release_by_its_name() {
    passes(with_kernel_capsule(VENDOR));
    use CapsuleFault::{Developer, LocalBuild, PathOnly};
    for (k, fault) in [(PATH_ONLY, PathOnly), (DEVELOPER, Developer), (LOCAL_BUILD, LocalBuild)] {
        assert_eq!(refused(with_kernel_capsule(k)), Refusal::Pcr14(fault), "kind {k}");
    }
}
