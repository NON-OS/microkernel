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
 * Programs this personality ships, installed by name with nothing to
 * download: one per Qwen tier. The table is part of the signed personality,
 * so the program, its arguments and the model files it needs are covered by
 * the personality's measurement; the market only says which one a person
 * chose. A tier runs the chat window on its model.
 */

pub struct App {
    pub name: &'static str,
    pub program: &'static [u8],
    pub args: &'static [&'static [u8]],
    /*
     * The model files it needs, by their names on the data volume; each is
     * pinned in file::models::pinned.
     */
    pub models: &'static [&'static [u8]],
}

const CHAT: &[u8] = b"/bin/qwenchat";

pub const APPS: &[App] = &[
    App {
        name: "qwen-small",
        program: CHAT,
        args: &[b"-ui", b"window", b"-m", b"/models/qwen2.5-0.5b-instruct-q4_k_m.gguf"],
        models: &[b"/qwen2.5-0.5b-instruct-q4_k_m.gguf"],
    },
    App {
        name: "qwen-medium",
        program: CHAT,
        args: &[b"-ui", b"window", b"-m", b"/models/qwen2.5-1.5b-instruct-q4_k_m.gguf"],
        models: &[b"/qwen2.5-1.5b-instruct-q4_k_m.gguf"],
    },
    App {
        name: "qwen-large",
        program: CHAT,
        args: &[b"-ui", b"window", b"-m", b"/models/qwen2.5-3b-instruct-q4_k_m.gguf"],
        models: &[b"/qwen2.5-3b-instruct-q4_k_m.gguf"],
    },
    App {
        name: "qwen-xlarge",
        program: CHAT,
        args: &[
            b"-ui",
            b"window",
            b"-m",
            b"/models/qwen2.5-7b-instruct-q4_k_m-00001-of-00002.gguf",
        ],
        models: &[
            b"/qwen2.5-7b-instruct-q4_k_m-00001-of-00002.gguf",
            b"/qwen2.5-7b-instruct-q4_k_m-00002-of-00002.gguf",
        ],
    },
];

pub fn app(name: &str) -> Option<&'static App> {
    APPS.iter().find(|a| a.name == name)
}
