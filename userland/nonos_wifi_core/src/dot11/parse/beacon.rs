// NONOS Operating System (AGPL-3.0-or-later)

//! Beacon and probe-response frames: the fields a scan and a join read.

use super::super::header::{
    fc_subtype, fc_type, MAC_HEADER_LEN, SUBTYPE_BEACON, SUBTYPE_PROBE_RESP, TYPE_MGMT,
};
use super::ie::find_ie;

/// Beacon and probe-response fixed fields after the MAC header: timestamp (8),
/// beacon interval (2), capability info (2).
pub const FIXED_FIELDS_LEN: usize = 12;

/// Information-element ids a scan reads.
pub const IE_SSID: u8 = 0;
pub const IE_DS_PARAMS: u8 = 3;
pub const IE_RSN: u8 = 48;

/// The longest SSID (IEEE Std 802.11-2020, 9.4.2.2).
pub const SSID_LEN_MAX: usize = 32;

/// A parsed view of a beacon or probe response. `ssid` borrows the frame.
pub struct BeaconInfo<'a> {
    pub bssid: [u8; 6],
    pub capability: u16,
    pub ssid: &'a [u8],
    pub channel: u8,
    pub rsn: bool,
}

/// Parse a beacon or probe-response frame that starts at the MAC header.
/// Returns `None` if the frame is too short for the header and fixed fields, or
/// if it carries no SSID element or one longer than 32 octets. A missing
/// DS-parameter element leaves the channel at zero; an RSN element sets `rsn`.
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
    let ssid = find_ie(tags, IE_SSID).filter(|s| s.len() <= SSID_LEN_MAX)?;
    let channel = match find_ie(tags, IE_DS_PARAMS) {
        Some(ds) if !ds.is_empty() => ds[0],
        _ => 0,
    };
    let rsn = find_ie(tags, IE_RSN).is_some();
    Some(BeaconInfo { bssid, capability, ssid, channel, rsn })
}
