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

//! Parse a beacon or probe-response frame that starts at the MAC header.

use super::*;

/// Parse a beacon or probe-response frame that starts at the MAC header.
/// Returns `None` if the frame is too short for the header and fixed fields, or
/// if it carries no SSID element. A missing DS-parameter element leaves the
/// channel at zero; an RSN element sets `rsn`.
pub fn parse_beacon(frame: &[u8]) -> Option<BeaconInfo<'_>> {
    if frame.len() < MAC_HEADER_LEN + FIXED_FIELDS_LEN {
        return None;
    }
    let fc = u16::from_le_bytes([frame[0], frame[1]]);
    if fc_type(fc) != TYPE_MGMT
        || (fc_subtype(fc) != SUBTYPE_BEACON && fc_subtype(fc) != SUBTYPE_PROBE_RESP)
    {
        return None;
    }
    let mut bssid = [0u8; 6];
    bssid.copy_from_slice(&frame[16..22]);
    let fixed = MAC_HEADER_LEN;
    let capability = u16::from_le_bytes([frame[fixed + 10], frame[fixed + 11]]);
    let tags = &frame[MAC_HEADER_LEN + FIXED_FIELDS_LEN..];
    let ssid = find_ie(tags, IE_SSID)?;
    let channel = match find_ie(tags, IE_DS_PARAMS) {
        Some(ds) if !ds.is_empty() => ds[0],
        _ => 0,
    };
    let rsn = find_ie(tags, IE_RSN).is_some();
    Some(BeaconInfo { bssid, capability, ssid, channel, rsn })
}

/// The status code of an open-system authentication response, or `None` if the
/// frame is not an authentication frame. Zero is success. The body after the
/// MAC header is algorithm (2), transaction sequence (2), status (2); a
/// well-formed open-auth response carries algorithm 0 and sequence 2.
pub fn parse_auth_status(frame: &[u8]) -> Option<u16> {
    if frame.len() < MAC_HEADER_LEN + 6 {
        return None;
    }
    let fc = u16::from_le_bytes([frame[0], frame[1]]);
    if fc_type(fc) != TYPE_MGMT || fc_subtype(fc) != super::super::header::SUBTYPE_AUTH {
        return None;
    }
    Some(u16::from_le_bytes([frame[MAC_HEADER_LEN + 4], frame[MAC_HEADER_LEN + 5]]))
}
