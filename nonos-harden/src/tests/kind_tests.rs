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

//! A record that says it is a release while it carries the development
//! attestation contradicts itself: `release.kind` fails it, and it is NO-GO.

use super::fake::Fake;
use super::sealed_fixture::Sealed;
use crate::exit;
use serde_json::json;

#[test]
fn a_release_profile_carrying_the_development_attestation_is_no_go() {
    let config = json!({"release": true, "profile": "qemu", "features": ["nonos-dev-attest"], "rollback_index": 1});
    let (code, v) = Sealed::new(config).verdict(Fake::default());
    let kind = v["checks"].as_array().unwrap().iter().find(|c| c["id"] == "release.kind").unwrap();
    assert_eq!((code, kind["status"].as_str()), (exit::NO_GO, Some("fail")), "{v:#}");
    assert!(kind["reason"].as_str().unwrap().contains("nonos-dev-attest"), "{kind:#}");
    assert_eq!(v["overall"].as_str(), Some("no-go"), "{v:#}");
}
