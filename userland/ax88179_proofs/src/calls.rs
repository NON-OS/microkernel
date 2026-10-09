// NONOS Operating System (AGPL-3.0-or-later)
//! The register requests a test expects, built from Linux's convention
//! (ax88179_read_cmd, ax88179_write_cmd), not from the driver's code.

use nonos_usbnet::mock::Call;
use nonos_usbnet::Setup;

/// A MAC register write: 0x40, AX_ACCESS_MAC, the register, its width.
pub fn mac_w(reg: u16, data: &[u8]) -> Call {
    Call::Out(Setup::new(0x40, 0x01, reg, data.len() as u16), data.to_vec())
}

/// A MAC register read: 0xC0, AX_ACCESS_MAC, the register, its width.
pub fn mac_r(reg: u16, width: u16) -> Call {
    Call::In(Setup::new(0xC0, 0x01, reg, width), width as usize)
}

/// A PHY register write: 0x40, AX_ACCESS_PHY, PHY address 3, the register.
pub fn phy_w(reg: u16, v: u16) -> Call {
    Call::Out(Setup::new(0x40, 0x02, 3, reg), v.to_le_bytes().to_vec())
}
