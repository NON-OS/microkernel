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

//! What a run found, and its two renderings: text for a person, and the v1
//! JSON document for a script. A result that could not be checked is never
//! rendered, counted or exited as verified.

use nonos_attest_path::Kind;

use crate::check::{Proof, Refused};
use crate::hex;
use crate::roots::{release_of, Release};

pub const FORMAT: &str = "nonos-proof-verify";
pub const FORMAT_VERSION: u32 = 1;

/// The run's answer, and the process's exit code for it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Verdict {
    Verified,
    Failed,
    CouldNotCheck,
}

impl Verdict {
    pub fn name(self) -> &'static str {
        match self {
            Verdict::Verified => "verified",
            Verdict::Failed => "failed",
            Verdict::CouldNotCheck => "could-not-check",
        }
    }

    pub fn exit_code(self) -> i32 {
        match self {
            Verdict::Verified => 0,
            Verdict::Failed => 1,
            Verdict::CouldNotCheck => 2,
        }
    }
}

/// A refusal: the stage that refused, its stable code, and why.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Failure {
    pub stage: &'static str,
    pub code: u32,
    pub reason: String,
}

impl From<Refused> for Failure {
    fn from(r: Refused) -> Failure {
        Failure { stage: r.stage(), code: r.code(), reason: r.reason() }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Outcome {
    Verified(Proof),
    Failed(Failure),
    CouldNotCheck(String),
}

impl Outcome {
    pub fn verdict(&self) -> Verdict {
        match self {
            Outcome::Verified(_) => Verdict::Verified,
            Outcome::Failed(_) => Verdict::Failed,
            Outcome::CouldNotCheck(_) => Verdict::CouldNotCheck,
        }
    }
}

/// One trailer checked, and what it was checked for.
#[derive(Clone, Debug)]
pub struct Subject {
    pub trailer: String,
    pub image: Option<String>,
    pub kind: Option<Kind>,
    pub slot: Option<usize>,
    pub measurement: Option<[u8; 32]>,
    pub caps: Option<u64>,
    pub outcome: Outcome,
}

impl Subject {
    pub fn new(trailer: &str) -> Subject {
        Subject {
            trailer: trailer.into(),
            image: None,
            kind: None,
            slot: None,
            measurement: None,
            caps: None,
            outcome: Outcome::CouldNotCheck("not checked".into()),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Report {
    pub root: Option<[u8; 32]>,
    pub transcript: Option<(String, usize)>,
    /// A refusal of the whole run, before or instead of any trailer: a root
    /// that cannot be read, a transcript that does not reproduce its root.
    pub refusal: Option<(Verdict, String)>,
    pub subjects: Vec<Subject>,
}

impl Report {
    /// The worst of the run: failed if anything was refused; else
    /// could-not-check if anything could not be checked, or nothing was;
    /// verified only when every trailer was.
    pub fn verdict(&self) -> Verdict {
        let rank = |v: &Verdict| match v {
            Verdict::Verified => 0,
            Verdict::CouldNotCheck => 1,
            Verdict::Failed => 2,
        };
        let refusal = self.refusal.iter().map(|(v, _)| *v);
        refusal
            .chain(self.subjects.iter().map(|s| s.outcome.verdict()))
            .max_by_key(rank)
            .unwrap_or(Verdict::CouldNotCheck)
    }

    fn release_root(&self) -> Option<&'static Release> {
        self.root.as_ref().and_then(release_of)
    }

    /// A release's verdict: verified, under a committed release root, with
    /// every trailer a STARK. A development trailer or root is never one.
    pub fn release(&self) -> bool {
        self.verdict() == Verdict::Verified
            && self.release_root().is_some()
            && self.subjects.iter().all(|s| s.outcome == Outcome::Verified(Proof::Stark))
    }

    pub fn text(&self) -> String {
        let mut s = format!("{FORMAT} {}\n", env!("CARGO_PKG_VERSION"));
        if let Some(root) = &self.root {
            s += &match self.release_root() {
                Some(r) => format!("root {}  the release {} ({})\n", hex(root), r.name, r.file),
                None => format!("root {}  development root, not a release\n", hex(root)),
            };
        }
        if let Some((file, n)) = &self.transcript {
            s += &format!("transcript {}: {n} slots, root reproduced\n", shown(file));
        }
        for x in &self.subjects {
            s += &line(x, self.release_root());
        }
        if let Some((_, why)) = &self.refusal {
            s += &format!("refused: {}\n", shown(why));
        }
        let total = self.subjects.len();
        let count = |v| self.subjects.iter().filter(|x| x.outcome.verdict() == v).count();
        s += &match self.verdict() {
            Verdict::Verified if self.release() => {
                format!("verdict: VERIFIED, {total} of {total}, release root, STARK proofs\n")
            }
            Verdict::Verified => format!(
                "verdict: VERIFIED, {total} of {total}, NOT A RELEASE ({})\n",
                if self.release_root().is_none() {
                    "development root"
                } else {
                    "development trailers, no STARK"
                }
            ),
            Verdict::Failed => {
                format!("verdict: FAILED, {} of {total} refused\n", count(Verdict::Failed))
            }
            Verdict::CouldNotCheck => format!(
                "verdict: COULD NOT CHECK, {} of {total} not checked; this is not a verification\n",
                count(Verdict::CouldNotCheck)
            ),
        };
        s
    }

    pub fn json(&self) -> String {
        let root = match (&self.root, self.release_root()) {
            (None, _) => "null".to_string(),
            (Some(r), Some(rel)) => format!(
                "{{\"hex\":{},\"release\":true,\"name\":{},\"file\":{}}}",
                q(&hex(r)),
                q(rel.name),
                q(rel.file)
            ),
            (Some(r), None) => format!(
                "{{\"hex\":{},\"release\":false,\"name\":null,\"file\":null,\
                 \"note\":\"development root, not a release\"}}",
                q(&hex(r))
            ),
        };
        let transcript = match &self.transcript {
            Some((f, n)) => format!("{{\"file\":{},\"slots\":{n},\"root_reproduced\":true}}", q(f)),
            None => "null".into(),
        };
        let refusal = self.refusal.as_ref().map_or("null".into(), |(_, w)| q(w));
        let results: Vec<String> = self.subjects.iter().map(subject_json).collect();
        let v = self.verdict();
        format!(
            "{{\"format\":{},\"version\":{FORMAT_VERSION},\"tool_version\":{},\"verdict\":{},\
             \"exit_code\":{},\"release\":{},\"root\":{root},\"transcript\":{transcript},\
             \"refusal\":{refusal},\"results\":[{}]}}\n",
            q(FORMAT),
            q(env!("CARGO_PKG_VERSION")),
            q(v.name()),
            v.exit_code(),
            self.release(),
            results.join(",")
        )
    }
}

pub fn kind_name(k: Kind) -> &'static str {
    match k {
        Kind::Kernel => "kernel",
        Kind::Capsule => "capsule",
        Kind::Bootloader => "bootloader",
        Kind::Pad => "pad",
    }
}

fn line(x: &Subject, release: Option<&Release>) -> String {
    let what = x.kind.map_or("trailer".to_string(), |k| kind_name(k).to_string())
        + &x.slot.map_or(String::new(), |i| format!(" slot {i}"));
    let t = shown(&x.trailer);
    let mut s = match &x.outcome {
        Outcome::Verified(Proof::Stark) => format!("VERIFIED      {what}  {t}  (path and STARK)"),
        Outcome::Verified(Proof::PathOnly) => format!(
            "VERIFIED      {what}  {t}  (path only: a development trailer, no STARK, never a release)"
        ),
        Outcome::Failed(f) => {
            format!("FAILED        {what}  {t}  {}: {} [{}]", f.stage, shown(&f.reason), f.code)
        }
        Outcome::CouldNotCheck(why) => format!("COULD NOT CHECK  {what}  {t}  {}", shown(why)),
    };
    if let (Some(k), Some(r)) = (x.kind, release) {
        if k != r.kind && matches!(x.outcome, Outcome::Failed(_)) {
            s += &format!(" (this root is the {}, which holds no {} slot)", r.name, kind_name(k));
        }
    }
    s + "\n"
}

/// A name or reason as one line of text: control characters and line
/// separators escaped, so a file's name cannot add a line to the report.
pub fn shown(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            c if c.is_control() || c == '\u{2028}' || c == '\u{2029}' => {
                c.escape_default().to_string()
            }
            c => c.to_string(),
        })
        .collect()
}

fn subject_json(x: &Subject) -> String {
    let opt = |o: Option<String>| o.unwrap_or_else(|| "null".into());
    let (proof, stage, code, reason) = match &x.outcome {
        Outcome::Verified(Proof::Stark) => (q("stark"), None, None, None),
        Outcome::Verified(Proof::PathOnly) => (q("path-only"), None, None, None),
        Outcome::Failed(f) => {
            ("null".into(), Some(q(f.stage)), Some(f.code.to_string()), Some(q(&f.reason)))
        }
        Outcome::CouldNotCheck(w) => ("null".into(), None, None, Some(q(w))),
    };
    format!(
        "{{\"trailer\":{},\"image\":{},\"kind\":{},\"slot\":{},\"measurement\":{},\"caps\":{},\
         \"verdict\":{},\"proof\":{proof},\"stage\":{},\"code\":{},\"reason\":{}}}",
        q(&x.trailer),
        opt(x.image.as_deref().map(q)),
        opt(x.kind.map(|k| q(kind_name(k)))),
        opt(x.slot.map(|i| i.to_string())),
        opt(x.measurement.map(|m| q(&hex(&m)))),
        opt(x.caps.map(|c| q(&format!("{c:#018x}")))),
        q(x.outcome.verdict().name()),
        opt(stage),
        opt(code),
        opt(reason)
    )
}

/// A JSON string: quoted, with quotes, backslashes and every control
/// character escaped, so a file name cannot break the document.
pub fn q(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 || c == '\u{7f}' || c == '\u{2028}' || c == '\u{2029}' => {
                out.push_str(&format!("\\u{:04x}", c as u32))
            }
            c => out.push(c),
        }
    }
    out.push('"');
    out
}
