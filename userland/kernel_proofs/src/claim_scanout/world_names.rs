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

//! The takeover model's invariants by name (L79), for a failing test to
//! say which broke; the bits are world_check's, in the same order.

const NAMES: [&str; 14] = [
    "a covering claim ready while a running bootfb page shows the framebuffer",
    "a covering claim ready while the kernel writes the framebuffer",
    "bootfb's claim ready once the scanout was taken",
    "the console stopped for good with no covering claim made ready",
    "bootfb's entry gone while a running page of its claim shows the framebuffer",
    "a kernel draw under way while bootfb stores to the framebuffer",
    "a page not showing its grant's frame was repointed",
    "a refused admit left its wipe record written",
    "told to ask again with no bootfb claim made ready since the takeover looked",
    "every path not ended is waiting",
    "an entry left at the end that no claim holds ready",
    "a takeover still counted at the end",
    "a take-back's retire refused on the very stage its read saw",
    "the console left stopped at an end with no covering claim made ready",
];

pub(super) fn names(bits: u16) -> Vec<&'static str> {
    (0..NAMES.len()).filter(|i| bits & 1 << i != 0).map(|i| NAMES[i]).collect()
}
