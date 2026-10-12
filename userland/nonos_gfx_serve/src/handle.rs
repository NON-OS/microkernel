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

//! One request in, one reply out: another output than the device has is
//! ENXIO, an op not served ENOSYS, a body that does not decode EINVAL.

use nonos_gfx_wire::body::mode::MAX_MODES;
use nonos_gfx_wire::errno::{EINVAL, ENOSYS};
use nonos_gfx_wire::{bytes, ops, reply, Cursor, Errno, Flush, Mode, Modes, Request};

use super::display::Display;
use super::platform::Platform;
use super::server::Server;

impl<D: Display, P: Platform> Server<D, P> {
    /// Answer `frame` from `sender` into `out`; the reply's length, or 0
    /// for a frame too broken to answer.
    pub fn handle(&mut self, sender: u32, frame: &[u8], out: &mut [u8]) -> usize {
        let Ok(req) = Request::decode(frame) else { return 0 };
        let mut body = [0u8; 252];
        let (status, n) = match self.op(sender, &req, &mut body) {
            Ok(n) => (0, n),
            Err(e) => (e, 0),
        };
        reply(&req.header, status, &body[..n], out).unwrap_or(0)
    }

    fn op(&mut self, sender: u32, req: &Request, body: &mut [u8; 252]) -> Result<usize, Errno> {
        let (h, p) = (req.header, req.payload);
        if h.op >= ops::CAPS && h.op != ops::RETIRE {
            self.slot(h.output)?;
        }
        let put = |body: &mut [u8; 252], b: &[u8]| {
            body[..b.len()].copy_from_slice(b);
            Ok(b.len())
        };
        match h.op {
            ops::HEALTHCHECK => Ok(0),
            ops::CAPS => put(body, &self.display.caps().encode()),
            ops::MODES => {
                let mut m = Modes { list: [Mode::default(); MAX_MODES], len: 0 };
                m.len = self.display.modes(h.output, &mut m.list)?;
                m.encode(body)
            }
            ops::STATE => {
                let mut st = self.display.state(h.output)?;
                st.serial = self.slot(h.output)?.current.map_or(0, |c| c.serial);
                put(body, &st.encode())
            }
            ops::SURFACE => put(body, &self.surface(sender, h.output)?.encode()),
            ops::MODE_SET => {
                let w = bytes::u32_at(p, 0).ok_or(EINVAL)?;
                let ht = bytes::u32_at(p, 4).ok_or(EINVAL)?;
                put(body, &self.mode_set(sender, h.output, w, ht)?.encode())
            }
            ops::RETIRE => self.retire(bytes::u32_at(p, 0).ok_or(EINVAL)?).map(|_| 0),
            ops::FLUSH => self.flush(h.output, &Flush::decode(p)?).map(|_| 0),
            ops::CURSOR => self.cursor(h.output, &Cursor::decode(p)?).map(|_| 0),
            _ => Err(ENOSYS),
        }
    }
}
