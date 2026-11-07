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

//! The Linux personality's half, as it ships (capsule_linux file/user):
//! which guest paths are the person's folders after the personality's own
//! resolution, and a guest's open flags as the broker takes them.

use nonos_user_files::path::parts;

use super::reference::Reference;
use super::{fresh, APP};
use crate::doubles::client;
use crate::linux::file::resolve::visible;
use crate::linux::file::user::flags::of;
use crate::linux::file::user::place_rule::broker_path;
use crate::rng::Rng;

#[test]
fn only_the_two_folders_in_the_home_are_the_brokers() {
    let yes: [(&[u8], &[u8]); 8] = [
        (b"/root/Documents", b"Documents"),
        (b"/root/Downloads", b"Downloads"),
        (b"/root/Documents/a.odt", b"Documents/a.odt"),
        (b"/root/Downloads/d/e", b"Downloads/d/e"),
        (b"/root/Documents/.hidden", b"Documents/.hidden"),
        (b"/root/Pictures/2026/cat.png", b"Pictures/2026/cat.png"),
        (b"/root/Music", b"Music"),
        (b"/root/Videos/a.mkv", b"Videos/a.mkv"),
    ];
    for (path, want) in yes {
        assert_eq!(broker_path(path), Some(want));
    }
    let no: [&[u8]; 10] = [
        b"/root",
        b"/root/",
        b"/root/Documentsx",
        b"/root/documents",
        b"/rootDocuments",
        b"/home/Documents",
        b"/root/.config/Documents",
        b"/Documents",
        b"Documents",
        b"/root/Desktop",
    ];
    for path in no {
        assert_eq!(broker_path(path), None, "{}", String::from_utf8_lossy(path));
    }
}

#[test]
fn no_path_a_guest_names_reaches_past_the_folders() {
    let words: [&[u8]; 10] =
        [b"..", b".", b"", b"Documents", b"Pictures", b"Downloads", b"root", b"etc", b"x", b"..."];
    let cwds: [&[u8]; 4] = [b"/", b"/root", b"/root/Documents/a", b"/tmp"];
    let mut rng = Rng(77);
    for _ in 0..50_000 {
        let n = 1 + rng.below(7) as usize;
        let mut path: Vec<u8> = if rng.below(2) == 0 { b"/".to_vec() } else { Vec::new() };
        for i in 0..n {
            if i > 0 {
                path.push(b'/');
            }
            path.extend_from_slice(words[rng.below(words.len() as u64) as usize]);
        }
        let cwd = cwds[rng.below(cwds.len() as u64) as usize];
        let seen = visible(cwd, &path);
        let Some(asked) = broker_path(&seen) else { continue };
        assert!(parts(asked).is_ok(), "{:?} from {:?}: the broker refuses {:?}", path, cwd, asked);
        let top = asked.split(|b| *b == b'/').next().unwrap();
        assert!([&b"Documents"[..], b"Downloads", b"Music", b"Pictures", b"Videos"].contains(&top));
        let resolved_under = seen.starts_with(b"/root/") && seen[6..].starts_with(top);
        assert!(resolved_under, "{:?} resolved to {:?}", String::from_utf8_lossy(&path), seen);
    }
}

/// Linux's open flags, every mix a program can pass of the ones the
/// broker takes, opened by the broker through the personality's mapping
/// and by Linux, on a missing file, a file and a directory.
#[test]
fn every_open_a_guest_makes_answers_as_linux_does() {
    let (creat, excl, trunc, append, dir) = (0o100, 0o200, 0o1000, 0o2000, 0o200000);
    let (mut b, desk) = fresh();
    let linux = Reference::new(0x7a7a);
    std::fs::create_dir(linux.at("d")).unwrap();
    std::fs::write(linux.at("f"), b"body").unwrap();
    client(&mut b, &desk, APP).mkdir(b"Documents/d", 0o755).unwrap();
    let (h, _) = client(&mut b, &desk, APP).open(b"Documents/f", 2 | 4, 0o644).unwrap();
    client(&mut b, &desk, APP).write(h, 0, b"body").unwrap();
    client(&mut b, &desk, APP).close(h).unwrap();
    let mut n = 0;
    for access in [0u64, 1, 2] {
        for extra in 0..32u64 {
            let bits = [creat, excl, trunc, append, dir];
            let flags = access
                | bits
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| extra >> i & 1 == 1)
                    .fold(0, |a, (_, b)| a | b);
            // O_CREAT with O_DIRECTORY is EINVAL from 6.4 and something else before; O_TRUNC
            // without a write truncates on Linux and not here: neither is a program's call.
            if flags & creat != 0 && flags & dir != 0 || flags & trunc != 0 && access == 0 {
                continue;
            }
            for name in ["absent", "f", "d"] {
                let p = format!("Documents/{name}");
                let ours = client(&mut b, &desk, APP).open(p.as_bytes(), of(flags), 0o644);
                let theirs = linux.open_linux(name, flags);
                match (&ours, &theirs) {
                    (Ok(_), Ok(_)) => {}
                    (Err(a), Err(e)) => assert_eq!(a, e, "{name} {flags:#o}"),
                    _ => panic!(
                        "{name} {flags:#o}: broker {:?}, linux {:?}",
                        ours.map(drop),
                        theirs.map(drop)
                    ),
                }
                if let Ok((h, _)) = ours {
                    client(&mut b, &desk, APP).close(h).unwrap();
                }
                let _ = client(&mut b, &desk, APP).unlink(b"Documents/absent");
                let _ = std::fs::remove_file(linux.at("absent"));
                std::fs::write(linux.at("f"), b"body").unwrap();
                let (h, _) =
                    client(&mut b, &desk, APP).open(b"Documents/f", 2 | 4 | 16, 0o644).unwrap();
                client(&mut b, &desk, APP).write(h, 0, b"body").unwrap();
                client(&mut b, &desk, APP).close(h).unwrap();
                n += 1;
            }
        }
    }
    assert_eq!(n, 180, "opens compared");
}
