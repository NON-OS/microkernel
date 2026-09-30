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

//! The model files the Qwen team publishes, written out here from the
//! LFS pointers of its Hugging Face GGUF repositories, independently of
//! the personality's tables, so a digest changed there fails a proof here.
//! In table order: Qwen2.5, its 14B and 32B, Qwen3, then Coder.

#[rustfmt::skip]
pub const PUBLISHED: [(&str, &str, u64, &str); 28] = [
    ("small", "qwen2.5-0.5b-instruct-q4_k_m.gguf", 491_400_032, "74a4da8c9fdbcd15bd1f6d01d621410d31c6fc00986f5eb687824e7b93d7a9db"),
    ("medium", "qwen2.5-1.5b-instruct-q4_k_m.gguf", 1_117_320_736, "6a1a2eb6d15622bf3c96857206351ba97e1af16c30d7a74ee38970e434e9407e"),
    ("large", "qwen2.5-3b-instruct-q4_k_m.gguf", 2_104_932_768, "626b4a6678b86442240e33df819e00132d3ba7dddfe1cdc4fbb18e0a9615c62d"),
    ("xlarge", "qwen2.5-7b-instruct-q4_k_m-00001-of-00002.gguf", 3_993_201_344, "dfce12e3862a5283ccfb88221b48480e58745165de856439950d0f22590580db"),
    ("xlarge", "qwen2.5-7b-instruct-q4_k_m-00002-of-00002.gguf", 689_872_288, "539cf93f78e887edea1c04e2d7d8cdaca9d01dae9c9025bcb8accbe29df3d72a"),
    ("xxl", "qwen2.5-14b-instruct-q4_k_m-00001-of-00003.gguf", 3_991_999_872, "a09ea5e7b1eafb1b30b241726c3cc3c905c96f14ad41e246ffa5f44e53904f68"),
    ("xxl", "qwen2.5-14b-instruct-q4_k_m-00002-of-00003.gguf", 3_989_373_504, "21b9457d079680d284e90ef69607c4b2d8ef64a09d4729cb7b5e1357bdba41ae"),
    ("xxl", "qwen2.5-14b-instruct-q4_k_m-00003-of-00003.gguf", 1_006_737_120, "c8d37006760a387a35216e070e6664d7da927f10be8eb870fef2e3d4833d9976"),
    ("max", "qwen2.5-32b-instruct-q4_k_m-00001-of-00005.gguf", 3_961_498_272, "403434e5c845452c013661d586b97d5a53cf207462d180a07c301eafa9390d05"),
    ("max", "qwen2.5-32b-instruct-q4_k_m-00002-of-00005.gguf", 3_948_996_064, "7371e5c5d717a8c20f526dbfce5d3f201dc3f02140d1c836314cd0756e02e8d7"),
    ("max", "qwen2.5-32b-instruct-q4_k_m-00003-of-00005.gguf", 3_993_478_688, "f023ccc294c3bd3b2eac5b2dd40dea3aa4ca1d06c7460ead67c36386cd62e8fc"),
    ("max", "qwen2.5-32b-instruct-q4_k_m-00004-of-00005.gguf", 3_950_347_744, "05fe76d941454390cd7aa0de3b342f83a1d1226a959479af85bbfae7cd93e771"),
    ("max", "qwen2.5-32b-instruct-q4_k_m-00005-of-00005.gguf", 3_997_015_616, "8c2e8ecc686129c37821bd9f3b3e251e6ae80deadd3d3348f7e3e84492a63c24"),
    ("qwen3-0.6b", "Qwen3-0.6B-Q8_0.gguf", 639_446_688, "9465e63a22add5354d9bb4b99e90117043c7124007664907259bd16d043bb031"),
    ("qwen3-1.7b", "Qwen3-1.7B-Q8_0.gguf", 1_834_426_016, "061b54daade076b5d3362dac252678d17da8c68f07560be70818cace6590cb1a"),
    ("qwen3-4b", "Qwen3-4B-Q4_K_M.gguf", 2_497_280_256, "7485fe6f11af29433bc51cab58009521f205840f5b4ae3a32fa7f92e8534fdf5"),
    ("qwen3-8b", "Qwen3-8B-Q4_K_M.gguf", 5_027_783_488, "d98cdcbd03e17ce47681435b5150e34c1417f50b5c0019dd560e4882c5745785"),
    ("qwen3-14b", "Qwen3-14B-Q4_K_M.gguf", 9_001_752_960, "500a8806e85ee9c83f3ae08420295592451379b4f8cf2d0f41c15dffeb6b81f0"),
    ("qwen3-30b-a3b", "Qwen3-30B-A3B-Q4_K_M.gguf", 18_556_685_824, "0d003f6662faee786ed5da3e31b29c978de5ae5d275c8794c606a7f3c01aa8f5"),
    ("qwen3-32b", "Qwen3-32B-Q4_K_M.gguf", 19_762_149_024, "efd971561896866f0e910cce52761ca77b1b138090c7f15fe284676d57d1f689"),
    ("coder-1.5b", "qwen2.5-coder-1.5b-instruct-q4_k_m.gguf", 1_117_320_768, "cc324af070c2ecbfd324a30884d2f951a7ff756aba85cb811a6ec436933bb046"),
    ("coder-7b", "qwen2.5-coder-7b-instruct-q4_k_m-00001-of-00002.gguf", 3_993_201_376, "89f120544682078148c5a86117de9af3a65c339111262f2d3ff01d80d48b14be"),
    ("coder-7b", "qwen2.5-coder-7b-instruct-q4_k_m-00002-of-00002.gguf", 689_872_288, "0183b3c850cfa96c31082c3af0123115300d3f62798c4448fa8f57bd0eac05e0"),
    ("coder-14b", "qwen2.5-coder-14b-instruct-q4_k_m-00001-of-00002.gguf", 8_000_444_480, "310a553a856d7b238c05ed1b0cb877c4dca5b65c2286324854a1397149a00af2"),
    ("coder-14b", "qwen2.5-coder-14b-instruct-q4_k_m-00002-of-00002.gguf", 987_665_920, "7a52538b39090d99ce93b1b05a406b805427171f890dbef8684c57fb28b2d96a"),
    ("coder-32b", "qwen2.5-coder-32b-instruct-q4_k_m-00001-of-00003.gguf", 7_990_120_512, "0a9145d25318b7584094e77044cc256bc9f1374c8f168a3812119242b456f69b"),
    ("coder-32b", "qwen2.5-coder-32b-instruct-q4_k_m-00002-of-00003.gguf", 7_943_826_304, "de1e27aa436e0856582eed095418fd3db8538b0d5d6e71b6362208b3a7f6d16f"),
    ("coder-32b", "qwen2.5-coder-32b-instruct-q4_k_m-00003-of-00003.gguf", 3_917_389_312, "4d893bec57ae6b2c898c0f2f0f9804a5d855dc7091255b76f3671cb8787919fe"),
];

/// Every tier the tables ship, in the same order.
pub const TIERS: [&str; 17] = [
    "small",
    "medium",
    "large",
    "xlarge",
    "xxl",
    "max",
    "qwen3-0.6b",
    "qwen3-1.7b",
    "qwen3-4b",
    "qwen3-8b",
    "qwen3-14b",
    "qwen3-30b-a3b",
    "qwen3-32b",
    "coder-1.5b",
    "coder-7b",
    "coder-14b",
    "coder-32b",
];
