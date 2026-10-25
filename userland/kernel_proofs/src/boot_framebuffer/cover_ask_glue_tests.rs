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

//! What a claim asks of find, read as text (L79): covers answers from the
//! list find stored once, and nothing else; and the kind codes the model
//! passes are the kernel's own.

use super::cover_glue_tests::COVER;
use super::glue_text::{body, code};

#[test]
fn covers_reads_only_what_find_stored() {
    let src = code(COVER);
    assert_eq!(src.matches("COVER.call_once(").count(), 1, "stored outside find");
    let covers = body(&src, "pub(incrate::hardware::broker)fncovers(device_id:u64)->bool{");
    assert_eq!(covers, "COVER.get().is_some_and(|v|v.contains(&device_id))");
}

#[test]
fn the_model_kind_codes_are_the_kernels() {
    let bar = code("hardware/broker/device/bar.rs");
    let codes = ["constBAR_KIND_NONE:u8=0;", "constBAR_KIND_MMIO:u8=1;", "constBAR_KIND_PIO:u8=2;"];
    let kinds = ["None=BAR_KIND_NONE,", "Mmio=BAR_KIND_MMIO,", "Pio=BAR_KIND_PIO,"];
    for want in codes.iter().chain(&kinds) {
        assert!(bar.contains(want), "device/bar.rs lacks `{want}`");
    }
}
