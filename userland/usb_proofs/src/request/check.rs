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

//! What must hold for any frame: it decodes to the request and exact body
//! it carries, or draws one whole refusal naming the op, flags and request
//! id it carried (zeros when it is too short to carry them).

use super::wire::{le16, le32, BODY_MAX, MAGIC, VERSION};
use crate::usb_hid_wire::{
    parse, refused, response_header, write_status, Request, E_INVAL, HDR_LEN, STATUS_LEN,
};

/// What must hold for any input. Returns whether it decoded.
pub fn check(buf: &[u8]) -> bool {
    if let Some((req, body)) = parse(buf) {
        assert!(buf.len() >= HDR_LEN);
        assert_eq!((le32(buf, 0), le16(buf, 4)), (MAGIC, VERSION));
        assert_eq!(
            (req.op, req.flags, req.request_id),
            (le16(buf, 6), le16(buf, 8), le32(buf, 12))
        );
        assert_eq!(body.len() as u64, u64::from(le32(buf, 16)));
        assert_eq!(body, &buf[HDR_LEN..], "the body is the rest of the frame");
        return true;
    }
    let req: Request = refused(buf);
    let echo =
        if buf.len() >= HDR_LEN { (le16(buf, 6), le16(buf, 8), le32(buf, 12)) } else { (0, 0, 0) };
    assert_eq!((req.op, req.flags, req.request_id), echo, "the refusal names its request");
    let mut tx = vec![0u8; HDR_LEN + BODY_MAX];
    response_header(&mut tx, &req, STATUS_LEN as u32);
    write_status(&mut tx, E_INVAL);
    let reply = &tx[..HDR_LEN + STATUS_LEN];
    assert_eq!((le32(reply, 0), le16(reply, 4)), (MAGIC, VERSION));
    assert_eq!(
        (le16(reply, 6), le32(reply, 12), le32(reply, 16)),
        (req.op, req.request_id, STATUS_LEN as u32)
    );
    assert_eq!(le32(reply, HDR_LEN) as i32, E_INVAL);
    false
}
