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

//! Who has the camera or the microphone on (Decision 9), by pid: one turning
//! off leaves it lit while another captures; the upkeep drops the dead.

pub const KIND_CAMERA: u32 = 1;
pub const KIND_MICROPHONE: u32 = 2;
/// Capturers of one kind at once; one more is refused, never dropped.
pub const MAX_HOLDERS: usize = 8;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Indicators {
    pub camera: [u32; MAX_HOLDERS],
    pub microphone: [u32; MAX_HOLDERS],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refused {
    Kind,
    Full,
}

impl Indicators {
    fn holders(&mut self, kind: u32) -> Result<&mut [u32; MAX_HOLDERS], Refused> {
        match kind {
            KIND_CAMERA => Ok(&mut self.camera),
            KIND_MICROPHONE => Ok(&mut self.microphone),
            _ => Err(Refused::Kind),
        }
    }

    /// `pid` turns `kind` on or off; whether what is shown changed.
    pub fn apply(&mut self, kind: u32, on: bool, pid: u32) -> Result<bool, Refused> {
        let before = *self;
        let h = self.holders(kind)?;
        let held = h.iter().position(|&p| p == pid);
        match (on, held) {
            (true, None) => *h.iter_mut().find(|p| **p == 0).ok_or(Refused::Full)? = pid,
            (false, Some(i)) => h[i] = 0,
            _ => {}
        }
        Ok(before.shown(kind) != self.shown(kind))
    }

    pub fn shown(&self, kind: u32) -> bool {
        let h = if kind == KIND_CAMERA { self.camera } else { self.microphone };
        matches!(kind, KIND_CAMERA | KIND_MICROPHONE) && h.iter().any(|&p| p != 0)
    }

    /// Drop holders that no longer live; whether what is shown changed.
    pub fn reap(&mut self, alive: impl Fn(u32) -> bool) -> bool {
        let before = *self;
        for p in self.camera.iter_mut().chain(self.microphone.iter_mut()) {
            if *p != 0 && !alive(*p) {
                *p = 0;
            }
        }
        before.shown(KIND_CAMERA) != self.shown(KIND_CAMERA)
            || before.shown(KIND_MICROPHONE) != self.shown(KIND_MICROPHONE)
    }
}
