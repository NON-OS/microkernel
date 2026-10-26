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
//! `agentproof-receipt`: reads the two files named and prints what cli.rs
//! answers.

use std::process::ExitCode;

use agentproof_receipt::cli::{answer, Answer, USAGE};

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let a = match args.iter().map(String::as_str).collect::<Vec<_>>().as_slice() {
        ["verify", receipt, trust] => {
            match (std::fs::read(receipt), std::fs::read_to_string(trust)) {
                (Ok(r), Ok(t)) => answer(&r, &t),
                (Err(e), _) | (_, Err(e)) => {
                    Answer { code: 2, out: vec![], err: vec![format!("agentproof-receipt: {e}")] }
                }
            }
        }
        ["pin", receipt] => match std::fs::read(receipt) {
            Ok(r) => agentproof_receipt::cli_pin::pinned(&r),
            Err(e) => {
                Answer { code: 2, out: vec![], err: vec![format!("agentproof-receipt: {e}")] }
            }
        },
        _ => Answer { code: 2, out: vec![], err: vec![USAGE.into()] },
    };
    a.out.iter().for_each(|l| println!("{l}"));
    a.err.iter().for_each(|l| eprintln!("{l}"));
    ExitCode::from(a.code)
}
