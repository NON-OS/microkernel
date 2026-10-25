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

//! The apps and tools this kernel image's own spawn sites declare,
//! with their namespaces, reserved as baked_names.rs says.

pub(crate) const APPS: &[(&str, &str)] = &[
    ("app.about", "systems.nonos.app.about"),
    ("app.audio_player", "systems.nonos.app.audio_player"),
    ("app.boot_splash", "systems.nonos.app.boot_splash"),
    ("app.browser", "systems.nonos.app.browser"),
    ("app.calculator", "systems.nonos.app.calculator"),
    ("app.clock", "systems.nonos.app.clock"),
    ("app.file_manager", "systems.nonos.app.file_manager"),
    ("app.hello", "systems.nonos.app.hello"),
    ("app.image_viewer", "systems.nonos.image_viewer"),
    ("app.input_probe", "systems.nonos.app.input_probe"),
    ("app.input_proof", "systems.nonos.app.input_proof"),
    ("app.install", "systems.nonos.app.install"),
    ("app.linux", "systems.nonos.app.linux"),
    ("app.nonos_install", "systems.nonos.app.nonos_install"),
    ("app.nonos_wallet", "systems.nonos.app.nonos_wallet"),
    ("app.process_manager", "systems.nonos.app.process_manager"),
    ("app.prove.job", ""),
    ("app.settings", "systems.nonos.app.settings"),
    ("app.setup_wizard", "systems.nonos.app.setup_wizard"),
    ("app.snake", "systems.nonos.app.snake"),
    ("app.store", "systems.nonos.app.store"),
    ("app.terminal", "systems.nonos.app.terminal"),
    ("app.text_editor", "systems.nonos.app.text_editor"),
    ("app.video_player", "systems.nonos.app.video_player"),
    ("tool.csview", "systems.nonos.tool.csview"),
    ("tool.dotenv-linter", "systems.nonos.tool.dotenv-linter"),
    ("tool.grex", "systems.nonos.tool.grex"),
    ("tool.huniq", "systems.nonos.tool.huniq"),
    ("tool.install", "systems.nonos.tool.install"),
    ("tool.jsonxf", "systems.nonos.tool.jsonxf"),
    ("tool.linux", ""),
    ("tool.model-fetch", "systems.nonos.tool.model-fetch"),
    ("tool.pastel", "systems.nonos.tool.pastel"),
    ("tool.qwen", ""),
    ("tool.ripgrep", "systems.nonos.tool.ripgrep"),
    ("tool.sd", "systems.nonos.tool.sd"),
    ("tool.tokei", "systems.nonos.tool.tokei"),
];
