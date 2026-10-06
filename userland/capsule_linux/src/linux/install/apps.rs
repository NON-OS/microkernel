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
 * download: one per Qwen tier. The tables are part of the signed
 * personality, so the program, its arguments and the model files it needs
 * are covered by the personality's measurement; the market only says which
 * one a person chose. A tier runs the chat window on its model; a model in
 * parts is opened by its first, and the program finds the rest by their
 * split names. The tables sit beside this file, one a family.
 */

use super::apps_coder::CODER;
use super::apps_qwen25::QWEN25;
use super::apps_qwen3::QWEN3;

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

pub const CHAT: &[u8] = b"/bin/qwenchat";

/*
 * The tier the release stick carries, imported from the stick with no
 * network when this boot's disk carries it; the fetcher's need.rs names
 * the same one, and model_fetch_proofs holds the two equal.
 */
pub const STICK_TIER: &str = "qwen3-0.6b";

/*
 * The namespace every shipped tier's name is in; the market lists each as
 * `linux.qwen-<tier>`, and its catalogue lists no package inside it.
 */
pub const QWEN: &str = "qwen-";

/* What an install names, told apart before any package index is read. */
pub enum Wanted {
    /*
     * A shipped tier: its program from the image, its model a dependency
     * the model fetcher brings, never a package.
     */
    Tier(&'static App),
    /* A name in the tiers' namespace that is no shipped tier: refused by name. */
    UnknownTier,
    /* Anything else: a package of the family's index. */
    Package,
}

pub fn wanted(name: &str) -> Wanted {
    match app(name) {
        Some(app) => Wanted::Tier(app),
        None if name.starts_with(QWEN) => Wanted::UnknownTier,
        None => Wanted::Package,
    }
}

impl App {
    /*
     * The tier of the signed model catalogue its model files are fetched
     * as: its name past the namespace, the tier each file is pinned under.
     */
    pub fn tier(&self) -> &'static str {
        self.name.strip_prefix(QWEN).unwrap_or(self.name)
    }
}

/*
 * The tier named `name`, running the chat window on the model whose first
 * file is `first`, needing every file listed.
 */
macro_rules! tier {
    ($name:literal, $first:literal $(, $rest:literal)* $(,)?) => {
        super::apps::App {
            name: $name,
            program: super::apps::CHAT,
            args: &[b"-ui", b"window", b"-m", concat!("/models", $first).as_bytes()],
            models: &[$first.as_bytes() $(, $rest.as_bytes())*],
        }
    };
}
pub(super) use tier;

/*
 * Every table, smallest family first.
 */
pub const FAMILIES: &[&[App]] = &[QWEN25, QWEN3, CODER];

/*
 * Every shipped tier, family by family.
 */
pub fn all() -> impl Iterator<Item = &'static App> {
    FAMILIES.iter().flat_map(|f| f.iter())
}

pub fn app(name: &str) -> Option<&'static App> {
    all().find(|a| a.name == name)
}
