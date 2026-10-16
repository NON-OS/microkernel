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

//! Identification mode (4.2): CMD8, CMD55, ACMD41, CMD2 and CMD3.

use super::super::card::{Out, Reply, St};
use super::card::{out, APP_CMD};
use super::SdCard;

impl SdCard {
    pub(super) fn identification(&mut self, app: bool, idx: u8, arg: u32) -> Option<Out> {
        Some(match (app, idx, self.state) {
            // A version 1.x card does not know CMD8; one that does echoes a
            // range it takes, and stays silent at one it does not.
            (false, 8, St::Idle) if self.cfg.v2 && (arg >> 8) & 0xf == 1 => {
                out(Reply::Short(arg & 0xfff))
            }
            (false, 8, St::Idle) => out(Reply::Silent),
            (_, 55, _) => self.app_cmd(arg),
            (true, 41, St::Idle) => self.op_cond(arg),
            (false, 2, St::Ready) => {
                self.state = St::Ident;
                out(Reply::Long(self.cid()))
            }
            // R6: the published address, then status bits 12:0 as the
            // command found them (4.10.1: CURRENT_STATE is the state the
            // command was received in), so ident the first time.
            (false, 3, St::Ident | St::Stby) => {
                let before = self.status() & 0x1fff;
                self.state = St::Stby;
                out(Reply::Short((self.cfg.rca as u32) << 16 | before))
            }
            _ => return None,
        })
    }

    /// CMD55 in idle state takes address 0; selected or standing by, the
    /// card's own.
    fn app_cmd(&mut self, arg: u32) -> Out {
        let me = (arg >> 16) as u16 == self.cfg.rca;
        let heard = match self.state {
            St::Idle => true,
            St::Stby | St::Tran => me,
            _ => false,
        };
        if !heard {
            return out(Reply::Silent);
        }
        self.app = true;
        out(Reply::Short(self.status() | APP_CMD))
    }
}
