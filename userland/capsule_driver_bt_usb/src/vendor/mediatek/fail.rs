/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! Why a MediaTek controller was not set up. Each is a whole phrase for
//! the log; none carries anything of the chip but what went wrong.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fail {
    /// A control transfer failed, or a register read came back short.
    Usb,
    /// A chip, or a ROM revision of one, the package has no patch for.
    Chip,
    /// The package's file is not on the boot medium.
    Missing,
    /// A file is there, but not at its pinned length and SHA-256.
    Mismatch,
    /// The patch's header or a section map points outside the file.
    Image,
    /// No WMT event within 10 s of a command.
    Silent,
    /// A WMT event that answers another op, or is cut short.
    Answer,
    /// A patch status the ROM has no meaning for.
    Status,
    /// The patch semaphore still held after 20 looks, 100 ms apart.
    Busy,
    /// The subsystem reset not done within 1 s, or its chip id gone.
    Reset,
}

impl Fail {
    pub fn words(self) -> &'static [u8] {
        match self {
            Fail::Usb => b"a control transfer failed",
            Fail::Chip => b"no patch in the package for this chip",
            Fail::Missing => b"the firmware package is missing from the boot medium",
            Fail::Mismatch => b"the patch file is not the pinned one",
            Fail::Image => b"the patch file does not parse",
            Fail::Silent => b"no WMT answer within 10 s",
            Fail::Answer => b"a WMT answer to another command",
            Fail::Status => b"a patch status the chip should not send",
            Fail::Busy => b"the patch semaphore stayed held",
            Fail::Reset => b"the subsystem reset did not finish",
        }
    }
}
