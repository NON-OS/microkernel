// NONOS Operating System (AGPL-3.0-or-later)

//! The download's end on the console, naming the exact sub-step that failed,
//! so a serial-less boot says which of staging, DDMA, checksum or the 8051
//! boot went wrong.

use super::FwLoadError;
use crate::fw::{prep, DownloadError};
use crate::status;

pub(super) fn say_download(e: DownloadError) {
    status::line(match e {
        DownloadError::BadImage => b"[rtl8821ce] fw: bad image\n".as_slice(),
        DownloadError::Stage => b"[rtl8821ce] fw: reserved-page staging failed\n".as_slice(),
        DownloadError::Ddma => b"[rtl8821ce] fw: ddma transfer failed\n".as_slice(),
        DownloadError::Checksum => b"[rtl8821ce] fw: section checksum failed\n".as_slice(),
    });
}

// The close-out separates a bad-checksum download (the DDMA delivered wrong
// bytes) from an 8051 that never booted, and prints the control register so
// its individual bits say how far the handshake reached without a serial port.
pub(super) fn finish(finished: prep::Finish) -> Result<(), FwLoadError> {
    match finished {
        prep::Finish::Ready => Ok(()),
        prep::Finish::ChecksumBad(ctrl) => {
            status::line(b"[rtl8821ce] fw: hardware checksum not ok, ctrl=0x");
            status::hex16(ctrl);
            status::line(b" (ddma delivered bad data)\n");
            Err(FwLoadError::NotReady)
        }
        prep::Finish::NotReady(ctrl) => {
            status::line(b"[rtl8821ce] fw: 8051 not ready, ctrl=0x");
            status::hex16(ctrl);
            status::line(b"\n");
            Err(FwLoadError::NotReady)
        }
    }
}
