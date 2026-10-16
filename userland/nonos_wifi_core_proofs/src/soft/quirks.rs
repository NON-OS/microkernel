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

//! What an access point can be told to get wrong, for the proofs of what
//! the station does then.

#[derive(Default)]
pub struct Quirks {
    /// The RSNE message 3 carries, in place of the beacon's.
    pub msg3_rsne: Option<Vec<u8>>,
    /// Accept a (re)association, then never send message 1 or answer one.
    pub mute_eapol: bool,
    /// A Transition Disable KDE with these bits, last in message 3.
    pub transition_disable: Option<u8>,
    /// Beacon protection, once the AP is keyed (`protect`).
    pub beacons: Beacons,
}

#[derive(Default)]
pub struct Beacons {
    /// Give a BIGTK in message 3 and end each beacon with its MME.
    pub protect: bool,
    /// Give the BIGTK, but send every beacon bare, as a forger in the AP's
    /// name would.
    pub bare: bool,
    /// The BIPN the last protected beacon carried.
    pub bipn: u64,
}
