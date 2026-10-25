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

//! Join a WPA2 or WPA3 network. The shared `Mlme` state machine owns all the
//! protocol and crypto: it takes the SSID, the passphrase and the person's
//! policy, and given the frames received while joining it produces the frames
//! to send (Open System or SAE authentication, association, the four-way
//! handshake) and finally the keys and the supplicant. This module is the radio
//! half: it feeds received frames to the machine (management frames drive
//! scan/auth/assoc, EAPOL data frames from the access point to this station
//! drive the four-way handshake) and transmits what the machine returns
//! (management frames go out raw, EAPOL replies are wrapped into an
//! unencrypted 802.11 data frame, since the keys do not exist yet). The frame
//! classification and the EAPOL wrapping are pure and checked on the host;
//! only the transmit and receive calls touch a radio.

pub mod classify;
mod clock;
mod drive;
pub mod eapol;
mod frame;
mod outcome;
mod radio;
mod report;
pub mod resend;
mod run;
mod send;
mod state_code;

pub use clock::JoinClock;
pub use outcome::Outcome;
pub use radio::Radio;
pub use report::Report;
pub use run::*;
