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

//! A build may compile in a mirror on the local network, dialled past
//! nonos_route_link, only with the dev-mirror feature; NONOS_ALPINE_MIRROR
//! is refused in every build, since Alpine's mirror is compiled in.

use alloc::string::String;

use super::fence::mirror_fence::refused;

fn only(var: &'static str, value: &'static str) -> impl Fn(&str) -> Option<String> {
    move |v| (v == var).then(|| String::from(value))
}

#[test]
fn no_setting_is_any_build_s_to_have() {
    assert_eq!(refused(|_| None, false), None);
    assert_eq!(refused(only("NONOS_ALPINE_MIRROR", ""), false), None, "set empty is unset");
}

#[test]
fn alpine_s_mirror_is_set_by_no_build() {
    for at in ["10.0.2.2:8080", "dl-cdn.alpinelinux.org:80", "mirror.example.org"] {
        for dev in [false, true] {
            assert!(refused(only("NONOS_ALPINE_MIRROR", at), dev).is_some(), "{at}, dev {dev}");
        }
    }
}

#[test]
fn another_mirror_on_the_local_network_needs_dev_mirror() {
    for var in ["NONOS_DEB_MIRROR", "NONOS_PACMAN_MIRROR", "NONOS_PACKAGE_MIRROR"] {
        for at in ["10.0.2.2:8080", "192.168.1.5", "172.16.0.1:80", "169.254.1.1:80"] {
            assert!(refused(only(var, at), false).is_some(), "{var}={at}");
            assert_eq!(refused(only(var, at), true), None, "{var}={at}");
        }
        for at in ["deb.example.org:80", "8.8.8.8:80", "mirror.nonos.systems"] {
            let why = "a named mirror goes over the route layer";
            assert_eq!(refused(only(var, at), false), None, "{var}={at}: {why}");
        }
    }
}
