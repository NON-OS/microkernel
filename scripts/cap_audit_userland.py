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
"""Which syscalls a capsule can reach, read from its source and from every
crate it links. Read by cap_audit.

Two crates are read function by function: the libc and nonos_abi, the
wrappers every syscall goes through. Linking them is not evidence of
anything, since each wraps every call the kernel has; calling a wrapper is.
Every other crate is read whole: a call anywhere in a linked crate counts for
every capsule that links it, because the audit cannot tell which of a
library's functions a capsule reaches, and must not call a bit unused because
of that.
"""

import re
from pathlib import Path

from cap_audit_rust import Source, cargo_manifest, lock_packages, module_files

WRAPPERS = {"libc": Path("userland/libc"), "nonos_abi": Path("userland/nonos_abi")}
STD_PAL = Path("toolchain/nonos-std/sys")
UPSTREAM = Path("userland/upstream-src")
VENDOR = Path("userland/vendor")
# What each libc feature links without the capsule calling it by name.
IMPLICIT = {"panic-handler": ("panic.rs",), "heap": ("heap/",)}
CONST_REF = re.compile(r"\bN_[A-Z0-9_]+\b")
# Service and endpoint names are dotted (net.tcp, driver.virtio_blk0) or
# joined with underscores (vfs_pool, crypto_pool).
NAME_LIT = re.compile(r"[a-z][\w\-]*(?:\.[\w\-]+)+|[a-z][a-z0-9]*(?:_[a-z0-9]+)+")
# A service reached by its fixed port rather than its name.
PORT_CONST = re.compile(r"\b(\w*PORT\w*)\s*:\s*u32\s*=\s*(\d+)\s*;")
# Client modules that write a service's name once, inside themselves, so a
# crate reaches the service by importing the module and never writes the name.
# Each maps the service to the pattern that imports its client and the
# directory the client lives in, whose own name and port are its definition,
# not a use of it.
SERVICE_CLIENTS = {
    "vfs_pool": (re.compile(r"\bclients\s*::\s*(?:vfs\b|\{[^}]*\bvfs\b)"),
                 "userland/app_skeleton/src/clients/vfs/"),
}
USE_STMT = re.compile(r"\buse\s+([\w:]+)::\{([^;]*)\}\s*;"
                      r"|\buse\s+([\w:]+)::(\w+)(?:\s+as\s+(\w+))?\s*;")
PUB_USE = re.compile(r"\bpub\s+use\s+([\w:]+?)"
                     r"(?:::\{([^}]*)\}|::(\w+)(?:\s+as\s+(\w+))?|::\*)\s*;")
LINK_SECTION = re.compile(r'#\[link_section\s*=\s*"\.nonos\.caps"\]\s*'
                          r"pub\s+static\s+\w+\s*:\s*u64\s*=\s*([^;]+);")


class Capsule:
    """One userland/<dir>/Capsule.mk."""

    def __init__(self, mk_path, root):
        text = mk_path.read_text()
        self.mk = mk_path.relative_to(root)
        fields = {}
        for m in re.finditer(r"^(CAPSULE_\w+)\s*[:?]?=\s*(.*?)\s*$", text, re.M):
            fields[m.group(1)] = m.group(2)
        self.fields = fields
        self.slug = fields.get("CAPSULE_SLUG", mk_path.parent.name)
        self.handle = fields.get("CAPSULE_HANDLE", "")
        self.dir = Path(fields.get("CAPSULE_DIR", str(mk_path.parent.relative_to(root))))
        self.bin_name = fields.get("CAPSULE_BIN_NAME", "")
        self.required = _int(fields.get("CAPSULE_REQUIRED_CAPS"))
        self.optional = _int(fields.get("CAPSULE_OPTIONAL_CAPS"))
        ceiling = fields.get("CAPSULE_CAPS_CEILING")
        self.ceiling = _int(ceiling) if ceiling else None
        self.mirror = fields.get("CAPSULE_KERNEL_MIRROR") or None
        self.prebuilt = fields.get("CAPSULE_PREBUILT_BIN") or None
        self.std = "std" in fields.get("CAPSULE_BUILD_STD", "") or bool(self.prebuilt)
        names = [self.handle]
        names += re.findall(r"(?:service|reply):\d+:(\S+)",
                            " ".join(fields.get(k, "") for k in (
                                "CAPSULE_SERVICE_ENDPOINT", "CAPSULE_REPLY_ENDPOINT",
                                "CAPSULE_INSTANCE_ENDPOINTS")))
        self.own_names = sorted(set(n for n in names if n))
        self.service_names = sorted(set(
            [self.handle] + re.findall(r"service:\d+:(\S+)", " ".join(
                fields.get(k, "") for k in ("CAPSULE_SERVICE_ENDPOINT",
                                            "CAPSULE_INSTANCE_ENDPOINTS")))) - {""})

    @property
    def granted(self):
        return self.required | self.optional


def _int(text):
    text = (text or "0").strip()
    return int(text.replace("_", ""), 0)


def capsules(root):
    return [Capsule(mk, root) for mk in sorted((root / "userland").glob("*/Capsule.mk"))]


# ---------------------------------------------------------------------------
# The wrapper crates, function by function


class WrapperFn:
    def __init__(self, crate, name, modules, cite):
        self.crate = crate
        self.name = name
        self.modules = modules
        self.cite = cite
        self.tags = set()
        self.callees = set()


class Wrappers:
    """Every function of the libc and nonos_abi, with the syscall tags each
    reaches through the functions it calls."""

    def __init__(self, root):
        self.root = root
        self.fns = {}          # name -> [WrapperFn]
        self.exports = {}      # crate key -> {name callable at the crate root}
        self.modules = {}      # crate key -> {module name}
        self.consts = {}       # const name -> tag, for constants a caller may name
        self.implicit = {}     # libc feature -> {fn name}
        self.crate_of = {}     # resolved dir -> crate key
        for key, rel in WRAPPERS.items():
            self._read(key, root / rel)
        self._close()

    def _read(self, key, base):
        self.crate_of[base.resolve()] = key
        files = [Source.read(p) for p in module_files(base / "src/lib.rs")]
        consts = {}
        for s in files:
            consts.update(s.tag_constants())
        self.consts.update(consts)
        exports, modules = set(), set()
        for s in files:
            rel = s.path.relative_to(base / "src")
            parts = set(rel.parent.parts) | ({rel.stem} - {"mod", "lib"})
            modules |= parts
            for name, start, end in s.functions():
                cite = f"{s.path.relative_to(self.root)}:{s.line(start)}"
                fn = WrapperFn(key, name, parts, cite)
                body = s.code[start:end]
                fn.tags |= {consts[c] for c in CONST_REF.findall(body) if c in consts}
                fn.tags |= {t for t, off in s.tags() if start <= off < end}
                fn.callees = {n for _, n, _ in s.calls(start, end)}
                self.fns.setdefault(name, []).append(fn)
            for feature, prefixes in IMPLICIT.items():
                if key == "libc" and any(str(rel).startswith(p) for p in prefixes):
                    self.implicit.setdefault(feature, set()).update(
                        n for n, _, _ in s.functions())
        lib = Source.read(base / "src/lib.rs")
        for m in PUB_USE.finditer(lib.code):
            if m.group(2) is not None:
                for item in m.group(2).split(","):
                    parts = item.split(" as ")
                    exports.add(parts[-1].strip())
                    if len(parts) == 2:
                        self._alias(parts[1].strip(), parts[0].strip())
            elif m.group(3):
                exports.add(m.group(4) or m.group(3))
                if m.group(4):
                    self._alias(m.group(4), m.group(3))
            else:
                glob_mod = m.group(1).split("::")[-1]
                exports.update(n for n, fs in self.fns.items()
                               if any(glob_mod in f.modules for f in fs))
        self.exports[key] = {e for e in exports if e}
        self.modules[key] = modules

    def _alias(self, alias, target):
        if target in self.fns and alias not in self.fns:
            self.fns[alias] = self.fns[target]

    def _close(self):
        changed = True
        while changed:
            changed = False
            for fns in self.fns.values():
                for fn in fns:
                    for callee in list(fn.callees):
                        for other in self.fns.get(callee, ()):
                            if not other.tags <= fn.tags:
                                fn.tags |= other.tags
                                changed = True

    def tags_of(self, name):
        out = set()
        for fn in self.fns.get(name, ()):
            out |= fn.tags
        return out


# ---------------------------------------------------------------------------
# Evidence from one crate


class Use:
    """One thing a crate does that the kernel may check: a syscall tag it
    reaches (through `via`, a wrapper, or written directly), or a service name
    it writes down, or the fixed port it sends to. `listed` marks a name
    written as one entry of an array of them, a table that names a service
    rather than a message sent to it."""

    def __init__(self, crate, cite, tags=(), via=None, literal=None, raw=False,
                 listed=False, port=None):
        self.crate = crate
        self.cite = cite
        self.tags = frozenset(tags)
        self.via = via
        self.literal = literal
        self.raw = raw
        self.listed = listed
        self.port = port


def _listed(code, off):
    """Whether the string literal at `off` is one entry of an array: a `[`
    or `,` before it and a `,` or `]` after it."""
    i = off - 1
    if i >= 0 and code[i] == "b":
        i -= 1
    while i >= 0 and code[i].isspace():
        i -= 1
    j = code.find('"', off + 1)
    if code[off] != '"' or j < 0:
        return False
    j += 1
    while j < len(code) and code[j].isspace():
        j += 1
    return i >= 0 and code[i] in "[," and j < len(code) and code[j] in ",]"


def _imports(src, aliases):
    """Names a file imports from a wrapper crate, with their local names."""
    found = {}
    for m in USE_STMT.finditer(src.code):
        path = m.group(1) or m.group(3)
        if path.split("::")[0] not in aliases:
            continue
        items = m.group(2).split(",") if m.group(2) is not None else [
            m.group(4) + (f" as {m.group(5)}" if m.group(5) else "")]
        for item in items:
            parts = [p.strip() for p in item.split(" as ")]
            leaf = parts[0].split("::")[-1].strip("{} ")
            if leaf and re.fullmatch(r"\w+", leaf):
                found[parts[-1].strip("{} ") if len(parts) == 2 else leaf] = leaf
    return found


def crate_uses(root, label, files, wrappers, wrapper_aliases):
    """Every Use in a set of files. `wrapper_aliases` maps the names this
    crate calls a wrapper crate by to that crate's key; a crate that does not
    depend on a wrapper crate cannot call into it, whatever its functions are
    named."""
    keys = set(wrapper_aliases.values())
    exported = set()
    for k in keys:
        exported |= wrappers.exports.get(k, set())
    uses = []
    for path in files:
        src = Source.read(path)
        rel = path.relative_to(root)
        imported = _imports(src, wrapper_aliases) if keys else {}
        for segs, name, off in (src.calls() if keys else ()):
            target = None
            if segs and segs[0] in wrapper_aliases:
                target = name
            elif segs and any(segs[-1] in f.modules and f.crate in keys
                              for f in wrappers.fns.get(name, ())):
                target = name
            elif not segs and name in imported:
                target = imported[name]
            elif not segs and name in exported:
                target = name
            if target is None or target not in wrappers.fns:
                continue
            uses.append(Use(label, f"{rel}:{src.line(off)}", wrappers.tags_of(target), via=name))
        # A tag written in a file that issues the syscall instruction itself,
        # or hands a number to the raw entry, is a syscall number; anywhere
        # else it may be a wire opcode. Only there is a bare 32-bit constant
        # that spells a tag read as one.
        raw = '"syscall"' in src.plain or "mk_syscall_raw" in src.code
        for tag, off in src.tags():
            uses.append(Use(label, f"{rel}:{src.line(off)}", {tag}, raw=raw))
        if raw:
            for m in HEX_TAG.finditer(src.code):
                tag = _spelled(m.group(1))
                if tag:
                    uses.append(Use(label, f"{rel}:{src.line(m.start())}", {tag}, raw=True))
        if keys:
            for m in CONST_REF.finditer(src.code):
                if m.group(0) in wrappers.consts:
                    uses.append(Use(label, f"{rel}:{src.line(m.start())}",
                                    {wrappers.consts[m.group(0)]}, via=m.group(0)))
        defines = {name for name, (_, home) in SERVICE_CLIENTS.items()
                   if str(rel).startswith(home)}
        for off, text in src.literals:
            if NAME_LIT.fullmatch(text) and text not in defines:
                uses.append(Use(label, f"{rel}:{src.line(off)}", literal=text,
                                listed=_listed(src.code, off)))
        if not defines:
            for m in PORT_CONST.finditer(src.code):
                uses.append(Use(label, f"{rel}:{src.line(m.start())}", via=m.group(1),
                                port=int(m.group(2))))
        for name, (imports, home) in SERVICE_CLIENTS.items():
            m = None if name in defines else imports.search(src.code)
            if m:
                uses.append(Use(label, f"{rel}:{src.line(m.start())}", literal=name,
                                via=" ".join(m.group(0).split())))
    return uses


HEX_TAG = re.compile(r"\b0[xX]([0-9A-Fa-f_]{8,11})\b")
ASM_STR = re.compile(r'"([^"\n]*)"')
INCLUDE_STR = re.compile(r'include_str!\(\s*"([^"]+\.[sS])"\s*\)')


def _spelled(digits):
    """The tag a 32-bit constant spells in little-endian order, or None."""
    digits = digits.replace("_", "")
    if len(digits) != 8:
        return None
    tag = int(digits, 16).to_bytes(4, "little")
    return tag.decode() if all(65 <= b <= 90 for b in tag) else None


def asm_uses(root, label, path):
    """Uses in an assembly file a crate pulls in with include_str!: a 32-bit
    constant that spells a tag in little-endian order, and dotted names."""
    uses = []
    text = path.read_text(errors="replace")
    raw = "syscall" in text
    rel = path.relative_to(root)
    for i, line in enumerate(text.splitlines(), 1):
        code = line.split("#", 1)[0] if not line.lstrip().startswith("#include") else ""
        for m in HEX_TAG.finditer(code):
            tag = _spelled(m.group(1))
            if tag:
                uses.append(Use(label, f"{rel}:{i}", {tag}, raw=raw))
        for m in ASM_STR.finditer(code):
            if NAME_LIT.fullmatch(m.group(1)):
                uses.append(Use(label, f"{rel}:{i}", literal=m.group(1)))
    return uses


def asm_files(files):
    """Assembly sources named by include_str! in a crate's Rust files."""
    out = []
    for path in files:
        for m in INCLUDE_STR.finditer(Source.read(path).plain):
            asm = (path.parent / m.group(1)).resolve()
            if asm.is_file() and asm not in out:
                out.append(asm)
    return out


# ---------------------------------------------------------------------------
# The crate graph of one capsule


class Linked:
    """The crates one capsule links, read into Uses, and what the audit
    could not read."""

    def __init__(self):
        self.uses = []
        self.crates = []
        self.libc_features = set()
        self.gaps = []


def _entries(crate_dir, bin_name, as_root):
    """The files a crate's build starts from. A capsule's binary can call its
    own package's library by the package name, with no dependency line to
    say so, so the root reads both."""
    toml = crate_dir / "Cargo.toml"
    deps, lib, bins, default, feats = cargo_manifest(toml)
    lib = lib or crate_dir / "src/lib.rs"
    entries = [lib]
    if as_root:
        entries = [bins.get(bin_name) or (crate_dir / "src/main.rs"), lib]
    return deps, [e for e in entries if e.is_file()], default, feats


def link(root, capsule, wrappers, cache):
    """Walk the capsule's path dependencies and collect every Use."""
    out = Linked()
    crate_dir = root / capsule.dir
    if (crate_dir / "Cargo.toml").is_file():
        _walk(root, crate_dir, capsule.bin_name, True, wrappers, cache, out, set())
    elif capsule.prebuilt:
        _prebuilt(root, capsule, out, cache)
    else:
        out.gaps.append(f"{capsule.dir} has no Cargo.toml and no prebuilt source")
    if capsule.std:
        files = sorted((root / STD_PAL).rglob("*.rs"))
        out.uses += _cached(cache, ("pal",), lambda: crate_uses(
            root, "std PAL", files, wrappers, {}))
        out.crates.append(str(STD_PAL))
    for feature, names in wrappers.implicit.items():
        if feature in out.libc_features:
            tags = set()
            for n in names:
                tags |= wrappers.tags_of(n)
            out.uses.append(Use("libc", f"userland/libc/src/{IMPLICIT[feature][0]}",
                                tags, via=f"libc feature {feature}"))
    return out


def _cached(cache, key, make):
    if key not in cache:
        cache[key] = make()
    return cache[key]


def _walk(root, crate_dir, bin_name, as_root, wrappers, cache, out, seen):
    crate_dir = crate_dir.resolve()
    if crate_dir in seen:
        return
    seen.add(crate_dir)
    deps, entries, default, feats = _entries(crate_dir, bin_name, as_root)
    label = crate_dir.name
    out.crates.append(str(crate_dir.relative_to(root.resolve())))
    aliases = {}
    for dep in deps:
        key = wrappers.crate_of.get(dep.path)
        if key:
            aliases[dep.name.replace("-", "_")] = key
            if key == "libc":
                out.libc_features |= set(dep.features)
                if dep.default_features:
                    out.libc_features |= {"heap", "panic-handler"}
    # A feature this crate turns on by default may forward to libc's.
    if as_root:
        for f in default:
            for target in feats.get(f, ()):
                if "/" in target and target.split("/")[1] in IMPLICIT:
                    out.libc_features.add(target.split("/")[1])
    files = []
    for entry in entries:
        files += [f for f in module_files(entry) if f not in files]
    if not entries:
        files = sorted((crate_dir / "src").rglob("*.rs"))
    if not files:
        out.gaps.append(f"{label}: no sources found")
    # A file under src/ the module walk did not reach may still be built (a
    # macro can declare modules). It is read under its own label, so a call
    # in it keeps a bit without being able to say the capsule fails.
    stray = [p for p in sorted((crate_dir / "src").rglob("*.rs"))
             if p not in set(files) and not _test_only(p)]
    key = (str(crate_dir), tuple(str(e) for e in entries))
    out.uses += _cached(cache, key, lambda: crate_uses(
        root.resolve(), label, files, wrappers, aliases)
        + [u for a in asm_files(files) for u in asm_uses(root.resolve(), label, a)]
        + crate_uses(root.resolve(), label + " (file outside the module tree)", stray,
                     wrappers, aliases))
    for dep in deps:
        if dep.path in wrappers.crate_of or not (dep.path / "Cargo.toml").is_file():
            if dep.path not in wrappers.crate_of:
                out.gaps.append(f"{label}: path dependency {dep.path} not found")
            continue
        _walk(root, dep.path, None, False, wrappers, cache, out, seen)


def _test_only(path):
    """A file only a test build compiles: under tests/ or named for it."""
    return ("tests" in path.parts or path.stem in ("tests", "test")
            or path.stem.endswith("_tests"))


def _prebuilt(root, capsule, out, cache):
    """A crates.io tool: its vendored source when the tree has it, and every
    NONOS shim its lockfile names. Linking a locked crate is not proven: a
    lockfile lists dependencies of every target, not only this one."""
    name = re.search(r"upstream-([\w\-]+)", capsule.prebuilt or "")
    name = name.group(1) if name else capsule.slug
    found = [d for d in sorted((root / UPSTREAM).glob(f"{name}*"))
             if (d / "Cargo.toml").is_file()]
    if not found:
        out.gaps.append(f"{capsule.slug}: source for {capsule.prebuilt} is not in the tree; "
                        f"only the std PAL is read")
        return
    src = found[0]
    files = sorted((src / "src").rglob("*.rs"))
    label = src.name
    out.crates.append(str(src.relative_to(root)))
    out.uses += _cached(cache, (str(src),), lambda: crate_uses(
        root, label, files, Wrappers.EMPTY, {}))
    for pkg in sorted(lock_packages(src / "Cargo.lock")):
        for shim in list((root / UPSTREAM).glob(f"{pkg}-[0-9]*")) + [root / VENDOR / pkg]:
            if (shim / "src").is_dir():
                shim_files = sorted((shim / "src").rglob("*.rs"))
                out.crates.append(str(shim.relative_to(root)) + " (lockfile)")
                read = _cached(cache, (str(shim),), lambda: crate_uses(
                    root, shim.name, shim_files, Wrappers.EMPTY, {}))
                out.uses += [_unproven(u) for u in read]


def _unproven(use):
    return Use(use.crate + " (lockfile only)", use.cite, use.tags, use.via, use.literal,
               use.raw, use.listed, use.port)


class _Empty:
    fns, exports, modules, consts = {}, {}, {}, {}

    @staticmethod
    def tags_of(_name):
        return set()


Wrappers.EMPTY = _Empty()


def declared_caps(root, capsule):
    """The `.nonos.caps` value the capsule's own source declares, or None.
    Names resolve through userland/nonos_cap, as the source's do."""
    names = {n: int(v) for n, v in re.findall(
        r"pub const (CAP_\w+)\s*:\s*u64\s*=\s*(\d+)\s*;",
        (root / "userland/nonos_cap/src/bits.rs").read_text())}
    for path in sorted((root / capsule.dir / "src").rglob("*.rs")):
        src = Source.read(path)
        m = LINK_SECTION.search(src.plain)
        if m:
            value = 0
            for term in m.group(1).split("|"):
                term = term.strip()
                if term in names:
                    value |= names[term]
                elif re.fullmatch(r"0[xX][0-9a-fA-F_]+|\d+", term):
                    value |= int(term.replace("_", ""), 0)
                else:
                    return None, f"{path.relative_to(root)}: cannot read term {term}"
            return value, f"{path.relative_to(root)}:{src.line(m.start())}"
    return None, None
