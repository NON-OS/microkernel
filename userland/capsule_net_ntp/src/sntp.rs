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

//! One SNTP exchange: the request, and the checks a reply must pass before its
//! time is believed. A reply must echo the request's transmit timestamp as its
//! originate timestamp (RFC 5905), which an off-path sender has to guess.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SntpError {
    Malformed,
    NotServer,
    /// Leap indicator 3: the server says its own clock is not synchronized.
    Unsynchronized,
    /// The originate timestamp is not the one this request carried.
    NotOurs,
    BadTimestamp,
}

const NTP_UNIX_DELTA_SECS: u64 = 2_208_988_800;
const ERA: u64 = 1 << 32;

/// A client request (version 4, mode 3) carrying `nonce` as its transmit time.
pub fn build_request(nonce: [u8; 8]) -> [u8; 48] {
    let mut b = [0u8; 48];
    b[0] = 0x23;
    b[40..48].copy_from_slice(&nonce);
    b
}

/// The server's transmit time in unix milliseconds, if the reply answers the
/// request that carried `nonce`.
pub fn parse_reply(buf: &[u8], nonce: &[u8; 8]) -> Result<u64, SntpError> {
    let Ok(b) = <&[u8; 48]>::try_from(buf) else {
        return Err(SntpError::Malformed);
    };
    let (leap, version, mode, stratum) = (b[0] >> 6, (b[0] >> 3) & 7, b[0] & 7, b[1]);
    if mode != 4 || !(3..=4).contains(&version) || stratum == 0 || stratum > 15 {
        return Err(SntpError::NotServer);
    }
    if leap == 3 {
        return Err(SntpError::Unsynchronized);
    }
    if b[24..32] != nonce[..] {
        return Err(SntpError::NotOurs);
    }
    let secs = u64::from(u32::from_be_bytes([b[40], b[41], b[42], b[43]]));
    if secs == 0 {
        return Err(SntpError::BadTimestamp);
    }
    /* Era 0 ends in February 2036. A value with the top bit clear is read as
     * era 1, so the clock keeps working past the rollover. One from 1968 or
     * 1969 has no unix time: it was taken unchecked, and the subtraction
     * wrapped to a clock thousands of years off. */
    let ntp = if secs & 0x8000_0000 != 0 { secs } else { secs + ERA };
    let Some(unix) = ntp.checked_sub(NTP_UNIX_DELTA_SECS) else {
        return Err(SntpError::BadTimestamp);
    };
    Ok(unix * 1000)
}
