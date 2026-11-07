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

use crate::state::direct_rule::{pick, refusal, seen, way, Facts, Over, Refusal, Top, Way};

const SCREEN: (u32, u32) = (1920, 1080);

fn facts() -> Facts {
    Facts { able: true, screen: SCREEN, plain: true, cursor_plane: true }
}

/// A video player's buffer filling the screen, opaque, as presented.
fn video() -> Top {
    Top {
        handle: 0x2_0000_0011,
        at: (0, 0),
        size: SCREEN,
        buffer: SCREEN,
        stride: 1920 * 4,
        opaque: true,
        refused: false,
    }
}

#[test]
fn a_full_screen_opaque_swapchain_buffer_is_scanned_out() {
    assert_eq!(pick(&facts(), Some(&video())), Some(0x2_0000_0011));
}

#[test]
fn nothing_on_top_means_compose() {
    assert_eq!(pick(&facts(), None), None);
}

#[test]
fn every_screen_condition_alone_sends_it_back_to_composite() {
    let t = video();
    let no = |f: Facts| pick(&f, Some(&t));
    assert_eq!(no(Facts { able: false, ..facts() }), None, "no IOMMU, or no band plane");
    assert_eq!(no(Facts { plain: false, ..facts() }), None, "a look, a lens, scale, a second output");
    assert_eq!(no(Facts { cursor_plane: false, ..facts() }), None, "the pointer is drawn");
}

#[test]
fn a_layer_that_does_not_cover_the_screen_exactly_is_composed() {
    let f = facts();
    let cases = [
        Top { at: (1, 0), ..video() },
        Top { size: (1920, 1079), ..video() },
        Top { size: (1280, 720), at: (320, 180), ..video() },
        Top { buffer: (3840, 2160), ..video() },
        Top { buffer: (1280, 720), ..video() },
        Top { stride: 1920 * 4 - 4, ..video() },
    ];
    for t in cases {
        assert_eq!(pick(&f, Some(&t)), None, "{t:?}");
    }
}

#[test]
fn a_popup_or_prompt_over_the_video_is_the_top_and_is_composed() {
    // The scene's top is the popup: it does not cover the screen.
    let popup = Top { handle: 7, at: (800, 400), size: (320, 200), buffer: (320, 200), ..video() };
    assert_eq!(pick(&facts(), Some(&popup)), None);
}

#[test]
fn a_translucent_or_refused_buffer_is_composed() {
    assert_eq!(pick(&facts(), Some(&Top { opaque: false, ..video() })), None);
    assert_eq!(pick(&facts(), Some(&Top { refused: true, ..video() })), None);
}

#[test]
fn the_plane_is_asked_only_when_something_changed() {
    let (a, b) = (1u64, 2u64);
    assert_eq!(way(Some(a), None, true), Way::Scan(a), "entering");
    assert_eq!(way(Some(a), Some(a), false), Way::Idle, "nothing new");
    assert_eq!(way(Some(a), Some(a), true), Way::Scan(a), "the band changed");
    assert_eq!(way(Some(b), Some(a), true), Way::Scan(b), "the next buffer");
    assert_eq!(way(None, Some(a), false), Way::Compose { whole: true }, "leaving");
    assert_eq!(way(None, None, true), Way::Compose { whole: false });
}

#[test]
fn refusals_are_sorted_by_what_they_say_of_the_next_frame() {
    use nonos_gfx_wire::errno::{EINVAL, ENODEV, ENOSYS, EPERM, ESTALE, ETIMEDOUT};
    assert_eq!(refusal(ENODEV), Refusal::Driver, "no IOMMU confines the device");
    assert_eq!(refusal(ENOSYS), Refusal::Driver);
    assert_eq!(refusal(EPERM), Refusal::Buffer, "the client did not let the driver read it");
    assert_eq!(refusal(EINVAL), Refusal::Buffer);
    assert_eq!(refusal(ESTALE), Refusal::Frame);
    assert_eq!(refusal(ETIMEDOUT), Refusal::Frame);
}

/// A Linux window filling the screen, its frame's ground the layer's own
/// surface and the client's pinned XRGB buffer drawn over it.
fn pinned() -> Over {
    let handle = 0x3_0000_0021;
    let (buffer, stride) = (SCREEN, 1920 * 4);
    Over { handle, at: (0, 0), buffer, stride, opaque: true, refused: false, whole: true }
}

fn ground() -> Top {
    Top { handle: 0x2_0000_0005, opaque: false, ..video() }
}

#[test]
fn a_layer_with_no_content_is_seen_as_its_own_surface() {
    assert_eq!(seen(video(), None), Some(video()));
}

#[test]
fn a_full_screen_window_is_scanned_out_from_its_pinned_buffer_not_its_ground() {
    let t = seen(ground(), Some(pinned())).expect("one picture");
    assert_eq!(t.handle, 0x3_0000_0021);
    assert_eq!((t.at, t.size), ((0, 0), SCREEN), "where the layer is");
    assert_eq!(pick(&facts(), Some(&t)), Some(0x3_0000_0021));
}

#[test]
fn content_that_blends_or_sits_off_the_corner_is_two_pictures_and_composed() {
    let blended = Over { opaque: false, ..pinned() };
    assert_eq!(seen(ground(), Some(blended)), None, "ARGB blends over the ground");
    let below_title = Over { at: (0, 30), ..pinned() };
    assert_eq!(seen(ground(), Some(below_title)), None, "a title bar above it");
    assert_eq!(seen(ground(), Some(Over { at: (1, 0), ..pinned() })), None);
    assert_eq!(seen(ground(), Some(Over { handle: 0, ..pinned() })), None);
    let part = Over { whole: false, ..pinned() };
    assert_eq!(seen(ground(), Some(part)), None, "a window's part of its buffer: composed");
}

#[test]
fn pinned_content_that_does_not_fill_the_screen_or_was_refused_is_composed() {
    let f = facts();
    let cases = [
        Over { buffer: (1920, 1050), ..pinned() },
        Over { buffer: (3840, 2160), ..pinned() },
        Over { stride: 1920 * 4 - 4, ..pinned() },
        Over { refused: true, ..pinned() },
    ];
    for c in cases {
        let t = seen(ground(), Some(c));
        assert_eq!(t.and_then(|t| pick(&f, Some(&t))), None, "{c:?}");
    }
    let window = Top { size: (1280, 720), at: (320, 180), ..ground() };
    let t = seen(window, Some(Over { buffer: (1280, 720), stride: 1280 * 4, ..pinned() }));
    assert_eq!(t.and_then(|t| pick(&f, Some(&t))), None, "a window not filling the screen");
}

#[test]
fn the_ground_being_refused_or_translucent_does_not_matter_under_an_opaque_content() {
    let own = Top { refused: true, opaque: false, ..ground() };
    let t = seen(own, Some(pinned())).expect("one picture");
    assert_eq!(pick(&facts(), Some(&t)), Some(0x3_0000_0021));
}
