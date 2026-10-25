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

//! register_boot_framebuffer's order, read as text (L79). Who covers the
//! framebuffer is found the moment its window is known, before any way out,
//! so it is found whether the record is published or not; and before the
//! record exists, so the framebuffer's own BAR never counts as holding it.
//! The sink is made before the record is published, and the record's id is
//! what the rest of the broker asks after.

use super::glue_text::{body, code, in_order};

const BOOTFB: &str = "hardware/broker/bootfb";
const READ: &str = "letSome(w)=phys_window()else{return;};";

#[test]
fn find_runs_first_and_before_the_record_is_published() {
    let src = code(&format!("{BOOTFB}/register.rs"));
    let steps = in_order(
        body(&src, "pubfnregister_boot_framebuffer(){"),
        &[
            READ,
            "super::cover::find(w.page,w.len);",
            "letSome((base,size))=window(w.page,w.len,w.offset)else{",
            "if!super::sink::make(){",
            "letid=register_platform_device(DeviceRecord{",
            "BOOTFB.call_once(||id);",
        ],
    );
    assert_eq!(steps[1], steps[0] + READ.len(), "a step between the window and find");
    let fb = body(&src, "fnis_boot_framebuffer(device_id:u64)->bool{");
    assert_eq!(fb, "BOOTFB.get()==Some(&device_id)", "the published id is not the one asked");
}

#[test]
fn find_is_called_from_registration_only() {
    let mut calls = Vec::new();
    for entry in std::fs::read_dir(format!("{}/../../src/{BOOTFB}", env!("CARGO_MANIFEST_DIR")))
        .expect("bootfb readable")
    {
        let name = entry.expect("an entry").file_name().into_string().expect("a name");
        let n = code(&format!("{BOOTFB}/{name}")).matches("cover::find(").count();
        calls.extend(std::iter::repeat_n(name, n));
    }
    assert_eq!(calls, ["register.rs"], "find called elsewhere, or more than once");
}
