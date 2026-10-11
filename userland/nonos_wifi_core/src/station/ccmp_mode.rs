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

//! How a link protects its frames. The engine keeps the difference between a
//! hardware-CCMP chip (the radio encrypts from its key cache, so the station
//! only frames) and a software-CCMP chip (the station runs AES-CCM itself) in
//! one place, so no driver picks the wrong path and double-encrypts.

/// How the link protects data frames.
pub enum Ccmp {
    /// The chip encrypts and decrypts in hardware from its key cache. The
    /// station frames plaintext and lets the radio insert the CCMP header and
    /// manage packet numbers.
    Hardware,
    /// The station runs AES-CCM in software with the pairwise temporal key for
    /// both directions.
    Software { tk: [u8; 16] },
    /// The station encrypts transmit frames in software but leaves receive
    /// decryption to the radio. For a chip whose hardware transmit encryption
    /// does not work but whose receive decryption does.
    SoftwareTxHwRx { tk: [u8; 16] },
}
