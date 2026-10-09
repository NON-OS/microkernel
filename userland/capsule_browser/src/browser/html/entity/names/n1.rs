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

use super::rows::rows;

/// Named references `LeftUpTeeVector;` to `SquareIntersection;`, sorted by bytes.
pub static ROWS: &[(&str, char, char)] = rows! {
    "LeftUpTeeVector" 0x2960 "LeftUpVector" 0x21BF "LeftUpVectorBar" 0x2958 "LeftVector" 0x21BC
    "LeftVectorBar" 0x2952 "Leftarrow" 0x21D0 "Leftrightarrow" 0x21D4 "LessEqualGreater" 0x22DA
    "LessFullEqual" 0x2266 "LessGreater" 0x2276 "LessLess" 0x2AA1 "LessSlantEqual" 0x2A7D
    "LessTilde" 0x2272 "Lfr" 0x1D50F "Ll" 0x22D8 "Lleftarrow" 0x21DA "Lmidot" 0x13F
    "LongLeftArrow" 0x27F5 "LongLeftRightArrow" 0x27F7 "LongRightArrow" 0x27F6
    "Longleftarrow" 0x27F8 "Longleftrightarrow" 0x27FA "Longrightarrow" 0x27F9 "Lopf" 0x1D543
    "LowerLeftArrow" 0x2199 "LowerRightArrow" 0x2198 "Lscr" 0x2112 "Lsh" 0x21B0 "Lstrok" 0x141
    "Lt" 0x226A "Map" 0x2905 "Mcy" 0x41C "MediumSpace" 0x205F "Mellintrf" 0x2133 "Mfr" 0x1D510
    "MinusPlus" 0x2213 "Mopf" 0x1D544 "Mscr" 0x2133 "Mu" 0x39C "NJcy" 0x40A "Nacute" 0x143
    "Ncaron" 0x147 "Ncedil" 0x145 "Ncy" 0x41D "NegativeMediumSpace" 0x200B
    "NegativeThickSpace" 0x200B "NegativeThinSpace" 0x200B "NegativeVeryThinSpace" 0x200B
    "NestedGreaterGreater" 0x226B "NestedLessLess" 0x226A "NewLine" 0xA "Nfr" 0x1D511
    "NoBreak" 0x2060 "NonBreakingSpace" 0xA0 "Nopf" 0x2115 "Not" 0x2AEC "NotCongruent" 0x2262
    "NotCupCap" 0x226D "NotDoubleVerticalBar" 0x2226 "NotElement" 0x2209 "NotEqual" 0x2260
    "NotEqualTilde" 0x2242+0x338 "NotExists" 0x2204 "NotGreater" 0x226F "NotGreaterEqual" 0x2271
    "NotGreaterFullEqual" 0x2267+0x338 "NotGreaterGreater" 0x226B+0x338 "NotGreaterLess" 0x2279
    "NotGreaterSlantEqual" 0x2A7E+0x338 "NotGreaterTilde" 0x2275 "NotHumpDownHump" 0x224E+0x338
    "NotHumpEqual" 0x224F+0x338 "NotLeftTriangle" 0x22EA "NotLeftTriangleBar" 0x29CF+0x338
    "NotLeftTriangleEqual" 0x22EC "NotLess" 0x226E "NotLessEqual" 0x2270 "NotLessGreater" 0x2278
    "NotLessLess" 0x226A+0x338 "NotLessSlantEqual" 0x2A7D+0x338 "NotLessTilde" 0x2274
    "NotNestedGreaterGreater" 0x2AA2+0x338 "NotNestedLessLess" 0x2AA1+0x338 "NotPrecedes" 0x2280
    "NotPrecedesEqual" 0x2AAF+0x338 "NotPrecedesSlantEqual" 0x22E0 "NotReverseElement" 0x220C
    "NotRightTriangle" 0x22EB "NotRightTriangleBar" 0x29D0+0x338 "NotRightTriangleEqual" 0x22ED
    "NotSquareSubset" 0x228F+0x338 "NotSquareSubsetEqual" 0x22E2 "NotSquareSuperset" 0x2290+0x338
    "NotSquareSupersetEqual" 0x22E3 "NotSubset" 0x2282+0x20D2 "NotSubsetEqual" 0x2288
    "NotSucceeds" 0x2281 "NotSucceedsEqual" 0x2AB0+0x338 "NotSucceedsSlantEqual" 0x22E1
    "NotSucceedsTilde" 0x227F+0x338 "NotSuperset" 0x2283+0x20D2 "NotSupersetEqual" 0x2289
    "NotTilde" 0x2241 "NotTildeEqual" 0x2244 "NotTildeFullEqual" 0x2247 "NotTildeTilde" 0x2249
    "NotVerticalBar" 0x2224 "Nscr" 0x1D4A9 "Ntilde" 0xD1 "Nu" 0x39D "OElig" 0x152 "Oacute" 0xD3
    "Ocirc" 0xD4 "Ocy" 0x41E "Odblac" 0x150 "Ofr" 0x1D512 "Ograve" 0xD2 "Omacr" 0x14C "Omega" 0x3A9
    "Omicron" 0x39F "Oopf" 0x1D546 "OpenCurlyDoubleQuote" 0x201C "OpenCurlyQuote" 0x2018
    "Or" 0x2A54 "Oscr" 0x1D4AA "Oslash" 0xD8 "Otilde" 0xD5 "Otimes" 0x2A37 "Ouml" 0xD6
    "OverBar" 0x203E "OverBrace" 0x23DE "OverBracket" 0x23B4 "OverParenthesis" 0x23DC
    "PartialD" 0x2202 "Pcy" 0x41F "Pfr" 0x1D513 "Phi" 0x3A6 "Pi" 0x3A0 "PlusMinus" 0xB1
    "Poincareplane" 0x210C "Popf" 0x2119 "Pr" 0x2ABB "Precedes" 0x227A "PrecedesEqual" 0x2AAF
    "PrecedesSlantEqual" 0x227C "PrecedesTilde" 0x227E "Prime" 0x2033 "Product" 0x220F
    "Proportion" 0x2237 "Proportional" 0x221D "Pscr" 0x1D4AB "Psi" 0x3A8 "QUOT" 0x22 "Qfr" 0x1D514
    "Qopf" 0x211A "Qscr" 0x1D4AC "RBarr" 0x2910 "REG" 0xAE "Racute" 0x154 "Rang" 0x27EB
    "Rarr" 0x21A0 "Rarrtl" 0x2916 "Rcaron" 0x158 "Rcedil" 0x156 "Rcy" 0x420 "Re" 0x211C
    "ReverseElement" 0x220B "ReverseEquilibrium" 0x21CB "ReverseUpEquilibrium" 0x296F "Rfr" 0x211C
    "Rho" 0x3A1 "RightAngleBracket" 0x27E9 "RightArrow" 0x2192 "RightArrowBar" 0x21E5
    "RightArrowLeftArrow" 0x21C4 "RightCeiling" 0x2309 "RightDoubleBracket" 0x27E7
    "RightDownTeeVector" 0x295D "RightDownVector" 0x21C2 "RightDownVectorBar" 0x2955
    "RightFloor" 0x230B "RightTee" 0x22A2 "RightTeeArrow" 0x21A6 "RightTeeVector" 0x295B
    "RightTriangle" 0x22B3 "RightTriangleBar" 0x29D0 "RightTriangleEqual" 0x22B5
    "RightUpDownVector" 0x294F "RightUpTeeVector" 0x295C "RightUpVector" 0x21BE
    "RightUpVectorBar" 0x2954 "RightVector" 0x21C0 "RightVectorBar" 0x2953 "Rightarrow" 0x21D2
    "Ropf" 0x211D "RoundImplies" 0x2970 "Rrightarrow" 0x21DB "Rscr" 0x211B "Rsh" 0x21B1
    "RuleDelayed" 0x29F4 "SHCHcy" 0x429 "SHcy" 0x428 "SOFTcy" 0x42C "Sacute" 0x15A "Sc" 0x2ABC
    "Scaron" 0x160 "Scedil" 0x15E "Scirc" 0x15C "Scy" 0x421 "Sfr" 0x1D516 "ShortDownArrow" 0x2193
    "ShortLeftArrow" 0x2190 "ShortRightArrow" 0x2192 "ShortUpArrow" 0x2191 "Sigma" 0x3A3
    "SmallCircle" 0x2218 "Sopf" 0x1D54A "Sqrt" 0x221A "Square" 0x25A1 "SquareIntersection" 0x2293
};
