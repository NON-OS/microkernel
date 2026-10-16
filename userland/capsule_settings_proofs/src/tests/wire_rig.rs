// NONOS Operating System (AGPL-3.0-or-later)

//! Scan buffers built entry by entry, and what a parse yields, for the wire
//! proofs.

use crate::wifi::network::ScanNetwork;
use crate::wifi::wire::parse_scan;

pub(crate) fn collect(buf: &[u8]) -> Vec<ScanNetwork> {
    let mut out = Vec::new();
    parse_scan(buf, |n| out.push(n));
    out
}

// [count] then [signal][flags][ssid_len][ssid] per entry.
pub(crate) fn entry(signal: u8, secured: bool, ssid: &[u8]) -> Vec<u8> {
    let mut e = vec![signal, if secured { 1 } else { 0 }, ssid.len() as u8];
    e.extend_from_slice(ssid);
    e
}
