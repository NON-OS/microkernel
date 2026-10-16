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

//! Interrupt lines a test scripts: the device's answer while the driver
//! sleeps, and a line that is quiet after it.

use nonos_libc::{on_wait, Wake};

use super::model::Rig;

/// On the driver's first sleep the device does `answer` and the line fires
/// at once; every later sleep is slept out whole on a quiet line.
pub fn answer_once(rig: &Rig, answer: impl Fn(&Rig) + 'static) {
    let dev = rig.clone();
    let mut done = false;
    on_wait(move |timeout| {
        if done {
            return Wake { after_ms: timeout, fired: false };
        }
        done = true;
        answer(&dev);
        Wake { after_ms: 0, fired: true }
    });
}

/// Every sleep answers the request just posted, whole, as `len` bytes.
pub fn answer_each(rig: &Rig, len: u32) {
    let dev = rig.clone();
    on_wait(move |_| {
        dev.answer(len);
        Wake { after_ms: 0, fired: true }
    });
}
