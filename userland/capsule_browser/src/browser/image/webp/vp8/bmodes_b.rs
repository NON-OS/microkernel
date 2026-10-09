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

/* Key-frame 4x4 mode probabilities, above modes 6 to 9 (see bmodes_a). */
pub(super) const BMODES_B: [u8; 360] = [
    125, 98, 42, 88, 104, 85, 117, 175, 82, 95, 84, 53, 89, 128, 100, 113, 101, 45, 75, 79, 123,
    47, 51, 128, 81, 171, 1, 57, 17, 5, 71, 102, 57, 53, 41, 49, 38, 33, 13, 121, 57, 73, 26, 1,
    85, 41, 10, 67, 138, 77, 110, 90, 47, 114, 115, 21, 2, 10, 102, 255, 166, 23, 6, 101, 29, 16,
    10, 85, 128, 101, 196, 26, 57, 18, 10, 102, 102, 213, 34, 20, 43, 117, 20, 15, 36, 163, 128,
    68, 1, 26, 102, 61, 71, 37, 34, 53, 31, 243, 192, 69, 60, 71, 38, 73, 119, 28, 222, 37, 68, 45,
    128, 34, 1, 47, 11, 245, 171, 62, 17, 19, 70, 146, 85, 55, 62, 70, 37, 43, 37, 154, 100, 163,
    85, 160, 1, 63, 9, 92, 136, 28, 64, 32, 201, 85, 75, 15, 9, 9, 64, 255, 184, 119, 16, 86, 6,
    28, 5, 64, 255, 25, 248, 1, 56, 8, 17, 132, 137, 255, 55, 116, 128, 58, 15, 20, 82, 135, 57,
    26, 121, 40, 164, 50, 31, 137, 154, 133, 25, 35, 218, 51, 103, 44, 131, 131, 123, 31, 6, 158,
    86, 40, 64, 135, 148, 224, 45, 183, 128, 22, 26, 17, 131, 240, 154, 14, 1, 209, 45, 16, 21, 91,
    64, 222, 7, 1, 197, 56, 21, 39, 155, 60, 138, 23, 102, 213, 83, 12, 13, 54, 192, 255, 68, 47,
    28, 85, 26, 85, 85, 128, 128, 32, 146, 171, 18, 11, 7, 63, 144, 171, 4, 4, 246, 35, 27, 10,
    146, 174, 171, 12, 26, 128, 190, 80, 35, 99, 180, 80, 126, 54, 45, 85, 126, 47, 87, 176, 51,
    41, 20, 32, 101, 75, 128, 139, 118, 146, 116, 128, 85, 56, 41, 15, 176, 236, 85, 37, 9, 62, 71,
    30, 17, 119, 118, 255, 17, 18, 138, 101, 38, 60, 138, 55, 70, 43, 26, 142, 146, 36, 19, 30,
    171, 255, 97, 27, 20, 138, 45, 61, 62, 219, 1, 81, 188, 64, 32, 41, 20, 117, 151, 142, 20, 21,
    163, 112, 19, 12, 61, 195, 128, 48, 4, 24,
];
