#!/usr/bin/env python3
# NONOS Operating System
# Copyright (C) 2026 NONOS Contributors
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU Affero General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version. See <https://www.gnu.org/licenses/>.
"""Build an extraction crate for one kernel file and the modules it refers to.

mirror_crate.py mirrors a single file, which only works for files that name
nothing else in the kernel. Most kernel files name their neighbours through
`crate::` and `super::`, so this follows those paths (and the `pub use`
re-exports in each directory's mod.rs) to the files that define what is named,
recreates their place in the kernel's module tree with real directories and
`#[path]`, and forwards the target file's functions from the crate root with
mirror_crate.py. Modules gated off an x86_64 host build are not followed. A
closure that reaches an unsupported external crate, or grows past the file
bound, is refused rather than patched.

The kernel source is still never copied: every file is included by `#[path]`,
so the MIR Charon reads is the MIR rustc compiles for the kernel.

usage: closure_crate.py SRC NAME [--max-files N]
"""
import json, os, re, shutil, subprocess, sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
SRC = ROOT / 'src'
TREE = ROOT / 'verification/extraction/tree'
LEANDIR = ROOT / 'verification/extraction/lean/NonosExtraction'
EXTERNAL = re.compile(r'(?:^\s*(?:pub\s+)?use\s+|[^\w:])(spin|x86_64|bitflags|log|lazy_static|hashbrown|heapless|blake3|sha2|sha3|zeroize|subtle|rand_core|curve25519_dalek|ed25519_dalek|aes|chacha20|poly1305|ring)::', re.M)
SUPPORTED = {
    'spin': 'spin = { version = "0.9", default-features = false, features = ["mutex", "spin_mutex", "lazy", "rwlock"] }',
    'heapless': 'heapless = { version = "0.8", default-features = false }',
    'bitflags': 'bitflags = { version = "2.4", default-features = false }',
    'sha3': 'sha3 = { version = "0.10", default-features = false }',
    'sha2': 'sha2 = { version = "0.10", default-features = false, features = ["force-soft"] }',
}
DEPS = set()
REF = re.compile(r'\b(crate|super(?:::super)*)((?:::[A-Za-z_]\w*)+)')


def run(cmd, cwd, timeout=600):
    try:
        p = subprocess.run(cmd, cwd=cwd, capture_output=True, text=True, timeout=timeout)
        return p.returncode, (p.stdout or '') + (p.stderr or '')
    except subprocess.TimeoutExpired:
        return 124, 'timed out'


def module_of(path):
    """Module path components of a kernel source file."""
    rel = path.relative_to(SRC).with_suffix('')
    parts = list(rel.parts)
    if parts[-1] == 'mod':
        parts = parts[:-1]
    return parts


DEF = r'(?:pub(?:\([^)]*\))?\s+)?(?:const\s+fn|fn|struct|enum|trait|type|const|static|union|mod)\s+%s\b'


def reexports(d):
    """(submodule path, names or '*') for each `pub use` in D/mod.rs."""
    text = (SRC.joinpath(*d) / 'mod.rs').read_text(errors='replace')
    out = []
    for m in re.finditer(r'^\s*pub(?:\([^)]*\))?\s+use\s+(?:self::)?([A-Za-z_][\w:]*?)(?:::\{([^}]*)\}|::(\*|[A-Za-z_]\w*))\s*;', text, re.M):
        base = [x for x in m.group(1).split('::') if x]
        names = [x.strip().split(' as ')[0] for x in (m.group(2) or m.group(3)).split(',') if x.strip()]
        out.append((base, names))
    return out


def resolve(parts, depth=0):
    """Deepest module file for an absolute module path, following `pub use`
    re-exports out of a directory's mod.rs: ('file', comps) for a leaf .rs,
    ('dir', comps) when the item lives in a mod.rs itself."""
    if depth > 6:
        return None
    for i in range(1, len(parts) + 1):
        base = SRC.joinpath(*parts[:i])
        if base.with_suffix('.rs').is_file():
            return ('file', tuple(parts[:i]))
        if (base / 'mod.rs').is_file():
            if i == len(parts):
                return ('dir', tuple(parts[:i]))
            nxt = parts[i]
            if base.joinpath(nxt).with_suffix('.rs').is_file() or (base / nxt / 'mod.rs').is_file():
                continue
            d = parts[:i]
            for sub, names in reexports(d):
                if nxt in names:
                    return resolve(d + sub + [nxt], depth + 1)
            for sub, names in reexports(d):
                if names == ['*']:
                    f = SRC.joinpath(*(d + sub))
                    cand = [f.with_suffix('.rs')] if f.with_suffix('.rs').is_file() else list(f.rglob('*.rs'))
                    for c in cand:
                        if re.search(DEF % re.escape(nxt), c.read_text(errors='replace')):
                            return resolve(d + sub + [nxt], depth + 1)
            return ('dir', tuple(d))
        return None
    return None


def expand(text):
    """Flatten `crate::{a::b, c::{d, e}}` use-trees into plain paths."""
    out = []
    for m in re.finditer(r'\b(crate|super(?:::super)*)::\{', text):
        i = m.end(); depth = 1; j = i
        while j < len(text) and depth:
            depth += {'{': 1, '}': -1}.get(text[j], 0); j += 1
        body = text[i:j - 1]
        stack = [(m.group(1), body)]
        while stack:
            pre, b = stack.pop()
            parts, d, cur = [], 0, ''
            for ch in b:
                if ch == '{': d += 1
                if ch == '}': d -= 1
                if ch == ',' and d == 0:
                    parts.append(cur); cur = ''
                else:
                    cur += ch
            parts.append(cur)
            for q in parts:
                q = q.strip()
                if not q or q == 'self':
                    continue
                if '{' in q:
                    k = q.index('{')
                    stack.append((pre + '::' + q[:k].rstrip(':').strip(), q[k + 1:q.rindex('}')]))
                else:
                    out.append(pre + '::' + q.split(' as ')[0].strip())
    return text + '\n' + '\n'.join(out)


OFF_CFG = re.compile(r'#\[cfg\((?:test|target_arch\s*=\s*"(?:aarch64|riscv64)"|all\(target_arch\s*=\s*"(?:aarch64|riscv64)"[^\]]*|any\(target_arch\s*=\s*"(?:aarch64|riscv64)"\s*(?:,\s*target_arch\s*=\s*"(?:aarch64|riscv64)"\s*)*\)|kani|feature\s*=\s*"[^"]*")\)\]')


def strip_off_cfg(text):
    """Drop the item after a cfg attribute that is false on an x86_64 host build
    without features: a line, or a braced block."""
    out, i = [], 0
    for m in OFF_CFG.finditer(text):
        out.append(text[i:m.start()])
        j = m.end()
        k = j
        while k < len(text) and text[k] not in ';{':
            k += 1
        if k < len(text) and text[k] == '{':
            depth = 0
            while k < len(text):
                depth += {'{': 1, '}': -1}.get(text[k], 0)
                k += 1
                if depth == 0:
                    break
        else:
            k += 1
        i = k
    out.append(text[i:])
    return ''.join(out)


def compiled_files(modrs):
    """Files a mod.rs pulls into an x86_64 host build, following `mod x;`."""
    out, todo = [], [modrs]
    while todo:
        f = todo.pop()
        out.append(f)
        text = strip_off_cfg(re.sub(r'//[^\n]*', '', f.read_text(errors='replace')))
        here = f.parent if f.name == 'mod.rs' else f.with_suffix('')
        for m in re.finditer(r'(#\[path\s*=\s*"([^"]+)"\]\s*)?(?:pub(?:\([^)]*\))?\s+)?mod\s+(\w+)\s*;', text):
            if m.group(2):
                g = (f.parent / m.group(2)).resolve()
                if g.is_file(): todo.append(g)
                continue
            n = m.group(3)
            if (here / (n + '.rs')).is_file(): todo.append(here / (n + '.rs'))
            elif (here / n / 'mod.rs').is_file(): todo.append(here / n / 'mod.rs')
    return out


def refs(path):
    text = path.read_text(errors='replace')
    text = re.sub(r'//[^\n]*', '', text)
    text = strip_off_cfg(text)
    text = expand(text)
    me = module_of(path)
    out = set()
    for m in REF.finditer(text):
        head, tail = m.group(1), [x for x in m.group(2).split('::') if x]
        if head == 'crate':
            out.add(tuple(tail))
        else:
            ups = head.count('super')
            base = me[:len(me) - ups] if ups <= len(me) else []
            out.add(tuple(base + tail))
    return text, out


def closure(target, max_files):
    nodes = {}                      # comps -> 'file' | 'dir'
    todo = [target]
    seen = set()
    why = {target: None}
    while todo:
        f = todo.pop()
        if f in seen:
            continue
        seen.add(f)
        if len(seen) > max_files:
            return None, 'closure over %d files' % max_files
        text, rs = refs(f)
        ext = set(EXTERNAL.findall(text))
        DEPS.update(ext & set(SUPPORTED))
        if (ext - set(SUPPORTED)) or len(seen) > max_files:
            if os.environ.get('CLOSURE_TRACE'):
                g = f
                while g is not None:
                    print('   <-', g.relative_to(ROOT), why[g][1] if why.get(g) else '')
                    g = why[g][0] if why.get(g) else None
            if len(seen) > max_files:
                return None, 'closure over %d files' % max_files
            return None, 'external crate %s in %s' % (sorted(ext - set(SUPPORTED)), f.relative_to(ROOT))
        for r in rs:
            hit = resolve(list(r))
            if not hit:
                continue
            kind, comps = hit
            if any(comps[:i] in nodes and nodes[comps[:i]] == 'dir' for i in range(1, len(comps))):
                continue
            if nodes.get(comps) == 'dir':
                continue
            if kind == 'file' and SRC.joinpath(*comps).is_dir():
                comps = comps[:-1]
                kind = 'dir'
                if not comps or not (SRC.joinpath(*comps) / 'mod.rs').is_file():
                    return None, 'module file with children and no parent mod.rs'
                if nodes.get(comps) == 'dir':
                    continue
            nodes[comps] = kind
            if kind == 'file':
                g = SRC.joinpath(*comps).with_suffix('.rs'); why.setdefault(g, (f, '::'.join(r))); todo.append(g)
            else:
                for g in compiled_files(SRC.joinpath(*comps) / 'mod.rs'):
                    why.setdefault(g, (f, '::'.join(r) + ' [dir]')); todo.append(g)
    tcomps = tuple(module_of(target))
    if not any(tcomps[:i] in nodes and nodes[tcomps[:i]] == 'dir' for i in range(1, len(tcomps) + 1)):
        nodes[tcomps] = 'file'
    # drop leaves covered by an included directory
    for c in list(nodes):
        if any(c[:i] in nodes and nodes[c[:i]] == 'dir' for i in range(1, len(c))):
            del nodes[c]
    return nodes, len(seen)


def covered(nodes, hit):
    if not hit:
        return False
    comps = hit[1]
    return any(comps[:i] in nodes for i in range(1, len(comps) + 1))


def write_tree(cdir, nodes):
    src = cdir / 'src'
    children = {}
    for comps, kind in nodes.items():
        for i in range(1, len(comps)):
            children.setdefault(comps[:i - 1], set()).add(comps[i - 1])
        children.setdefault(comps[:-1], set()).add(comps[-1])
    decls = {}
    for parent, kids in children.items():
        lines = []
        pdir = src.joinpath(*parent)
        for k in sorted(kids):
            c = parent + (k,)
            kind = nodes.get(c)
            if kind == 'file':
                real = SRC.joinpath(*c).with_suffix('.rs')
            elif kind == 'dir':
                real = SRC.joinpath(*c) / 'mod.rs'
            else:
                lines.append('pub mod %s;' % k)
                continue
            rel = os.path.relpath(real, pdir)
            lines.append('#[path = "%s"]\npub mod %s;' % (rel, k))
        decls[parent] = lines
    for parent, lines in decls.items():
        if parent == ():
            continue
        d = src.joinpath(*parent)
        d.mkdir(parents=True, exist_ok=True)
        kids = children.get(parent, set())
        uses = []
        realmod = SRC.joinpath(*parent) / 'mod.rs'
        if realmod.is_file():
            for m in re.finditer(r'^\s*(pub(?:\([^)]*\))?\s+use\s+)(?:self::)?([A-Za-z_]\w*)((?:::\w+)*)::(\{[^}]*\}|\*|\w+(?:\s+as\s+\w+)?)\s*;', realmod.read_text(errors='replace'), re.M):
                if m.group(2) not in kids:
                    continue
                mid = [x for x in m.group(3).split('::') if x]
                names = m.group(4)
                if names.startswith('{'):
                    keep = [n.strip() for n in names[1:-1].split(',') if n.strip()
                            and covered(nodes, resolve(list(parent) + [m.group(2)] + mid + [n.strip().split(' as ')[0]]))]
                    if not keep:
                        continue
                    names = '{' + ', '.join(keep) + '}'
                elif names != '*' and not covered(nodes, resolve(list(parent) + [m.group(2)] + mid + [names.split(' as ')[0]])):
                    continue
                uses.append('%s%s%s::%s;' % (m.group(1), m.group(2), ''.join('::' + x for x in mid), names))
        (d / 'mod.rs').write_text('// NONOS Operating System (AGPL-3.0-or-later)\n\n'
                                  + '\n\n'.join(lines) + '\n' + ('\n' + '\n'.join(uses) + '\n' if uses else ''))
    return decls.get((), [])


def main():
    target = ROOT / sys.argv[1]
    name = sys.argv[2]
    max_files = 80
    if '--max-files' in sys.argv:
        max_files = int(sys.argv[sys.argv.index('--max-files') + 1])
    nodes, info = closure(target, max_files)
    if nodes is None:
        print('skip', sys.argv[1], info); return 2
    cdir = TREE / name
    if cdir.exists():
        shutil.rmtree(cdir)
    crate = 'nonos_x_%s' % name
    lean = ''.join(w.capitalize() for w in name.split('_'))[:40]
    tcomps = module_of(target)
    rc, out = run([sys.executable, str(ROOT / 'tools/extraction/mirror_crate.py'),
                   str(target.relative_to(ROOT)), '--crate', crate, '--out', str(cdir),
                   '--module', 'zzmod', '--lean-module', lean,
                   '--emit-lean', str(LEANDIR / ('%sRefinement.lean' % lean)),
                   '--repo-root', str(ROOT)], ROOT)
    if rc != 0:
        print('skip', sys.argv[1], 'mirror'); return 2
    starts = re.findall(r"--start-from '([^']+)'", out)
    if not starts:
        print('skip', sys.argv[1], 'no forwardable functions')
        shutil.rmtree(cdir); os.remove(LEANDIR / ('%sRefinement.lean' % lean)); return 2
    lib = (cdir / 'src/lib.rs').read_text()
    top = write_tree(cdir, nodes)
    rust_mod = 'crate::' + '::'.join(tcomps)
    lib = re.sub(r'#\[path = "[^"]+"\]\npub mod zzmod;\n',
                 'extern crate alloc;\n\n' + '\n\n'.join(top) + '\n', lib)
    for owner in set(re.findall(r'zzmod::([A-Z]\w*)', lib)):
        home = None
        for c, k in nodes.items():
            files = [SRC.joinpath(*c).with_suffix('.rs')] if k == 'file' else compiled_files(SRC.joinpath(*c) / 'mod.rs')
            for f in files:
                if re.search(r'^\s*pub(?:\([^)]*\))?\s+(?:struct|enum|type)\s+%s\b' % owner, f.read_text(errors='replace'), re.M):
                    home = module_of(f); break
            if home: break
        if home and home != tcomps:
            lib = lib.replace('zzmod::%s' % owner, 'crate::%s::%s' % ('::'.join(home), owner))
            ref = LEANDIR / ('%sRefinement.lean' % lean)
            ref.write_text(re.sub(r'zzmod\.%s(?![.\w])' % owner, '%s.%s' % ('.'.join(home), owner), ref.read_text()))
    lib = lib.replace('zzmod::', rust_mod + '::')
    lib = lib.replace('//! Extraction root mirroring', '//! Extraction root, with the modules it refers to, mirroring')
    (cdir / 'src/lib.rs').write_text(lib)
    ref = LEANDIR / ('%sRefinement.lean' % lean)
    ref.write_text(ref.read_text().replace('zzmod.', '.'.join(tcomps) + '.').replace(
        'zzmod, on the extracted code', '%s, on the extracted code' % tcomps[-1]))
    toml = (cdir / 'Cargo.toml').read_text()
    if DEPS:
        toml = toml.rstrip('\n') + '\n' + '\n'.join(SUPPORTED[d] for d in sorted(DEPS)) + '\n'
        (cdir / 'Cargo.toml').write_text(toml)
    rc, out = run(['cargo', 'check', '--quiet'], cdir)
    if rc != 0:
        errs = [l for l in out.splitlines() if l.startswith('error')][:3]
        print('skip', sys.argv[1], 'cargo check', errs); return 3
    args = ['charon', 'cargo', '--preset=aeneas']
    for s in starts:
        args += ['--start-from', s]
    args += ['--dest-file', '%s.llbc' % name]
    rc, out = run(args, cdir)
    if rc != 0:
        print('skip', sys.argv[1], 'charon', out.strip().splitlines()[-1:]); return 4
    dest = Path('/tmp/tree') / name
    shutil.rmtree(dest, ignore_errors=True); dest.mkdir(parents=True)
    rc, out = run(['aeneas', '-backend', 'lean', '%s.llbc' % name, '-dest', str(dest)], cdir)
    if rc != 0:
        print('skip', sys.argv[1], 'aeneas', out.strip().splitlines()[-1:]); return 5
    produced = list(dest.glob('*.lean'))
    if not produced:
        print('skip', sys.argv[1], 'aeneas produced nothing'); return 5
    shutil.copy(produced[0], LEANDIR / ('%s.lean' % lean))
    print('ok', sys.argv[1], name, len(starts), 'files=%s' % info)
    print(json.dumps({'name': name, 'crate': crate, 'dir': str(cdir.relative_to(ROOT)),
                      'lean': lean, 'generated': produced[0].name, 'starts': starts,
                      'source': sys.argv[1]}))
    return 0


if __name__ == '__main__':
    sys.exit(main())
