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

//! Rules over the driver's whole source: nothing unmaps DMA itself (the
//! claim's release does, after a reset), no accessor is unsafe to call and
//! every unsafe block says why it holds, and no file is over 75 lines.

use super::source::files;

#[test]
fn nothing_in_the_driver_unmaps_dma_itself() {
    for (path, text) in files() {
        assert!(!text.contains("mk_dma_unmap"), "{} unmaps DMA", path.display());
    }
}

#[test]
fn every_unsafe_block_says_why_it_holds_and_no_accessor_is_unsafe_to_call() {
    for (path, text) in files() {
        let lines: Vec<&str> = text.lines().collect();
        for (n, line) in lines.iter().enumerate() {
            let at = format!("{}:{}", path.display(), n + 1);
            assert!(!line.contains("pub unsafe fn"), "{at} is unsafe to call");
            if !line.contains("unsafe {") && !line.contains("unsafe impl") {
                continue;
            }
            let said = lines[n.saturating_sub(4)..=n].iter().any(|l| l.contains("SAFETY:"));
            assert!(said, "{at} has no SAFETY comment");
        }
    }
}

#[test]
fn no_driver_file_is_longer_than_seventy_five_lines() {
    for (path, text) in files() {
        let n = text.lines().count();
        assert!(n <= 75, "{} is {n} lines", path.display());
    }
}
