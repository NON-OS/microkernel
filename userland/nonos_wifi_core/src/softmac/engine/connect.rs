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

//! The connect op: leave any network held, run the join where the scanner
//! last heard the name, and on success open the session the data path runs
//! on. The radio stays pointed at the BSS only while a session holds it.

use super::join_error::join_error;
use super::station::SoftStation;
use crate::softmac::control::result::{failure_code, CODE_BAD_REQUEST, CODE_NO_ENTROPY};
use crate::softmac::control::result::{CODE_REFUSED, CODE_TIMED_OUT};
use crate::softmac::control::ConnectResult;
use crate::softmac::join::{join, parse_connect, Outcome};
use crate::softmac::radio::{Env, Radio};
use crate::softmac::roam::Kit;
use crate::softmac::session::{Keys, Session};

impl<R: Radio, E: Env> SoftStation<R, E> {
    pub(super) fn connect(&mut self, body: &[u8]) -> ConnectResult {
        let (Some(req), true) = (parse_connect(body), self.radio.is_some()) else {
            return ConnectResult::code(CODE_BAD_REQUEST);
        };
        if let (Some(r), Some(s)) = (self.radio.as_mut(), self.session.as_mut()) {
            s.leave(r);
        }
        self.end_session();
        let hint = self.scanner.hint(req.ssid);
        let Some(r) = self.radio.as_mut() else { return ConnectResult::code(CODE_BAD_REQUEST) };
        let Some(addresses) = self.addresses.as_mut() else {
            return ConnectResult::code(CODE_NO_ENTROPY);
        };
        let held = (&mut self.pmksa, addresses);
        let j = match join(r, &mut self.env, self.domain, &req, hint.as_slice(), held) {
            Ok(j) => j,
            Err(e) => return join_error(e),
        };
        let rep = j.report;
        let mut res = ConnectResult {
            code: 0,
            counters: rep.counters,
            state: rep.state,
            akm: rep.akm.map_or(0, |a| a.suite_type()),
            ap_code: 0,
        };
        let (now, joined) = (self.env.now_ms(), matches!(rep.outcome, Outcome::Joined { .. }));
        let kit = joined.then(|| Kit::for_join(&req, &rep, &self.pmksa, r.ciphers(), now));
        match rep.outcome {
            Outcome::Joined { bssid, channel, tk, akm, supplicant } => {
                let at = (j.band, if channel != 0 { channel } else { j.channel });
                let net = (req.ssid, bssid, akm.suite_type());
                let mut s = Session::start(r.mac(), net, at, Keys { tk, supplicant }, now);
                s.kit = kit.flatten();
                self.session = Some(s);
            }
            Outcome::Refused => {
                (res.code, res.ap_code) = rep.failure.map_or((CODE_REFUSED, 0), failure_code);
            }
            Outcome::TimedOut => res.code = CODE_TIMED_OUT,
        }
        res
    }
}
