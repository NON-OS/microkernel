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

//! Host proofs for the image codec service. Any app can hand it an image, and
//! everything the image's bytes decide happens in one file, decode_sized: the
//! size from the header, the output buffer, and the toolkit decoder. The
//! `#[path]` includes pull that file and the codec's protocol module in under
//! the `crate::` paths they name, beside the table of outputs the service
//! holds for its clients.

extern crate alloc;

#[path = "../../capsule_image_codec/src/protocol/mod.rs"]
pub mod protocol;

#[path = "../../capsule_image_codec/src/server/handlers/decode_sized.rs"]
pub mod decode_sized;

/// The outputs image_codec holds for its clients, as it ships.
#[allow(clippy::new_without_default)]
#[path = "../../capsule_image_codec/src/server/outputs.rs"]
pub mod outputs;

#[cfg(test)]
mod decode_fuzz_tests;
#[cfg(test)]
mod outputs_tests;
