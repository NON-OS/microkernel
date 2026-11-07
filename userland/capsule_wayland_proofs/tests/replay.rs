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

//! Recorded sessions of real clients played back into the server: each is
//! served whole, with no request left unserved and no protocol error, and
//! what it was sent and what the desktop was asked are held to what the
//! session did live. The records are what the clients sent the host server
//! (src/bin/host.rs) with NONOS_WAYLAND_RECORD set; a change to what the
//! server sends (a new event, a serial more) makes them again, by the
//! procedure in 

use nonos_app_skeleton::desk::Call;
use nonos_capsule_wayland_proofs::client::Ev;
use nonos_capsule_wayland_proofs::host::dump::{pixels, surface_pixels};
use nonos_capsule_wayland_proofs::record::{replay, replay_watched};

/// Played back: the lines the server said, what it sent, and what the
/// desktop was asked.
fn played(name: &str) -> (Vec<String>, Vec<Ev>, Vec<Call>) {
    let path = format!("{}/tests/sessions/{name}.rec", env!("CARGO_MANIFEST_DIR"));
    let rec = std::fs::read(&path).expect("the record is in the tree");
    let (s, sent) = replay(&rec).expect("the record is whole");
    let said_now: Vec<String> = nonos_libc::world::with(|w| {
        w.said.iter().map(|l| String::from_utf8_lossy(l).into()).collect()
    });
    assert!(!s.guest.objects.failed(), "the client was not ended: {said_now:?}");
    let said = nonos_libc::world::with(|w| {
        w.said.iter().map(|l| String::from_utf8_lossy(l).into()).collect()
    });
    let calls = nonos_app_skeleton::desk::with(|d| d.calls.clone());
    (said, events(&sent), calls)
}

fn events(bytes: &[u8]) -> Vec<Ev> {
    let mut out = Vec::new();
    let mut at = 0;
    while at + 8 <= bytes.len() {
        let word = |i: usize| u32::from_le_bytes(bytes[i..i + 4].try_into().expect("a word"));
        let (object, head) = (word(at), word(at + 4));
        let size = (head >> 16) as usize;
        out.push(Ev { object, opcode: head as u16, body: bytes[at + 8..at + size].to_vec() });
        at += size;
    }
    out
}

fn assert_clean(lines: &[String]) {
    let bad: Vec<&String> =
        lines.iter().filter(|l| l.contains("unserved") || l.contains("client ended")).collect();
    assert!(bad.is_empty(), "the server said: {bad:?}");
}

#[test]
fn foot_session_is_served_whole() {
    let (lines, _, calls) = played("foot");
    assert_clean(&lines);
    assert!(calls.iter().any(|c| matches!(c, Call::Open(..))), "its window opened");
    // F11 took it full screen and back.
    assert!(calls.iter().any(|c| matches!(c, Call::Maximize(_, _, true))));
    assert!(lines.iter().any(|l| l.contains("[WAYLAND] window ")));
}

/// keepassxc's Ctrl+O opens a second toplevel, its file dialog, only once
/// the first is activated: Qt takes activation from that state alone.
#[test]
fn keepassxc_activated_opens_its_dialog_and_the_keyboard_goes_to_it() {
    let (lines, evs, _) = played("keepassxc");
    assert_clean(&lines);
    const ACTIVATED: u32 = 4;
    // Configures that carried `activated`, by toplevel: words are width,
    // height, then the states array.
    let toplevels: std::collections::BTreeSet<u32> = evs
        .iter()
        .filter(|e| e.opcode == 0 && e.body.len() >= 12 && e.body.len() == 12 + e.word(2) as usize)
        .filter(|e| e.array(2).contains(&ACTIVATED))
        .map(|e| e.object)
        .collect();
    assert_eq!(toplevels.len(), 2, "the window and its dialog were activated: {toplevels:?}");
    // The keyboard entered two surfaces: the window's, then the dialog's.
    let entered: std::collections::BTreeSet<u32> = evs
        .iter()
        .filter(|e| {
            e.opcode == 1 && e.body.len() == 12 && e.word(2) == 0 && is_keyboard(&evs, e.object)
        })
        .map(|e| e.word(1))
        .collect();
    assert_eq!(entered.len(), 2, "{entered:?}");
}

/// The object a keymap was sent to is the keyboard.
fn is_keyboard(evs: &[Ev], object: u32) -> bool {
    evs.iter().any(|e| e.object == object && e.opcode == 0 && e.body.len() == 8 && e.word(0) == 1)
}

/// pcmanfm (GTK 3): the tooltip under its Up button configured where its
/// positioner puts it, a character no US key gives typed into its location
/// bar, and the text copied there offered back as the selection.
#[test]
fn pcmanfm_tooltip_typing_and_selection() {
    let (lines, evs, _) = played("pcmanfm");
    assert_clean(&lines);
    // xdg_popup.configure: four ints, x, y, width, height.
    let tip = evs.iter().find(|e| e.opcode == 0 && e.body.len() == 16 && e.word(2) == 153);
    let tip = tip.expect("the tooltip was configured");
    assert_eq!((tip.word(0), tip.word(1), tip.word(3)), (125, 68, 45));
    // é's own key, evdev 140 + 0x49, pressed and let go.
    let e_acute = 140 + 0x49;
    let keys: Vec<&Ev> = evs
        .iter()
        .filter(|e| e.opcode == 3 && e.body.len() == 16 && e.word(2) == e_acute)
        .collect();
    assert_eq!(keys.len(), 2, "pressed and let go");
    // The selection it set is offered to its device: data_offer, then selection.
    let offers = evs.iter().filter(|e| e.object >= 0xff00_0000 && e.opcode == 0).count();
    assert!(offers > 0, "its offers were told their types");
}

/// A window's pixels as shown: width, height, and every pixel.
type Shown = (u32, u32, Vec<u32>);

/// What a session played back showed around its dialogs: each time a
/// dialog first went over a window, that window and its pixels as it last
/// showed them on its own with the keyboard; the window shown at the end and
/// its pixels; and how many windows were still behind it.
struct AroundDialogs {
    before_dialog: Vec<(u32, Shown)>,
    shown: Option<u32>,
    after: Option<Shown>,
    behind: usize,
}

fn around_dialogs(name: &str) -> AroundDialogs {
    let path = format!("{}/tests/sessions/{name}.rec", env!("CARGO_MANIFEST_DIR"));
    let rec = std::fs::read(&path).expect("the record is in the tree");
    let mut before_dialog = Vec::new();
    let mut alone: Option<(u32, Shown)> = None;
    let (s, _) = replay_watched(&rec, |s| {
        let scene = &s.guest.scene;
        if scene.behind.is_empty() {
            // The window on its own, focused: as the dialog will find it.
            let Some(main) = scene.win.shown.map(|w| w.surface).filter(|_| scene.win.focused)
            else {
                return;
            };
            alone = scene.win.out.and_then(surface_pixels).map(|px| (main, px));
        } else if let Some(was) = alone.take() {
            before_dialog.push(was);
        }
    })
    .expect("the record is whole");
    let shown = s.guest.scene.win.shown.map(|w| w.surface);
    let after = pixels(&s.guest);
    AroundDialogs { before_dialog, shown, after, behind: s.guest.scene.behind.len() }
}

/// The window shown at the end had a dialog over it, and is shown again
/// byte for byte as it was just before, the keyboard back on it.
fn main_window_survives(name: &str) {
    let AroundDialogs { before_dialog, shown, after, behind } = around_dialogs(name);
    assert_eq!(behind, 0, "no window is left behind");
    let shown = shown.expect("a window is shown");
    let before = before_dialog.iter().rev().find(|p| p.0 == shown);
    let (_, before) = before.expect("the window shown had a dialog over it");
    let after = after.expect("its pixels are shown");
    assert_eq!((after.0, after.1), (before.0, before.1));
    assert!(after.2 == before.2, "byte for byte as before the dialog");
}

/// libreoffice-writer (GTK 3): Ctrl+O's file chooser, then Escape.
#[test]
fn libreoffice_open_dialog_closed_leaves_the_document_as_it_was() {
    main_window_survives("libreoffice-open");
}

/// keepassxc (Qt 5): Ctrl+O's Open database dialog, then Escape.
#[test]
fn keepassxc_open_dialog_closed_leaves_its_window_as_it_was() {
    main_window_survives("keepassxc-open");
}

/// firefox-esr (GTK 3): Ctrl+O's file chooser, then Escape.
#[test]
fn firefox_open_dialog_closed_leaves_its_window_as_it_was() {
    main_window_survives("firefox-open");
}
