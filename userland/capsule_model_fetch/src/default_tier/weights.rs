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

/*
 * Each tier's pinned files, summed, smallest first: what a default is
 * weighed by. model_fetch_proofs holds this table equal to the pins.
 */
#[rustfmt::skip]
pub const WEIGHTS: &[(&str, u64)] = &[
    ("small", 491_400_032),
    ("qwen3-0.6b", 639_446_688),
    ("qwen3.5-0.8b", 833_592_096),
    ("medium", 1_117_320_736),
    ("coder-1.5b", 1_117_320_768),
    ("qwen3-1.7b", 1_834_426_016),
    ("large", 2_104_932_768),
    ("qwen3-4b", 2_497_280_256),
    ("xlarge", 4_683_073_632),
    ("coder-7b", 4_683_073_664),
    ("qwen3-8b", 5_027_783_488),
    ("coder-14b", 8_988_110_400),
    ("xxl", 8_988_110_496),
    ("qwen3-14b", 9_001_752_960),
    ("qwen3-30b-a3b", 18_556_685_824),
    ("qwen3.6-27b", 19_095_766_304),
    ("qwen3-32b", 19_762_149_024),
    ("coder-32b", 19_851_336_128),
    ("max", 19_851_336_384),
];
