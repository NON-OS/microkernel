// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Clipboard text into one of the shell's one-line fields (the rename box,
//! the Launchpad search). The clipboard holds whatever an app copied: maybe
//! several lines, maybe bytes cut mid-character at the paste buffer's end. A
//! field takes the first line only, without control characters, whole
//! characters only, and no more than its own limit.

use alloc::string::String;

/// Append `bytes` to `field`, keeping it within `max_bytes`. Returns whether
/// anything was added.
pub fn paste_line(field: &mut String, bytes: &[u8], max_bytes: usize) -> bool {
    let text = match core::str::from_utf8(bytes) {
        Ok(s) => s,
        // A buffer cut mid-character keeps the whole characters before it.
        Err(e) => core::str::from_utf8(&bytes[..e.valid_up_to()]).unwrap_or(""),
    };
    let line = text.split(['\n', '\r']).next().unwrap_or("");
    let before = field.len();
    for ch in line.chars().filter(|c| !c.is_control()) {
        if field.len() + ch.len_utf8() > max_bytes {
            break;
        }
        field.push(ch);
    }
    field.len() != before
}
