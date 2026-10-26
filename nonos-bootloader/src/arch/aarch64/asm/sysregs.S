/* NØNOS Operating System
 * Copyright (C) 2026 NØNOS Contributors
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

/* The system registers the set/way walk reads and the operation it issues,
 * as macros, so a host test can put stand-ins in their place and run the
 * walk itself. These are the real ones, for the loader.
 *   read_clidr  reg     CLIDR_EL1
 *   read_mmfr2  reg     ID_AA64MMFR2_EL1
 *   read_ccsidr sel out CSSELR_EL1 = sel, then CCSIDR_EL1; the ISB makes
 *                       the selection visible to the read
 *   op_cisw     reg     DC CISW: clean and invalidate one set and way */
.macro read_clidr reg
    mrs     \reg, clidr_el1
.endm
.macro read_mmfr2 reg
    mrs     \reg, id_aa64mmfr2_el1
.endm
.macro read_ccsidr sel, out
    msr     csselr_el1, \sel
    isb
    mrs     \out, ccsidr_el1
.endm
.macro op_cisw reg
    dc      cisw, \reg
.endm
