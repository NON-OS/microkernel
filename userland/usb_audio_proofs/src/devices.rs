// NONOS Operating System (AGPL-3.0-or-later)
//! Configuration descriptors built a descriptor at a time, and the devices
//! the proofs bind: QEMU's usb-audio (hw/usb/dev-audio.c, from memory of
//! it: one stereo 48 kHz alternate, a synchronous isoch OUT endpoint of 192
//! bytes, a speaker terminal) and a USB headset with a capture side.

pub fn config(parts: &[&[u8]]) -> Vec<u8> {
    let body: Vec<u8> = parts.concat();
    let total = (9 + body.len()) as u16;
    let mut c = vec![9, 0x02, total as u8, (total >> 8) as u8, 2, 1, 0, 0x80, 50];
    c.extend(body);
    c
}

pub fn iface(num: u8, alt: u8, sub: u8, proto: u8) -> Vec<u8> {
    vec![9, 0x04, num, alt, 1, 0x01, sub, proto, 0]
}

pub fn output_terminal(kind: u16) -> Vec<u8> {
    vec![9, 0x24, 0x03, 3, kind as u8, (kind >> 8) as u8, 0, 2, 0]
}

pub fn as_general(format: u16) -> Vec<u8> {
    vec![7, 0x24, 0x01, 1, 1, format as u8, (format >> 8) as u8]
}

fn rate(hz: u32) -> [u8; 3] {
    [hz as u8, (hz >> 8) as u8, (hz >> 16) as u8]
}

/// Format Type I with the listed rates, or a continuous range for `[]`.
pub fn format_i(channels: u8, bits: u8, rates: &[u32], range: (u32, u32)) -> Vec<u8> {
    let mut d = vec![0, 0x24, 0x02, 1, channels, bits / 8, bits, rates.len() as u8];
    let listed: Vec<u32> = if rates.is_empty() { vec![range.0, range.1] } else { rates.to_vec() };
    listed.iter().for_each(|&hz| d.extend(rate(hz)));
    d[0] = d.len() as u8;
    d
}

pub fn iso_ep(address: u8, attrs: u8, max_packet: u16, interval: u8) -> Vec<u8> {
    vec![9, 0x05, address, attrs, max_packet as u8, (max_packet >> 8) as u8, interval, 0, 0]
}

pub fn cs_ep(attrs: u8) -> Vec<u8> {
    vec![7, 0x25, 0x01, attrs, 0, 0, 0]
}

pub fn qemu_usb_audio() -> Vec<u8> {
    let (ac, off, on) = (iface(0, 0, 1, 0), iface(1, 0, 2, 0), iface(1, 1, 2, 0));
    let (term, gen, fmt) =
        (output_terminal(0x0301), as_general(1), format_i(2, 16, &[48_000], (0, 0)));
    let (ep, cs) = (iso_ep(0x01, 0x0d, 192, 1), cs_ep(0x00));
    config(&[&ac, &term, &off, &on, &gen, &fmt, &ep, &cs])
}

/// Playback at 44.1 or 48 kHz on interface 1 (a 24-bit alternate first),
/// and a mono microphone on interface 2.
pub fn headset() -> Vec<u8> {
    let (ac, mic_term, term) =
        (iface(0, 0, 1, 0), output_terminal(0x0101), output_terminal(0x0402));
    let (off, alt24, alt16) = (iface(1, 0, 2, 0), iface(1, 1, 2, 0), iface(1, 2, 2, 0));
    let gen = as_general(1);
    let fmt24 = format_i(2, 24, &[48_000], (0, 0));
    let fmt16 = format_i(2, 16, &[44_100, 48_000], (0, 0));
    let (ep24, ep16, cs) = (iso_ep(0x02, 0x09, 288, 1), iso_ep(0x02, 0x09, 192, 1), cs_ep(0x01));
    let (mic, mic_fmt) = (iface(2, 1, 2, 0), format_i(1, 16, &[48_000], (0, 0)));
    let mic_ep = iso_ep(0x81, 0x05, 96, 1);
    config(&[
        &ac, &mic_term, &term, &off, &alt24, &gen, &fmt24, &ep24, &cs, &alt16, &gen, &fmt16, &ep16,
        &cs, &mic, &gen, &mic_fmt, &mic_ep, &cs,
    ])
}
