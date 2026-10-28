// NONOS Operating System (AGPL-3.0-or-later)

//! What discovery leads to, decided from the broker's records alone so it is
//! proven on the host. A function this driver runs is brought up. An Intel
//! Wi-Fi function that is there but not run (a generation with no boot path
//! here, or an id this driver does not know) is not absent: the driver names
//! it in one line and leaves with `EXIT_GAVE_UP`, which the kernel tells as
//! "device present, bring-up failed and was given up". Only a machine with no
//! Intel Wi-Fi function at all is `EXIT_ABSENT`.

use crate::firmware::generation::name;
use crate::pci_match::{is_intel_wifi, is_supported_adapter, Candidate};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Verdict {
    /// Bring up the function at this position.
    Run(usize),
    /// An Intel Wi-Fi function with this PCI device id is there, not run.
    NotRun(u16),
    Absent,
}

/// The first function this driver runs, else the first Intel Wi-Fi function
/// there, else nothing.
pub fn verdict(candidates: impl IntoIterator<Item = Candidate>) -> Verdict {
    let mut not_run = None;
    for (i, c) in candidates.into_iter().enumerate() {
        if is_supported_adapter(&c) {
            return Verdict::Run(i);
        }
        if not_run.is_none() && is_intel_wifi(&c) {
            not_run = Some(c.device);
        }
    }
    not_run.map_or(Verdict::Absent, Verdict::NotRun)
}

pub const LINE_MAX: usize = 200;

/// The one console line for a function there but not run: its id and what
/// it is, built whole. Nothing in it identifies the machine.
pub fn not_run_line(device: u16) -> ([u8; LINE_MAX], usize) {
    let mut buf = [0u8; LINE_MAX];
    let mut len = 0;
    let mut put = |t: &[u8]| {
        let n = t.len().min(LINE_MAX - 1 - len);
        buf[len..len + n].copy_from_slice(&t[..n]);
        len += n;
    };
    put(b"[iwlwifi] pci 8086:");
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    put(&core::array::from_fn::<u8, 4, _>(|i| DIGITS[usize::from((device >> (12 - i * 4)) & 0xF)]));
    put(b" ");
    put(name(device)
        .map_or(&b"an Intel Wi-Fi id this driver does not know"[..], |g| g.what.as_bytes()));
    put(b": present, not run by this driver; given up");
    buf[len] = b'\n';
    (buf, len + 1)
}
