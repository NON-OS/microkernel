// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! A request run to a report: files read, measurements taken, each trailer
//! checked. Every file that cannot be read, and every image that cannot be
//! measured, is a could-not-check, never a pass and never a failure. Files
//! come through a reader, the file system's for the command line and the
//! page's own for the browser build, so both run this code and no other.

use nonos_attest_path::{root_words, Kind};

use crate::check::check_v4;
use crate::measure::{authenticode_of, blake3_of, Slot};
use crate::report::{kind_name, Failure, Outcome, Report, Subject, Verdict};
use crate::slot::opens;
use crate::transcript;

/// Files larger than this are refused unread: no image, trailer, root or
/// transcript the gates take comes near it.
pub const MAX_FILE: u64 = 1 << 30;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Mode {
    Capsule { image: String, caps: u64 },
    Kernel { image: String },
    Bootloader { image: String },
    Transcript { file: String },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Request {
    pub mode: Mode,
    pub root: Option<String>,
    pub trailers: Vec<String>,
    pub allow_dev: bool,
}

/// Where a request's named files come from.
pub type Reader<'a> = &'a dyn Fn(&str) -> Result<Vec<u8>, String>;

/// A file from the file system, refused when it is not a regular file or is
/// larger than `MAX_FILE`.
pub fn read_file(path: &str) -> Result<Vec<u8>, String> {
    read_file_within(path, MAX_FILE)
}

/// `read_file` with another bound.
pub fn read_file_within(path: &str, max: u64) -> Result<Vec<u8>, String> {
    let meta = std::fs::metadata(path).map_err(|e| format!("cannot read {path}: {e}"))?;
    if !meta.is_file() {
        return Err(format!("{path} is not a file"));
    }
    if meta.len() > max {
        return Err(format!("{path} is larger than {max} bytes"));
    }
    std::fs::read(path).map_err(|e| format!("cannot read {path}: {e}"))
}

fn read_root(read: Reader, path: &str) -> Result<[u8; 32], String> {
    let b = read(path)?;
    let root: [u8; 32] =
        b.try_into().map_err(|b: Vec<u8>| format!("root {path} is {} bytes, not 32", b.len()))?;
    root_words(&root).ok_or(format!("root {path} is not four canonical field words"))?;
    Ok(root)
}

/// Run `req` over the file system.
pub fn run(req: &Request) -> Report {
    run_with(req, &read_file)
}

/// Run `req` with its files from `read`.
pub fn run_with(req: &Request, read: Reader) -> Report {
    let mut report = Report::default();
    if req.trailers.is_empty() {
        report.refusal = Some((Verdict::CouldNotCheck, "no trailer given".into()));
        return report;
    }
    let given = match req.root.as_deref().map(|p| read_root(read, p)) {
        Some(Ok(r)) => Some(r),
        Some(Err(e)) => {
            report.refusal = Some((Verdict::CouldNotCheck, e));
            return report;
        }
        None => None,
    };
    match &req.mode {
        Mode::Transcript { file } => by_transcript(read, &mut report, file, given, req),
        mode => {
            let Some(root) = given else {
                report.refusal = Some((Verdict::CouldNotCheck, "no root given (--root)".into()));
                return report;
            };
            report.root = Some(root);
            let slot = measure(read, mode);
            for t in &req.trailers {
                report.subjects.push(direct(read, &root, mode, &slot, t, req.allow_dev));
            }
        }
    }
    report
}

/// The slot an image fills, measured the way its gate measures it.
fn measure(read: Reader, mode: &Mode) -> Result<Slot, String> {
    match mode {
        Mode::Capsule { image, caps } => Ok(Slot::Capsule(blake3_of(&read(image)?), *caps)),
        Mode::Kernel { image } => Ok(Slot::Kernel(blake3_of(&read(image)?))),
        Mode::Bootloader { image } => {
            let bytes = read(image)?;
            Ok(Slot::Bootloader(authenticode_of(&bytes).map_err(|e| format!("{image}: {e}"))?))
        }
        Mode::Transcript { .. } => Err("no image in transcript mode".into()),
    }
}

fn direct(
    read: Reader,
    root: &[u8; 32],
    mode: &Mode,
    slot: &Result<Slot, String>,
    t: &str,
    allow_dev: bool,
) -> Subject {
    let mut s = Subject::new(t);
    s.image = match mode {
        Mode::Capsule { image, .. } | Mode::Kernel { image } | Mode::Bootloader { image } => {
            Some(image.clone())
        }
        Mode::Transcript { .. } => None,
    };
    s.kind = Some(match mode {
        Mode::Capsule { .. } => Kind::Capsule,
        Mode::Kernel { .. } => Kind::Kernel,
        _ => Kind::Bootloader,
    });
    if let Mode::Capsule { caps, .. } = mode {
        s.caps = Some(*caps);
    }
    let slot = match slot {
        Ok(slot) => slot,
        Err(e) => {
            s.outcome = Outcome::CouldNotCheck(e.clone());
            return s;
        }
    };
    match read(t) {
        Err(e) => {
            s.measurement = Some(*slot.measurement());
            s.outcome = Outcome::CouldNotCheck(e);
            s
        }
        Ok(bytes) => Subject { image: s.image, ..against(root, slot, t, &bytes, allow_dev) },
    }
}

/// A trailer's bytes checked against a slot measured from its image, under
/// `root`: the check behind `capsule`, `kernel` and `bootloader`, here and in
/// the browser build.
pub fn against(
    root: &[u8; 32],
    slot: &Slot,
    label: &str,
    bytes: &[u8],
    allow_dev: bool,
) -> Subject {
    let mut s = Subject::new(label);
    s.kind = Some(slot.kind());
    s.caps = slot.caps();
    s.measurement = Some(*slot.measurement());
    s.slot = opens(bytes).map(|o| o.index);
    s.outcome = outcome(check_v4(root, slot.kind(), &slot.context(), bytes, allow_dev));
    s
}

fn by_transcript(
    read: Reader,
    report: &mut Report,
    file: &str,
    given: Option<[u8; 32]>,
    req: &Request,
) {
    let text = match read(file)
        .and_then(|b| String::from_utf8(b).map_err(|_| format!("{file} is not text")))
    {
        Ok(t) => t,
        Err(e) => {
            report.refusal = Some((Verdict::CouldNotCheck, e));
            return;
        }
    };
    let tr = match transcript::read(&text) {
        Ok(tr) => tr,
        Err(e) => {
            report.refusal = Some((Verdict::Failed, format!("transcript {file}: {e}")));
            return;
        }
    };
    report.root = Some(given.unwrap_or(tr.root));
    if given.is_some_and(|g| g != tr.root) {
        report.refusal = Some((
            Verdict::Failed,
            format!("transcript {file} is for root {}, not the root given", crate::hex(&tr.root)),
        ));
        return;
    }
    report.transcript = Some((file.into(), tr.slots.len()));
    for t in &req.trailers {
        report.subjects.push(slotted(read, &tr, t, req.allow_dev));
    }
}

fn slotted(read: Reader, tr: &transcript::Transcript, t: &str, allow_dev: bool) -> Subject {
    match read(t) {
        Ok(bytes) => in_slot(tr, t, &bytes, allow_dev),
        Err(e) => Subject { outcome: Outcome::CouldNotCheck(e), ..Subject::new(t) },
    }
}

/// A trailer's bytes checked in the transcript slot its path opens: the check
/// behind `transcript`, here and in the browser build.
pub fn in_slot(tr: &transcript::Transcript, label: &str, bytes: &[u8], allow_dev: bool) -> Subject {
    let mut s = Subject::new(label);
    let Some(o) = opens(bytes) else {
        s.outcome = Outcome::Failed(crate::check::Refused::Malformed.into());
        return s;
    };
    s.slot = Some(o.index);
    s.kind = o.kind;
    let Some(slot) = tr.slots.get(o.index) else {
        s.outcome = Outcome::Failed(Failure {
            stage: "slot",
            code: 305,
            reason: format!("opens slot {}, which the transcript leaves as padding", o.index),
        });
        return s;
    };
    s.measurement = Some(*slot.measurement());
    s.caps = slot.caps();
    if o.kind.is_some_and(|k| k != slot.kind()) {
        s.outcome = Outcome::Failed(Failure {
            stage: "slot",
            code: 306,
            reason: format!(
                "a {} trailer for a {} slot",
                o.kind.map_or("development", kind_name),
                kind_name(slot.kind())
            ),
        });
        return s;
    }
    s.kind = Some(slot.kind());
    s.outcome = outcome(check_v4(&tr.root, slot.kind(), &slot.context(), bytes, allow_dev));
    s
}

fn outcome(r: Result<crate::check::Proof, crate::check::Refused>) -> Outcome {
    match r {
        Ok(p) => Outcome::Verified(p),
        Err(e) => Outcome::Failed(e.into()),
    }
}
