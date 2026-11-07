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

//! What a client sends and reads (app_skeleton clients/wm/popup.rs) is what
//! the window manager reads and sends.

use crate::window::popup::decode;
use crate::wm_notify_encode::{
    encode_notify, NOTIFY_KIND_KEYS, NOTIFY_KIND_POPUP_DONE, NOTIFY_LEN,
};
use nonos_app_skeleton::clients::wm::{popup_body, popup_done, PopupSpec, POPUP_GRAB};

#[test]
fn the_clients_request_is_the_one_the_window_manager_reads() {
    let spec = PopupSpec {
        window_id: 7,
        parent_id: 3,
        anchor_x: -12,
        anchor_y: 40,
        anchor_w: 90,
        anchor_h: 22,
        width: 240,
        height: 310,
        anchor: 6,
        gravity: 8,
        adjust: 13,
        offset_x: 4,
        offset_y: -2,
        flags: POPUP_GRAB,
    };
    let ask = decode(&popup_body(&spec)).expect("read whole");
    let p = ask.positioner;
    assert_eq!((ask.window_id, ask.parent_id, ask.grab), (7, 3, true));
    assert_eq!((p.anchor_x, p.anchor_y, p.anchor_w, p.anchor_h), (-12, 40, 90, 22));
    assert_eq!((p.width, p.height, p.anchor, p.gravity, p.adjust), (240, 310, 6, 8, 13));
    assert_eq!((p.offset_x, p.offset_y), (4, -2));
}

#[test]
fn the_popup_done_notice_names_the_popup_and_nothing_else_does() {
    let mut f = [0u8; NOTIFY_LEN];
    encode_notify(&mut f, NOTIFY_KIND_POPUP_DONE, 30, 9, 0, 0);
    assert_eq!(popup_done(&f), Some(9));
    encode_notify(&mut f, NOTIFY_KIND_KEYS, 30, 9, 1, 0);
    assert_eq!(popup_done(&f), None);
    assert_eq!(popup_done(&f[..NOTIFY_LEN - 1]), None);
}
