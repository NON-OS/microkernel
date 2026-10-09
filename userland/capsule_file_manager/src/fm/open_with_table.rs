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

extern crate alloc;

use super::file_ext::ext;

/// An app a file can be handed to: the service the shell launches, the name
/// people know it by (the dock's), and what the status line says once the
/// shell took the request.
pub struct Handler {
    pub service: &'static str,
    pub name: &'static str,
    pub opened: &'static [u8],
}

const EDITOR: Handler =
    Handler { service: "app.text_editor", name: "Editor", opened: b"opened in Editor" };
const IMAGES: Handler = Handler {
    service: "app.image_viewer",
    name: "Image Viewer",
    opened: b"opened in Image Viewer",
};
const MUSIC: Handler =
    Handler { service: "app.audio_player", name: "Music", opened: b"opened in Music" };
const VIDEO: Handler =
    Handler { service: "app.video_player", name: "Video", opened: b"opened in Video" };

// Extension to handler, most-preferred first; Enter uses the first. Every
// handler names an app that is built and can read the format: Music decodes
// MP3 and WAV, Video Motion-JPEG AVI, the image viewer what its codec reads.
const TABLE: &[(&str, &[Handler])] = &[
    ("txt", &[EDITOR]),
    ("md", &[EDITOR]),
    ("log", &[EDITOR]),
    ("rs", &[EDITOR]),
    ("toml", &[EDITOR]),
    ("json", &[EDITOR]),
    ("html", &[EDITOR]),
    ("wav", &[MUSIC]),
    ("mp3", &[MUSIC]),
    ("avi", &[VIDEO]),
    ("png", &[IMAGES]),
    ("jpg", &[IMAGES]),
    ("jpeg", &[IMAGES]),
    ("bmp", &[IMAGES]),
    ("gif", &[IMAGES]),
];

pub fn handlers_for(path: &str) -> &'static [Handler] {
    let want = ext(path).to_ascii_lowercase();
    if want.is_empty() {
        return &[];
    }
    match TABLE.iter().find(|(e, _)| *e == want.as_str()) {
        Some((_, handlers)) => handlers,
        None => &[],
    }
}
