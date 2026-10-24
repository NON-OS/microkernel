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

//! The Debian and pacman lookups: a provider may meet a need, but the
//! package the person asked for, whose bytes the market's pin names, is
//! only ever the one of that name.

use alloc::string::String;
use alloc::vec;

use crate::install::{deb, pacman};

fn deb(name: &str, provides: &[&str]) -> deb::packages::Record {
    let provides = provides.iter().map(|p| String::from(*p)).collect();
    deb::packages::Record { name: name.into(), provides, ..Default::default() }
}

fn arch(name: &str, provides: &[&str]) -> (pacman::desc::Record, &'static str) {
    let provides = provides.iter().map(|p| String::from(*p)).collect();
    (pacman::desc::Record { name: name.into(), provides, ..Default::default() }, "extra")
}

#[test]
fn a_debian_listed_package_is_never_a_provider_of_its_name() {
    let index = vec![deb("vim-gtk3", &["vim"]), deb("vim", &[]), deb("only", &["ghost"])];
    let name = |r: Option<&deb::packages::Record>| r.map(|r| r.name.clone());
    use deb::find::find;
    assert_eq!(name(find(&index, "vim", false)), Some("vim".into()), "a need: its own name first");
    assert_eq!(name(find(&index, "vim", true)), Some("vim".into()));
    assert_eq!(name(find(&index, "ghost", false)), Some("only".into()), "a need: a provider");
    assert_eq!(name(find(&index, "ghost", true)), None, "listed: nothing is named ghost");
}

#[test]
fn a_pacman_listed_package_is_never_a_provider_of_its_name() {
    let db = vec![arch("gvim", &["vim"]), arch("only", &["ghost"]), arch("vim", &[])];
    let name = |r: Option<&(pacman::desc::Record, &str)>| r.map(|(r, _)| r.name.clone());
    use pacman::find::find;
    assert_eq!(name(find(&db, "vim", true)), Some("vim".into()));
    assert_eq!(name(find(&db, "vim", false)), Some("vim".into()));
    assert_eq!(name(find(&db, "ghost", false)), Some("only".into()), "a need: a provider");
    assert_eq!(name(find(&db, "ghost", true)), None, "listed: nothing is named ghost");
}
