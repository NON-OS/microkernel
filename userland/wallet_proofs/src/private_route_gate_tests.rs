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

/*
 * A source gate: no code in the wallet reaches net.sockets or net.dns, and
 * its one stream opening asks private_route first.
 */

/* Every source line of the wallet that is code, not comment. */
fn wallet_code() -> Vec<(String, String)> {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../capsule_wallet_nonos/src");
    let mut out = Vec::new();
    let mut dirs = vec![root];
    while let Some(dir) = dirs.pop() {
        for entry in std::fs::read_dir(&dir).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                dirs.push(path);
            } else if path.extension().is_some_and(|e| e == "rs") {
                let text = std::fs::read_to_string(&path).unwrap();
                let code = text.lines().filter(|l| !l.trim_start().starts_with(['/', '*']));
                out.extend(code.map(|l| (path.display().to_string(), l.to_string())));
            }
        }
    }
    out
}

#[test]
fn no_code_in_the_wallet_reaches_net_sockets_or_net_dns() {
    let code = wallet_code();
    assert!(code.len() > 10_000, "the wallet's sources were read");
    for word in ["net.sockets", "net.dns", "Route::chosen", "TcpStream", "RouteStream::connect"] {
        let hit = code.iter().find(|(_, l)| l.contains(word));
        assert!(hit.is_none(), "{word} in {hit:?}");
    }
    /* The one opening of a stream, and it asks the gate first. */
    let opens: Vec<_> = code.iter().filter(|(_, l)| l.contains("RouteOpening::begin")).collect();
    assert_eq!(opens.len(), 1, "{opens:?}");
    assert!(opens[0].0.ends_with("net/step/open.rs"));
    let gated =
        code.iter().any(|(f, l)| f.ends_with("net/step/open.rs") && l.contains("refused(route)"));
    assert!(gated, "the opening is gated by private_route::refused");
}
