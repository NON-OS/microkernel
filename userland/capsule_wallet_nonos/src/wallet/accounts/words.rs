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

//! Account 0's recovery words.

use super::root_id;
use crate::wallet::state::State;

/* Open account 0's sealed words into the keyring, or say why not. A wallet
 * whose words file is missing is taken for one from a key only when it is
 * known to be one, or is older than that record. */
pub fn words(state: &mut State) {
    use crate::wallet::state::{WORDS_HELD, WORDS_NONE, WORDS_UNREAD};
    use crate::wallet::vault::Words;
    let root = root_id(state);
    /* A wallet recorded as from a key has no words, whatever a words file
     * left by an earlier wallet holds: it is never opened beside this key. */
    if crate::wallet::vault::recall_kind() == Some(true) {
        state.words = WORDS_NONE;
        return;
    }
    state.words = match crate::wallet::vault::recall_words(
        state.vault.keyring_port,
        state.vault.owner_pid,
        root,
    ) {
        Words::Opened => WORDS_HELD,
        Words::Absent => match crate::wallet::vault::recall_kind() {
            Some(false) => WORDS_UNREAD,
            _ => WORDS_NONE,
        },
        Words::Unread => WORDS_UNREAD,
    };
}
