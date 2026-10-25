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

//! H-06(a): why there is no boot framebuffer, from the loader's word to the
//! kernel's decoder and what userland is told. Both sides by path.

#[path = "../../../../src/boot/handoff/types/constants.rs"]
pub mod constants;
#[path = "../../../../nonos-bootloader/src/handoff/types/constants.rs"]
pub mod loader_constants;
/// The loader's half: the word it makes and its own flag constants.
#[path = "../../../../nonos-bootloader/src/display/gop/refusal_word.rs"]
pub mod loader_word;
#[path = "../../../../src/boot/handoff/types/refusal.rs"]
pub mod refusal;
#[path = "../../../../src/boot/handoff/types/refusal_says.rs"]
mod refusal_says;

#[cfg(test)]
mod decode_tests;
#[cfg(test)]
mod glue_tests;
#[cfg(test)]
mod mirror_tests;
