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

//! Help > Keyboard Shortcuts against the handlers that answer the keys. The
//! sheet said Ctrl-B cycled the theme (it shows the file tree; the theme is
//! Ctrl-K then T) and left out Ctrl-I, Ctrl-M, Ctrl-G, Ctrl-P, Ctrl-N and
//! Ctrl-W. Both directions are read from the sources, so neither the sheet
//! nor the key map can move without the other.

const SHEET: &str = include_str!("../../capsule_text_editor/src/editor/info_text.rs");
const ON_CTRL: &str = include_str!("../../capsule_text_editor/src/editor/on_ctrl.rs");
const SHELL: &str = include_str!("../../capsule_text_editor/src/editor/event_editor.rs");
const NAV: &str = include_str!("../../capsule_text_editor/src/editor/on_ctrl_nav.rs");

/// The shortcut sheet's text, between its constant and the About sheet.
fn shortcuts() -> &'static str {
    let from = SHEET.find("SHORTCUTS").expect("sheet");
    let to = SHEET.find("pub(super) const ABOUT").expect("about");
    &SHEET[from..to]
}

/// The key named after every "Ctrl-" (and "Ctrl-Shift-") on the sheet.
fn documented() -> Vec<String> {
    let text = shortcuts();
    let mut out = Vec::new();
    for (i, _) in text.match_indices("Ctrl-") {
        let rest = &text[i + 5..];
        let rest = rest.strip_prefix("Shift-").unwrap_or(rest);
        let key: String = match rest.chars().next() {
            Some(c) if c.is_ascii_alphabetic() => {
                rest.chars().take_while(|c| c.is_ascii_alphabetic()).collect()
            }
            Some(c) => c.to_string(),
            None => continue,
        };
        out.push(key);
    }
    out
}

/// Where a documented key is answered, as the handler spells it.
fn handled(key: &str) -> bool {
    let code = match key {
        "Backspace" => return NAV.contains("KEY_BACKSPACE"),
        "Left" | "Right" | "Home" | "End" => {
            return NAV.contains(&format!("KEY_{}", key.to_ascii_uppercase()))
        }
        k if k.len() == 1 => k.as_bytes()[0].to_ascii_uppercase(),
        _ => return false,
    };
    let hex = format!("0x{code:02X}");
    ON_CTRL.contains(&hex) || SHELL.contains(&hex)
}

#[test]
fn every_shortcut_on_the_sheet_has_a_handler() {
    let keys = documented();
    assert!(keys.len() >= 25, "parsed {} shortcuts", keys.len());
    for key in keys {
        assert!(handled(&key), "Ctrl-{key} is on the sheet and nothing answers it");
    }
}

/// Every Ctrl+letter the document engine dispatches is on the sheet.
#[test]
fn every_ctrl_letter_the_editor_answers_is_on_the_sheet() {
    let keys = documented();
    for line in ON_CTRL.lines().filter(|l| l.contains("=>")) {
        let head = line.split("=>").next().unwrap_or("");
        for part in head.split("0x").skip(1) {
            let Ok(code) = u8::from_str_radix(&part[..2], 16) else { continue };
            if code.is_ascii_uppercase() {
                let key = (code as char).to_string();
                assert!(keys.contains(&key), "Ctrl-{key} is handled and not on the sheet");
            }
        }
    }
}

#[test]
fn the_theme_is_not_said_to_be_on_ctrl_b() {
    assert!(!shortcuts().contains("Ctrl-B cycle theme"));
    assert!(shortcuts().contains("Ctrl-K then T cycle theme"));
    assert!(SHELL.contains("0x54 | 0x74"), "the T that follows Ctrl-K");
}
