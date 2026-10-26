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

//! The document carries every event log the loader boots with: its cap is
//! the loader's own (nonos-boot-measure's `MAX_LOG_BYTES`), and a longer log
//! is refused rather than cut, since a verifier replays it whole.

use super::document::{log_fits, MAX_LOG};

const LOADER: &str = include_str!("../../../../../nonos-boot-measure/src/tcg/consts.rs");

#[test]
fn the_document_takes_every_log_the_loader_keeps_and_no_longer_one() {
    assert!(LOADER.contains("pub const MAX_LOG_BYTES: usize = 1024 * 1024;"));
    assert_eq!(MAX_LOG, 1024 * 1024, "the loader's cap");
    assert!(log_fits(&vec![0; MAX_LOG]) && !log_fits(&vec![0; MAX_LOG + 1]));
}
