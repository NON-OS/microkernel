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

/*
 * The points a launch passes, in the order it passes them. The first eight
 * happen inside the spawn call; the last three happen once the capsule runs
 * and are the first of their kind for that pid. The spawn path names only
 * the middle ones; the rest exist where the trace is built.
 */
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub(crate) enum Phase {
    #[cfg(feature = "nonos-launch-trace")]
    Begin = 0,
    CertVerified = 1,
    ManifestVerified = 2,
    Attested = 3,
    ProcessCreated = 4,
    ImageLoaded = 5,
    Installed = 6,
    #[cfg(feature = "nonos-launch-trace")]
    Queued = 7,
    #[cfg(feature = "nonos-launch-trace")]
    FirstSyscall = 8,
    #[cfg(feature = "nonos-launch-trace")]
    FirstSurface = 9,
    #[cfg(feature = "nonos-launch-trace")]
    FirstFrame = 10,
}
