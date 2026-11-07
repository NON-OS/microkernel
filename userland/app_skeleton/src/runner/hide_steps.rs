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

//! The order a window is hidden to the dock and shown again in, and what
//! each step waits on. Pure, so a host proof drives it; window_ops/restore.rs
//! gives it the calls.
//!
//! Neither looked at what the window manager or the compositor answered. A
//! minimize the window manager refused still took the layer away, so the
//! window manager kept hit-testing a window nobody saw and presses on that
//! empty place went to it. A restore whose layer the compositor refused (the
//! scene full, or the window left past a display that shrank while it was on
//! the dock) still counted as shown: the window manager, told by the dock
//! click it was visible, gave it presses, and a second dock click did
//! nothing since the app no longer took itself for minimized. And the dock
//! click raised the window in the window manager before its layer was back,
//! so a window raised in between was on top there while this one was drawn
//! on top. Now a window is hidden only once the window manager took the
//! minimize, it counts as shown only once its layer is back, it is hidden
//! again in the window manager when that fails, and it is raised once its
//! layer is in the scene.

pub trait HideSteps {
    fn wm_minimize(&mut self) -> bool;
    fn scene_remove(&mut self) -> bool;
}

pub trait ShowSteps {
    /// The layer back in the scene at the window's rect, kept on the display.
    fn scene_submit(&mut self) -> bool;
    fn wm_raise(&mut self);
    fn wm_minimize(&mut self) -> bool;
    fn repaint(&mut self);
}

/// Hide the window. True when it is minimized: the window manager took it.
/// A layer the compositor did not take away leaves the window drawn but no
/// longer hit-tested, and the restore that follows submits it in its place.
pub fn hide(steps: &mut impl HideSteps) -> bool {
    if !steps.wm_minimize() {
        return false;
    }
    let _ = steps.scene_remove();
    true
}

/// Show the window again. True when it is shown; false leaves it minimized,
/// and hidden in the window manager, so the next dock click tries again.
pub fn show(steps: &mut impl ShowSteps) -> bool {
    if !steps.scene_submit() {
        let _ = steps.wm_minimize();
        return false;
    }
    steps.wm_raise();
    steps.repaint();
    true
}

/// The frame control lit under the pointer once a hide is taken: none. The
/// window is off the screen, so no pointer is over it and no motion reaches
/// it to put the control out; kept, the minimize control was still lit when
/// a dock click showed the window again. A pointer event the window takes
/// after its minimize (a tablet reports its place with each button) can
/// light it again, so the restore puts the hover out as well
/// (window_ops/restore.rs).
pub fn hover_after_hide<H>(hidden: bool, hover: H, none: H) -> H {
    if hidden {
        none
    } else {
        hover
    }
}

/// Whether the frame loop waits for a whole frame shown before it reads its
/// inbox: only for a window that is not on the dock. A window on the dock has
/// no layer, so every present is refused and its frame never counts as
/// shown; waited for, the loop never read the dock's click that restores it,
/// nor a close. The window is painted whole as it is shown again (show).
pub fn waits_for_frame(primed: bool, minimized: bool) -> bool {
    !primed && !minimized
}

/// Whether a repaint an input or a peer asked for is painted now: not while
/// the window is on the dock, where its present is refused and the frame
/// would be marked unshown (waits_for_frame).
pub fn paints(minimized: bool) -> bool {
    !minimized
}
