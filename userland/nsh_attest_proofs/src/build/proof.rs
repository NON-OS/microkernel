//! `HOST_PROOF` bodies, laid out as nsh-v1-attest.md gives them and signed
//! by the host keys over the transcript.

use nsh_crypto::{labelled, Label, Transcript};
use nsh_wire::Kind;

use super::keys::hybrid;
use super::Built;

/// Everything before the signatures.
pub fn unsigned(b: &Built) -> Vec<u8> {
    let h = hybrid(b.host_key);
    let mut out = [&h.ed.public[..], &h.ml_public[..]].concat();
    out.extend((b.enrol.len() as u16).to_le_bytes());
    out.extend(&b.enrol);
    out.extend((b.doc.len() as u32).to_le_bytes());
    out.extend(&b.doc);
    out.extend((b.entries.len() as u32).to_le_bytes());
    out.extend(&b.entries);
    out
}

pub fn host_proof(b: &Built, t: &Transcript) -> Vec<u8> {
    let mut out = unsigned(b);
    let digest = labelled(Label::HostSig, &t.without_sigs(Kind::HostProof, &out));
    out.extend(hybrid(b.host_key).sign(&digest));
    out
}
