// NONOS Operating System (AGPL-3.0-or-later)
//! The bulk pipes. Each bulk IN takes the next scripted answer, or that of
//! a unit never ready (`unready`); a bulk OUT is taken whole, or said to
//! have gone a byte short (`short_outs`), and a whole CBW sets the tag the
//! device's next CSW echoes. Pipe resets are recorded and succeed.

use super::device::{setup, Answer};
use super::unready::unready;
use super::window::{call, never_completes};
use super::E_PIPE;
use crate::descriptors::MscBinding;

const CBW_SIGNATURE: [u8; 4] = *b"USBC";
const CSW_SIGNATURE: [u8; 4] = *b"USBS";

pub fn configure_bulk(_xhci: u32, _slot: u8, _b: &MscBinding) -> Result<(), i32> {
    call()
}

pub fn bulk_out(_xhci: u32, _slot: u8, data: &[u8]) -> Result<usize, i32> {
    call()?;
    if setup(|d| d.short_outs.checked_sub(1).map(|left| d.short_outs = left)).is_some() {
        return Ok(data.len().saturating_sub(1));
    }
    if data.len() == 31 && data[0..4] == CBW_SIGNATURE {
        let tag = u32::from_le_bytes([data[4], data[5], data[6], data[7]]);
        setup(|d| {
            (d.tag, d.phase) = (tag, 0);
            d.commands.push(data[15]);
        });
    }
    Ok(data.len())
}

/// The next answer; none left is a transfer that never completes.
pub fn bulk_in(_xhci: u32, _slot: u8, out: &mut [u8]) -> Result<usize, i32> {
    call()?;
    let next = setup(|d| {
        let unready = || unready(*d.commands.last()?, d.phase);
        let next = d.answers.pop_front().or_else(|| d.not_ready.then(unready).flatten());
        d.phase = d.phase.saturating_add(1);
        next.map(|a| (a, d.tag))
    });
    let bytes = match next {
        Some((Answer::Data(bytes), _)) => bytes,
        Some((Answer::Zlp, _)) => Vec::new(),
        Some((Answer::Csw { residue, status }, tag)) => {
            let mut csw = CSW_SIGNATURE.to_vec();
            csw.extend_from_slice(&tag.to_le_bytes());
            csw.extend_from_slice(&residue.to_le_bytes());
            csw.push(status);
            csw
        }
        Some((Answer::Stall, _)) => return Err(E_PIPE),
        None => return never_completes(),
    };
    let n = bytes.len().min(out.len());
    out[..n].copy_from_slice(&bytes[..n]);
    Ok(n)
}

pub fn reset_bulk(_xhci: u32, _slot: u8, dir_in: bool) -> Result<(), i32> {
    call()?;
    setup(|d| d.resets.push(dir_in));
    Ok(())
}
