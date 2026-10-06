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

//! What a page asked the reader with `alert`, `confirm` or `prompt`.

use alloc::string::String;

use super::ffi::njs_take_dialog;
use super::lifecycle::Engine;

/// Which of the three a page called.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Dialog {
    /// `alert`, which only tells.
    Alert,
    /// `confirm`, a yes or no question, answered false.
    Confirm,
    /// `prompt`, which asks for text, answered null.
    Prompt,
}

/// The last dialog a page asked for since the browser last looked.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Asked {
    pub kind: Dialog,
    /// Its message, as the page passed it, at most 511 bytes.
    pub text: String,
    /// How many dialogs the page asked for in all since the last look,
    /// this one included.
    pub count: u32,
}

impl Engine {
    /// The dialog a script asked for since this was last called.
    ///
    /// A script runs to its end in one go on a time budget, so no dialog
    /// can wait for the reader: the page was answered at once (confirm
    /// false, prompt null) and the browser says what was asked and what
    /// it answered. Reading clears it.
    pub fn take_dialog(&self) -> Option<Asked> {
        let mut text: *const u8 = core::ptr::null();
        let mut count = 0i32;
        let kind = match unsafe { njs_take_dialog(&mut text, &mut count) } {
            0 => Dialog::Alert,
            1 => Dialog::Confirm,
            2 => Dialog::Prompt,
            _ => return None,
        };
        let text = unsafe {
            let mut n = 0;
            while *text.add(n) != 0 {
                n += 1;
            }
            String::from_utf8_lossy(core::slice::from_raw_parts(text, n)).into_owned()
        };
        Some(Asked { kind, text, count: count.max(1) as u32 })
    }
}
