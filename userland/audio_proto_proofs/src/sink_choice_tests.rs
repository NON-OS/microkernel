// NONOS Operating System (AGPL-3.0-or-later)
//! Which driver the server plays through: a ready USB device, else HD Audio.

use crate::server::choose::choose;

const HDA: u32 = 4218;
const USB: u32 = 4246;

#[test]
fn a_ready_usb_device_plays_over_hd_audio() {
    assert_eq!(choose(true, Some(HDA), Some(USB)), Some(USB));
    assert_eq!(choose(true, None, Some(USB)), Some(USB));
}

#[test]
fn hd_audio_plays_while_the_usb_driver_has_no_device() {
    assert_eq!(choose(false, Some(HDA), Some(USB)), Some(HDA));
    assert_eq!(choose(false, Some(HDA), None), Some(HDA));
    // Readiness from a driver that is not there changes nothing.
    assert_eq!(choose(true, Some(HDA), None), Some(HDA));
}

#[test]
fn the_usb_driver_alone_is_kept_so_a_device_can_still_come() {
    assert_eq!(choose(false, None, Some(USB)), Some(USB));
    assert_eq!(choose(false, None, None), None);
    assert_eq!(choose(true, None, None), None);
}
