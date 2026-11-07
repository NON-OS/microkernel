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

//! The app skeleton as the Wayland server reaches it, on the host. The
//! chrome rule and the input event are the shipped files; the services
//! (compositor, window manager, input router, clipboard) are one desktop
//! per thread, kept in desk.rs, that a test or the host server reads back.

pub mod desk;

/// A window's swapchain, the source app_skeleton ships, its fence over the
/// libc double's.
#[path = "."]
pub mod swap {
    #[path = "../../../app_skeleton/src/swap/fence.rs"]
    mod fence;
    #[path = "../../../nonos_surface/src/present.rs"]
    mod present;
    #[path = "../../../nonos_surface/src/swap_driver.rs"]
    mod swap_driver;
    #[path = "../../../nonos_surface/src/swapchain.rs"]
    pub mod swapchain;
    #[path = "../../../nonos_surface/src/swapchain_damage.rs"]
    pub mod swapchain_damage;

    pub use present::{
        on_layer, parts_block, with_parts, Part, Present, MAX_PARTS, OP_PRESENT, OP_PRESENT_LAYER,
        OP_PRESENT_LAYER_PARTS, PARTS_LEN, PRESENT_LAYER_LEN, PRESENT_LAYER_PARTS_LEN, PRESENT_LEN,
    };
    pub use swap_driver::{Pending, Swap};
    pub use swapchain::Chain;
    pub use swapchain_damage::{Damage, Stale};
}

#[path = "../../../app_skeleton/src/runner/chrome.rs"]
pub mod chrome;

#[path = "../../../app_skeleton/src/runner/inbox/keys.rs"]
pub mod inbox_keys;

pub mod runner {
    pub use super::chrome;
    pub mod inbox {
        pub use crate::inbox_keys as keys;
    }
}

#[path = "../../../app_skeleton/src/input/decode.rs"]
mod decode;
#[path = "../../../app_skeleton/src/input/event.rs"]
mod event;
#[path = "../../../app_skeleton/src/input/keys.rs"]
mod keys;
#[path = "../../../app_skeleton/src/input/kind.rs"]
mod kind;
#[path = "../../../app_skeleton/src/input/modifiers.rs"]
mod modifiers;
#[path = "../../../app_skeleton/src/input/routed.rs"]
mod routed;
/// The window manager's popup placement, which the desk's popup_open uses.
#[path = "../../../capsule_wm/src/geometry/rect.rs"]
pub mod wm_rect;
#[path = "../../../capsule_wm/src/geometry/positioner.rs"]
pub mod wm_positioner;
use wm_rect as rect;

pub mod input {
    pub use super::event::InputEvent;
    pub use super::keys::*;
    pub use super::kind::InputKind;
    pub use super::modifiers::*;
    pub use super::routed::parse_routed;
}

pub use input::KEY_F1;
/// The event the real routed decoder (input/routed.rs) reads into.
pub use input::InputEvent;

pub mod discover {
    pub struct ServicePeer {
        pub port: u32,
        pub pid: u32,
    }

    /// The desktop's services each have a port; the clipboard only when
    /// the host has turned it on (desk.rs).
    pub fn lookup_port(service: &[u8]) -> Option<u32> {
        super::desk::port_of(service)
    }

    pub fn lookup_service(service: &[u8]) -> Option<ServicePeer> {
        let port = super::desk::port_of(service)?;
        Some(ServicePeer { port, pid: port })
    }

    /// The router is the service whose pid is its port.
    pub fn from_router(sender: u32) -> bool {
        lookup_port(b"input_router") == Some(sender)
    }

    /// The window manager likewise.
    pub fn from_wm(sender: u32) -> bool {
        lookup_port(b"wm") == Some(sender)
    }
}

pub mod clients {
    pub mod compositor {
        pub use crate::desk::{layer_content, Content, DisplayInfo, FrameTime, KeyedLayer};
        pub use crate::desk::{layer_content_parts, present_layer_parts};
        pub use crate::desk::{
            cursor_shape, damage_commit, display_info, frame_time, present_layer, scanout_peer,
            scene_remove, scene_remove_layer, scene_submit, scene_submit_layer,
        };
    }
    pub mod wm {
        pub use crate::desk::{
            popup_done, popup_open, window_close, window_focus, window_maximize, window_minimize,
            window_move, window_open, window_raise, window_resize, window_restore, PopupPlacement,
            PopupSpec,
            WindowPlacement, POPUP_GRAB,
        };
    }
    pub mod clipboard {
        pub use crate::desk::{clipboard_clear, clipboard_copy, clipboard_paste};
    }
    pub mod input_router {
        pub use crate::desk::subscribe;
    }
}
