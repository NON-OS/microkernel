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

//! The Sound rows say what they reach: the desktop's own tones, which is all
//! the shell reads them for. Music keeps a volume of its own.

use crate::field_note_user::note;
use nonos_policy_proto::Field;

#[test]
fn the_sound_rows_say_they_reach_the_desktops_tones_and_not_music() {
    for field in [Field::SoundEnabled, Field::Volume] {
        let said = note(field).expect("a sound row says what it reaches");
        assert!(said.contains("desktop"), "{said}");
        assert!(said.contains("Music has its own volume"), "{said}");
    }
    let shell = include_str!("../../capsule_desktop_shell/src/sound/levels.rs");
    assert!(shell.contains("Field::Volume") && shell.contains("Field::SoundEnabled"));
    let music = include_str!("../../capsule_audio_player/src/model.rs");
    assert!(!music.contains("Field::Volume"), "if Music follows the row, the note must change");
}
