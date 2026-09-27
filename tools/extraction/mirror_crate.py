#!/usr/bin/env python3
# NONOS Operating System
# Copyright (C) 2026 NONOS Contributors
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU Affero General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
# GNU Affero General Public License for more details.
#
# You should have received a copy of the GNU Affero General Public License
# along with this program. If not, see <https://www.gnu.org/licenses/>.
"""Build an extraction crate that mirrors one kernel file, and list its entry points.

Charon reaches a function by path, and a path naming an inherent method does not
resolve, so every method needs a free function in the crate root that forwards to
it. Writing those by hand is the reason the extraction surface grew one file at a
time. This emits the crate and the --start-from lines for it.

The kernel source is never copied. It is included with #[path], so the MIR Charon
reads is the MIR rustc compiles for the kernel itself, and a change to the kernel
shows up here as a diff.

Signatures this cannot forward are reported rather than guessed at.
"""

import argparse
import re
import sys
from pathlib import Path

LICENSE = "// NONOS Operating System (AGPL-3.0-or-later)\n"

# Rust scalar to the Aeneas type the extracted Lean uses.
LEAN_TY = {'u8': 'Std.U8', 'u16': 'Std.U16', 'u32': 'Std.U32', 'u64': 'Std.U64',
           'usize': 'Std.Usize', 'i8': 'Std.I8', 'i16': 'Std.I16',
           'i32': 'Std.I32', 'i64': 'Std.I64', 'isize': 'Std.Isize',
           'bool': 'Bool'}

# Parameter and return types we know how to forward by value.
SCALAR = re.compile(r'^(?:u8|u16|u32|u64|usize|i8|i16|i32|i64|isize|bool|char)$')

IMPL = re.compile(r'^\s*impl\s+([A-Za-z_]\w*)\s*\{')
METHOD = re.compile(
    r'^\s*pub(?:\([^)]*\))?\s+(?:const\s+)?fn\s+([a-z_]\w*)\s*\(([^)]*)\)\s*'
    r'(?:->\s*([^{]+?))?\s*\{')
FREEFN = re.compile(
    r'^\s*pub(?:\([^)]*\))?\s+(?:const\s+)?fn\s+([a-z_]\w*)\s*\(([^)]*)\)\s*'
    r'(?:->\s*([^{]+?))?\s*\{')


def parse_params(raw, owner):
    """Return (recv, [(name, ty)]) or None when a parameter is not forwardable."""
    parts, depth, cur = [], 0, ''
    for ch in raw:
        if ch in '<([':
            depth += 1
        elif ch in '>)]':
            depth -= 1
        if ch == ',' and depth == 0:
            parts.append(cur.strip())
            cur = ''
        else:
            cur += ch
    if cur.strip():
        parts.append(cur.strip())

    recv, args = None, []
    for p in parts:
        if p in ('self', '&self', 'mut self'):
            # `mut self` is a by-value receiver; the mutation is local to the
            # callee and the value comes back in the return, so it forwards.
            recv = 'value'
            continue
        if p == '&mut self':
            recv = 'mut'
            continue
        if ':' not in p:
            return None
        name, ty = (x.strip() for x in p.split(':', 1))
        ty = ty.replace('Self', owner) if owner else ty
        if not (SCALAR.match(ty) or ty == owner):
            return None
        args.append((name, ty))
    return recv, args


def forwardable_ret(ret, owner):
    if ret is None:
        return ''
    ret = ret.strip().replace('Self', owner) if owner else ret.strip()
    if SCALAR.match(ret) or (owner and ret == owner):
        return ret
    return None


def scan(path):
    """Yield ('method', owner, name, params, ret) and ('free', None, ...)."""
    owner, depth = None, 0
    for line in path.read_text(errors='replace').splitlines():
        m = IMPL.match(line)
        if m:
            owner, depth = m.group(1), 0
        if owner is not None:
            depth += line.count('{') - line.count('}')
            if depth <= 0 and '}' in line and not IMPL.match(line):
                pass
        m = METHOD.match(line)
        if m:
            yield ('method' if owner else 'free', owner, m.group(1), m.group(2), m.group(3))


def main():
    ap = argparse.ArgumentParser(description=__doc__.split('\n', 1)[0])
    ap.add_argument('source', help='kernel file to mirror, relative to the repo root')
    ap.add_argument('--crate', required=True, help='crate name, e.g. nonos_uefi_attrs')
    ap.add_argument('--out', required=True, help='directory to create the crate in')
    ap.add_argument('--module', required=True, help='module name inside the crate')
    ap.add_argument('--repo-root', default='.')
    ap.add_argument('--emit-lean', metavar='PATH',
                    help='also write a refinement skeleton proving each forwarder '
                         'is its method')
    ap.add_argument('--lean-module', default=None,
                    help='generated Lean module name, e.g. Riscv')
    args = ap.parse_args()

    root = Path(args.repo_root).resolve()
    src = (root / args.source).resolve()
    if not src.is_file():
        print('no such file: %s' % src, file=sys.stderr)
        return 1
    out = Path(args.out).resolve()
    (out / 'src').mkdir(parents=True, exist_ok=True)

    rel = Path('../' * (len(out.relative_to(root).parts) + 1)) / args.source

    fwd, skipped = [], []
    for kind, owner, name, raw, ret in scan(src):
        parsed = parse_params(raw, owner)
        rty = forwardable_ret(ret, owner)
        if parsed is None or rty is None:
            skipped.append((owner or '-', name))
            continue
        recv, argv = parsed
        if recv == 'mut':
            skipped.append((owner or '-', name))
            continue
        sig, call = [], []
        if recv == 'value' and owner:
            sig.append('this: %s::%s' % (args.module, owner))
            call.append('this')
        for n, t in argv:
            t2 = '%s::%s' % (args.module, t) if owner and t == owner else t
            sig.append('%s: %s' % (n, t2))
            call.append(n)
        rt = ' -> %s::%s' % (args.module, rty) if owner and rty == owner else (
            ' -> %s' % rty if rty else '')
        fname = ('%s_%s' % (owner.lower(), name)) if owner else name
        if recv == 'value' and owner:
            body = '%s.%s(%s)' % (call[0], name, ', '.join(call[1:]))
        elif owner:
            body = '%s::%s::%s(%s)' % (args.module, owner, name, ', '.join(call))
        else:
            body = '%s::%s(%s)' % (args.module, name, ', '.join(call))
        lean_args = []
        if recv == 'value' and owner:
            lean_args.append('%s.%s' % (args.module, owner))
        for _n, t in argv:
            lean_args.append(LEAN_TY.get(t, '%s.%s' % (args.module, t)))
        fwd.append((fname, 'pub fn %s(%s)%s {\n    %s\n}' % (
            fname, ', '.join(sig), rt, body), owner, name, lean_args))

    (out / 'Cargo.toml').write_text(
        '# NONOS Operating System\n'
        '# Copyright (C) 2026 NONOS Contributors\n#\n'
        '# Extraction root mirroring %s. The kernel source is included with\n'
        '# #[path], never copied, so Charon reads the MIR rustc compiles.\n'
        '[package]\nname = "%s"\nversion = "0.1.0"\nedition = "2021"\n\n'
        '[lib]\npath = "src/lib.rs"\n\n[dependencies]\n' % (args.source, args.crate))

    lib = [LICENSE,
           '//! Extraction root mirroring `%s`.\n' % args.source,
           '//!\n//! The forwarding functions below exist because a Charon entry point cannot\n'
           '//! name an inherent method. They add no logic.\n\n',
           '#[path = "%s"]\npub mod %s;\n\n' % (rel.as_posix(), args.module)]
    for entry in fwd:
        lib.append(entry[1] + '\n\n')
    (out / 'src' / 'lib.rs').write_text(''.join(lib))

    print('crate      %s' % (out / 'Cargo.toml'))
    print('forwarded  %d' % len(fwd))
    print('skipped    %d' % len(skipped))
    for owner, name in skipped:
        print('   skip: %s::%s' % (owner, name))
    print('\nstart-from lines:')
    for entry in fwd:
        print("            --start-from '%s::%s' \\" % (args.crate, entry[0]))

    if args.emit_lean:
        mod = args.lean_module or args.module.capitalize()
        write_refinement(Path(args.emit_lean), args.crate, args.module, mod, fwd)
        print('\nrefinement skeleton %s' % args.emit_lean)
    return 0


REFINEMENT_HEAD = '''/-
NONOS Operating System
Copyright (C) 2026 NONOS Contributors

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU Affero General Public License as published by the Free
Software Foundation, either version 3 of the License, or (at your option) any
later version. See <https://www.gnu.org/licenses/>.

%(module)s, on the extracted code.

Charon cannot take an inherent method as an entry point, so the crate root
carries a free function per method and those are the names the manifest counts.
A wrapper that quietly did something other than call through would make every
theorem about the method a statement about code nobody runs, so each one is
proven to be its method below.

These are generated by tools/extraction/mirror_crate.py and are the floor, not
the point. The properties that matter go beneath them, and the shapes most of
them take are stated once in NonosExtraction.Shapes.
-/

import NonosExtraction.%(lean_module)s

open Aeneas Aeneas.Std Result
open %(crate)s

set_option linter.hashCommand false
set_option maxRecDepth 100000

namespace NonosExtraction.%(lean_module)s

/-! ### The forwarding functions add nothing -/

'''


def write_refinement(path, crate, module, lean_module, fwd):
    names = []
    body = []
    letters = 'abcdefg'
    for fname, _text, owner, method, lean_args in fwd:
        if len(lean_args) > len(letters):
            continue
        thm = 'the_%s_wrapper_is_its_method' % fname
        names.append(thm)
        target = ('%s.%s.%s' % (module, owner, method)) if owner else (
            '%s.%s' % (module, method))
        binder = ''.join(' (%s : %s)' % (letters[i], t)
                         for i, t in enumerate(lean_args))
        call = ''.join(' ' + letters[i] for i in range(len(lean_args)))
        body.append('theorem %s%s :\n    %s%s = %s%s := rfl\n'
                    % (thm, binder, fname, call, target, call))
    text = REFINEMENT_HEAD % {'module': module, 'crate': crate,
                              'lean_module': lean_module}
    text += '\n'.join(body)
    text += '\n/-! ### Axiom profile -/\n\n'
    for thm in names:
        text += '#print axioms NonosExtraction.%s.%s\n' % (lean_module, thm)
    text += '\nend NonosExtraction.%s\n' % lean_module
    path.write_text(text)


if __name__ == '__main__':
    sys.exit(main())
