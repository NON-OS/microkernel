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

use alloc::string::String;

mod entries;

pub(super) use entries::split_top;

/* Pick the source to load from an @font-face src list: the first url()
 * entry in a format the engine reads, as CSS Fonts 4 takes the first
 * usable one. local() faces, EOT, SVG fonts and tech() needs beyond
 * variations are passed over. Without a format() hint the extension or
 * the data: type decides, and an unknown extension is fetched and its
 * bytes sniffed on arrival. */
pub(super) fn pick_src(src: &str) -> Option<String> {
    split_top(src, b',').into_iter().find_map(|entry| {
        let url = unquote(inner(entry, "url(")?);
        let usable = match inner(entry, "format(") {
            Some(f) => loadable(&unquote(f).to_ascii_lowercase()),
            None => sniffable(url),
        };
        let tech_ok =
            inner(entry, "tech(").is_none_or(|t| unquote(t).eq_ignore_ascii_case("variations"));
        (usable && tech_ok).then(|| String::from(url))
    })
}

/// format() hints of sfnt outlines, bare or in WOFF or WOFF2.
fn loadable(format: &str) -> bool {
    matches!(format.trim_end_matches("-variations"), "truetype" | "opentype" | "woff" | "woff2")
}

fn unquote(s: &str) -> &str {
    s.trim().trim_matches('"').trim_matches('\'')
}

/// The text between `name` (ending in an open paren) and the next `)`.
fn inner<'a>(entry: &'a str, name: &str) -> Option<&'a str> {
    let at = entry.to_ascii_lowercase().find(name)? + name.len();
    let end = at + entry[at..].find(')')?;
    Some(&entry[at..end])
}

fn sniffable(url: &str) -> bool {
    let lower = url.to_ascii_lowercase();
    if let Some(data) = lower.strip_prefix("data:") {
        let head = data.split_once(',').map_or(data, |(h, _)| h);
        let fontish =
            ["font", "truetype", "opentype", "octet-stream"].iter().any(|k| head.contains(k));
        return fontish && !head.contains("svg");
    }
    let path = &lower[..lower.find(['?', '#']).unwrap_or(lower.len())];
    ![".eot", ".svg", ".svgz"].iter().any(|ext| path.ends_with(ext))
}
