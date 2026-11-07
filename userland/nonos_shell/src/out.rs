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

//! The one environment coupling the shell language exposes: where a command's
//! output lines go.
//!
//! A builtin that prints does not care whether its bytes land in a GUI
//! scrollback ring or go out over an `nsh` session stream. It writes whole
//! lines to a [`ShellOut`]. The GUI terminal implements this over its
//! `command::output::Output` (which pushes into the scrollback); a headless
//! session implements it over its byte stream.
//!
//! Each call is one line; the sink appends the line terminator its medium
//! needs, exactly as the capsule's scrollback `push_line` does today. This
//! mirrors the capsule's existing `Output::writeln(&[u8])` one-to-one, so a
//! builtin migrated from `&mut Output` to `&mut dyn ShellOut` prints the same
//! bytes in the same order.
pub trait ShellOut {
    /// Append one output line (without a trailing newline; the sink supplies
    /// whatever its medium terminates a line with).
    fn writeln(&mut self, line: &[u8]);

    /// Whether the sink takes no more lines, so a caller running a line
    /// should stop it. A sink without a bound is never full.
    fn full(&self) -> bool {
        false
    }
}

/// A `ShellOut` that drops every line. Useful to a headless caller that wants a
/// command's status without keeping its text, and to tests.
pub struct Sink;

impl ShellOut for Sink {
    #[inline]
    fn writeln(&mut self, _line: &[u8]) {}
}

impl<T: ShellOut + ?Sized> ShellOut for &mut T {
    #[inline]
    fn writeln(&mut self, line: &[u8]) {
        (**self).writeln(line)
    }

    #[inline]
    fn full(&self) -> bool {
        (**self).full()
    }
}

/// The most bytes a [`Crlf`] gathers for one line's run.
pub const CRLF_MAX: usize = 1024 * 1024;

/// Said once, after the last line that fit.
pub const CRLF_FULL: &[u8] = b"shell: output stopped at 1 MiB\r\n";

/// A `ShellOut` that gathers lines into one byte string, each ended with
/// `\r\n`, as a remote terminal reading a session stream expects. It holds
/// at most [`CRLF_MAX`] bytes of lines and then [`CRLF_FULL`]; a line that
/// would pass the bound is dropped with every line after it, and the sink
/// reports itself full so the session stops the line.
#[derive(Default)]
pub struct Crlf(pub alloc::vec::Vec<u8>, bool);

impl ShellOut for Crlf {
    fn writeln(&mut self, line: &[u8]) {
        if self.1 {
            return;
        }
        let fits = self.0.len().checked_add(line.len()).and_then(|n| n.checked_add(2));
        if fits.is_none_or(|n| n > CRLF_MAX) {
            self.0.extend_from_slice(CRLF_FULL);
            self.1 = true;
            return;
        }
        self.0.extend_from_slice(line);
        self.0.extend_from_slice(b"\r\n");
    }

    fn full(&self) -> bool {
        self.1
    }
}
