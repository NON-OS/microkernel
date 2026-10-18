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

//! The eject sequence written out longhand, step by step, to hold the
//! real one against.

use super::release_rig::Call;
use crate::hotplug::{Has, Released, Stopped};

/// The calls and outcome the sequence must give, written out step by step.
pub fn want(h: Has, fail: Option<Call>) -> (Vec<Call>, Result<Released, Stopped>) {
    if !h.ej0 && !h.removable {
        return (Vec::new(), Err(Stopped::NotRemovable));
    }
    let mut c = Vec::new();
    let relock = |c: &mut Vec<Call>| h.lck.then(|| c.push(Call::Lock(true)));
    if h.lck {
        c.push(Call::Lock(false));
        if fail == Some(Call::Lock(false)) {
            return (c, Err(Stopped::Unlock));
        }
    }
    if h.dock {
        c.push(Call::Undock);
        if fail == Some(Call::Undock) {
            relock(&mut c);
            return (c, Err(Stopped::Undock));
        }
    }
    if !h.ej0 {
        return (c, Ok(Released::ForTheUser));
    }
    c.push(Call::Eject);
    if fail == Some(Call::Eject) {
        relock(&mut c);
        return (c, Err(Stopped::Eject));
    }
    (c, Ok(Released::ByFirmware))
}
