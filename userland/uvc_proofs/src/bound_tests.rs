// NONOS Operating System (AGPL-3.0-or-later)
//! A committed frame's bound, held to the frame the camera described: the
//! probe answer's dwMaxVideoFrameSize believed only up to four bytes a
//! pixel of that frame, and a format or frame never described refused.

use nonos_uvc::{camera, frame_bound, Camera, Probe, MAX_FRAME_BYTES};

use crate::webcam::config;

fn agreed(format: u8, frame: u8, max_frame: u32) -> Probe {
    Probe { format, frame, max_frame, ..Probe::default() }
}

fn cam() -> Camera {
    camera(&config()).unwrap()
}

#[test]
fn the_device_bound_is_taken_within_the_frame() {
    assert_eq!(frame_bound(&cam(), &agreed(1, 1, 614_400)), Some(614_400));
    assert_eq!(frame_bound(&cam(), &agreed(1, 1, 0)), Some(614_400), "the descriptor's");
    assert_eq!(frame_bound(&cam(), &agreed(1, 2, 320 * 240 * 4)), Some(320 * 240 * 4));
}

#[test]
fn a_bound_past_four_bytes_a_pixel_is_refused() {
    assert_eq!(frame_bound(&cam(), &agreed(1, 1, 640 * 480 * 4 + 1)), None);
    assert_eq!(frame_bound(&cam(), &agreed(2, 1, u32::MAX)), None);
}

#[test]
fn a_format_or_frame_never_described_is_refused() {
    assert_eq!(frame_bound(&cam(), &agreed(1, 9, 1000)), None);
    assert_eq!(frame_bound(&cam(), &agreed(7, 1, 1000)), None);
}

#[test]
fn a_huge_frame_is_held_to_the_drivers_largest() {
    let mut c = cam();
    let fr = &mut c.formats[0].frames[0];
    (fr.width, fr.height, fr.max_bytes) = (u16::MAX, u16::MAX, 0);
    assert_eq!(frame_bound(&c, &agreed(1, 1, 0)), Some(MAX_FRAME_BYTES as u32));
    assert_eq!(frame_bound(&c, &agreed(1, 1, MAX_FRAME_BYTES as u32 + 1)), None);
}
