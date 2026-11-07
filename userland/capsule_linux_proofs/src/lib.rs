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

//! The shipped source, included and exercised.

extern crate alloc;

#[path = "../../capsule_linux/src/linux/wayland/args.rs"]
pub mod args;
#[path = "../../capsule_linux/src/linux/wayland/frame_len.rs"]
pub mod frame_len;
#[path = "../../capsule_linux/src/linux/wayland/shm_fit.rs"]
pub mod shm_fit;
#[path = "../../capsule_linux/src/linux/guest/memfd_spans.rs"]
pub mod memfd_spans;
#[path = "../../capsule_linux/src/linux/wayland/opaque.rs"]
pub mod opaque;
#[path = "../../capsule_linux/src/linux/wayland/object.rs"]
pub mod wayland_object;
#[path = "../../capsule_linux/src/linux/wayland/wire.rs"]
pub mod wire;
/* A key as the Linux key it came from, and the modifiers it carries. */
#[path = "../../capsule_linux/src/linux/wayland/evdev.rs"]
pub mod evdev;
#[path = "../../capsule_linux/src/linux/wayland/keymap_extra.rs"]
pub mod keymap_extra;
#[path = "../../capsule_linux/src/linux/wayland/evdev_keys.rs"]
pub mod evdev_keys;
#[path = "../../capsule_linux/src/linux/wayland/held_keys.rs"]
pub mod held_keys;
#[path = "../../capsule_linux/src/linux/wayland/pointer_codes.rs"]
pub mod pointer_codes;
#[path = "../../capsule_linux/src/linux/wayland/present/pinned/rule.rs"]
pub mod wayland_pinned;
#[path = "../../capsule_linux/src/linux/wayland/region_cover.rs"]
pub mod wayland_region_cover;
#[path = "../../capsule_linux/src/linux/wayland/focus_step.rs"]
pub mod wayland_focus_step;
#[path = "../../capsule_linux/src/linux/wayland/input/cursor_code.rs"]
pub mod wayland_cursor_code;
#[path = "../../capsule_linux/src/linux/wayland/toplevel/size.rs"]
pub mod wayland_toplevel_size;
#[path = "../../capsule_linux/src/linux/wayland/swap_copy.rs"]
pub mod swap_copy;
#[path = "../../capsule_linux/src/linux/wayland/frame_cost.rs"]
pub mod frame_cost;
#[path = "../../capsule_linux/src/linux/wayland/present/crop.rs"]
pub mod geometry_crop;
#[path = "../../capsule_linux/src/linux/unix/ready_rule.rs"]
pub mod display_ready;
#[path = "../../capsule_linux/src/linux/unix/give_rule.rs"]
pub mod display_give;
#[path = "../../capsule_linux/src/linux/unix/path_rule.rs"]
pub mod display_path;
#[path = "../../capsule_linux/src/linux/net/poll/answer.rs"]
pub mod poll_answer;
#[path = "../../capsule_linux/src/linux/wayland/kde_mode.rs"]
pub mod kde_mode;
#[path = "../../capsule_linux/src/linux/wayland/damage.rs"]
pub mod damage;
#[path = "../../capsule_linux/src/linux/wayland/damage_box.rs"]
pub mod damage_box;
#[path = "../../capsule_linux/src/linux/wayland/damage_boxes.rs"]
pub mod damage_boxes;
/// What any boxes a client names must leave, for the tests and the fuzz target.
pub mod damage_boxes_check;
#[path = "../../capsule_linux/src/linux/guest/mem_rows.rs"]
pub mod mem_rows;
#[path = "../../capsule_linux/src/linux/wayland/damage_lay.rs"]
pub mod damage_lay;
#[path = "../../capsule_linux/src/linux/wayland/damage_scale.rs"]
pub mod damage_scale;
#[path = "../../capsule_linux/src/linux/wayland/xkb_mods.rs"]
pub mod xkb_mods;
/* A guest window full screen and back, and taken down when it ends. */
#[path = "../../capsule_linux/src/linux/wayland/configure.rs"]
pub mod configure;
#[path = "../../capsule_linux/src/linux/wayland/fit/mod.rs"]
pub mod fit;
#[path = "../../capsule_linux/src/linux/wayland/ops.rs"]
pub mod ops;
#[path = "../../capsule_linux/src/linux/wayland/out.rs"]
pub mod out;
#[path = "../../capsule_linux/src/linux/wayland/reshape.rs"]
pub mod reshape;
#[path = "../../capsule_linux/src/linux/wayland/scene_pixels.rs"]
pub mod scene_pixels;
pub mod scene_pixels_map;
pub mod wayland_clip;
pub mod wayland_dnd;
pub mod wayland_popup;
pub mod wayland_sub;
pub mod wayland_frame;
#[path = "../../capsule_linux/src/linux/wayland/window_life/mod.rs"]
pub mod window_life;
#[path = "../../capsule_linux/src/linux/wayland/window_front/ids.rs"]
pub mod window_ids;
mod host_doubles;
pub use host_doubles::{clamp, private};
#[path = "../../../src/userspace/capsule_linux/family.rs"]
pub mod listing_family;

#[path = "../../capsule_linux/src/linux/file/family.rs"]
pub mod family;
/* What a refused SERVE_ONE listen is answered and said as. */
#[path = "../../capsule_linux/src/linux/net/serve_one/refusal.rs"]
pub mod serve_refusal;
/* Where a terminal name another family's tree holds is said to be. */
#[path = "../../capsule_linux/src/linux/terminal_launch/elsewhere.rs"]
pub mod terminal_elsewhere;
#[path = "../../capsule_linux/src/linux/file/owner/chown_rule.rs"]
pub mod chown_rule;
/* A program's pin, hashed a window at a time (`hash_windows_tests`). */
#[path = "../../capsule_linux/src/linux/file/hash_windows.rs"]
pub mod hash_windows;
#[path = "../../capsule_linux/src/linux/file/dev_metrics_parse.rs"]
pub mod metrics_parse;
#[path = "../../capsule_linux/src/linux/file/models/name.rs"]
pub mod model_name;
#[path = "../../capsule_linux/src/linux/file/resolve.rs"]
pub mod resolve;
#[path = "../../capsule_linux/src/linux/file/root.rs"]
pub mod root;
#[path = "../../capsule_linux/src/linux/file/root_write.rs"]
pub mod root_write;
#[path = "../../capsule_linux/src/linux/file/models/stat_size.rs"]
pub mod stat_size;
/* What the `linux` command says when the store has nothing (`store_why_tests`). */
#[path = "../../capsule_linux/src/linux/file/dir_children.rs"]
pub mod dir_children;
#[path = "../../capsule_linux/src/linux/file/dirent.rs"]
pub mod dirent;
#[path = "../../capsule_linux/src/linux/file/meta/statbuf/mod.rs"]
pub mod statbuf;
#[path = "../../capsule_linux/src/linux/store_why.rs"]
pub mod store_why;

#[path = "../../capsule_linux/src/linux/net/host_body.rs"]
pub mod host_body;

/*
 * net.sockets' own reader for a connect-by-host body, mounted at the crate
 * paths it names, so the capsule's encoder is held to the real parser.
 */
#[path = "../../capsule_net_sockets/src/protocol/errno.rs"]
pub mod protocol;
pub mod server;

#[path = "../../capsule_linux/src/linux/net/route.rs"]
pub mod route;

/* nonos_route_link's own files, at the crate root (route_link_files.rs). */
include!("route_link_files.rs");

/*
 * net.anon's own operations, statuses and END reasons, which the capsule's
 * client of its handle front is held to.
 */
#[path = "../../capsule_net_anon/src/stream/end.rs"]
pub mod anon_server_end;
#[path = "../../capsule_net_anon/src/protocol/errno.rs"]
pub mod anon_server_errno;
#[path = "../../capsule_net_anon/src/protocol/ops.rs"]
pub mod anon_server_ops;

#[path = "../../capsule_linux/src/linux/applets.rs"]
pub mod applets;
#[path = "../../capsule_linux/src/linux/file/private/rule.rs"]
pub mod private_rule;

/* The box a guest runs in: its policy, its identity, what it allows. */
pub mod app;

/* `linux --box`: its presets, what it refused, and its report. */
pub mod boxed;

pub mod calls;
pub mod console;
/* The call rules, at the crate paths they name in the capsule. */
pub mod linux;
pub mod models;
pub use calls::{exec_shebang, sigframe, sigframe_build, sigframe_read, sigtimer};

#[cfg(any(test, fuzzing))]
pub mod image;

/// The installer's parsers, which read bytes fetched off a network.
pub mod install;

/// The sealed diagnostic chain's link.
#[path = "../../capsule_linux/src/linux/serve/trace_link.rs"]
pub mod trace_link;

#[cfg(test)]
mod tests;

/// A family's home, seeded once: the rule over any place to write it.
#[path = "../../capsule_linux/src/linux/file/private/seed_rule.rs"]
pub mod seed_rule;

// GTK's caches: the image's precomputed member, or this run's /tmp.
#[path = "../../capsule_linux/src/linux/gtk_prep/shipped.rs"]
pub mod gtk_shipped;
#[cfg(test)]
mod gtk_shipped_tests;
