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

//! A press on a publisher transaction's review. Sign and send reaches here
//! only armed (event/etna_click.rs asked arm::gate::held_back first): the
//! chain's pending nonce is read, then it signs (publish/send_sign.rs).
//! Cancel and Back leave it; a transaction already sent may still land.

use nonos_app_skeleton::EventOutcome;

use crate::wallet::intent::IntentStep;
use crate::wallet::screen::hits::Press;
use crate::wallet::state::State;

pub fn click(state: &mut State, press: Press) -> EventOutcome {
    let p = &mut state.publisher;
    match press {
        Press::Footer(0) => match p.sending.as_mut() {
            Some(s) if matches!(s.step, IntentStep::Review { .. }) && !s.pressed => {
                s.pressed = true
            }
            _ => return EventOutcome::Idle,
        },
        Press::Footer(1) | Press::Back => p.sending = None,
        Press::Dismiss => state.failure = None,
        _ => return EventOutcome::Idle,
    }
    EventOutcome::Repaint
}
