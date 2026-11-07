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

//! Moving frames: the model's commands out to the transport, and what the
//! transport has in to the model.

use super::app::SimplexUi;
use crate::model::focus::Conn;
use crate::proto::envelope::incoming;
use crate::proto::frame::frame;

/// The most frames one pump reads, so a flood cannot hold the window.
const MAX_FRAMES: usize = 64;

impl SimplexUi {
    /// The outbox to the core while the channel is up; a command the channel
    /// went down on waits, with every one after it.
    fn flush(&mut self) {
        if !self.link.up() {
            return;
        }
        let (link, corr) = (&mut self.link, &mut self.corr);
        self.model.flush(|cmd| {
            *corr = corr.wrapping_add(1);
            link.send(&frame(*corr, &cmd.text()));
            link.up()
        });
    }

    /// Sends what waits and reads what came; true when the window must
    /// repaint.
    pub fn pump(&mut self) -> bool {
        let mut changed = false;
        if self.link.up() && self.model.conn == Conn::Offline {
            self.model.conn = Conn::Waiting;
            changed = true;
        }
        if !self.link.up() && self.model.conn != Conn::Offline {
            self.model.lost();
            changed = true;
        }
        for _ in 0..MAX_FRAMES {
            self.flush();
            let Some(text) = self.link.recv() else { break };
            if let Some(msg) = incoming(&text) {
                changed |= self.model.apply(msg);
            }
        }
        self.flush();
        changed
    }
}
