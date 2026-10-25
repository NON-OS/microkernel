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

/// The Finished verify_data for `secret`, given the transcript hash it covers.
pub fn finished_value(secret: &[u8; 32], th: &[u8; 32]) -> Option<[u8; 32]> {
    let mut key = super::finished_key::finished_key(secret)?;
    let value = super::hmac_sha256::tag(&key, &[th]);
    super::traffic_keys::wipe(&mut key);
    value
}
