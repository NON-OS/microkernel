/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! What the MediaTek setup asks of the capsule: the controller's default
//! endpoint, a clock to wait on, the firmware package and a digest. The
//! capsule's is `glue.rs`; the proofs' is a modelled controller.

use nonos_bt_hci::usb::Setup;

pub trait Port {
    /// A control transfer with `data` as its data stage out.
    fn control_out(&mut self, s: Setup, data: &[u8]) -> Result<(), i32>;
    /// A control transfer with up to `out.len()` octets in; the octets read.
    fn control_in(&mut self, s: Setup, out: &mut [u8]) -> Result<usize, i32>;
    /// Wait `ms` milliseconds.
    fn wait(&mut self, ms: u64);
    /// `MkInstallSource`: the octets of package file `kind` from `at` into
    /// `out`, zero at its end, negative when the kernel holds no such file.
    fn file(&mut self, kind: u64, at: u64, out: &mut [u8]) -> i64;
    /// The SHA-256 of `data`, or `None` when it could not be taken.
    fn sha256(&mut self, data: &[u8]) -> Option<[u8; 32]>;
}
