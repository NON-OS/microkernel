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

//! INQUIRY's standard data (SPC-5, 6.7.2) and the unit serial number page
//! (7.7.17).

/// The standard data the layer asks for: through the product revision.
pub const STANDARD_LEN: usize = 36;

/// Direct access block device: a disk.
pub const TYPE_DISK: u8 = 0x00;
/// CD/DVD device (MMC).
pub const TYPE_CDROM: u8 = 0x05;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Inquiry {
    /// 0: the unit is connected; anything else, no device behind it.
    pub qualifier: u8,
    pub device_type: u8,
    pub removable: bool,
    pub vendor: [u8; 8],
    pub product: [u8; 16],
}

pub fn parse(b: &[u8]) -> Option<Inquiry> {
    if b.len() < STANDARD_LEN {
        return None;
    }
    let mut vendor = [0u8; 8];
    vendor.copy_from_slice(&b[8..16]);
    let mut product = [0u8; 16];
    product.copy_from_slice(&b[16..32]);
    Some(Inquiry {
        qualifier: b[0] >> 5,
        device_type: b[0] & 0x1F,
        removable: b[1] & 0x80 != 0,
        vendor,
        product,
    })
}

/// The serial number in page 80h, its padding trimmed; None for another
/// page or one shorter than it says.
pub fn serial(b: &[u8]) -> Option<&[u8]> {
    if b.len() < 4 || b[1] != crate::cdb::VPD_SERIAL {
        return None;
    }
    let len = u16::from_be_bytes([b[2], b[3]]) as usize;
    let s = b.get(4..4 + len)?;
    let start = s.iter().position(|&c| c != b' ' && c != 0).unwrap_or(s.len());
    let end = s.iter().rposition(|&c| c != b' ' && c != 0).map_or(start, |i| i + 1);
    Some(&s[start..end])
}

#[cfg(test)]
mod tests {
    use super::*;

    fn standard(byte0: u8, rmb: bool) -> [u8; 36] {
        let mut b = [b' '; 36];
        b[0] = byte0;
        b[1] = if rmb { 0x80 } else { 0 };
        b[2..8].copy_from_slice(&[5, 2, 31, 0, 0, 0]);
        b[8..12].copy_from_slice(b"QEMU");
        b[16..29].copy_from_slice(b"QEMU HARDDISK");
        b
    }

    #[test]
    fn a_disk_and_a_cd_drive() {
        let i = parse(&standard(0x00, false)).unwrap();
        assert_eq!((i.qualifier, i.device_type, i.removable), (0, TYPE_DISK, false));
        assert_eq!(&i.vendor, b"QEMU    ");
        assert_eq!(&i.product[..13], b"QEMU HARDDISK");
        let i = parse(&standard(0x05, true)).unwrap();
        assert_eq!((i.device_type, i.removable), (TYPE_CDROM, true));
    }

    #[test]
    fn a_unit_with_nothing_behind_it() {
        let i = parse(&standard(0x7F, false)).unwrap();
        assert_eq!((i.qualifier, i.device_type), (3, 0x1F));
        assert!(parse(&standard(0, false)[..35]).is_none());
    }

    #[test]
    fn serial_page() {
        let mut p = vec![0x00, 0x80, 0x00, 0x0A];
        p.extend_from_slice(b"  SN-1234 ");
        assert_eq!(serial(&p), Some(&b"SN-1234"[..]));
        p[3] = 0x20;
        assert_eq!(serial(&p), None, "shorter than it says");
        assert_eq!(serial(&[0, 0x83, 0, 0]), None);
        assert_eq!(serial(&[0, 0x80, 0, 0]), Some(&b""[..]));
    }
}
