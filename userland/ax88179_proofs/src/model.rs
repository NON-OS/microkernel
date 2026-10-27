// NONOS Operating System (AGPL-3.0-or-later)
//! The model the driver is held to, written here from Linux v6.12's
//! ax88179_178a.c (read for behaviour) with this driver's choices marked,
//! never from the driver's own tables: the vendor requests of the
//! bring-up in order, and what each refusal is called.

/// A vendor request: (bRequest, wValue, wIndex, data out or bytes in).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Want {
    Out(u8, u16, u16, Vec<u8>),
    In(u8, u16, u16, usize),
}

fn mac(reg: u16, data: &[u8]) -> Want {
    Want::Out(1, reg, data.len() as u16, data.to_vec())
}

fn phy(reg: u16, v: u16) -> Want {
    Want::Out(2, 3, reg, v.to_le_bytes().to_vec())
}

/// Linux's reset order. Ours: no factory-address read; the drawn address
/// written instead; a 4 KiB bulk-in queue; checksum offload written off;
/// the RX control its multicast path sets (no CRC-drop or AP bit); no
/// monitor-mode, LED or auto-detach writes.
pub fn bring_up(station: [u8; 6]) -> Vec<(Want, &'static str)> {
    vec![
        (mac(0x26, &[0, 0]), "PHY reset refused"),
        (mac(0x26, &[0x20, 0]), "PHY power refused"),
        (mac(0x33, &[0x03]), "clocks refused"),
        (mac(0x10, &station), "station address refused"),
        (mac(0x2e, &[7, 0x4f, 0, 2, 0xff]), "bulk-in queue refused"),
        (mac(0x55, &[0x34]), "pause levels refused"),
        (mac(0x54, &[0x52]), "pause levels refused"),
        (mac(0x34, &[0]), "checksum offload refused"),
        (mac(0x35, &[0]), "checksum offload refused"),
        (mac(0x16, &[0xff; 8]), "group table refused"),
        (mac(0x0b, &[0x8a, 0x02]), "receive control refused"),
        (mac(0x22, &[0x33, 0x01]), "medium refused"),
        (phy(0x1f, 3), "EEE refused"),
        (phy(0x19, 0x3246), "EEE refused"),
        (phy(0x1f, 0), "EEE refused"),
        (phy(0x0d, 7), "EEE refused"),
        (phy(0x0e, 0x3c), "EEE refused"),
        (phy(0x0d, 0x4007), "EEE refused"),
        (phy(0x0e, 0), "EEE refused"),
        (Want::In(2, 3, 0, 2), "PHY control unread"),
        // Autonegotiation enabled and restarted over what the PHY held.
        (phy(0, 0x1140 | 0x1200), "autonegotiation refused"),
    ]
}

/// The waits, each after the vendor request it follows (counted from 1).
pub const PAUSES: [(u64, usize); 2] = [(500, 2), (200, 3)];

/// What link-up sends for a PHY status: one receiver restart and its FIFO
/// read, the queue again, the medium.
pub fn link_up(medium: u16) -> Vec<Want> {
    vec![
        mac(0x0b, &[0, 0]),
        mac(0x0b, &[0x8a, 0x02]),
        Want::In(0x81, 0x8c, 0, 4),
        mac(0x2e, &[7, 0x4f, 0, 2, 0xff]),
        mac(0x22, &medium.to_le_bytes()),
    ]
}
