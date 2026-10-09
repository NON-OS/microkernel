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

//! The two reference sheets Help opens as documents: the shortcut table the
//! capsule really dispatches, and what the editor is and is not.

pub(super) const SHORTCUTS: &str = concat!(
    "# Keyboard Shortcuts\n\n",
    "File\n",
    "Ctrl-N new tab   Ctrl-O open   Ctrl-P open by name   Ctrl-W close tab\n",
    "Ctrl-S save   Ctrl-Shift-S save as   Ctrl-E export\n\n",
    "Edit\n",
    "Ctrl-Z undo   Ctrl-Y or Ctrl-Shift-Z redo   Ctrl-X cut   Ctrl-C copy\n",
    "Ctrl-V paste   Ctrl-A select all   Ctrl-Backspace delete word\n",
    "Ctrl-F find   Ctrl-H replace   Ctrl-Shift-H replace all   Ctrl-G go to line\n\n",
    "Lines\n",
    "Ctrl-D duplicate line   Ctrl-Shift-K delete line   Ctrl-/ toggle comment\n",
    "Tab indent   Shift-Tab dedent\n\n",
    "Move\n",
    "Ctrl-Left and Ctrl-Right by word   Ctrl-Home and Ctrl-End to the ends\n\n",
    "View\n",
    "Ctrl-= zoom in   Ctrl-- zoom out   Ctrl-0 reset zoom\n",
    "Ctrl-M page or code view   Ctrl-B file tree   Ctrl-K then T cycle theme\n\n",
    "Format\n",
    "Ctrl-Shift-B bold   Ctrl-I italic   Ctrl-U underline\n",
);

pub(super) const ABOUT: &str = concat!(
    "# About NONOS Docs\n\n",
    "The NONOS text editor capsule: a userland document editor running at\n",
    "CPL=3, reaching files only through the vfs service.\n\n",
    "The text buffer is the document. Headings are stored as a leading # so\n",
    "they are in the file. Bold, italic, underline, strike, colour, font, size\n",
    "and alignment are kept beside the text and survive every edit, but they\n",
    "are not in the text: Save writes the text alone, and Export (.docx, .pdf,\n",
    "or .md for what Markdown can say) writes them out.\n",
);
