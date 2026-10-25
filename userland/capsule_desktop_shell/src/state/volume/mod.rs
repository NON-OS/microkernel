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

//! What the volume keys do to the master volume, and the notice that says
//! what came of it. Kept apart from the IPC (sound/volume.rs) so each rule
//! is proven on the host (desktop_proofs volume_tests).
//!
//! The keys reach the shell whatever has focus (the input router's
//! route/shell_keys.rs). A held key repeats as more presses, so Volume Up held
//! keeps stepping; Mute acts once per press, its repeats dropped by the
//! keyboard drivers.

pub mod keys;
mod line;
mod notice;

pub use keys::{press, volume_key, VolumeKey};
pub use notice::{is_volume_notice, notice, Heard};
