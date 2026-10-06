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

/// Named references `AElig;` to `LeftUpDownVector;`, sorted by bytes.
pub static ROWS: &[(&str, char, char)] = rows! {
    "AElig" 0xC6 "AMP" 0x26 "Aacute" 0xC1 "Abreve" 0x102 "Acirc" 0xC2 "Acy" 0x410 "Afr" 0x1D504
    "Agrave" 0xC0 "Alpha" 0x391 "Amacr" 0x100 "And" 0x2A53 "Aogon" 0x104 "Aopf" 0x1D538
    "ApplyFunction" 0x2061 "Aring" 0xC5 "Ascr" 0x1D49C "Assign" 0x2254 "Atilde" 0xC3 "Auml" 0xC4
    "Backslash" 0x2216 "Barv" 0x2AE7 "Barwed" 0x2306 "Bcy" 0x411 "Because" 0x2235
    "Bernoullis" 0x212C "Beta" 0x392 "Bfr" 0x1D505 "Bopf" 0x1D539 "Breve" 0x2D8 "Bscr" 0x212C
    "Bumpeq" 0x224E "CHcy" 0x427 "COPY" 0xA9 "Cacute" 0x106 "Cap" 0x22D2
    "CapitalDifferentialD" 0x2145 "Cayleys" 0x212D "Ccaron" 0x10C "Ccedil" 0xC7 "Ccirc" 0x108
    "Cconint" 0x2230 "Cdot" 0x10A "Cedilla" 0xB8 "CenterDot" 0xB7 "Cfr" 0x212D "Chi" 0x3A7
    "CircleDot" 0x2299 "CircleMinus" 0x2296 "CirclePlus" 0x2295 "CircleTimes" 0x2297
    "ClockwiseContourIntegral" 0x2232 "CloseCurlyDoubleQuote" 0x201D "CloseCurlyQuote" 0x2019
    "Colon" 0x2237 "Colone" 0x2A74 "Congruent" 0x2261 "Conint" 0x222F "ContourIntegral" 0x222E
    "Copf" 0x2102 "Coproduct" 0x2210 "CounterClockwiseContourIntegral" 0x2233 "Cross" 0x2A2F
    "Cscr" 0x1D49E "Cup" 0x22D3 "CupCap" 0x224D "DD" 0x2145 "DDotrahd" 0x2911 "DJcy" 0x402
    "DScy" 0x405 "DZcy" 0x40F "Dagger" 0x2021 "Darr" 0x21A1 "Dashv" 0x2AE4 "Dcaron" 0x10E
    "Dcy" 0x414 "Del" 0x2207 "Delta" 0x394 "Dfr" 0x1D507 "DiacriticalAcute" 0xB4
    "DiacriticalDot" 0x2D9 "DiacriticalDoubleAcute" 0x2DD "DiacriticalGrave" 0x60
    "DiacriticalTilde" 0x2DC "Diamond" 0x22C4 "DifferentialD" 0x2146 "Dopf" 0x1D53B "Dot" 0xA8
    "DotDot" 0x20DC "DotEqual" 0x2250 "DoubleContourIntegral" 0x222F "DoubleDot" 0xA8
    "DoubleDownArrow" 0x21D3 "DoubleLeftArrow" 0x21D0 "DoubleLeftRightArrow" 0x21D4
    "DoubleLeftTee" 0x2AE4 "DoubleLongLeftArrow" 0x27F8 "DoubleLongLeftRightArrow" 0x27FA
    "DoubleLongRightArrow" 0x27F9 "DoubleRightArrow" 0x21D2 "DoubleRightTee" 0x22A8
    "DoubleUpArrow" 0x21D1 "DoubleUpDownArrow" 0x21D5 "DoubleVerticalBar" 0x2225 "DownArrow" 0x2193
    "DownArrowBar" 0x2913 "DownArrowUpArrow" 0x21F5 "DownBreve" 0x311 "DownLeftRightVector" 0x2950
    "DownLeftTeeVector" 0x295E "DownLeftVector" 0x21BD "DownLeftVectorBar" 0x2956
    "DownRightTeeVector" 0x295F "DownRightVector" 0x21C1 "DownRightVectorBar" 0x2957
    "DownTee" 0x22A4 "DownTeeArrow" 0x21A7 "Downarrow" 0x21D3 "Dscr" 0x1D49F "Dstrok" 0x110
    "ENG" 0x14A "ETH" 0xD0 "Eacute" 0xC9 "Ecaron" 0x11A "Ecirc" 0xCA "Ecy" 0x42D "Edot" 0x116
    "Efr" 0x1D508 "Egrave" 0xC8 "Element" 0x2208 "Emacr" 0x112 "EmptySmallSquare" 0x25FB
    "EmptyVerySmallSquare" 0x25AB "Eogon" 0x118 "Eopf" 0x1D53C "Epsilon" 0x395 "Equal" 0x2A75
    "EqualTilde" 0x2242 "Equilibrium" 0x21CC "Escr" 0x2130 "Esim" 0x2A73 "Eta" 0x397 "Euml" 0xCB
    "Exists" 0x2203 "ExponentialE" 0x2147 "Fcy" 0x424 "Ffr" 0x1D509 "FilledSmallSquare" 0x25FC
    "FilledVerySmallSquare" 0x25AA "Fopf" 0x1D53D "ForAll" 0x2200 "Fouriertrf" 0x2131 "Fscr" 0x2131
    "GJcy" 0x403 "GT" 0x3E "Gamma" 0x393 "Gammad" 0x3DC "Gbreve" 0x11E "Gcedil" 0x122 "Gcirc" 0x11C
    "Gcy" 0x413 "Gdot" 0x120 "Gfr" 0x1D50A "Gg" 0x22D9 "Gopf" 0x1D53E "GreaterEqual" 0x2265
    "GreaterEqualLess" 0x22DB "GreaterFullEqual" 0x2267 "GreaterGreater" 0x2AA2
    "GreaterLess" 0x2277 "GreaterSlantEqual" 0x2A7E "GreaterTilde" 0x2273 "Gscr" 0x1D4A2
    "Gt" 0x226B "HARDcy" 0x42A "Hacek" 0x2C7 "Hat" 0x5E "Hcirc" 0x124 "Hfr" 0x210C
    "HilbertSpace" 0x210B "Hopf" 0x210D "HorizontalLine" 0x2500 "Hscr" 0x210B "Hstrok" 0x126
    "HumpDownHump" 0x224E "HumpEqual" 0x224F "IEcy" 0x415 "IJlig" 0x132 "IOcy" 0x401 "Iacute" 0xCD
    "Icirc" 0xCE "Icy" 0x418 "Idot" 0x130 "Ifr" 0x2111 "Igrave" 0xCC "Im" 0x2111 "Imacr" 0x12A
    "ImaginaryI" 0x2148 "Implies" 0x21D2 "Int" 0x222C "Integral" 0x222B "Intersection" 0x22C2
    "InvisibleComma" 0x2063 "InvisibleTimes" 0x2062 "Iogon" 0x12E "Iopf" 0x1D540 "Iota" 0x399
    "Iscr" 0x2110 "Itilde" 0x128 "Iukcy" 0x406 "Iuml" 0xCF "Jcirc" 0x134 "Jcy" 0x419 "Jfr" 0x1D50D
    "Jopf" 0x1D541 "Jscr" 0x1D4A5 "Jsercy" 0x408 "Jukcy" 0x404 "KHcy" 0x425 "KJcy" 0x40C
    "Kappa" 0x39A "Kcedil" 0x136 "Kcy" 0x41A "Kfr" 0x1D50E "Kopf" 0x1D542 "Kscr" 0x1D4A6
    "LJcy" 0x409 "LT" 0x3C "Lacute" 0x139 "Lambda" 0x39B "Lang" 0x27EA "Laplacetrf" 0x2112
    "Larr" 0x219E "Lcaron" 0x13D "Lcedil" 0x13B "Lcy" 0x41B "LeftAngleBracket" 0x27E8
    "LeftArrow" 0x2190 "LeftArrowBar" 0x21E4 "LeftArrowRightArrow" 0x21C6 "LeftCeiling" 0x2308
    "LeftDoubleBracket" 0x27E6 "LeftDownTeeVector" 0x2961 "LeftDownVector" 0x21C3
    "LeftDownVectorBar" 0x2959 "LeftFloor" 0x230A "LeftRightArrow" 0x2194 "LeftRightVector" 0x294E
    "LeftTee" 0x22A3 "LeftTeeArrow" 0x21A4 "LeftTeeVector" 0x295A "LeftTriangle" 0x22B2
    "LeftTriangleBar" 0x29CF "LeftTriangleEqual" 0x22B4 "LeftUpDownVector" 0x2951
};
