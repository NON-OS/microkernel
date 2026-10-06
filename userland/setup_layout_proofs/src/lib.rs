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

//! Host proofs for first-boot setup's and the installer's layouts. The brand
//! crate's scale and text files, the compositor's canvas rule and each
//! layout are the real sources, included by #[path]. The layouts name the
//! brand as `nonos_brand::`, so this crate answers to that name too, with the
//! brand's files at its root where their `super::` paths expect them.

extern crate alloc;
extern crate self as nonos_brand;

// The brand: its scale, palette, faces and the text measures built on them.
#[path = "../../capsule_install/brand/src/scale.rs"]
pub mod scale;

#[path = "../../capsule_install/brand/src/palette.rs"]
pub mod palette;

#[path = "../../capsule_install/brand/src/faces.rs"]
mod faces;

#[path = "../../capsule_install/brand/src/type_set.rs"]
mod type_set;

#[path = "../../capsule_install/brand/src/labels.rs"]
mod labels;

#[path = "../../capsule_install/brand/src/release.rs"]
mod release;

pub use faces::Face;
pub use labels::{label, label_w, marker};
pub use release::release;
pub use type_set::{line_h, measure, text};

// The compositor's rule for which screens get a half-size canvas.
#[path = "../../compositor/src/setup/canvas_scale.rs"]
pub mod canvas_scale;

// The kernel boot console's copy of the same rule.
#[cfg(test)]
#[path = "../../../src/kernel_core/init/framebuffer/hidpi.rs"]
pub mod kernel_hidpi;

// First-boot setup's layout, its step names and the Qwen tier names.
#[path = "../../capsule_setup_wizard/src/render/layout.rs"]
pub mod wizard_layout;

#[path = "../../capsule_setup_wizard/src/render/theme.rs"]
pub mod wizard_theme;

#[path = "../../capsule_setup_wizard/src/qwen/labels.rs"]
pub mod qwen_labels;

// The Qwen step's offer and starting row, over the real pins; `need` sits
// beside it, as in setup, so `super::need` is the fetcher's rule.
#[path = "../../capsule_setup_wizard/src/qwen/default.rs"]
pub mod qwen_default;
#[path = "../../capsule_model_fetch/src/need.rs"]
pub mod need;
#[path = "../../capsule_model_fetch/src/default_tier.rs"]
pub mod default_tier;
#[path = "../../capsule_model_fetch/src/pins/mod.rs"]
pub mod qwen_pins;

// The answers the settings service refused, as the review names them.
#[path = "../../capsule_setup_wizard/src/render/screens/unsaved.rs"]
pub mod unsaved;

// When setup writes the keyboard layout: as the keyboard step is confirmed.
#[path = "../../capsule_setup_wizard/src/render/screens/keyboard_live.rs"]
pub mod keyboard_live;

// The installer's sizes, full-screen layout, text placement and wrap.
pub mod installer;

#[cfg(test)]
mod displays;
#[cfg(test)]
mod glyph_edge_tests;
#[cfg(test)]
mod hidpi_tests;
#[cfg(test)]
mod installer_fit_tests;
#[cfg(test)]
mod installer_layout_tests;
#[cfg(test)]
mod installer_rescan_tests;
#[cfg(test)]
mod keyboard_live_tests;
#[cfg(test)]
mod literals;
#[cfg(test)]
mod proofs_subtitle_tests;
#[cfg(test)]
mod qwen_default_tests;
#[cfg(test)]
mod scale_tests;
#[cfg(test)]
mod unsaved_tests;
#[cfg(test)]
mod wizard_fit_tests;
#[cfg(test)]
mod wizard_layout_tests;
