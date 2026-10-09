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

/* The switches whose apps this kernel carries, from the capsules built in. */

use super::bits::{BROWSER, CALCULATOR, EDITOR, FILES, LINUX, MEDIA, STORE, WALLET};

const fn has(built_in: bool, bit: u32) -> u32 {
    if built_in {
        bit
    } else {
        0
    }
}

const MEDIA_BUILT_IN: bool = cfg!(feature = "nonos-capsule-audio-player")
    || cfg!(feature = "nonos-capsule-video-player")
    || cfg!(feature = "nonos-capsule-image-viewer");

pub(crate) const PRESENT: u32 = has(cfg!(feature = "nonos-capsule-browser"), BROWSER)
    | has(cfg!(feature = "nonos-capsule-wallet-nonos"), WALLET)
    | has(cfg!(feature = "nonos-capsule-app-store"), STORE)
    | has(cfg!(feature = "nonos-capsule-file-manager"), FILES)
    | has(cfg!(feature = "nonos-capsule-text-editor"), EDITOR)
    | has(cfg!(feature = "nonos-capsule-calculator"), CALCULATOR)
    | has(MEDIA_BUILT_IN, MEDIA)
    | has(cfg!(feature = "nonos-capsule-linux"), LINUX);
