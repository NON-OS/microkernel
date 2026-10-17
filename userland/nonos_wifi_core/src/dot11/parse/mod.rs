// NONOS Operating System (AGPL-3.0-or-later)

//! Parsing of 802.11 beacon and probe-response frames, the counterpart to the
//! management-frame builders. A scan turns received beacons into a list of
//! visible networks: BSSID, SSID, operating channel, and whether the network
//! advertises RSN (WPA2). Pure parsing over untrusted input with a bounded
//! walk and no panics, checked by `nonos_wifi_core_proofs`.

mod beacon;
mod ie;
mod status;

pub use beacon::{
    parse_beacon, BeaconInfo, FIXED_FIELDS_LEN, IE_DS_PARAMS, IE_RSN, IE_SSID, SSID_LEN_MAX,
};
pub use ie::find_ie;
pub use status::{parse_assoc_response, parse_auth_status};
