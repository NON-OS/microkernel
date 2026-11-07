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

//! The lock's words, kept together so the copy check reads them in one
//! place.

pub const LOCKED: &str = "Locked";
pub const FIELD_HINT: &str = "Type the passphrase and press Enter";
pub const CHECKING: &str = "Checking the passphrase";
pub const WRONG_ONE: &str = "Wrong passphrase";
pub const WRONG_MANY: &str = "wrong entries in a row";
pub const WAIT_BEFORE: &str = "Next try in";
pub const SECONDS: &str = "s";
pub const NO_ANSWER: &str = "The keyring did not answer. The screen stays locked.";

pub const CHOOSE_FIRST: &str = "Choose a lock screen passphrase";
pub const CHOOSE_CHANGE: &str = "Change the lock screen passphrase";
pub const ASK_CURRENT: &str = "Current passphrase";
pub const ASK_NEW: &str = "New passphrase, at least 8 characters";
pub const ASK_AGAIN: &str = "The new passphrase again";
pub const SAVING: &str = "Saving";
pub const CANNOT_RECOVER: &str =
    "There is no way to recover a forgotten passphrase. The lock screen asks for it after the \
time set in Settings, on Super+L, and when the lid closes.";
pub const TOO_SHORT: &str = "At least 8 characters, and not only spaces.";
pub const TOO_LONG: &str = "At most 256 bytes.";
pub const NOT_TEXT: &str = "Use letters, digits, spaces and punctuation.";
pub const MISMATCH: &str = "The two entries differ. Type the new passphrase again.";
pub const NOT_CURRENT: &str = "That is not the current passphrase.";
pub const WAIT_THEN: &str = "Too many wrong entries. Wait, then try again.";
pub const NOT_SAVED: &str = "The passphrase was not saved. Try again.";
pub const SAVE: &str = "Save";
pub const NEXT: &str = "Next";
pub const NOT_NOW: &str = "Not now";
pub const CANCEL: &str = "Cancel";

pub const SET_FIRST: &[u8] = b"Choose a lock screen passphrase in Settings to lock the screen";
pub const CHANGED: &[u8] = b"Lock screen passphrase saved";
pub const KEPT_THIS_BOOT: &[u8] =
    b"Passphrase saved for this session; this boot keeps nothing on disk";
