// NONOS Operating System (AGPL-3.0-or-later)
//! An ACM modem's configuration (CDC PSTN 1.2, as recalled, unverified):
//! a communications interface (class 2, subclass 2, protocol 1, AT
//! commands) with its Header, Call Management, ACM and Union functional
//! descriptors and an interrupt IN endpoint, then a data interface (class
//! 0x0A) with a bulk pair.

pub fn config(parts: &[&[u8]]) -> Vec<u8> {
    let body: Vec<u8> = parts.concat();
    let total = (9 + body.len()) as u16;
    let mut c = vec![9, 0x02, total as u8, (total >> 8) as u8, 2, 1, 0, 0x80, 50];
    c.extend(body);
    c
}

pub fn iface(num: u8, alt: u8, endpoints: u8, class: (u8, u8, u8)) -> Vec<u8> {
    vec![9, 0x04, num, alt, endpoints, class.0, class.1, class.2, 0]
}

pub fn ep(address: u8, attrs: u8, max_packet: u16, interval: u8) -> Vec<u8> {
    vec![7, 0x05, address, attrs, max_packet as u8, (max_packet >> 8) as u8, interval]
}

pub const HEADER: [u8; 5] = [5, 0x24, 0x00, 0x10, 0x01];
pub const CALL: [u8; 5] = [5, 0x24, 0x01, 0x00, 1];
pub fn acm(caps: u8) -> Vec<u8> {
    vec![4, 0x24, 0x02, caps]
}
pub const UNION: [u8; 5] = [5, 0x24, 0x06, 0, 1];

pub const COMM: (u8, u8, u8) = (0x02, 0x02, 0x01);
pub const DATA: (u8, u8, u8) = (0x0A, 0x00, 0x00);

/// The modem, with `functional` after its communications interface and
/// `notify` as its endpoint (empty for none).
pub fn modem(functional: &[&[u8]], notify: &[u8]) -> Vec<u8> {
    let mut parts: Vec<Vec<u8>> = vec![iface(0, 0, 1, COMM)];
    parts.extend(functional.iter().map(|d| d.to_vec()));
    parts.push(notify.to_vec());
    parts.extend([iface(1, 0, 2, DATA), ep(0x81, 0x02, 64, 0), ep(0x02, 0x02, 64, 0)]);
    let refs: Vec<&[u8]> = parts.iter().map(|p| p.as_slice()).collect();
    config(&refs)
}

pub fn plain() -> Vec<u8> {
    modem(&[&HEADER, &CALL, &acm(0x06), &UNION], &ep(0x83, 0x03, 16, 10))
}
