// NONOS Operating System (AGPL-3.0-or-later)

//! The status codes of authentication and association responses.

use super::super::header::{
    fc_subtype, fc_type, MAC_HEADER_LEN, TYPE_MGMT,
};

/// The status code of an open-system authentication response, or `None` if the
/// frame is not an authentication frame. Zero is success. The body after the
/// MAC header is algorithm (2), transaction sequence (2), status (2); a
/// well-formed open-auth response carries algorithm 0 and sequence 2.
pub fn parse_auth_status(frame: &[u8]) -> Option<u16> {
    if frame.len() < MAC_HEADER_LEN + 6 {
        return None;
    }
    let fc = u16::from_le_bytes([frame[0], frame[1]]);
    if fc_type(fc) != TYPE_MGMT || fc_subtype(fc) != super::super::header::SUBTYPE_AUTH {
        return None;
    }
    Some(u16::from_le_bytes([frame[MAC_HEADER_LEN + 4], frame[MAC_HEADER_LEN + 5]]))
}

/// The (status, association id) of an association response, or `None` if the
/// frame is not one. Zero status is success; the AID is the low 14 bits. The
/// body after the MAC header is capability (2), status (2), AID (2).
pub fn parse_assoc_response(frame: &[u8]) -> Option<(u16, u16)> {
    if frame.len() < MAC_HEADER_LEN + 6 {
        return None;
    }
    let fc = u16::from_le_bytes([frame[0], frame[1]]);
    if fc_type(fc) != TYPE_MGMT || fc_subtype(fc) != super::super::header::SUBTYPE_ASSOC_RESP {
        return None;
    }
    let status = u16::from_le_bytes([frame[MAC_HEADER_LEN + 2], frame[MAC_HEADER_LEN + 3]]);
    let aid = u16::from_le_bytes([frame[MAC_HEADER_LEN + 4], frame[MAC_HEADER_LEN + 5]]) & 0x3FFF;
    Some((status, aid))
}
