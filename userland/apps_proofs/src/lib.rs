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

//! Host proofs for what the desktop apps say when a service is missing, a list
//! comes back empty or an input is too large. Each app keeps that decision in a
//! small file of its own, away from its drawing and IPC code, and the file is
//! included here by `#[path]`, so these tests pin the source the capsule builds
//! rather than a copy of it. Apps with a proofs crate of their own (the file
//! manager, the editor, settings) are proven there instead.

extern crate alloc;

/// The calculator's arithmetic, why it stops and how it shows numbers and
/// errors, and its keypad: the window state, every button and the actions they
/// run, the key map, where every control sits and the hover that names it,
/// under `calc` as in the capsule so the files' `crate::calc::` and `super::`
/// paths resolve. Only the painting and the skeleton glue are left out.
#[path = "."]
pub mod calc {
    #[path = "../../capsule_calculator/src/calc/actions/mod.rs"]
    pub mod actions;
    #[path = "../../capsule_calculator/src/calc/buttons/mod.rs"]
    pub mod buttons;
    #[path = "../../capsule_calculator/src/calc/convert/mod.rs"]
    pub mod convert;
    #[path = "../../capsule_calculator/src/calc/error_kind.rs"]
    pub mod error_kind;
    #[path = "../../capsule_calculator/src/calc/fixed.rs"]
    pub mod fixed;
    #[path = "../../capsule_calculator/src/calc/format/mod.rs"]
    pub mod format;
    #[path = "../../capsule_calculator/src/calc/history/mod.rs"]
    pub mod history;
    #[path = "../../capsule_calculator/src/calc/hit.rs"]
    pub mod hit;
    #[path = "../../capsule_calculator/src/calc/manifest.rs"]
    pub mod manifest;
    #[path = "../../capsule_calculator/src/calc/mode.rs"]
    pub mod mode;
    #[path = "../../capsule_calculator/src/calc/op.rs"]
    pub mod op;
    #[path = "../../capsule_calculator/src/calc/paste.rs"]
    pub mod paste;
    #[path = "../../capsule_calculator/src/calc/prog/mod.rs"]
    pub mod prog;
    #[path = "../../capsule_calculator/src/calc/sci/mod.rs"]
    pub mod sci;
    #[path = "../../capsule_calculator/src/calc/state/mod.rs"]
    pub mod state;
    #[path = "../../capsule_calculator/src/calc/unary.rs"]
    pub mod unary;
    #[path = "."]
    pub mod event {
        #[path = "../../capsule_calculator/src/calc/event/hover.rs"]
        pub mod hover;
        #[path = "../../capsule_calculator/src/calc/event/key_classifier.rs"]
        pub mod key_classifier;
    }
    #[path = "."]
    pub mod ui {
        #[path = "../../capsule_calculator/src/calc/ui/bits_geom.rs"]
        pub mod bits_geom;
        #[path = "../../capsule_calculator/src/calc/ui/convert_geom.rs"]
        pub mod convert_geom;
        #[path = "../../capsule_calculator/src/calc/ui/convert_hit.rs"]
        pub mod convert_hit;
        #[path = "../../capsule_calculator/src/calc/ui/history_geom.rs"]
        pub mod history_geom;
        #[path = "../../capsule_calculator/src/calc/ui/keypad_geom.rs"]
        pub mod keypad_geom;
        #[path = "../../capsule_calculator/src/calc/ui/metrics.rs"]
        pub mod metrics;
        #[path = "../../capsule_calculator/src/calc/ui/nav_geom.rs"]
        pub mod nav_geom;
        #[path = "../../capsule_calculator/src/calc/ui/radix_geom.rs"]
        pub mod radix_geom;
        #[path = "../../capsule_calculator/src/calc/ui/rect.rs"]
        pub mod rect;
        #[path = "../../capsule_calculator/src/calc/ui/target.rs"]
        pub mod target;
    }
}

/// The clock's calendar arithmetic and what it says when the system clock
/// cannot be read or set.
#[path = "../../capsule_clock/src/clock/civil.rs"]
pub mod clock_civil;
#[path = "../../capsule_clock/src/clock/says.rs"]
pub mod clock_says;
#[path = "../../capsule_clock/src/clock/stopwatch.rs"]
pub mod clock_stopwatch;
#[path = "../../capsule_clock/src/clock/timer.rs"]
pub mod clock_timer;

/// The clock's tabs and where its buttons sit, which painting, clicks and
/// hover all read, and the window they are laid out in.
#[path = "."]
pub mod clock {
    #[path = "../../capsule_clock/src/clock/controls.rs"]
    pub mod controls;
    #[path = "../../capsule_clock/src/clock/manifest.rs"]
    pub mod manifest;
    #[path = "../../capsule_clock/src/clock/tabs.rs"]
    pub mod tabs;
}

/// The file manager's target table where it needs no store: what a click or a
/// hover names in the Browse list and grid, scrolled or not, on the selection
/// band and in the sidebar, and the hover that feeds on it. `state` is the
/// shim `fs_proofs` keeps for the two enums the list's columns name.
#[path = "."]
pub mod fm {
    #[path = "../../capsule_file_manager/src/fm/browse_hit.rs"]
    pub mod browse_hit;
    #[path = "../../capsule_file_manager/src/fm/grid_geom.rs"]
    pub mod grid_geom;
    #[path = "../../capsule_file_manager/src/fm/header_slots.rs"]
    pub mod header_slots;
    #[path = "../../capsule_file_manager/src/fm/hover.rs"]
    pub mod hover;
    #[path = "../../capsule_file_manager/src/fm/icon_path.rs"]
    pub mod icon_path;
    #[path = "../../capsule_file_manager/src/fm/layout.rs"]
    pub mod layout;
    #[path = "../../capsule_file_manager/src/fm/list_cols.rs"]
    pub mod list_cols;
    #[path = "../../capsule_file_manager/src/fm/pane.rs"]
    pub mod pane;
    #[path = "../../capsule_file_manager/src/fm/row_geom.rs"]
    pub mod row_geom;
    #[path = "../../capsule_file_manager/src/fm/screen.rs"]
    pub mod screen;
    #[path = "../../capsule_file_manager/src/fm/sel_geom.rs"]
    pub mod sel_geom;
    #[path = "../../capsule_file_manager/src/fm/sel_model.rs"]
    pub mod sel_model;
    #[path = "../../capsule_file_manager/src/fm/sel_slots.rs"]
    pub mod sel_slots;
    #[path = "../../capsule_file_manager/src/fm/sidebar_model.rs"]
    pub mod sidebar_model;
    #[path = "../../capsule_file_manager/src/fm/sidebar_rows.rs"]
    pub mod sidebar_rows;
    #[path = "../../fs_proofs/src/fm_logic/state.rs"]
    pub mod state;
    #[path = "../../capsule_file_manager/src/fm/target.rs"]
    pub mod target;
}

/// What the image viewer says when it has no image to show, and the largest
/// file and image it takes in.
#[path = "../../capsule_image_viewer/src/viewer/budget.rs"]
pub mod image_budget;
#[path = "../../capsule_image_viewer/src/viewer/says.rs"]
pub mod image_says;

/// What the image viewer's overlays say, where its gallery scrolls and which
/// thumbnails it keeps, its prev and next buttons and the tiles a click or a
/// hover names, the window it opens at, and how often it asks the shell for a
/// file. Mounted as `viewer` at the root so the files' `crate::viewer::` paths
/// resolve.
#[path = "."]
pub mod viewer {
    #[path = "../../capsule_image_viewer/src/viewer/arg_cadence.rs"]
    pub mod arg_cadence;
    #[path = "../../capsule_image_viewer/src/viewer/caption.rs"]
    pub mod caption;
    #[path = "../../capsule_image_viewer/src/viewer/hover.rs"]
    pub mod hover;
    #[path = "../../capsule_image_viewer/src/viewer/manifest.rs"]
    pub mod manifest;
    #[path = "../../capsule_image_viewer/src/viewer/nav.rs"]
    pub mod nav;
    #[path = "../../capsule_image_viewer/src/viewer/target.rs"]
    pub mod target;
    #[path = "../../capsule_image_viewer/src/viewer/viewport.rs"]
    pub mod viewport;
    #[path = "."]
    pub mod gallery {
        #[path = "../../capsule_image_viewer/src/viewer/gallery/input.rs"]
        pub mod input;
        #[path = "../../capsule_image_viewer/src/viewer/gallery/layout.rs"]
        pub mod layout;
        #[path = "../../capsule_image_viewer/src/viewer/gallery/state.rs"]
        pub mod state;
    }
}

/// Whether the markdown viewer lays out what it read from /readme.txt. Its
/// parser's side (a blank file parses to no blocks) is proven with the parser
/// in capsule_mdview/layout_tests.
#[path = "../../capsule_mdview/src/mdview/verdict.rs"]
pub mod mdview_verdict;

/// Where the markdown viewer's lines sit, how far its wheel scrolls the page
/// and the margin and window it draws them in, over the viewer's own line
/// metrics. Mounted with a `layout` that names what the capsule's does, so the
/// files' `super::layout` resolves.
#[path = "."]
pub mod mdview_page {
    #[path = "."]
    pub mod layout {
        #[path = "../../capsule_mdview/src/mdview/layout/block.rs"]
        pub mod block;
        #[path = "../../capsule_mdview/src/mdview/layout/metrics.rs"]
        pub mod metrics;
        pub use block::{Line, Span, Style};
        pub use metrics::{gap, indent, line_height, px};
    }
    #[path = "../../capsule_mdview/src/mdview/draw.rs"]
    pub mod draw;
    #[path = "../../capsule_mdview/src/mdview/manifest.rs"]
    pub mod manifest;
    #[path = "../../capsule_mdview/src/mdview/scroll.rs"]
    pub mod scroll;
}

/// Why the video library is empty when its folders were never read.
#[path = "../../capsule_video_player/src/catalog/says.rs"]
pub mod video_says;

/// What the video library lists (only what the player decodes), the folders
/// it reads, and its search and folder filter. Mounted as `catalog` at the
/// root so the files' `crate::catalog::` paths resolve.
#[path = "."]
pub mod catalog {
    #[path = "../../capsule_video_player/src/catalog/entry.rs"]
    pub mod entry;
    #[path = "../../capsule_video_player/src/catalog/folders.rs"]
    pub mod folders;
    #[path = "../../capsule_video_player/src/catalog/kind.rs"]
    pub mod kind;
    #[path = "../../capsule_video_player/src/catalog/media.rs"]
    pub mod media;
}
#[path = "../../capsule_video_player/src/app/browse.rs"]
pub mod video_browse;
/// When the video player asks for its window full screen.
#[path = "../../capsule_video_player/src/app/full_screen.rs"]
pub mod video_full_screen;
#[path = "../../capsule_video_player/src/app/open_arg_reply.rs"]
pub mod video_open_arg;

/// The video player's key map on its list pages, where printable keys type
/// into the search. `action.rs` names `crate::ui::screen::Route`.
#[path = "."]
pub mod video_event {
    #[path = "../../capsule_video_player/src/event/action.rs"]
    pub mod action;
    #[path = "../../capsule_video_player/src/event/key.rs"]
    pub mod key;
}
/// The video player's routes, where its list starts so the selection stays
/// in sight, where its pages and its player put each control, the one table
/// a click and the hover search, and the hover itself.
#[path = "."]
pub mod ui {
    #[path = "."]
    pub mod frame {
        #[path = "../../capsule_video_player/src/ui/frame/geom.rs"]
        pub mod geom;
        #[path = "../../capsule_video_player/src/ui/frame/region.rs"]
        pub mod region;
    }
    #[path = "../../capsule_video_player/src/ui/hover.rs"]
    pub mod hover;
    #[path = "../../capsule_video_player/src/ui/layout.rs"]
    pub mod layout;
    #[path = "."]
    pub mod player {
        #[path = "../../capsule_video_player/src/ui/player/transport.rs"]
        pub mod transport;
    }
    #[path = "../../capsule_video_player/src/ui/rows.rs"]
    pub mod rows;
    #[path = "../../capsule_video_player/src/ui/screen.rs"]
    pub mod screen;
    #[path = "../../capsule_video_player/src/ui/target.rs"]
    pub mod target;
    #[path = "../../capsule_video_player/src/ui/targets.rs"]
    pub mod targets;
    #[path = "."]
    pub mod view {
        #[path = "../../capsule_video_player/src/ui/view/list_geom.rs"]
        pub mod list_geom;
        #[path = "../../capsule_video_player/src/ui/view/page_geom.rs"]
        pub mod page_geom;
        #[path = "../../capsule_video_player/src/ui/view/rail_geom.rs"]
        pub mod rail_geom;
    }
}

/// The process monitor's table and status-strip notes.
#[path = "../../capsule_process_manager/src/pm/state/notes.rs"]
pub mod pm_notes;

/// Which processes the process monitor will not end, and its percent text.
#[path = "../../capsule_process_manager/src/pm/critical.rs"]
pub mod pm_critical;
#[path = "../../capsule_process_manager/src/pm/format.rs"]
pub mod pm_format;

/// A pasted line into the process monitor's search field.
#[path = "../../capsule_process_manager/src/pm/state/query_paste.rs"]
pub mod pm_query_paste;

/// The audio player's transport (its play, pause and feed state machine) and
/// what the window says when it cannot play. The transport names its decoder,
/// resampler and debug marker by `crate::` paths, so those are mounted at this
/// crate's root under the same names; the marker is a no-op here. `#[path = "."]`
/// keeps the inline modules' includes relative to this directory, which has no
/// `decode/` or `transport/` of its own.
#[path = "../../capsule_audio_player/src/trouble.rs"]
pub mod audio_trouble;

#[path = "."]
pub mod decode {
    #[path = "../../capsule_audio_player/src/decode/decoder.rs"]
    mod decoder;
    #[path = "../../capsule_audio_player/src/decode/mp3_index.rs"]
    pub mod mp3_index;
    #[path = "../../capsule_audio_player/src/decode/wav.rs"]
    pub mod wav;
    #[path = "../../capsule_audio_player/src/decode/wav_pcm.rs"]
    mod wav_pcm;
    pub use decoder::{AudioInfo, Decoder};
}

/// The audio player's waveform bars, built as a track decodes, and the largest
/// track it reads whole.
#[path = "../../capsule_audio_player/src/peaks.rs"]
pub mod peaks;
#[path = "../../capsule_audio_player/src/track_limit.rs"]
pub mod track_limit;
/// The format label Now Playing shows for the loaded track.
#[path = "../../capsule_audio_player/src/track_fmt.rs"]
pub mod track_fmt;

pub mod mark {
    pub fn mark(_line: &str) {}
}

/// Which files the audio player's library lists (only what its decoder opens),
/// the order of its "By format" tab and what it does with a file the desktop
/// shell hands it, under `library` as in the capsule so `super::track`
/// resolves.
#[path = "."]
pub mod library {
    #[path = "../../capsule_audio_player/src/library/handed.rs"]
    pub mod handed;
    #[path = "../../capsule_audio_player/src/library/order.rs"]
    pub mod order;
    #[path = "../../capsule_audio_player/src/library/playable.rs"]
    pub mod playable;
    #[path = "../../capsule_audio_player/src/library/rescan_gap.rs"]
    pub mod rescan_gap;
    #[path = "../../capsule_audio_player/src/library/track.rs"]
    pub mod track;
}

#[path = "../../capsule_audio_player/src/resample.rs"]
pub mod resample;

#[path = "."]
pub mod transport {
    #[path = "../../capsule_audio_player/src/transport/defs.rs"]
    pub mod defs;
    #[path = "../../capsule_audio_player/src/transport/machine.rs"]
    pub mod machine;
    #[path = "../../capsule_audio_player/src/transport/pump.rs"]
    pub mod pump;
}

#[cfg(test)]
mod audio_format_tests;
#[cfg(test)]
mod audio_library_tests;
#[cfg(test)]
mod audio_load_tests;
#[cfg(test)]
mod audio_open_tests;
#[cfg(test)]
mod audio_tests;
#[cfg(test)]
mod calc_controls_tests;
#[cfg(test)]
mod calc_convert_tests;
#[cfg(test)]
mod calc_hover_tests;
#[cfg(test)]
mod calc_keypad_tests;
#[cfg(test)]
mod calc_tests;
#[cfg(test)]
mod calc_paste_tests;
#[cfg(test)]
mod clock_span_tests;
#[cfg(test)]
mod clock_tests;
#[cfg(test)]
mod fm_check;
#[cfg(test)]
mod fm_chrome_tests;
#[cfg(test)]
mod fm_controls_tests;
#[cfg(test)]
mod fm_hover_tests;
#[cfg(test)]
mod fm_press_tests;
#[cfg(test)]
mod image_arg_tests;
#[cfg(test)]
mod image_controls_tests;
#[cfg(test)]
mod image_hover_tests;
#[cfg(test)]
mod image_tests;
#[cfg(test)]
mod image_view_tests;
#[cfg(test)]
mod mdview_tests;
#[cfg(test)]
mod pm_paste_tests;
#[cfg(test)]
mod pm_tests;
#[cfg(test)]
mod video_browse_tests;
#[cfg(test)]
mod video_check;
#[cfg(test)]
mod video_controls_tests;
#[cfg(test)]
mod video_full_screen_tests;
#[cfg(test)]
mod video_hover_tests;
#[cfg(test)]
mod video_list_tests;
#[cfg(test)]
mod video_player_tests;
#[cfg(test)]
mod video_scroll_tests;
#[cfg(test)]
mod video_tests;
#[cfg(test)]
mod wheel_tests;

/// How the process monitor shares its width between the sidebar, the screen
/// and the inspector. fit.rs names `crate::pm::state::Screen` and
/// `super::metrics`, so both are given those names here.
#[path = "../../capsule_process_manager/src/pm/state/screen.rs"]
pub mod pm_screen;
#[path = "../../capsule_process_manager/src/pm/ui/metrics.rs"]
pub mod metrics;
#[path = "../../capsule_process_manager/src/pm/ui/fit.rs"]
pub mod pm_fit;
/// The process monitor's hit test and the hover named by it, mounted as the
/// capsule has them over a stand-in for its state, under `pm` so the files'
/// `crate::pm::` and `super::` paths resolve.
#[path = "."]
pub mod pm {
    pub use crate::pm_critical as critical;
    #[path = "."]
    pub mod state {
        pub use crate::pm_screen::{Screen, SCREENS};
        pub use crate::pm_state::{Row, State};
        pub use filter::{Filter, FILTERS};
        pub use sort::Sort;
        #[path = "../../capsule_process_manager/src/pm/state/filter.rs"]
        mod filter;
        #[path = "../../capsule_process_manager/src/pm/state/sort.rs"]
        mod sort;
    }
    #[path = "."]
    pub mod security {
        #[path = "../../capsule_process_manager/src/pm/security/sensitive.rs"]
        pub mod sensitive;
    }
    #[path = "."]
    pub mod ui {
        pub use crate::metrics;
        pub use crate::pm_fit as fit;
        #[path = "../../capsule_process_manager/src/pm/ui/chips.rs"]
        pub mod chips;
        #[path = "../../capsule_process_manager/src/pm/ui/hit.rs"]
        pub mod hit;
        #[path = "../../capsule_process_manager/src/pm/ui/hover.rs"]
        pub mod hover;
        #[path = "../../capsule_process_manager/src/pm/ui/hover_look.rs"]
        pub mod hover_look;
        #[path = "../../capsule_process_manager/src/pm/ui/insp_geom.rs"]
        pub mod insp_geom;
        #[path = "../../capsule_process_manager/src/pm/ui/matrix_geom.rs"]
        pub mod matrix_geom;
        #[path = "../../capsule_process_manager/src/pm/ui/nav_geom.rs"]
        pub mod nav_geom;
        #[path = "../../capsule_process_manager/src/pm/ui/pane_geom.rs"]
        pub mod pane_geom;
        #[path = "../../capsule_process_manager/src/pm/ui/search.rs"]
        pub mod search;
        #[path = "../../capsule_process_manager/src/pm/ui/table_cols.rs"]
        pub mod table_cols;
        #[path = "../../capsule_process_manager/src/pm/ui/table_geom.rs"]
        pub mod table_geom;
        #[path = "../../capsule_process_manager/src/pm/ui/text.rs"]
        pub mod text;
        #[path = "."]
        pub mod screens {
            #[path = "../../capsule_process_manager/src/pm/ui/screens/sec_geom.rs"]
            pub mod sec_geom;
        }
    }
}
pub mod pm_state;
pub mod pm_window;

#[cfg(test)]
mod pm_cut_tests;
#[cfg(test)]
mod pm_fit_tests;

/// The audio player's load, read and decoded a tick at a time.
#[path = "../../capsule_audio_player/src/loading.rs"]
pub mod loading;

/// Where the audio player puts each control, which track a click on its queue
/// means, the one table a click and the hover search, and the hover itself,
/// on the player's own geometry and type scale.
#[path = "."]
pub mod audio_ui {
    #[path = "../../capsule_audio_player/src/ui/geometry.rs"]
    pub mod geometry;
    #[path = "../../capsule_audio_player/src/ui/hover.rs"]
    pub mod hover;
    #[path = "../../capsule_audio_player/src/ui/metrics.rs"]
    pub mod metrics;
    #[path = "."]
    pub mod screen {
        #[path = "../../capsule_audio_player/src/ui/screen/downloads_geom.rs"]
        pub mod downloads_geom;
        #[path = "../../capsule_audio_player/src/ui/screen/home_geom.rs"]
        pub mod home_geom;
        #[path = "../../capsule_audio_player/src/ui/screen/lib_geom.rs"]
        pub mod lib_geom;
        #[path = "../../capsule_audio_player/src/ui/screen/search_geom.rs"]
        pub mod search_geom;
        #[path = "../../capsule_audio_player/src/ui/screen/settings.rs"]
        pub mod settings;
    }
    #[path = "."]
    pub mod shell {
        #[path = "../../capsule_audio_player/src/ui/shell/bar.rs"]
        pub mod bar;
        #[path = "../../capsule_audio_player/src/ui/shell/ground.rs"]
        pub mod ground;
        #[path = "../../capsule_audio_player/src/ui/shell/chrome_geom.rs"]
        pub mod chrome_geom;
        #[path = "../../capsule_audio_player/src/ui/shell/rail_geom.rs"]
        pub mod rail_geom;
    }
    #[path = "../../capsule_audio_player/src/ui/search_key.rs"]
    pub mod search_key;
    #[path = "../../capsule_audio_player/src/ui/shown.rs"]
    pub mod shown;
    #[path = "../../capsule_audio_player/src/ui/target.rs"]
    pub mod target;
    #[path = "../../capsule_audio_player/src/ui/targets.rs"]
    pub mod targets;
    #[path = "../../capsule_audio_player/src/ui/targets_page.rs"]
    mod targets_page;
    #[path = "../../capsule_audio_player/src/ui/view.rs"]
    pub mod view;
}

/// What a file Music downloads is called in /home/nonos/music.
#[path = "../../capsule_audio_player/src/fetch/name.rs"]
pub mod audio_fetch_name;

#[cfg(test)]
mod audio_fetch_tests;

/// The player's volume controls as the system's master volume.
#[path = "../../capsule_audio_player/src/volume.rs"]
pub mod audio_volume;

/// The player's keys away from the Search field.
#[path = "../../capsule_audio_player/src/ui/shortcut.rs"]
pub mod audio_shortcut;

#[cfg(test)]
mod audio_keys_tests;

/// The Downloads page: its list and what each row says. row_text.rs reads
/// `super::list`, so both sit in one module as they do in `fetch`.
#[path = "."]
pub mod audio_downloads {
    #[path = "../../capsule_audio_player/src/fetch/list.rs"]
    pub mod list;
    #[path = "../../capsule_audio_player/src/fetch/row_text.rs"]
    pub mod row_text;
}

/// Where the cover art's grid motif draws its floor lines.
#[path = "../../capsule_audio_player/src/ui/art/rungs.rs"]
pub mod audio_rungs;

#[cfg(test)]
mod audio_downloads_tests;

/// What Music reads from a file's tags, and how it groups a library.
#[path = "../../capsule_audio_player/src/library/tags/mod.rs"]
pub mod audio_tags;

#[cfg(test)]
mod audio_tags_tests;

/// The audio player's proof windows, at the display scale in force.
pub mod audio_check;
#[cfg(test)]
mod audio_hover_tests;
#[cfg(test)]
mod audio_step_tests;
#[cfg(test)]
mod audio_stream_tests;
#[cfg(test)]
mod audio_time_tests;

/// Snake: the day's run the home card names and a click sets up, what a click
/// on a rules switch changes, and what the Ranks screen says became of the
/// ranks on disk. Mounted as one module so the files' `super::` paths resolve.
#[path = "."]
pub mod snake_state {
    #[path = "../../capsule_snake/src/snake/state/daily.rs"]
    pub mod daily;
    #[path = "../../capsule_snake/src/snake/state/difficulty.rs"]
    pub mod difficulty;
    #[path = "../../capsule_snake/src/snake/state/kept.rs"]
    pub mod kept;
    #[path = "../../capsule_snake/src/snake/state/mode.rs"]
    pub mod mode;
    #[path = "../../capsule_snake/src/snake/state/mode_text.rs"]
    pub mod mode_text;
    #[path = "../../capsule_snake/src/snake/state/options.rs"]
    pub mod options;
}

#[cfg(test)]
mod snake_tests;

/// Snake's controls: where each screen draws them, what a click or the
/// pointer names, and the hover that repaints only on crossing to another.
/// Mounted as Snake's own modules so the files' `crate::snake::` and `super::`
/// paths resolve.
#[path = "."]
pub mod snake {
    #[path = "../../capsule_snake/src/snake/grid.rs"]
    pub mod grid;
    #[path = "."]
    pub mod state {
        #[path = "../../capsule_snake/src/snake/state/difficulty.rs"]
        pub mod difficulty;
        #[path = "../../capsule_snake/src/snake/state/mode.rs"]
        pub mod mode;
        #[path = "../../capsule_snake/src/snake/state/mode_text.rs"]
        pub mod mode_text;
        #[path = "../../capsule_snake/src/snake/state/screen.rs"]
        pub mod screen;
        pub use screen::Screen;
    }
    #[path = "."]
    pub mod ui {
        #[path = "../../capsule_snake/src/snake/ui/home_geom.rs"]
        pub mod home_geom;
        #[path = "../../capsule_snake/src/snake/ui/metrics.rs"]
        pub mod metrics;
        #[path = "../../capsule_snake/src/snake/ui/over_geom.rs"]
        pub mod over_geom;
        #[path = "../../capsule_snake/src/snake/ui/pause_geom.rs"]
        pub mod pause_geom;
        #[path = "../../capsule_snake/src/snake/ui/play_geom.rs"]
        pub mod play_geom;
        #[path = "../../capsule_snake/src/snake/ui/play_geom_rows.rs"]
        pub mod play_geom_rows;
        #[path = "../../capsule_snake/src/snake/ui/rank_geom.rs"]
        pub mod rank_geom;
        #[path = "../../capsule_snake/src/snake/ui/rect.rs"]
        pub mod rect;
        #[path = "../../capsule_snake/src/snake/ui/setup_geom.rs"]
        pub mod setup_geom;
        #[path = "../../capsule_snake/src/snake/ui/setup_geom_rows.rs"]
        pub mod setup_geom_rows;
        #[path = "../../capsule_snake/src/snake/ui/text.rs"]
        pub mod text;
    }
    #[path = "."]
    pub mod input {
        #[path = "../../capsule_snake/src/snake/input/hover.rs"]
        pub mod hover;
        #[path = "../../capsule_snake/src/snake/input/surface.rs"]
        pub mod surface;
        #[path = "../../capsule_snake/src/snake/input/target.rs"]
        pub mod target;
        #[path = "../../capsule_snake/src/snake/input/target_screen.rs"]
        pub mod target_screen;
    }
}

pub mod snake_drawn;

#[cfg(test)]
mod snake_chip_tests;
#[cfg(test)]
mod snake_controls_tests;
#[cfg(test)]
mod snake_hover_tests;

/// About: the Overview badge read from how the kernel admitted the window,
/// the capability count from the word the kernel holds for it, and the
/// Display screen's present path. Mounted as About's `data` so the files'
/// `super::` paths resolve.
#[path = "."]
pub mod about_data {
    #[path = "../../capsule_about/src/about/data/admission.rs"]
    pub mod admission;
    #[path = "../../capsule_about/src/about/data/caps.rs"]
    pub mod caps;
    #[path = "../../capsule_about/src/about/data/own_caps.rs"]
    pub mod own_caps;
    #[path = "../../capsule_about/src/about/data/present.rs"]
    pub mod present;
    #[path = "../../capsule_about/src/about/data/trust.rs"]
    pub mod trust;
}

#[cfg(test)]
mod about_tests;
#[cfg(test)]
mod about_mono_tests;
#[cfg(test)]
mod about_old_wrap;
#[cfg(test)]
mod about_wrap_tests;

/// About whole: its sidebar, its wheel, every screen and the frame that
/// paints them, mounted as About's own modules so the files' `crate::about::`
/// and `super::` paths resolve. Its kernel reads are answered with fixed
/// content (`about_double`), so a screen paints alike on every run.
#[path = "."]
pub mod about {
    #[path = "about_double.rs"]
    pub mod data;
    #[path = "../../capsule_about/src/about/format.rs"]
    pub mod format;
    #[path = "../../capsule_about/src/about/format_hex.rs"]
    pub mod format_hex;
    #[path = "../../capsule_about/src/about/scroll_wheel.rs"]
    pub mod scroll_wheel;
    #[path = "../../capsule_about/src/about/section.rs"]
    pub mod section;
    #[path = "../../capsule_about/src/about/state.rs"]
    pub mod state;
    #[path = "../../capsule_about/src/about/ui/mod.rs"]
    pub mod ui;
}

#[cfg(test)]
mod about_hover_tests;
#[cfg(test)]
mod about_nav_tests;

/// GUI Demo, the reference app built only from shared widgets, mounted as its
/// own modules so the files' `super::` paths resolve.
#[path = "."]
pub mod gui_demo {
    #[path = "../../capsule_gui_demo/src/gui_demo/action.rs"]
    pub mod action;
    #[path = "../../capsule_gui_demo/src/gui_demo/app.rs"]
    pub mod app;
    #[path = "../../capsule_gui_demo/src/gui_demo/event.rs"]
    pub mod event;
    #[path = "../../capsule_gui_demo/src/gui_demo/manifest.rs"]
    pub mod manifest;
    #[path = "../../capsule_gui_demo/src/gui_demo/paint.rs"]
    pub mod paint;
}

#[cfg(test)]
mod feed_tests;
#[cfg(test)]
mod gui_demo_tests;

/// The input probe's echo, its surface state and its drawing, mounted as its
/// own modules so the files' `super::` paths resolve.
#[path = "."]
pub mod input_probe {
    #[path = "../../capsule_input_probe/src/state.rs"]
    pub mod state;
    #[path = "."]
    pub mod render {
        #[path = "../../capsule_input_probe/src/render/draw.rs"]
        pub mod draw;
        #[path = "../../capsule_input_probe/src/render/font.rs"]
        pub mod font;
        #[path = "../../capsule_input_probe/src/render/font_table.rs"]
        mod font_table;
    }
}

#[cfg(test)]
mod input_probe_tests;
