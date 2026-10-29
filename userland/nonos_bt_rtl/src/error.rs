// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)

//! Why no patch was loaded.

use nonos_bt_hci::HciError;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RtlError {
    /// The patch file does not begin with "Realtech".
    Signature,
    /// The file does not end with the extension signature.
    Extension,
    /// The extension names no project, or holds an instruction of length 0.
    NoProject,
    /// The patch is for this other project, not the RTL8821C's (10).
    Project(u8),
    /// The file holds no patch for this ROM version.
    NoPatchFor(u8),
    /// The header, its tables or the chosen patch run past the file's end.
    Truncated,
    /// The config file's signature or length is wrong.
    Config,
    /// Read ROM Version answered other than one octet after its status.
    RomReply,
    /// A fragment was answered other than with one octet after its status.
    DownloadReply,
    /// The controller or its transport failed.
    Hci(HciError),
}

impl From<HciError> for RtlError {
    fn from(e: HciError) -> Self {
        Self::Hci(e)
    }
}

impl RtlError {
    /// The words a log line gives it; none names the device.
    pub fn words(&self) -> &'static [u8] {
        match self {
            Self::Signature => b"patch file is not an epatch",
            Self::Extension | Self::NoProject => b"patch file has no project",
            Self::Project(_) => b"patch file is for another chip",
            Self::NoPatchFor(_) => b"no patch for this ROM version",
            Self::Truncated => b"patch file is cut short",
            Self::Config => b"config file is malformed",
            Self::RomReply => b"ROM version reply malformed",
            Self::DownloadReply => b"download reply malformed",
            Self::Hci(_) => b"controller failed or refused the setup",
        }
    }
}
