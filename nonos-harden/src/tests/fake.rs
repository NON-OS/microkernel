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

//! A stand-in for the tree's verifiers: it leaves the traces each real one
//! leaves (report, matrix summary, exit status), as each proof sets them.

pub use super::fake_report::report;
use crate::tree::runner::{Cmd, Done, Runner};
use serde_json::{json, Value};
use std::{cell::RefCell, fs, path::Path, rc::Rc};

pub struct Fake {
    /// The report a nonos-verify module writes; none: it writes no report.
    pub report: fn(&str) -> Option<Value>,
    pub step_rc: i64,
    pub inputs: i32,
    pub ran: Rc<RefCell<Vec<String>>>,
}

impl Default for Fake {
    fn default() -> Self {
        Self {
            report: |_| Some(report(&[("a check", "pass")], &[])),
            step_rc: 0,
            inputs: 0,
            ran: Rc::default(),
        }
    }
}

impl Runner for Fake {
    fn run(&self, cmd: &Cmd, log: &Path) -> Result<Done, String> {
        fs::write(log, "").unwrap();
        // A relative path would name another place from the verifier's own cwd.
        let rel = cmd.args.iter().find(|a| a.contains('/') && !a.starts_with('/'));
        assert!(cmd.cwd.is_absolute() && rel.is_none(), "relative: {:?} {rel:?}", cmd.cwd);
        let (tool, last) = (cmd.args[0].as_str(), cmd.args.last().unwrap().as_str());
        self.ran.borrow_mut().push(format!("{} {last}", cmd.program));
        let code = match (cmd.program.as_str(), tool) {
            ("cargo", _) => match (self.report)(last) {
                Some(r) => {
                    let dir = Path::new(&cmd.env[0].1).join(last);
                    fs::create_dir_all(&dir).unwrap();
                    fs::write(dir.join(format!("{last}-report.json")), r.to_string()).unwrap();
                    i32::from(r["checks"].as_array().unwrap().iter().any(|c| c["status"] == "fail"))
                }
                None => 1,
            },
            (_, t) if t.ends_with("nonos-verify-matrix") => {
                let dir = cmd.cwd.join("notes/evidence/nonos-harden");
                fs::create_dir_all(&dir).unwrap();
                let steps = json!([{"cmd": "cargo test --release", "rc": self.step_rc}]);
                let summary = json!({"rows": {last: {"pass": self.step_rc == 0, "steps": steps}}});
                fs::write(dir.join("summary.json"), summary.to_string()).unwrap();
                0
            }
            (_, t) if t.ends_with("inputs.py") => self.inputs,
            _ => return Err(format!("the stand-in does not know `{}`", cmd.program)),
        };
        Ok(Done { code: Some(code), tail: String::new() })
    }
}
