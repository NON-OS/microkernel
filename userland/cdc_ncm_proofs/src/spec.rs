// NONOS Operating System (AGPL-3.0-or-later)
//! Descriptors and NTB parameters laid out field by field as USB NCM 1.0
//! section 5 (table 5-1's interfaces, the NCM functional descriptor of
//! 5.2.1) and table 6-3 give them, with CDC ECM 1.2's Ethernet Networking
//! descriptor. They are built from the spec, not dumped from a device.

/// A device descriptor offering `configs` configurations (USB 2.0, 9-8).
pub fn device(vendor: u16, configs: u8) -> Vec<u8> {
    let [v0, v1] = vendor.to_le_bytes();
    vec![0x12, 0x01, 0x00, 0x02, 0, 0, 0, 0x40, v0, v1, 0x01, 0x00, 0, 1, 1, 2, 3, configs]
}

/// A configuration of value `value` with one NCM function: interface 0
/// communications (subclass 0x0D) with its interrupt endpoint, interface 1
/// data with alternate 0 empty and alternate 1 holding bulk IN 0x82 and
/// bulk OUT 0x03 of 512 bytes. The MAC string is index 4.
pub fn config_ncm(value: u8, caps: u8) -> Vec<u8> {
    let mut c = vec![0x09, 0x02, 0, 0, 0x02, value, 0x00, 0x80, 0xFA];
    c.extend([0x09, 0x04, 0x00, 0x00, 0x01, 0x02, 0x0D, 0x00, 0x00]); // NCM comm
    c.extend([0x05, 0x24, 0x00, 0x10, 0x01]); // header, CDC 1.10
    c.extend([0x05, 0x24, 0x06, 0x00, 0x01]); // union: 0 controls 1
    c.extend([0x0D, 0x24, 0x0F, 0x04, 0, 0, 0, 0, 0xEA, 0x05, 0, 0, 0]); // ethernet
    c.extend([0x06, 0x24, 0x1A, 0x00, 0x01, caps]); // NCM 1.00
    c.extend([0x07, 0x05, 0x81, 0x03, 0x10, 0x00, 0x09]); // interrupt IN
    c.extend([0x09, 0x04, 0x01, 0x00, 0x00, 0x0A, 0x00, 0x01, 0x00]); // data, off
    c.extend([0x09, 0x04, 0x01, 0x01, 0x02, 0x0A, 0x00, 0x01, 0x00]); // data, on
    c.extend([0x07, 0x05, 0x82, 0x02, 0x00, 0x02, 0x00]); // bulk IN
    c.extend([0x07, 0x05, 0x03, 0x02, 0x00, 0x02, 0x00]); // bulk OUT
    let total = (c.len() as u16).to_le_bytes();
    c[2..4].copy_from_slice(&total);
    c
}

/// A configuration of value 1 with one vendor-class interface and a bulk
/// pair: what a device may offer before its NCM configuration.
pub fn config_vendor() -> Vec<u8> {
    let mut c = vec![0x09, 0x02, 32, 0, 0x01, 0x01, 0x00, 0x80, 0xFA];
    c.extend([0x09, 0x04, 0x00, 0x00, 0x02, 0xFF, 0xFE, 0x01, 0x00]);
    c.extend([0x07, 0x05, 0x81, 0x02, 0x00, 0x02, 0x00]);
    c.extend([0x07, 0x05, 0x02, 0x02, 0x00, 0x02, 0x00]);
    c
}

/// "020000000001", UTF-16LE (CDC 1.2, 5.2.3.16).
pub fn mac_string() -> Vec<u8> {
    let mut s = vec![26, 0x03];
    "020000000001".bytes().for_each(|b| s.extend([b, 0]));
    s
}
pub const MAC: [u8; 6] = [0x02, 0, 0, 0, 0, 0x01];

/// The NTB parameter structure (NCM 1.0, table 6-3), the IN side's
/// divisor and alignment at 4 and wNtbOutMaxDatagrams 0.
pub fn ntb_params(formats: u16, in_max: u32, out_max: u32, out: (u16, u16, u16)) -> Vec<u8> {
    let mut p = vec![0x1C, 0x00];
    p.extend(formats.to_le_bytes());
    p.extend(in_max.to_le_bytes());
    p.extend([4, 0, 0, 0, 4, 0, 0, 0]);
    p.extend(out_max.to_le_bytes());
    [out.0, out.1, out.2, 0].iter().for_each(|w| p.extend(w.to_le_bytes()));
    p
}
