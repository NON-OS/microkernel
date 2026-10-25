/* NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors
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

/* Clear RFLAGS.AC on every kernel entry.
 *
 * An interrupt or exception gate leaves AC as ring 3 had it, and ring 3 may
 * set it with popf. With AC set SMAP does not fault, so a handler entered
 * from a capsule that set it ran with user memory open. SYSCALL clears AC
 * through FMASK; the gates have no such mask. CLAC is #UD on a CPU without
 * SMAP, so the flag is cleared through the stack instead, which every
 * x86_64 part takes. iretq restores the interrupted RFLAGS, AC included, so
 * a kernel window opened by `with_user_access` is back open on return.
 */
.macro CLEAR_AC
    pushfq
    and     qword ptr [rsp], ~(1 << 18)
    popfq
.endm
