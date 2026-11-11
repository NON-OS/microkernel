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

//! Host proofs for the shared widgets and their text helpers, drawn with the
//! real toolkit source into a buffer larger than each widget's box.

mod agree;
mod bounds;
mod boxes;
mod button;
mod button_drawn;
mod button_icon;
mod button_icon_bounds;
mod button_icon_group;
mod button_quiet;
mod button_read;
mod button_tonal;
mod canvas;
mod card;
mod card_read;
mod checkbox;
mod checkbox_read;
mod column;
mod controls_at;
mod controls_click;
mod controls_hover;
mod dialog;
mod dialog_armed_paint;
mod dialog_armed_place;
mod dialog_armed_read;
mod dialog_bounds;
mod dialog_cut;
mod dialog_disable_paint;
mod dialog_disable_place;
mod dialog_disable_read;
mod dialog_hit;
mod dialog_inks;
mod dialog_keys;
mod dialog_lines;
mod dialog_lines_short;
mod dialog_lines_whole;
mod dialog_lit;
mod dialog_read;
mod dialog_row;
mod dialog_scrim;
mod dialog_size;
mod dialog_spans;
mod field;
mod field_pick;
mod field_read;
mod field_role;
mod field_role_size;
mod field_select;
mod field_view;
mod hide;
mod hits;
mod icon_button;
mod icon_button_read;
mod label;
mod legible;
mod mask;
mod menu;
mod menu_danger;
mod menu_danger_paint;
mod menu_danger_read;
mod menu_dead;
mod menu_heading;
mod menu_heading_line;
mod menu_heading_read;
mod menu_hit;
mod menu_keys;
mod menu_place;
mod menu_read;
mod menu_size;
mod menu_tag;
mod menu_tag_hit;
mod menu_tag_line;
mod menu_tag_read;
mod nav_item;
mod nav_item_count;
mod nav_item_rail;
mod pill;
mod pill_category;
mod pill_count;
mod pill_icon;
mod pill_icon_read;
mod pill_marks;
mod pill_marks_read;
mod pill_read;
mod placing;
mod pointer;
mod progress;
mod radio;
mod radio_read;
mod read;
mod rect_new;
mod row;
mod row_icon;
mod row_icon_hit;
mod row_lead;
mod row_lead_hit;
mod row_lanes;
mod row_marks;
mod row_read;
mod samples;
mod scrollbar;
mod scrollbar_hit;
mod scrollbar_read;
mod scrolled;
mod scrolled_cut;
mod section_header;
mod section_header_caption;
mod separator;
mod serial;
mod slider;
mod slider_hit;
mod states;
mod switch;
mod tabs;
mod tabs_hit;
mod tabs_lead;
mod tabs_plain;
mod tabs_read;
mod table;
mod table_bounds;
mod table_cells;
mod table_first;
mod table_hit;
mod table_miss;
mod table_place;
mod table_place_head;
mod table_place_hit;
mod table_place_set;
mod table_place_text;
mod table_probe;
mod table_read;
mod table_size;
mod table_spans;
mod table_widths;
mod text_clip;
mod text_clip_tail;
mod text_fit_size;
mod text_measure;
mod text_mono;
mod text_tail;
mod text_truncate;
mod text_wrap;
mod text_wrap_hard;
mod toast;
mod toast_hit;
mod toast_lines;
mod toast_read;
mod toast_wide;
mod toast_wide_lines;
mod tooltip;
mod tooltip_place;
mod tooltip_read;
mod tooltip_wide;
mod tooltip_wide_place;
mod widget;
mod widget_odd;
