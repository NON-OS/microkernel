// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)

//! A Realtek RTL8821C is given its patch before bring-up (nonos_bt_rtl);
//! any other controller is sent nothing here. The lines name the ROM and
//! patch versions only, never the device.

use nonos_bt_hci::{HciError, Host, Transport};
use nonos_bt_rtl::{prepare, Outcome, RtlError};

mod firmware;

use crate::run::{digits, say};
use firmware::{CONFIG, FW};

/// The patch loaded where one is needed; a controller that cannot take it
/// is not driven, since its ROM alone is not what Realtek ships it to run.
pub fn patch<T: Transport>(host: &mut Host<T>, ids: (u16, u16)) -> Result<(), HciError> {
    match prepare(host, ids, FW, CONFIG) {
        Ok(Outcome::NotRealtek) => Ok(()),
        Ok(Outcome::Running) => {
            say(&[b"Realtek controller runs its own firmware"]);
            Ok(())
        }
        Ok(Outcome::Patched { rom, fw_version, fragments }) => {
            let (rom, n) = (digits(u64::from(rom), 1), digits(fragments as u64, 1));
            let v = digits(u64::from(fw_version), 1);
            say(&[
                b"Realtek patch loaded: ROM ",
                &rom,
                b", firmware ",
                &v,
                b", ",
                &n,
                b" fragments",
            ]);
            Ok(())
        }
        Err(RtlError::Hci(e)) => {
            say(&[b"Realtek patch not loaded: ", RtlError::Hci(e).words()]);
            Err(e)
        }
        Err(e) => {
            say(&[b"Realtek patch not loaded: ", e.words()]);
            Err(HciError::NotBluetooth)
        }
    }
}
