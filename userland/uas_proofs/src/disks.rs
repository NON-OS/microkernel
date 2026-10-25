// NONOS Operating System (AGPL-3.0-or-later)
//! A UAS disk's configuration (USB UAS 1.0, as recalled, unverified): the
//! mass-storage interface's BOT alternate 0 (protocol 50h) with a bulk
//! pair, then its UAS alternate 1 (62h) with four bulk endpoints, each
//! followed (after its SuperSpeed companion, at SuperSpeed) by a Pipe
//! Usage descriptor.

pub fn config(parts: &[Vec<u8>]) -> Vec<u8> {
    let body: Vec<u8> = parts.concat();
    let total = (9 + body.len()) as u16;
    let mut c = vec![9, 0x02, total as u8, (total >> 8) as u8, 1, 1, 0, 0x80, 50];
    c.extend(body);
    c
}

pub fn iface(alt: u8, endpoints: u8, protocol: u8) -> Vec<u8> {
    vec![9, 0x04, 0, alt, endpoints, 0x08, 0x06, protocol, 0]
}

pub fn bulk(address: u8, max_packet: u16) -> Vec<u8> {
    vec![7, 0x05, address, 0x02, max_packet as u8, (max_packet >> 8) as u8, 0]
}

pub fn companion(burst: u8, streams: u8) -> Vec<u8> {
    vec![6, 0x30, burst, streams, 0, 0]
}

pub fn usage(id: u8) -> Vec<u8> {
    vec![4, 0x24, id, 0]
}

/// The disk at high speed (`ss` false, 512-byte packets) or SuperSpeed
/// (1024, bursts of 16, 32 streams on all but the command pipe).
pub fn disk(ss: bool) -> Vec<u8> {
    let mps = if ss { 1024 } else { 512 };
    let mut parts = vec![iface(0, 2, 0x50), bulk(0x81, mps), bulk(0x02, mps)];
    parts.push(iface(1, 4, 0x62));
    for (address, id) in [(0x04, 1), (0x83, 2), (0x85, 3), (0x06, 4)] {
        parts.push(bulk(address, mps));
        if ss {
            parts.push(companion(15, if id == 1 { 0 } else { 5 }));
        }
        parts.push(usage(id));
    }
    config(&parts)
}
