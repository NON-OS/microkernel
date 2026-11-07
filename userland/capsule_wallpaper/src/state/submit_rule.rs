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

//! Whether the compositor has the wallpaper after a scene submit. Pure, so
//! wallpaper_catalog_proofs holds it.
//!
//! The kernel queues a call's request as it is sent and a timeout does not
//! take it back: a submit whose answer came late is served all the same once
//! the compositor gets to it, as a commit is (server/scene/commit.rs). The
//! compositor repaints the whole layer, here the whole screen, for every
//! submit, so asking again after a late answer had it compose the whole
//! screen once more; under load that frame made the next answer late too,
//! and the wallpaper asked again every few seconds for good, on a desktop
//! where nothing moved. Only a submit the kernel did not deliver or the
//! compositor refused is asked again.

/// The answer a submit got: `Ok`, or the error the call returned, with
/// `timed_out` the one that means no answer came in time.
pub fn taken(answer: Result<(), &str>, timed_out: &str) -> bool {
    match answer {
        Ok(()) => true,
        Err(e) => e == timed_out,
    }
}
