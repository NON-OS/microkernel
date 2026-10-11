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

//! Which network the access point runs, and the switches that make it fail.

/// What the access point is to do.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Security {
    Wpa2,
    Sae { h2e: bool },
}

#[derive(Default)]
pub struct Switches {
    /// Answer authentication with this status.
    pub refuse_auth: Option<u16>,
    /// Answer association with this status.
    pub refuse_assoc: Option<u16>,
    /// Answer nothing at all.
    pub silent: bool,
    /// Answer authentication, then nothing.
    pub silent_after_auth: bool,
    /// Answer association (and message 1), then nothing.
    pub silent_after_m1: bool,
    /// Deauthenticate (reason 15) instead of sending message 3.
    pub deauth_instead_of_m3: bool,
    /// Run SAE with another password.
    pub wrong_password: bool,
    /// Key the four-way handshake with another PMK (a wrong WPA2 passphrase).
    pub wrong_pmk: bool,
    /// Deliver protected frames as the firmware decrypted them.
    pub fw_decrypts: bool,
    /// After message 1, a message 1 from another BSS.
    pub stray_m1: bool,
    /// Lose the first authentication frame, as a busy channel does.
    pub drop_first_auth: bool,
}
