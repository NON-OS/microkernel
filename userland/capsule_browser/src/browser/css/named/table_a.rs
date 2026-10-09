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

/* Named colours from A to lightgoldenrodyellow, sorted, one 26-byte record
 * each: the name space-padded to 20 bytes, then six hex digits. */
pub(super) const ROWS: &[&str] = &[
    "accentcolor         0075ffaccentcolortext     ffffffactivetext          ff0000",
    "aliceblue           f0f8ffantiquewhite        faebd7aqua                00ffff",
    "aquamarine          7fffd4azure               f0ffffbeige               f5f5dc",
    "bisque              ffe4c4black               000000blanchedalmond      ffebcd",
    "blue                0000ffblueviolet          8a2be2brown               a52a2a",
    "burlywood           deb887buttonborder        767676buttonface          efefef",
    "buttontext          000000cadetblue           5f9ea0canvas              ffffff",
    "canvastext          000000chartreuse          7fff00chocolate           d2691e",
    "coral               ff7f50cornflowerblue      6495edcornsilk            fff8dc",
    "crimson             dc143ccyan                00ffffdarkblue            00008b",
    "darkcyan            008b8bdarkgoldenrod       b8860bdarkgray            a9a9a9",
    "darkgreen           006400darkgrey            a9a9a9darkkhaki           bdb76b",
    "darkmagenta         8b008bdarkolivegreen      556b2fdarkorange          ff8c00",
    "darkorchid          9932ccdarkred             8b0000darksalmon          e9967a",
    "darkseagreen        8fbc8fdarkslateblue       483d8bdarkslategray       2f4f4f",
    "darkslategrey       2f4f4fdarkturquoise       00ced1darkviolet          9400d3",
    "deeppink            ff1493deepskyblue         00bfffdimgray             696969",
    "dimgrey             696969dodgerblue          1e90fffield               ffffff",
    "fieldtext           000000firebrick           b22222floralwhite         fffaf0",
    "forestgreen         228b22fuchsia             ff00ffgainsboro           dcdcdc",
    "ghostwhite          f8f8ffgold                ffd700goldenrod           daa520",
    "gray                808080graytext            808080green               008000",
    "greenyellow         adff2fgrey                808080highlight           b5d5ff",
    "highlighttext       000000honeydew            f0fff0hotpink             ff69b4",
    "indianred           cd5c5cindigo              4b0082ivory               fffff0",
    "khaki               f0e68clavender            e6e6falavenderblush       fff0f5",
    "lawngreen           7cfc00lemonchiffon        fffacdlightblue           add8e6",
    "lightcoral          f08080lightcyan           e0fffflightgoldenrodyellowfafad2",
];
