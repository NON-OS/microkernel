// NONOS Operating System (AGPL-3.0-or-later)
//! Event helpers that depend on nothing but the DOM, compiled from the
//! capsule's own files so their decisions are checked on the host.

#[cfg(test)]
#[path = "../../../../capsule_browser/src/browser/event/dialog_line.rs"]
mod dialog_line;
#[cfg(test)]
mod dialog_line_tests;
#[cfg(test)]
#[path = "../../../../capsule_browser/src/browser/event/field_at.rs"]
mod field_at;
#[cfg(test)]
mod field_at_tests;
#[cfg(test)]
#[path = "../../../../capsule_browser/src/browser/event/select_list.rs"]
mod select_list;
#[cfg(test)]
mod select_list_tests;
#[cfg(test)]
#[path = "../../../../capsule_browser/src/browser/event/key_names.rs"]
mod key_names;
#[cfg(test)]
mod key_names_tests;
#[cfg(test)]
#[path = "../../../../capsule_browser/src/browser/event/label_for.rs"]
mod label_for;
#[cfg(test)]
mod label_for_tests;
#[cfg(test)]
#[path = "../../../../capsule_browser/src/browser/event/field_room.rs"]
mod field_room;
#[cfg(test)]
mod field_room_tests;
#[cfg(test)]
#[path = "../../../../capsule_browser/src/browser/event/field_value.rs"]
mod field_value;
#[cfg(test)]
#[path = "../../../../capsule_browser/src/browser/event/form_fields.rs"]
mod form_fields;
#[cfg(test)]
#[path = "../../../../capsule_browser/src/browser/event/field_toggle.rs"]
mod field_toggle;
#[cfg(test)]
mod form_tests;
