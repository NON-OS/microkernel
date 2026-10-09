// NONOS Operating System (AGPL-3.0-or-later)
//! RNDIS messages as QEMU 8.2 hw/usb/dev-network.c builds them
//! (rndis_init_response, rndis_query_response, rndis_set_response,
//! usbnet_receive) and reads them (usb_net_handle_dataout), plus the
//! INDICATE_STATUS layout of Remote NDIS 1.0, 2.2.12, which QEMU never
//! sends but phones do.

/// The station address QEMU gives its first NIC when none is set
/// (qemu_macaddr_default_if_unset): 52:54:00:12:34:56.
pub const NIC_MAC: [u8; 6] = [0x52, 0x54, 0x00, 0x12, 0x34, 0x56];
pub const STATUS_NOT_SUPPORTED: u32 = 0xC000_00BB;
pub const STATUS_MEDIA_CONNECT: u32 = 0x4001_000B;

pub fn words(w: &[u32]) -> Vec<u8> {
    w.iter().flat_map(|v| v.to_le_bytes()).collect()
}

pub fn le(b: &[u8], at: usize) -> u32 {
    u32::from_le_bytes([b[at], b[at + 1], b[at + 2], b[at + 3]])
}

/// INITIALIZE_CMPLT: success, RNDIS 1.0, connectionless, 802.3, one
/// packet per transfer of at most ETH_FRAME_LEN + 44 + 22 bytes.
pub fn init_cmplt(id: u32) -> Vec<u8> {
    words(&[0x8000_0002, 52, id, 0, 1, 0, 1, 0, 1, 1514 + 44 + 22, 0, 0, 0])
}

/// QUERY_CMPLT with `info` after its 24 bytes, or NOT_SUPPORTED for none.
pub fn query_cmplt(id: u32, info: Option<&[u8]>) -> Vec<u8> {
    let Some(info) = info else { return words(&[0x8000_0004, 24, id, STATUS_NOT_SUPPORTED, 0, 0]) };
    let off = if info.is_empty() { 0 } else { 16 };
    let mut r = words(&[0x8000_0004, 24 + info.len() as u32, id, 0, info.len() as u32, off]);
    r.extend_from_slice(info);
    r
}

pub fn set_cmplt(id: u32, status: u32) -> Vec<u8> {
    words(&[0x8000_0005, 16, id, status])
}

pub fn indicate(status: u32) -> Vec<u8> {
    words(&[7, 20, status, 0, 0])
}

/// A received frame as usbnet_receive queues it for the bulk IN pipe.
pub fn packet(frame: &[u8]) -> Vec<u8> {
    let mut m = words(&[1, 44 + frame.len() as u32, 36, frame.len() as u32, 0, 0, 0, 0, 0, 0, 0]);
    m.extend_from_slice(frame);
    m
}

/// What usb_net_handle_dataout sends on for one bulk OUT, and the bytes it
/// keeps for the next message, which must be none.
pub fn qemu_takes(out: &[u8]) -> (Option<Vec<u8>>, usize) {
    let len = le(out, 4) as usize;
    if out.len() < 8 || out.len() < len {
        return (None, out.len());
    }
    let (offs, size) = (8 + le(out, 8) as usize, le(out, 12) as usize);
    let ok = le(out, 0) == 1 && offs < len && size < len && offs + size <= len;
    (ok.then(|| out[offs..offs + size].to_vec()), out.len() - len)
}
