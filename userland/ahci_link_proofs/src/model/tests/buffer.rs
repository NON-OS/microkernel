// NONOS Operating System (AGPL-3.0-or-later)
//! The handlers' copies into and out of the data buffer, at and past its
//! length: one that does not fit copies nothing.

use crate::constants::ata::DATA_BUF_BYTES;
use crate::error::AhciError;
use crate::model::Rig;

const FULL: usize = DATA_BUF_BYTES as usize;

#[test]
fn a_payload_is_copied_in_only_if_it_fits() {
    let mut rig = Rig::new(Vec::new(), true);
    for len in [0, 1, 512, FULL - 1, FULL] {
        assert_eq!(rig.port.load(&vec![0x5a; len]), Ok(()), "{len} bytes");
    }
    rig.port.load(&vec![0x11; FULL]).unwrap();
    for len in [FULL + 1, FULL + 512, 2 * FULL] {
        assert_eq!(rig.port.load(&vec![0x22; len]), Err(AhciError::OutOfRange), "{len} bytes");
    }
    assert_eq!(rig.data(FULL), vec![0x11; FULL], "a payload too long copied in part");
}

#[test]
fn a_read_is_copied_out_only_as_far_as_the_buffer_goes() {
    let mut rig = Rig::new(Vec::new(), true);
    rig.port.load(&vec![0x77; FULL]).unwrap();
    for len in [0, 1, FULL] {
        let mut out = vec![0; len];
        assert_eq!(rig.port.take(&mut out), Ok(()), "{len} bytes");
        assert!(out.iter().all(|&b| b == 0x77));
    }
    for len in [FULL + 1, 2 * FULL] {
        let mut out = vec![0; len];
        assert_eq!(rig.port.take(&mut out), Err(AhciError::OutOfRange), "{len} bytes");
        assert!(out.iter().all(|&b| b == 0), "{len} bytes: copied in part");
    }
}
