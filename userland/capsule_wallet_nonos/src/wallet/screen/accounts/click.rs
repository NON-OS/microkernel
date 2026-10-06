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

/* A press on the Accounts screen: open an account, or add the next one. */

use nonos_app_skeleton::EventOutcome;

use crate::wallet::accounts::{add, switch, Added};
use crate::wallet::screen::hits::Press;
use crate::wallet::state::{State, VIEW_HOME};

const BUSY: &str = "A transaction or a shield job is under way for this account. Let it \
     finish, then open another.";

pub fn click(state: &mut State, press: Press) -> EventOutcome {
    let busy = crate::wallet::act::running(state) || state.shield_ui.waiting.is_some();
    if busy && matches!(press, Press::Row(_) | Press::Footer(0)) {
        state.failure = Some(BUSY);
        return EventOutcome::Repaint;
    }
    match press {
        Press::Back => state.view = VIEW_HOME,
        Press::Dismiss => state.failure = None,
        Press::Row(i) => {
            state.status = if switch(state, i as usize) {
                b"account opened"
            } else {
                b"account opened; the store would not keep it as the open one at reboot"
            };
        }
        Press::Footer(0) => match add(state) {
            Added::Opened => state.status = b"account added and opened",
            Added::OpenedUnsaved => {
                state.status = b"account added and opened; the store would not keep the list, so it is gone at reboot"
            }
            Added::WordsUnread => {
                state.failure = Some(
                    "The recovery words beside this wallet have not been read back from the disk \
                     yet, so no account can be derived from them. Try again in a moment.",
                )
            }
            Added::NoWords => {
                state.failure = Some(
                    "This wallet came from a private key, which has no further accounts. \
                     Restore from recovery words to add accounts.",
                )
            }
            Added::Full => {
                state.failure = Some("Eight accounts is the most one wallet holds on this machine.")
            }
            Added::Failed => {
                state.failure = Some("The keyring would not add an account. Nothing changed.")
            }
        },
        _ => return EventOutcome::Idle,
    }
    state.scroll = 0;
    EventOutcome::Repaint
}
