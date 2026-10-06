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

/*
 * A TCP stream carried by `net.socks5` to an exit on the Nym mixnet. The
 * host name goes to the exit unresolved, so this machine never looks it up.
 */

use alloc::vec::Vec;

use super::socks_call::{call, RESET};

/* No SOCKS reply could be read: the proxy did not answer in time. */
pub const NO_ANSWER: u8 = 0xFF;
/* SOCKS5's "network unreachable": net.socks5 has no Nym session to open a stream in. */
pub const NET_UNREACHABLE: u8 = 0x03;

pub struct SocksLink {
    pub(super) port: u32,
    pub(super) pending: Vec<u8>,
    pub(super) closed: bool,
}

impl SocksLink {
    /*
     * The link, or why not: the proxy's SOCKS reply code, or `NO_ANSWER`
     * when it gave none to read.
     */
    pub fn connect(port: u32, host: &str, dst: u16) -> Result<SocksLink, u8> {
        /*
         * Forget any conversation this capsule had before; none is normal.
         */
        let _ = call(port, &[RESET]);
        let mut s = SocksLink { port, pending: Vec::new(), closed: false };
        s.write_all(&[5, 1, 0]).map_err(|_| NO_ANSWER)?;
        if s.answer(2).map_err(|_| NO_ANSWER)? != [5, 0] || host.is_empty() || host.len() > 255 {
            return Err(NO_ANSWER);
        }
        let mut ask = Vec::from([5, 1, 0, 3, host.len() as u8]);
        ask.extend_from_slice(host.as_bytes());
        ask.extend_from_slice(&dst.to_be_bytes());
        s.write_all(&ask).map_err(|_| NO_ANSWER)?;
        let head = s.answer(5).map_err(|_| NO_ANSWER)?;
        let rest = match head[3] {
            1 => 4 + 2 - 1,
            3 => head[4] as usize + 2,
            4 => 16 + 2 - 1,
            _ => return Err(head[1].max(1)),
        };
        s.answer(rest).map_err(|_| NO_ANSWER)?;
        match head[1] {
            0 => Ok(s),
            code => Err(code),
        }
    }
}

impl Drop for SocksLink {
    /*
     * Ask the proxy to forget this conversation, which ends the tunnel, so
     * the next connection starts a handshake of its own.
     */
    fn drop(&mut self) {
        let _ = call(self.port, &[RESET]);
    }
}
