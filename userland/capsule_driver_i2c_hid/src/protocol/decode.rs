use crate::protocol::header::{Request, HDR_LEN, MAGIC, VERSION};

pub fn parse(buf: &[u8]) -> Option<(Request, &[u8])> {
    if buf.len() < HDR_LEN {
        return None;
    }
    if u32::from_le_bytes(buf[0..4].try_into().ok()?) != MAGIC {
        return None;
    }
    if u16::from_le_bytes(buf[4..6].try_into().ok()?) != VERSION {
        return None;
    }
    let op = u16::from_le_bytes(buf[6..8].try_into().ok()?);
    let request_id = u64::from_le_bytes(buf[8..16].try_into().ok()?);
    let len = u32::from_le_bytes(buf[16..20].try_into().ok()?) as usize;
    if HDR_LEN + len > buf.len() {
        return None;
    }
    Some((Request { op, request_id }, &buf[HDR_LEN..HDR_LEN + len]))
}

/// The request a frame `parse` refused is answered under: the op and request
/// id it names, or zeros when it is too short to name them. Its caller is
/// blocked in its call until a reply comes, so a refusal is answered too.
pub fn refused(buf: &[u8]) -> Request {
    let Some(h) = buf.first_chunk::<HDR_LEN>() else {
        return Request { op: 0, request_id: 0 };
    };
    Request {
        op: u16::from_le_bytes([h[6], h[7]]),
        request_id: u64::from_le_bytes([h[8], h[9], h[10], h[11], h[12], h[13], h[14], h[15]]),
    }
}
