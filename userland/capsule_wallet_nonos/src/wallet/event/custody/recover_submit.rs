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

//! The typed recovery phrase, resolved to word indices and handed to the
//! keyring; the indices and the typed text are wiped on every path.

use nonos_app_skeleton::EventOutcome;
use zeroize::Zeroize;

use super::recover::wipe;
use crate::wallet::state::State;

pub(in crate::wallet::event) fn submit(state: &mut State) -> EventOutcome {
    let mut indices = [0u16; 24];
    let mut count = 0usize;
    {
        let typed = &state.custody.recover_buf[..state.custody.recover_len];
        for word in typed.split(|&b| b == b' ').filter(|w| !w.is_empty()) {
            if count == 24 {
                count = usize::MAX;
                break;
            }
            match nonos_hd::bip39::word_index(word) {
                Some(idx) => {
                    indices[count] = idx;
                    count += 1;
                }
                None => {
                    wipe_indices(&mut indices);
                    state.status = b"a word is not in the BIP39 list";
                    return EventOutcome::Repaint;
                }
            }
        }
    }
    if !matches!(count, 12 | 15 | 18 | 21 | 24) {
        wipe_indices(&mut indices);
        state.status = b"phrase must be 12, 15, 18, 21 or 24 words";
        return EventOutcome::Repaint;
    }

    let outcome = super::recover_keyring::recovered(state, &indices[..count]);
    wipe_indices(&mut indices);
    wipe(state);
    outcome
}

fn wipe_indices(indices: &mut [u16; 24]) {
    indices.zeroize();
}
