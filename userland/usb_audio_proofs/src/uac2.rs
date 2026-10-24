// NONOS Operating System (AGPL-3.0-or-later)
//! USB Audio 2.0 descriptors built a descriptor at a time (2.0 4.7 and 4.9,
//! as recalled, unverified), and a high-speed DAC: one clock, a USB
//! streaming Input Terminal, headphones, and an asynchronous OUT endpoint
//! with its feedback IN endpoint in the same alternate setting.

use crate::devices::{config, iface};

pub const SOURCE: u8 = 0x10;

pub fn clock_source(id: u8, controls: u8) -> Vec<u8> {
    vec![8, 0x24, 0x0A, id, 0x01, controls, 0, 0]
}

pub fn clock_selector(id: u8, pins: &[u8]) -> Vec<u8> {
    let mut d = vec![0, 0x24, 0x0B, id, pins.len() as u8];
    d.extend(pins);
    d.extend([0x03, 0]);
    d[0] = d.len() as u8;
    d
}

pub fn streaming_in(id: u8, clock: u8) -> Vec<u8> {
    vec![17, 0x24, 0x02, id, 0x01, 0x01, 0, clock, 2, 3, 0, 0, 0, 0, 0, 0, 0]
}

pub fn headphones_out(id: u8, source: u8, clock: u8) -> Vec<u8> {
    vec![12, 0x24, 0x03, id, 0x02, 0x03, 0, source, clock, 0, 0, 0]
}

pub fn general2(link: u8, formats: u32, channels: u8) -> Vec<u8> {
    let mut d = vec![16, 0x24, 0x01, link, 0, 1];
    d.extend(formats.to_le_bytes());
    d.extend([channels, 3, 0, 0, 0, 0]);
    d
}

pub fn format2(subslot: u8, bits: u8) -> Vec<u8> {
    vec![6, 0x24, 0x02, 1, subslot, bits]
}

pub fn ep(address: u8, attrs: u8, max_packet: u16, interval: u8) -> Vec<u8> {
    vec![7, 0x05, address, attrs, max_packet as u8, (max_packet >> 8) as u8, interval]
}

pub fn cs_ep2() -> Vec<u8> {
    vec![8, 0x25, 0x01, 0, 0, 0, 0, 0]
}

/// The DAC with `control` (its AudioControl descriptors after the
/// interface) and `stream` (its playback alternate's class descriptors).
pub fn dac(control: &[&[u8]], stream: &[&[u8]]) -> Vec<u8> {
    let mut parts: Vec<Vec<u8>> = vec![iface(0, 0, 1, 0x20)];
    parts.extend(control.iter().map(|d| d.to_vec()));
    parts.extend([iface(1, 0, 2, 0x20), iface(1, 1, 2, 0x20)]);
    parts.extend(stream.iter().map(|d| d.to_vec()));
    // Asynchronous data OUT, then its feedback IN (usage 1, bits 5:4).
    parts.extend([ep(0x01, 0x05, 200, 4), cs_ep2(), ep(0x81, 0x11, 4, 4)]);
    let refs: Vec<&[u8]> = parts.iter().map(|p| p.as_slice()).collect();
    config(&refs)
}

pub fn plain_dac() -> Vec<u8> {
    let control =
        [clock_source(SOURCE, 0x07), streaming_in(1, SOURCE), headphones_out(3, 1, SOURCE)];
    let c: Vec<&[u8]> = control.iter().map(|d| d.as_slice()).collect();
    dac(&c, &[&general2(1, 1, 2), &format2(2, 16)])
}
