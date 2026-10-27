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

//! What one paint reads: the model, the transport's word for itself, the
//! sizes at the current text scale and the theme's colours.

use super::metrics::Metrics;
use super::theme::{colors, Colors};
use crate::model::state::Model;

pub struct View<'a> {
    pub model: &'a Model,
    pub label: &'a str,
    pub m: Metrics,
    pub c: Colors,
}

impl<'a> View<'a> {
    pub fn new(model: &'a Model, label: &'a str) -> Self {
        View { model, label, m: Metrics::now(), c: colors() }
    }
}

/// A column of the window.
#[derive(Clone, Copy)]
pub struct Span {
    pub x: u32,
    pub w: u32,
}
