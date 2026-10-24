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

//! The kernel that verifies is the kernel that boots: the bytes the check
//! verified are the bytes the record seals, the ESP's kernel is those bytes,
//! and a record that names no rollback index cannot have one checked.

use super::fake::Fake;
use super::fixture::release_config;
use super::kernel_tests::fails_with;
use super::sealed_fixture::Sealed;
use serde_json::Value;
use std::fs;

#[test]
fn a_kernel_changed_after_the_record_was_written_fails() {
    let s = Sealed::dev();
    let mut b = fs::read(s.release.path().join("kernel.bin")).unwrap();
    b[3] ^= 1;
    s.release.put("kernel.bin", &b);
    fails_with(&s, "are not the sealed");
}

#[test]
fn an_esp_kernel_other_than_the_signed_one_fails_though_the_record_agrees() {
    let s = Sealed::dev();
    s.release.put("esp/kernel.bin", b"an unsigned kernel");
    s.release.record();
    fails_with(&s, "is not the signed kernel");
    let (code, v) = s.verdict(Fake::default());
    assert_eq!((code, v["overall"].as_str()), (crate::exit::NO_GO, Some("no-go")), "{v:#}");
}

#[test]
fn a_record_without_an_esp_kernel_or_a_rollback_index_fails() {
    let s = Sealed::dev();
    fs::remove_file(s.release.path().join("esp/kernel.bin")).unwrap();
    let at = s.release.path().join("nonos-release.json");
    let mut rec: Value = serde_json::from_slice(&fs::read(&at).unwrap()).unwrap();
    rec["sealed"]["esp"]["files"].as_object_mut().unwrap().remove("kernel.bin");
    fs::write(&at, rec.to_string()).unwrap();
    fails_with(&s, "no kernel.bin");
    let mut cfg = release_config();
    cfg.as_object_mut().unwrap().remove("rollback_index");
    fails_with(&Sealed::new(cfg), "no rollback index");
}
