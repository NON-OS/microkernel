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

//! The local terminal. When standard input is a terminal, `nsh` puts it in
//! raw mode for the session, so every key reaches the remote shell, and
//! puts it back as it was when the session ends, however it ends. Ctrl-]
//! is kept back: it closes the session. The window size goes to the host
//! at login.

use std::io::IsTerminal;
use std::process::{Command, Stdio};
use std::string::{String, ToString};

/// Ctrl-]: close the session.
pub const CLOSE_KEY: u8 = 0x1D;

/// The terminal's settings before the session, put back on drop.
pub struct Raw(Option<String>);

fn stty(args: &[&str]) -> Option<String> {
    let out = Command::new("stty").args(args).stdin(Stdio::inherit()).output().ok()?;
    out.status.success().then(|| String::from_utf8_lossy(&out.stdout).trim().to_string())
}

impl Raw {
    /// Raw mode, if standard input is a terminal and `stty` can set it.
    pub fn enter() -> Raw {
        if !std::io::stdin().is_terminal() {
            return Raw(None);
        }
        let saved = stty(&["-g"]);
        match saved.as_deref().map(|_| stty(&["raw", "-echo"])) {
            Some(Some(_)) => Raw(saved),
            _ => Raw(None),
        }
    }

    pub fn is_raw(&self) -> bool {
        self.0.is_some()
    }
}

impl Drop for Raw {
    fn drop(&mut self) {
        if let Some(saved) = self.0.take() {
            let _ = stty(&[&saved]);
        }
    }
}

/// The terminal's columns and rows, if standard input is a terminal.
pub fn size() -> Option<(u16, u16)> {
    let s = stty(&["size"])?;
    let mut f = s.split_whitespace().map(|n| n.parse::<u16>().ok());
    let (rows, cols) = (f.next()??, f.next()??);
    Some((cols.clamp(1, 1000), rows.clamp(1, 1000)))
}
