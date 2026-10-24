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

//! `nonos-proof-verify` for a browser tab. The page hands the module its
//! files by name, then runs the command line on them, and reads back what
//! the command would print and exit with:
//!
//! ```text
//!     npv_alloc(len) -> ptr                 a buffer the page fills
//!     npv_free(ptr, len)                    give it back
//!     npv_reset()                           forget every file and output
//!     npv_file(name, name_len, data, len) -> 0, or 100 for a name not UTF-8
//!                                           or a null buffer
//!     npv_run(args, args_len) -> exit code  the arguments, NUL-separated,
//!                                           without the program name:
//!                                           0 verified, 1 failed, 2 could
//!                                           not check
//!     npv_stdout(buf, len) -> full len      the report, as the command
//!     npv_stderr(buf, len) -> full len      prints it (--json for v1 JSON)
//! ```
//!
//! A file the arguments name and the page did not hand over cannot be read,
//! exactly as a missing file on disk: the run says could-not-check.

use std::collections::BTreeMap;
use std::sync::Mutex;

use nonos_proof_verify::cli;

const BAD: u32 = 100;

struct State {
    files: BTreeMap<String, Vec<u8>>,
    stdout: String,
    stderr: String,
}

static STATE: Mutex<State> =
    Mutex::new(State { files: BTreeMap::new(), stdout: String::new(), stderr: String::new() });

fn state() -> std::sync::MutexGuard<'static, State> {
    STATE.lock().unwrap_or_else(|p| p.into_inner())
}

fn layout(len: usize) -> Option<std::alloc::Layout> {
    std::alloc::Layout::from_size_align(len, 8).ok()
}

/// A buffer of `len` bytes, or null when `len` is 0 or too large.
#[no_mangle]
pub extern "C" fn npv_alloc(len: usize) -> *mut u8 {
    match layout(len) {
        // SAFETY: a nonzero size and a valid layout.
        Some(l) if len != 0 => unsafe { std::alloc::alloc(l) },
        _ => core::ptr::null_mut(),
    }
}

/// # Safety
/// `ptr` came from `npv_alloc(len)` with this `len` and is not used after.
#[no_mangle]
pub unsafe extern "C" fn npv_free(ptr: *mut u8, len: usize) {
    if let (false, Some(l)) = (ptr.is_null() || len == 0, layout(len)) {
        // SAFETY: the caller's contract: `ptr` is `npv_alloc(len)`'s.
        unsafe { std::alloc::dealloc(ptr, l) }
    }
}

#[no_mangle]
pub extern "C" fn npv_reset() {
    let mut s = state();
    s.files.clear();
    s.stdout.clear();
    s.stderr.clear();
}

/// # Safety
/// `ptr` is null with `len` 0, or points to `len` readable bytes.
unsafe fn bytes<'a>(ptr: *const u8, len: usize) -> Option<&'a [u8]> {
    match (ptr.is_null(), len) {
        (_, 0) => Some(&[]),
        (true, _) => None,
        // SAFETY: non-null, and the caller's contract gives `len` bytes.
        (false, _) => Some(unsafe { core::slice::from_raw_parts(ptr, len) }),
    }
}

/// # Safety
/// `name` points to `name_len` readable bytes and `data` to `len` (either
/// may be null with a length of 0). Both are copied.
#[no_mangle]
pub unsafe extern "C" fn npv_file(
    name: *const u8,
    name_len: usize,
    data: *const u8,
    len: usize,
) -> u32 {
    // SAFETY: the caller's contract, passed through.
    let (Some(n), Some(d)) = (unsafe { bytes(name, name_len) }, unsafe { bytes(data, len) }) else {
        return BAD;
    };
    let Ok(n) = core::str::from_utf8(n) else {
        return BAD;
    };
    state().files.insert(n.to_string(), d.to_vec());
    0
}

/// The command line over the files handed in so far.
pub fn run(args: &[String]) -> i32 {
    let files = std::mem::take(&mut state().files);
    let read = |name: &str| -> Result<Vec<u8>, String> {
        files.get(name).cloned().ok_or_else(|| format!("cannot read {name}: not given to the page"))
    };
    let out = cli(args, &read);
    let mut s = state();
    s.files = files;
    s.stdout = out.stdout;
    s.stderr = out.stderr;
    out.code
}

/// # Safety
/// `args` points to `len` readable bytes, or is null with `len` 0.
#[no_mangle]
pub unsafe extern "C" fn npv_run(args: *const u8, len: usize) -> i32 {
    // SAFETY: the caller's contract, passed through.
    let Some(b) = (unsafe { bytes(args, len) }) else {
        return 2;
    };
    let Ok(text) = core::str::from_utf8(b) else {
        return 2;
    };
    let args: Vec<String> =
        if text.is_empty() { Vec::new() } else { text.split('\0').map(String::from).collect() };
    run(&args)
}

fn copy_out(s: &str, buf: *mut u8, len: usize) -> usize {
    if !buf.is_null() {
        let n = s.len().min(len);
        // SAFETY: the caller's contract gives `len` writable bytes at `buf`.
        unsafe { core::ptr::copy_nonoverlapping(s.as_ptr(), buf, n) };
    }
    s.len()
}

/// # Safety
/// `buf` is null or points to `len` writable bytes. Copies at most `len`
/// bytes of the last run's report and returns its full length.
#[no_mangle]
pub unsafe extern "C" fn npv_stdout(buf: *mut u8, len: usize) -> usize {
    copy_out(&state().stdout, buf, len)
}

/// # Safety
/// As `npv_stdout`, for what the last run wrote to standard error.
#[no_mangle]
pub unsafe extern "C" fn npv_stderr(buf: *mut u8, len: usize) -> usize {
    copy_out(&state().stderr, buf, len)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn give(name: &str, data: &[u8]) {
        // SAFETY: both slices are live for the call.
        assert_eq!(unsafe { npv_file(name.as_ptr(), name.len(), data.as_ptr(), data.len()) }, 0);
    }

    fn fx(n: &str) -> Vec<u8> {
        std::fs::read(format!("{}/../tests/fixture/{n}", env!("CARGO_MANIFEST_DIR"))).unwrap()
    }

    fn argv(a: &str) -> i32 {
        // SAFETY: the slice is live for the call.
        unsafe { npv_run(a.as_ptr(), a.len()) }
    }

    fn stdout() -> String {
        // SAFETY: a null buffer asks the length; then `buf` has that room.
        let n = unsafe { npv_stdout(core::ptr::null_mut(), 0) };
        let mut buf = vec![0u8; n];
        unsafe { npv_stdout(buf.as_mut_ptr(), n) };
        String::from_utf8(buf).unwrap()
    }

    /// One test, as the module's state is one per instance.
    #[test]
    fn the_page_answers_as_the_command_does() {
        npv_reset();
        give("caps_root.bin", &fx("caps_root.bin"));
        give("a.elf", &fx("a.elf"));
        give("a.trailer", &fx("a.trailer"));
        let ok = [
            "capsule",
            "--root",
            "caps_root.bin",
            "--image",
            "a.elf",
            "--caps",
            "0x18",
            "a.trailer",
            "--json",
        ]
        .join("\0");
        let ok = ok.as_str();
        assert_eq!(argv(ok), 0);
        assert!(stdout().contains("\"verdict\":\"verified\""), "{}", stdout());
        // A changed capability bit fails; a file never handed over cannot be checked.
        assert_eq!(argv(&ok.replace("0x18", "0x19")), 1);
        assert_eq!(argv(&ok.replace("a.trailer", "b.trailer")), 2);
        let mut flipped = fx("a.trailer");
        flipped[5000] ^= 1;
        give("a.trailer", &flipped);
        assert_eq!(argv(ok), 1);
        assert_eq!(argv(""), 2);
        assert_eq!(argv("--help"), 0);
        npv_reset();
        assert_eq!(argv(ok), 2);
    }
}
