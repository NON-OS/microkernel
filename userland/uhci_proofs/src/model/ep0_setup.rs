// NONOS Operating System (AGPL-3.0-or-later)
//! What a SETUP packet asks of the device: the bytes an IN request's data
//! stage carries, or what an OUT request changes; a hub answers its class
//! requests (hub.rs).

use super::device::Device;

pub(super) fn prepare(d: &mut Device, s: [u8; 8]) {
    let len = u16::from_le_bytes([s[6], s[7]]) as usize;
    if let Some(data) = d.hub.as_mut().and_then(|h| h.request(s)) {
        d.ep0.data = data.into_iter().take(len).collect();
        return;
    }
    let data: Vec<u8> = match (s[0], s[1], s[3]) {
        (0x80, 6, 1) => d.desc.to_vec(),
        (0x80, 6, 2) => d.config.clone(),
        (0x00, 5, _) => {
            d.ep0.new_addr = Some(s[2]);
            vec![]
        }
        // A vendor IN request: its bytes count up from zero.
        (0xC0, _, _) => (0..len).map(|i| i as u8).collect(),
        _ => vec![],
    };
    d.ep0.data = data.into_iter().take(len).collect();
}
