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

//! An .sfi file's framing, as btintel.c sends it: a 644-octet RSA header
//! (CSS header 128 octets at 0, public key 256 at 128, signature 256 at
//! 388); from TyP on, a 320-octet ECDSA header after it (CSS header 128 at
//! 644, public key 96 at 772, signature 96 at 868); then HCI commands to
//! the end. A CSS header's version is at its offset 8: 0x00010000 RSA,
//! 0x00020000 ECDSA, and the ECDSA one starts with 0x06.

/// Secure Send fragment kinds (`btintel_secure_send`'s fragment_type).
pub const CSS: u8 = 0x00;
pub const DATA: u8 = 0x01;
pub const SIGN: u8 = 0x02;
pub const PKEY: u8 = 0x03;
const RSA_LEN: usize = 644;
const ECDSA_LEN: usize = 320;

pub struct Layout {
    /// Each header section as (kind, offset, length), in sending order.
    pub headers: [(u8, usize, usize); 3],
    pub payload: usize,
}

/// The sections to send for `variant` with secure boot engine `sbe`.
pub fn layout(fw: &[u8], variant: u8, sbe: u8) -> Result<Layout, &'static str> {
    let at = |o: usize| fw.get(o + 8..o + 12).map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]));
    let rsa = [(CSS, 0, 128), (PKEY, 128, 256), (SIGN, 388, 256)];
    let ecdsa = [(CSS, 644, 128), (PKEY, 772, 96), (SIGN, 868, 96)];
    let ecdsa_there = fw.get(RSA_LEN) == Some(&0x06) && at(RSA_LEN) == Some(0x0002_0000);
    let (headers, payload) = match (variant <= 0x14, sbe) {
        _ if at(0) != Some(0x0001_0000) => return Err("firmware has no RSA header"),
        (true, 0) => (rsa, RSA_LEN),
        (true, _) => return Err("secure boot engine is not RSA"),
        _ if !ecdsa_there => return Err("firmware has no ECDSA header"),
        (false, 0) => (rsa, RSA_LEN + ECDSA_LEN),
        (false, 1) => (ecdsa, RSA_LEN + ECDSA_LEN),
        _ => return Err("secure boot engine unknown"),
    };
    let whole = fw.len() > payload;
    whole.then_some(Layout { headers, payload }).ok_or("firmware ends in its header")
}
