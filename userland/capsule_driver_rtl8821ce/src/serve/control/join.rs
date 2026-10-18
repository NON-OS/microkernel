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

//! The connect and disconnect replies.

use super::super::connect::{connect, disconnect, ConnectResult, Session};
use super::super::radio::Radio;

// A connect carries its handshake progress back so the panel can show where a
// failed join stopped:
// [status i32][sent u32][recv u32][data u32][eapol u32][probe u32][deauth u32]
// [to_us u32][state u8][akm u8][ap status or reason code u16].
pub(super) fn connect_reply(
    body: &[u8],
    radio: &mut Radio,
    session: &mut Option<Session>,
    out: &mut [u8],
) -> Option<usize> {
    /*
     * A join while associated first leaves the current network, so its keys
     * are cleared from the CAM rather than left beside the new ones.
     */
    if let (Radio::Up(up), true) = (&mut *radio, session.is_some()) {
        disconnect(&mut up.link, &mut up.keys, &up.regs, session);
    }
    let r = match radio {
        Radio::Up(up) => {
            let r = connect(body, &mut up.link, &mut up.keys, &up.regs, session);
            up.link.note_join(r.code);
            r
        }
        Radio::Down => ConnectResult::code(-1),
    };
    out[10..14].copy_from_slice(&r.code.to_le_bytes());
    out[14..18].copy_from_slice(&r.sent.to_le_bytes());
    out[18..22].copy_from_slice(&r.recv.to_le_bytes());
    out[22..26].copy_from_slice(&r.data.to_le_bytes());
    out[26..30].copy_from_slice(&r.eapol.to_le_bytes());
    out[30..34].copy_from_slice(&r.probe.to_le_bytes());
    out[34..38].copy_from_slice(&r.deauth.to_le_bytes());
    out[38..42].copy_from_slice(&r.to_us.to_le_bytes());
    out[42] = r.state;
    out[43] = r.akm;
    out[44..46].copy_from_slice(&r.ap_code.to_le_bytes());
    Some(46)
}
