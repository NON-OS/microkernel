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

//! Every kernel-mode fatal exception leaves through the panic path's `end`,
//! read from the handlers: none halts on its own any more, so none stops
//! the machine with RAM unwiped.

use std::path::Path;

fn sources(dir: &Path, out: &mut Vec<(String, String)>) {
    for e in std::fs::read_dir(dir).unwrap().flatten() {
        let p = e.path();
        if p.is_dir() {
            sources(&p, out);
        } else if p.extension().is_some_and(|x| x == "rs") {
            out.push((p.display().to_string(), std::fs::read_to_string(&p).unwrap()));
        }
    }
}

fn handlers() -> Vec<(String, String)> {
    let dir =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../src/interrupts/handlers/exceptions");
    let mut out = Vec::new();
    sources(&dir, &mut out);
    assert!(out.len() > 20, "the handlers were not found");
    out
}

#[test]
fn no_exception_handler_halts_on_its_own() {
    for (name, text) in handlers() {
        assert!(!text.contains("halt_loop"), "{name}");
    }
}

#[test]
fn each_fatal_kernel_branch_goes_to_the_panic_path() {
    let fatal: usize =
        handlers().iter().map(|(_, t)| t.matches("crate::boot::panic::fatal();").count()).sum();
    // AC BR DE OF UD GP NP TS VE PF SS DF MC, #DB, #MF, #NM twice.
    assert_eq!(fatal, 17);
    for (name, text) in handlers() {
        if text.contains("emit_fatal_notice") && !name.ends_with("mod.rs") {
            assert!(text.contains("crate::boot::panic::fatal();"), "{name}");
        }
    }
}
