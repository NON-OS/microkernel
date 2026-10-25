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

use super::system_names::{is_reserved_form, is_system_name};

#[test]
fn a_system_name_or_its_numbered_instance_is_refused_and_nothing_else() {
    for taken in ["input_router", "app.settings", "app.settings.1", "app.settings.12"] {
        assert!(is_system_name(taken), "{taken}");
    }
    for free in ["app.settings.", "app.settings.x", "app.settings.1a", "app.settingsx", "app"] {
        assert!(!is_system_name(free), "{free}");
    }
    assert!(!is_system_name("") && !is_system_name("com.example.app"));
}

#[test]
fn the_kernels_forms_and_roots_are_refused_and_a_named_reply_is_not() {
    let refused = [
        "endpoint.4098",
        "endpoint.4294967298",
        "endpoint.",
        "proc.12",
        "stdin.x",
        "driver.nvme1",
        "driver.bootfb0",
        "net.admin",
        "kernel.x",
    ];
    for name in refused {
        assert!(is_reserved_form(name), "{name}");
    }
    let allowed =
        ["endpoint.app.qrgen.reply", "endpoint.4098x", "app.qrgen", "com.example.x", "driverx"];
    for name in allowed {
        assert!(!is_reserved_form(name), "{name}");
    }
}
