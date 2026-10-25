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
//! Known answers for the WPA2 primitives, run against the core's own code:
//! SHA-1, HMAC-SHA1, PBKDF2 and the PMK (`sha1`); AES-128, AES-CCM and AES
//! key wrap (`aes`); the EAPOL-Key frame and its MIC (`eapol`); the CCMP
//! data frame (`data`); and hostile input to each parser (`hostile`). They
//! were checked against iwlwifi's private copies until those were deleted.

mod aes;
mod data;
mod eapol;
mod hostile;
mod sha1;
