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

//! Where the shell hooks the lock in, read from its source: input reaches
//! the lock before anything else; the lock screen is painted alone; the
//! chrome goes to the trusted band and holds the grab for it; Super+L locks
//! and nothing locks without a passphrase; the lid locks when Settings says
//! so; the record on disk is the keyring's blob and never the passphrase.

const FRAME: &str = include_str!("../../../../capsule_desktop_shell/src/server/input/frame.rs");
const CHROME: &str = include_str!("../../../../capsule_desktop_shell/src/render/chrome.rs");
const BAND: &str = include_str!("../../../../capsule_desktop_shell/src/server/trusted_band.rs");
const GRABS: &str = include_str!("../../../../capsule_desktop_shell/src/server/grabs.rs");
const CHORDS: &str = include_str!("../../../../capsule_desktop_shell/src/server/input/chords.rs");
const LOCK: &str = include_str!("../../../../capsule_desktop_shell/src/server/lock/mod.rs");
const CHECK: &str = include_str!("../../../../capsule_desktop_shell/src/server/lock/check.rs");
const KEEP: &str = include_str!("../../../../capsule_desktop_shell/src/server/lock/keep.rs");
const LID: &str = include_str!("../../../../capsule_desktop_shell/src/server/handlers/lid.rs");
const SCREEN: &str = include_str!("../../../../capsule_desktop_shell/src/render/lock/screen.rs");

#[test]
fn the_lock_takes_input_before_anything_else() {
    let lock = FRAME.find("crate::server::lock::filter(ctx, kind, buf)").unwrap();
    let attention = FRAME.find("handlers::attention::filter").unwrap();
    let keys = FRAME.find("super::key::key(").unwrap();
    assert!(lock < attention && lock < keys);
}

#[test]
fn the_lock_screen_is_painted_alone_and_opaque() {
    let lock = CHROME.find("super::lock::paint_screen(ctx)").unwrap();
    let clear = CHROME.find("clear_overlay::clear_overlay(ctx)").unwrap();
    let toasts = CHROME.find("super::toasts::paint_in_chrome(ctx)").unwrap();
    assert!(lock < clear && lock < toasts, "nothing of the chrome, toasts included, under it");
    assert!(CHROME[lock..clear].contains("return;"));
    assert!(SCREEN.contains("const CURTAIN: u32 = 0xFF"), "the curtain is opaque");
    assert!(SCREEN.contains("Rect { x: 0, y: 0, width: ctx.width, height: ctx.height }, CURTAIN"));
}

#[test]
fn the_band_and_the_grab_follow_the_lock() {
    assert!(BAND.contains("|| ctx.lock.in_band()"));
    assert!(GRABS.contains("|| ctx.lock.in_band()"));
}

#[test]
fn super_l_locks_and_nothing_locks_without_a_passphrase() {
    assert!(CHORDS.contains("escape::is_lock_chord(kind, code, flags)"));
    assert!(CHORDS.contains("crate::server::lock::lock_now(ctx)"));
    let offered = LOCK.find("if !ctx.lock.offered() {").unwrap();
    let cover = LOCK.find("    cover(ctx);\n}").unwrap();
    assert!(offered < cover);
}

#[test]
fn only_the_power_key_and_chords_are_dropped_while_locked() {
    assert!(CHECK.contains("Some(SystemKey::Power) => return,"));
    assert!(CHECK.contains("flags & (MOD_CTRL | MOD_ALT | MOD_META) != 0"));
}

#[test]
fn the_lid_locks_when_settings_say_so() {
    assert!(LID.contains("crate::state::lid::locks(lock_on_lid(), ctx.lock.offered())"));
    assert!(LID.contains("Field::LockOnLid"));
    use crate::shell_system_keys::lid::locks;
    assert!(locks(true, true));
    assert!(!locks(false, true));
    assert!(!locks(true, false));
}

#[test]
fn the_record_on_disk_is_the_keyrings_blob_on_a_keeping_boot() {
    assert!(KEEP.contains("const PATH: &[u8] = b\"/nonos/lock/record\";"));
    assert!(KEEP.contains("if !mk_keeping() {"));
    assert!(!KEEP.contains("field"), "no passphrase field reaches the disk");
}

#[test]
fn auto_lock_locks_only_with_a_passphrase_and_says_nothing_without() {
    let idle = LOCK.find("pub fn lock_idle(ctx: &mut Context) {").unwrap();
    let body = &LOCK[idle..LOCK[idle..].find("\n}\n").map_or(LOCK.len(), |e| idle + e)];
    assert!(body.contains("if ctx.lock.offered() && !ctx.lock.screen.covers() {"));
    assert!(!body.contains("toasts"), "nothing said to an empty room");
    let notice = FRAME.find("router_notice::is_idle_lock(buf)").unwrap();
    let lock = FRAME.find("crate::server::lock::filter(ctx, kind, buf)").unwrap();
    assert!(notice < lock, "an auto-lock notice is no key for the field");
}

/// Unlocked, the curtain still shown leaves the trusted band before the
/// desktop's chrome is painted, so the compositor moves an opaque twin
/// (compositor state/twin.rs) and composes the screen once, for the chrome.
#[test]
fn the_curtain_leaves_the_trusted_band_before_the_chrome_is_painted() {
    let release = CHECK.find("fn release(ctx: &mut Context) {").unwrap();
    let body = &CHECK[release..];
    let moved = body.find("crate::server::trusted_band::sync(ctx);").unwrap();
    let painted = body.find("crate::server::repaint::repaint(ctx);").unwrap();
    let told = body.find("super::router::tell(ctx);").unwrap();
    assert!(told < moved && moved < painted);
}
