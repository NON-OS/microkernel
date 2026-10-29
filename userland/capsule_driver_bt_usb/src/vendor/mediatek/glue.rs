/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! The capsule's `Port`: the bound controller's default endpoint through
//! driver.xhci0, the desk's wait (which answers NBTC meanwhile), the
//! package through `MkInstallSource` and SHA-256 through `CryptoHash`.

use alloc::vec::Vec;

use nonos_bt_hci::usb::Setup;
use nonos_libc::{crypto_hash, mk_debug, mk_install_source};
use nonos_usbnet::Bus;

use super::port::Port;
use crate::usb::XhciPipes;

/// `CryptoHash`'s number for SHA-256.
const SHA256: u64 = 1;

pub struct Glue<'a>(pub &'a mut XhciPipes);

fn usb(s: Setup) -> nonos_usbnet::Setup {
    nonos_usbnet::Setup::new(s.request_type, s.request, s.value, s.index)
}

impl Port for Glue<'_> {
    fn control_out(&mut self, s: Setup, data: &[u8]) -> Result<(), i32> {
        self.0.bus.control_out(usb(s), data)
    }
    fn control_in(&mut self, s: Setup, out: &mut [u8]) -> Result<usize, i32> {
        self.0.bus.control_in(usb(s), out)
    }
    fn wait(&mut self, ms: u64) {
        self.0.desk.wait(ms);
    }
    fn file(&mut self, kind: u64, at: u64, out: &mut [u8]) -> i64 {
        mk_install_source(kind, at, out)
    }
    fn sha256(&mut self, data: &[u8]) -> Option<[u8; 32]> {
        let mut d = [0u8; 32];
        let n = crypto_hash(SHA256, data.as_ptr(), data.len(), d.as_mut_ptr(), d.len());
        (n == 32).then_some(d)
    }
}

/// One whole `bt-usb:` line.
pub fn say(words: &[&[u8]]) {
    let mut line: Vec<u8> = b"bt-usb: ".to_vec();
    words.iter().for_each(|w| line.extend_from_slice(w));
    line.push(b'\n');
    let _ = mk_debug(line.as_ptr(), line.len());
}

/// `n` in decimal.
pub fn digits(n: usize) -> Vec<u8> {
    alloc::format!("{n}").into_bytes()
}
