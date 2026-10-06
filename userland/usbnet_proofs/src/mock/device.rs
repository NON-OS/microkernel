// NONOS Operating System (AGPL-3.0-or-later)
//! Answering GET_DESCRIPTOR from a device's descriptor bytes, as a device
//! does: the first `wLength` bytes of what was asked for.

use super::script::Call;

const GET_DESCRIPTOR: u8 = 0x06;

/// The reply to `call` when it is a GET_DESCRIPTOR this device has an
/// answer for; `None` for any other call.
pub fn descriptors(
    call: &Call,
    device: &[u8],
    configs: &[&[u8]],
    strings: &[(u8, &[u8])],
) -> Option<Vec<u8>> {
    let Call::In(s, len) = call else { return None };
    if s.request_type != 0x80 || s.request != GET_DESCRIPTOR {
        return None;
    }
    let (kind, index) = ((s.value >> 8) as u8, s.value as u8);
    let bytes: &[u8] = match kind {
        0x01 => device,
        0x02 => configs.get(index as usize)?,
        0x03 => strings.iter().find(|(i, _)| *i == index)?.1,
        _ => return None,
    };
    Some(bytes[..bytes.len().min(*len)].to_vec())
}
