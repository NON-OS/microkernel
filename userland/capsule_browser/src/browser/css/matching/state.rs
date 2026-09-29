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

use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

const NONE: usize = usize::MAX;
static HOVERED: AtomicUsize = AtomicUsize::new(NONE);
static ACTIVE: AtomicUsize = AtomicUsize::new(NONE);
static FOCUSED: AtomicUsize = AtomicUsize::new(NONE);
static TARGET: AtomicUsize = AtomicUsize::new(NONE);
static FOCUS_VISIBLE: AtomicBool = AtomicBool::new(false);

/* What :hover, :active, :focus, :focus-visible, :focus-within and :target
 * read, as node ids in the document being matched. Nothing in the capsule
 * reports pointer, press, focus or fragment navigation to the matcher, so
 * on device every field stays empty and those pseudo-classes match nothing;
 * the host harness sets them to prove the matching. */
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct ElementState {
    pub hovered: Option<usize>,
    pub active: Option<usize>,
    pub focused: Option<usize>,
    /* The focus came from the keyboard, so it shows as :focus-visible. */
    pub focus_visible: bool,
    pub target: Option<usize>,
}

fn read(a: &AtomicUsize) -> Option<usize> {
    let v = a.load(Ordering::Relaxed);
    (v != NONE).then_some(v)
}

pub fn element_state() -> ElementState {
    ElementState {
        hovered: read(&HOVERED),
        active: read(&ACTIVE),
        focused: read(&FOCUSED),
        focus_visible: FOCUS_VISIBLE.load(Ordering::Relaxed),
        target: read(&TARGET),
    }
}

#[cfg(feature = "harness")]
pub fn set_hovered(id: Option<usize>) {
    HOVERED.store(id.unwrap_or(NONE), Ordering::Relaxed);
}

#[cfg(feature = "harness")]
pub fn set_active(id: Option<usize>) {
    ACTIVE.store(id.unwrap_or(NONE), Ordering::Relaxed);
}

#[cfg(feature = "harness")]
pub fn set_focused(id: Option<usize>, from_keyboard: bool) {
    FOCUSED.store(id.unwrap_or(NONE), Ordering::Relaxed);
    FOCUS_VISIBLE.store(from_keyboard, Ordering::Relaxed);
}

#[cfg(feature = "harness")]
pub fn set_target(id: Option<usize>) {
    TARGET.store(id.unwrap_or(NONE), Ordering::Relaxed);
}
