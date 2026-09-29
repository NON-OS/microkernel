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

use alloc::vec::Vec;

use super::app_reader::AppReader;
use super::flight::ClientFlight;
use super::traffic_keys::TrafficKeys;

/// Decrypt a whole response with application keys already derived. A caller
/// reading as the response arrives keeps an `AppReader` instead, which opens
/// each record once rather than once per call.
pub fn application_plaintext_cached(app: &TrafficKeys, response: &[u8]) -> Vec<u8> {
    let mut reader = AppReader::new();
    reader.feed(app, response);
    reader.into_plaintext()
}

/// Verify the flight against `host`, then decrypt a whole response.
pub fn application_plaintext(
    client: &ClientFlight,
    flight: &[u8],
    response: &[u8],
    host: &[u8],
    now: u64,
) -> Option<Vec<u8>> {
    let done = super::server_complete::server_complete(client, flight, host, now)?;
    Some(application_plaintext_cached(&done.app, response))
}
