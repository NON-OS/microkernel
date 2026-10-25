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

//! The command line: arguments to a report, its rendering and the exit code.
//! The binary and the browser build both call `cli`, so a page given the
//! same arguments and files answers exactly as the command does. Exit 0 only
//! for verified, 1 for failed, 2 for could-not-check (a usage error included).

use crate::report::{Report, Verdict};
use crate::run::{run_with, Mode, Reader, Request};

pub const HELP: &str = "\
nonos-proof-verify: check a NONOS attestation trailer as the gates check it

usage:
  nonos-proof-verify capsule    --root ROOT --image ELF --caps HEX TRAILER...
  nonos-proof-verify kernel     --root ROOT --image IMAGE TRAILER...
  nonos-proof-verify bootloader --root ROOT --image EFI TRAILER...
  nonos-proof-verify transcript TRANSCRIPT [--root ROOT] TRAILER...

  capsule      the spawn gate's check: the ELF's BLAKE3, the capability word
               granted (hex), the policy epoch
  kernel       the loader's check of the kernel: the image's BLAKE3, the boot
               epoch
  bootloader   the kernel's check of its loader: the PE Authenticode SHA-256
               the firmware measures, the boot epoch
  transcript   rebuild the root from a policy transcript (the .transcript the
               enroll tool writes beside every root), then check each trailer
               against the slot its path opens, with no image needed

options:
  --root ROOT  the 32-byte policy root file to check against; in transcript
               mode, the transcript must be for this root
  --json       print the v1 JSON report instead of text
  --allow-dev  also accept a development trailer (the path alone, no STARK);
               it is reported as such and never as a release
  -h, --help   this text (as the command, alone)
  -V, --version

A trailer passes only when its path folds to the root for the measurement and
its STARK proof verifies for the same slot, with the gates' own crates. The
report names the root, and says \"development root, not a release\" for any
root other than the release roots committed in nonos-data/trust/policy.

exit status:
  0  verified: every trailer passed
  1  failed: a trailer, or the transcript, was refused
  2  could not check: a file unreadable, an image unmeasurable, a usage
     error; nothing was verified
";

/// What a run prints and exits with.
pub struct Output {
    pub code: i32,
    pub stdout: String,
    pub stderr: String,
}

/// Run the command line `args` (without the program name), with files from
/// `read`.
pub fn cli(args: &[String], read: Reader) -> Output {
    // Options end at "--": past it, "--json" is a file's name.
    let opts = &args[..args.iter().position(|a| a == "--").unwrap_or(args.len())];
    let json = opts.iter().any(|a| a == "--json");
    let out = |code, stdout: String| Output { code, stdout, stderr: String::new() };
    // Help and the version are commands, taken in the command's place alone:
    // anywhere else they are an unknown option or a file's name, never an
    // exit 0 for a run that verified nothing.
    match args.first().map(String::as_str) {
        Some("-h" | "--help") => return out(0, HELP.into()),
        Some("-V" | "--version") => {
            return out(0, format!("nonos-proof-verify {}\n", env!("CARGO_PKG_VERSION")))
        }
        _ => {}
    }
    let (report, stderr) = match parse(args) {
        Ok(req) => (run_with(&req, read), String::new()),
        Err(e) => {
            let msg = format!("nonos-proof-verify: {e} (try --help)\n");
            (Report { refusal: Some((Verdict::CouldNotCheck, e)), ..Report::default() }, msg)
        }
    };
    let stdout = if json { report.json() } else { report.text() };
    Output { code: report.verdict().exit_code(), stdout, stderr }
}

/// The arguments as a request, or why they are not one.
pub fn parse(args: &[String]) -> Result<Request, String> {
    let (verb, rest) = args.split_first().ok_or("no command given")?;
    let mut root = None;
    let mut image = None;
    let mut caps = None;
    let mut allow_dev = false;
    let mut free = Vec::new();
    let mut it = rest.iter();
    while let Some(a) = it.next() {
        let mut value = |name: &str| it.next().cloned().ok_or(format!("{name} needs a value"));
        match a.as_str() {
            "--root" => set(&mut root, "--root", value("--root")?)?,
            "--image" => set(&mut image, "--image", value("--image")?)?,
            "--caps" => set(&mut caps, "--caps", value("--caps")?)?,
            "--allow-dev" => allow_dev = true,
            "--json" => {}
            "--" => free.extend(it.by_ref().cloned()),
            o if o.starts_with('-') && o.len() > 1 => return Err(format!("unknown option {o}")),
            _ => free.push(a.clone()),
        }
    }
    let need = |o: Option<String>, name: &str| o.ok_or(format!("{verb} needs {name}"));
    let refuse = |o: &Option<String>, name: &str| match o {
        Some(_) => Err(format!("{verb} takes no {name}")),
        None => Ok(()),
    };
    let mode = match verb.as_str() {
        "capsule" => Mode::Capsule {
            image: need(image, "--image")?,
            caps: parse_caps(&need(caps, "--caps")?)?,
        },
        "kernel" | "bootloader" => {
            refuse(&caps, "--caps")?;
            let image = need(image, "--image")?;
            if verb == "kernel" {
                Mode::Kernel { image }
            } else {
                Mode::Bootloader { image }
            }
        }
        "transcript" => {
            refuse(&image, "--image")?;
            refuse(&caps, "--caps")?;
            if free.is_empty() {
                return Err("transcript needs the transcript file".into());
            }
            Mode::Transcript { file: free.remove(0) }
        }
        other => return Err(format!("unknown command {other}")),
    };
    if !matches!(mode, Mode::Transcript { .. }) && root.is_none() {
        return Err(format!("{verb} needs --root"));
    }
    if free.is_empty() {
        return Err("no trailer given".into());
    }
    Ok(Request { mode, root, trailers: free, allow_dev })
}

fn set(slot: &mut Option<String>, name: &str, v: String) -> Result<(), String> {
    match slot.replace(v) {
        Some(_) => Err(format!("{name} given twice")),
        None => Ok(()),
    }
}

/// A capability word in hex, `0x` optional, at most 16 digits.
fn parse_caps(s: &str) -> Result<u64, String> {
    let h = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")).unwrap_or(s);
    if h.is_empty() || h.len() > 16 || !h.bytes().all(|b| b.is_ascii_hexdigit()) {
        return Err(format!("--caps {s} is not a hex capability word"));
    }
    u64::from_str_radix(h, 16).map_err(|_| format!("--caps {s} is not a hex capability word"))
}
