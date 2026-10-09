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
"""The RUSTC_WRAPPER cargo invokes for every rustc call.

Why it exists
=============
Cargo seeds each compilation unit's crate identity (`-C metadata`, which rustc
hashes into the StableCrateId and therefore into every symbol name and the
codegen-unit layout) from inputs that depend on the build machine:

  * the rustc HOST triple, mixed in for every host-kind unit (build scripts and
    proc-macros, and under -Zbuild-std the compiler_builtins build script that
    alloc pulls in), so x86_64-unknown-linux-gnu and aarch64-apple-darwin give
    different symbol hashes with no path string left behind; and

  * the ABSOLUTE path of every path dependency that sits outside the crate's
    own workspace (each capsule's ../libc, ../toolkit, [patch] entries), so a
    build directory that changes per build changes the bytes.

Neither leak is a path string in the output, so --remap-path-prefix cannot
reach them. This wrapper recomputes `-C metadata` for every unit from inputs
that are already host- and path-independent (the crate name, version and root
source remapped through NONOS_REMAP, the crate types, the cfgs/features, the
target spec by base name only, the curated codegen flags, and the host-neutral
identity of each dependency). Two builds on any host, from any directory, then
produce byte-identical artifacts.

The file names leak too. Cargo names each output with `-C extra-filename`, a
hash with the same host triple in it, and rustc names every object inside an
rlib after that file. Fat LTO sorts the modules it merges by those names, so a
different hash merged them in a different order: the code was the same, laid
out differently, and capsules differed between a Linux and a macOS host. The
wrapper gives rustc a host-neutral extra-filename (from the same identity) and
moves each output to the name cargo expects as soon as rustc reports it, so
cargo, and the dependents pipelining starts on an rmeta, find what they look
for.

Preserving uniqueness
=====================
The recomputed metadata is a hash over a canonical description of the unit, so
two genuinely different units (a different crate, version, feature set, target,
or dependency graph) still get different identities and there are no symbol
collisions. Dependency identity is resolved host-neutrally through a sidecar:
after a unit is identified, its new metadata is written to
$NONOS_RUSTC_IDS/<extra-filename>, and a dependent reads it back by the hash in
the `--extern` file name, so a dependency's host-neutral identity flows into
its dependents exactly as cargo's host-dependent one used to.

What is NOT trusted
===================
The incoming `-C metadata` value cargo computed (it carries the host triple and
the absolute paths) is discarded, never hashed in. The absolute build root and
the real host triple never enter the output. Only remapped, host-neutral inputs
do.

Invocation: cargo calls `nonos-rustc <rustc> <args...>`.
Self-test:  nonos-rustc --self-test
"""

import hashlib
import json
import os
import re
import subprocess
import sys
import tempfile

# Codegen options that change the emitted machine code and are host- and
# path-independent, so they belong in the identity. Everything else under -C is
# either irrelevant to the bytes or carries a machine path, and is excluded.
CODEGEN_DENY = {
    "metadata",          # the very value we are replacing
    "extra-filename",    # names output files; left as cargo set it
    "incremental",       # a build-local directory
    "linker",            # a machine path
    "link-arg",
    "link-args",
    "linker-flavor",
    "link-self-contained",
}

# Metadata is hashed into a u64 StableCrateId regardless of its length, so a
# wide hash costs nothing in the output and removes any collision worry.
META_LEN = 32
DOMAIN = "nonos-rustc/metadata/1\0"


def parse_remaps(environ):
    """(from, to) prefix maps, in the order rustc will see them. rustc applies
    the LAST matching --remap-path-prefix, so the more specific NONOS_WORK map
    is placed last and wins over the general NONOS_REMAP ones."""
    remaps = []
    for tok in environ.get("NONOS_REMAP", "").split():
        if tok.startswith("--remap-path-prefix="):
            frm, _, to = tok[len("--remap-path-prefix="):].partition("=")
            if frm:
                remaps.append((frm, to))
    work = environ.get("NONOS_WORK", "")
    if work:
        remaps.append((work, "/build"))
    out = environ.get("OUT_DIR", "")
    if out:
        remaps.append((out, "/out"))
    return remaps


def remap(path, remaps):
    """Apply the remaps the way rustc does: last matching prefix wins, matched
    on whole path components."""
    for frm, to in reversed(remaps):
        if path == frm:
            return to
        if path.startswith(frm + os.sep):
            return to + path[len(frm):]
    return path


def remap_token(tok, remaps):
    """Remap a token that may be a bare path or key=path or key=val=path."""
    if "=" in tok:
        head, _, tail = tok.partition("=")
        return head + "=" + remap_token(tail, remaps)
    return remap(tok, remaps)


def read_argfile(path):
    """rustc @argfile: one argument per line, UTF-8, trailing newline dropped."""
    with open(path, "r", encoding="utf-8") as f:
        text = f.read()
    if text.endswith("\n"):
        text = text[:-1]
    return text.split("\n") if text else []


def expand_argfiles(args):
    """Return (expanded, used_argfile). rustc supports @file to get around the
    OS argument-length limit; expand so we can read and rewrite the real args."""
    out = []
    used = False
    for a in args:
        if a.startswith("@") and len(a) > 1 and os.path.isfile(a[1:]):
            out.extend(read_argfile(a[1:]))
            used = True
        else:
            out.append(a)
    return out, used


def split_opt(args, i):
    """For an option at index i that takes a value, return (value, next_index),
    handling both `--opt val` and `--opt=val` / `-Oval` glued forms. The caller
    passes the option name it already matched as a prefix."""
    tok = args[i]
    if "=" in tok and not tok.endswith("="):
        return tok.split("=", 1)[1], i + 1
    # glued short form like -Lfoo handled by caller; here the long/space form.
    if i + 1 < len(args):
        return args[i + 1], i + 2
    return "", i + 1


class Unit:
    """Everything parsed out of one rustc invocation that bears on identity."""

    def __init__(self):
        self.crate_name = ""
        self.crate_types = []
        self.edition = ""
        self.target = ""           # base name of a spec file, or the triple
        self.cfgs = []
        self.src = []              # positional .rs inputs
        self.codegen = []          # kept -C key=val, host-neutral
        self.zopts = []
        self.externs = []          # (name, path-or-None)
        self.metadata = None       # incoming value (discarded from identity)
        self.extra_filename = None


def parse_unit(args, remaps):
    u = Unit()
    i = 0
    n = len(args)
    while i < n:
        a = args[i]
        if a == "--crate-name":
            u.crate_name, i = split_opt(args, i)
            continue
        if a.startswith("--crate-name="):
            u.crate_name = a.split("=", 1)[1]
            i += 1
            continue
        if a == "--crate-type" or a.startswith("--crate-type="):
            val, i = split_opt(args, i)
            u.crate_types.extend(v for v in val.split(",") if v)
            continue
        if a == "--edition" or a.startswith("--edition="):
            u.edition, i = split_opt(args, i)
            continue
        if a == "--target" or a.startswith("--target="):
            val, i = split_opt(args, i)
            # A spec file is host-neutral by its base name; a triple is already
            # host-neutral for a cross target and simply absent for host units.
            u.target = os.path.basename(val) if val.endswith(".json") else val
            continue
        if a == "--cfg" or a.startswith("--cfg="):
            val, i = split_opt(args, i)
            u.cfgs.append(val)
            continue
        if a == "--extern" or a.startswith("--extern="):
            val, i = split_opt(args, i)
            name, _, path = val.partition("=")
            u.externs.append((name, path if path else None))
            continue
        if a in ("-C", "--codegen"):
            val, i = split_opt(args, i)
            _record_codegen(u, val, remaps)
            continue
        if a.startswith("-C"):
            _record_codegen(u, a[2:], remaps)
            i += 1
            continue
        if a.startswith("--codegen="):
            _record_codegen(u, a.split("=", 1)[1], remaps)
            i += 1
            continue
        if a == "-Z":
            val, i = split_opt(args, i)
            u.zopts.append(remap_token(val, remaps))
            continue
        if a.startswith("-Z"):
            u.zopts.append(remap_token(a[2:], remaps))
            i += 1
            continue
        # Options that take a value we do not fold into identity, but must step
        # over so their argument is not mistaken for a positional source.
        if a in ("-L", "-o", "--out-dir", "--emit", "--error-format", "--json",
                 "--color", "--cap-lints", "--sysroot", "--extern-location",
                 "--diagnostic-width", "--remap-path-prefix", "-A", "-W", "-D",
                 "-F", "--allow", "--warn", "--deny", "--forbid", "--print",
                 "--explain", "--pretty", "--out-file"):
            _, i = split_opt(args, i)
            continue
        if a.startswith(("-L", "--emit=", "--error-format=", "--json=",
                         "--color=", "--cap-lints=", "--sysroot=", "--print=",
                         "-A", "-W", "-D", "-F", "--out-dir=")):
            i += 1
            continue
        if a.startswith("-") and a != "-":
            i += 1
            continue
        # A positional argument: the crate root source.
        u.src.append(remap(a, remaps))
        i += 1
    return u


def _record_codegen(u, spec, remaps):
    key, _, val = spec.partition("=")
    if key == "metadata":
        u.metadata = val
    elif key == "extra-filename":
        u.extra_filename = val
    elif key in CODEGEN_DENY:
        pass
    else:
        u.codegen.append(key + "=" + remap_token(val, remaps) if val else key)


_HASH_RE = re.compile(r"^(.*)-([0-9A-Za-z]+)$")
_LIBEXT = (".rlib", ".rmeta", ".so", ".dylib", ".a", ".dll", ".exe")


def dep_id(name, path, ids_dir):
    """A dependency's host-neutral identity. The `--extern` file name carries
    the dependency's cargo extra-filename hash; look up the sidecar that the
    dependency's own invocation wrote under that hash, falling back to the file
    name with the hash stripped so the result never carries the raw hash."""
    if path is None:
        return "sysroot:" + name
    base = os.path.basename(path)
    for ext in _LIBEXT:
        if base.endswith(ext):
            base = base[: -len(ext)]
            break
    m = _HASH_RE.match(base)
    if m and ids_dir:
        side = os.path.join(ids_dir, m.group(2))
        try:
            with open(side, "r", encoding="utf-8") as f:
                return name + "=" + f.read().strip()
        except OSError:
            pass
    stem = m.group(1) if m else base
    return name + "=name:" + stem


def identity(u, ids_dir):
    parts = [DOMAIN, "name=" + u.crate_name,
             "types=" + ",".join(sorted(u.crate_types)),
             "edition=" + u.edition, "target=" + u.target]
    parts += ["cfg=" + c for c in sorted(u.cfgs)]
    parts += ["src=" + s for s in sorted(u.src)]
    parts += ["C=" + c for c in sorted(u.codegen)]
    parts += ["Z=" + z for z in sorted(u.zopts)]
    parts += ["extern=" + dep_id(name, path, ids_dir)
              for (name, path) in sorted(u.externs, key=lambda e: (e[0], e[1] or ""))]
    digest = hashlib.sha256("\0".join(parts).encode("utf-8")).hexdigest()
    return digest[:META_LEN]


def write_sidecar(ids_dir, key, value):
    if not ids_dir or not key:
        return
    key = key.lstrip("-")
    if not key:
        return
    try:
        os.makedirs(ids_dir, exist_ok=True)
        dest = os.path.join(ids_dir, key)
        fd, tmp = tempfile.mkstemp(dir=ids_dir)
        with os.fdopen(fd, "w", encoding="utf-8") as f:
            f.write(value)
        os.replace(tmp, dest)
    except OSError:
        pass


def rewrite_metadata(args, new_value):
    """Return args with the `-C metadata=` value replaced, in whichever form
    cargo used, and extra-filename left exactly as it was."""
    out = []
    i = 0
    n = len(args)
    while i < n:
        a = args[i]
        if a in ("-C", "--codegen") and i + 1 < n and args[i + 1].startswith("metadata="):
            out.append(a)
            out.append("metadata=" + new_value)
            i += 2
            continue
        if a.startswith("-Cmetadata="):
            out.append("-Cmetadata=" + new_value)
            i += 1
            continue
        if a.startswith("--codegen=metadata="):
            out.append("--codegen=metadata=" + new_value)
            i += 1
            continue
        out.append(a)
        i += 1
    return out


def rewrite_extra_filename(args, new_value):
    """Return args with the `-C extra-filename=` value replaced."""
    out = []
    i = 0
    n = len(args)
    while i < n:
        a = args[i]
        if a in ("-C", "--codegen") and i + 1 < n and args[i + 1].startswith("extra-filename="):
            out += [a, "extra-filename=" + new_value]
            i += 2
            continue
        if a.startswith("-Cextra-filename="):
            out.append("-Cextra-filename=" + new_value)
        elif a.startswith("--codegen=extra-filename="):
            out.append("--codegen=extra-filename=" + new_value)
        else:
            out.append(a)
        i += 1
    return out


def out_dir_of(args):
    for i, a in enumerate(args):
        if a == "--out-dir" and i + 1 < len(args):
            return args[i + 1]
        if a.startswith("--out-dir="):
            return a.split("=", 1)[1]
    return None


def link_name(path, neutral, cargo):
    """Give an output rustc has announced the name cargo expects as well, as a
    hard link: rustc still reads it (the rmeta goes into the rlib), so it must
    not move yet. Returns the cargo path, or the old one when it does not carry
    the neutral name."""
    base = os.path.basename(path)
    if neutral not in base:
        return path
    dest = os.path.join(os.path.dirname(path), base.replace(neutral, cargo))
    try:
        if os.path.lexists(dest):
            os.unlink(dest)
        os.link(path, dest)
    except OSError:
        return path
    return dest


def restore_name(path, neutral, cargo):
    """Move an output rustc wrote under the neutral extra-filename to the name
    cargo expects, rewriting the paths inside a dep-info file. Returns the new
    path, or the old one when it does not carry the neutral name."""
    base = os.path.basename(path)
    if neutral not in base:
        return path
    dest = os.path.join(os.path.dirname(path), base.replace(neutral, cargo))
    if base.endswith(".d"):
        try:
            with open(path, "r", encoding="utf-8") as f:
                text = f.read()
            with open(path, "w", encoding="utf-8") as f:
                f.write(text.replace(neutral, cargo))
        except OSError:
            pass
    try:
        os.replace(path, dest)
    except OSError:
        return path
    return dest


def run_renaming(rustc, args, neutral, cargo, out_dir):
    """Run rustc, linking each artifact it announces (--json=artifacts, the
    signal cargo pipelines on) under cargo's name before passing the line on,
    then moving everything it wrote under the neutral name once it exits. The
    jobserver descriptors cargo handed down stay open for rustc."""
    proc = subprocess.Popen([rustc] + args, stderr=subprocess.PIPE, close_fds=False)
    for raw in proc.stderr:
        line = raw
        if b'"artifact"' in raw:
            try:
                msg = json.loads(raw)
                art = msg.get("artifact")
                if isinstance(art, str):
                    msg["artifact"] = link_name(art, neutral, cargo)
                    line = (json.dumps(msg, separators=(",", ":")) + "\n").encode("utf-8")
            except ValueError:
                pass
        sys.stderr.buffer.write(line)
        sys.stderr.buffer.flush()
    rc = proc.wait()
    if out_dir and os.path.isdir(out_dir):
        for name in os.listdir(out_dir):
            if neutral in name:
                restore_name(os.path.join(out_dir, name), neutral, cargo)
    return rc


def remap_args(environ):
    """The --remap-path-prefix tokens to append: the general NONOS_REMAP first,
    then the specific NONOS_WORK map, so the latter wins in rustc."""
    toks = environ.get("NONOS_REMAP", "").split()
    work = environ.get("NONOS_WORK", "")
    if work:
        toks.append("--remap-path-prefix=" + work + "=/build")
    # A crate with a build script compiles what it generated into OUT_DIR,
    # target/<t>/release/build/<crate>-<hash>/out. That hash is cargo's for the
    # build script, which is compiled for the build host, so it differs between
    # a Linux and a macOS host; a generated file pulled in with include! then
    # put the host-specific path into panic locations and LLVM's symbol names,
    # and tokei came out with different bytes on each host. Fold it to /out,
    # last, so it wins over the general maps.
    out = environ.get("OUT_DIR", "")
    if out:
        toks.append("--remap-path-prefix=" + out + "=/out")
    return toks


def run(argv, environ):
    rustc = argv[1]
    raw = argv[2:]
    args, used_argfile = expand_argfiles(raw)
    remaps = parse_remaps(environ)
    ids_dir = environ.get("NONOS_RUSTC_IDS", "")
    normalize = environ.get("NONOS_RUSTC_NORMALIZE", "1") != "0"

    rename = None
    if normalize:
        u = parse_unit(args, remaps)
        if u.metadata is not None and u.crate_name:
            new_meta = identity(u, ids_dir)
            key = u.extra_filename if u.extra_filename is not None else u.metadata
            write_sidecar(ids_dir, key, new_meta)
            args = rewrite_metadata(args, new_meta)
            cargo = u.extra_filename
            if cargo and cargo.startswith("-") and len(cargo) > 1:
                neutral = "-" + new_meta[: len(cargo) - 1]
                if neutral != cargo:
                    args = rewrite_extra_filename(args, neutral)
                    rename = (neutral, cargo, out_dir_of(args))

    args = args + remap_args(environ)

    if rename:
        argv_rustc = args
        if used_argfile:
            fd, tmp = tempfile.mkstemp(suffix=".args")
            with os.fdopen(fd, "w", encoding="utf-8") as f:
                f.write("\n".join(args))
            argv_rustc = ["@" + tmp]
        sys.exit(run_renaming(rustc, argv_rustc, *rename))

    # execvp, not execv: cargo may hand us rustc as a bare name to resolve on
    # PATH (as the shell wrapper's `exec "$rustc"` did), not an absolute path.
    if used_argfile:
        fd, tmp = tempfile.mkstemp(suffix=".args")
        with os.fdopen(fd, "w", encoding="utf-8") as f:
            f.write("\n".join(args))
        os.execvp(rustc, [rustc, "@" + tmp])
    os.execvp(rustc, [rustc] + args)


# --------------------------------------------------------------------------- #
# Self-test: the identity ignores the build root and the incoming (host-tainted)
# metadata, and separates genuinely different units.
# --------------------------------------------------------------------------- #

def _self_test():
    def inv(root, host_meta, crate="proof_io", libpath=None):
        lib = libpath or (root + "/deps/libnonos_userland_libc-" + host_meta + ".rlib")
        return [
            "--crate-name", crate,
            "--edition=2021",
            "--crate-type", "bin",
            "--target", root + "/userland/x86_64-nonos-user.json",
            "--cfg", 'feature="default"',
            "-C", "opt-level=2",
            "-C", "panic=abort",
            "-C", "metadata=" + host_meta,
            "-C", "extra-filename=-" + host_meta,
            "-L", "dependency=" + root + "/target/deps",
            "--extern", "nonos_libc=" + lib,
            "--out-dir", root + "/target/x86_64-nonos-user/release/deps",
            root + "/capsule_proof_io/src/main.rs",
        ]

    ok = True

    def check(name, cond):
        nonlocal ok
        ok = ok and cond
        print(("PASS " if cond else "FAIL ") + name)

    # Two "machines": different build roots AND different incoming metadata
    # (standing in for the host triple cargo folds in). The target spec and the
    # lib dependency live under each root. With the work map, both roots fold to
    # /build and the identity must agree.
    env_a = {"NONOS_WORK": "/tmp/nonos-root-alpha", "NONOS_REMAP": ""}
    env_b = {"NONOS_WORK": "/b", "NONOS_REMAP": ""}
    ra, rb = "/tmp/nonos-root-alpha", "/b"
    ua = parse_unit(inv(ra, "aaaaaaaa11111111"), parse_remaps(env_a))
    ub = parse_unit(inv(rb, "bbbbbbbb22222222"), parse_remaps(env_b))
    ida = identity(ua, "")
    idb = identity(ub, "")
    check("root and host do not change identity", ida == idb)

    # A different crate name must change the identity (no collisions).
    uc = parse_unit(inv(ra, "aaaaaaaa11111111", crate="proof_net"), parse_remaps(env_a))
    check("a different crate changes identity", identity(uc, "") != ida)

    # A different cfg/feature set must change the identity.
    args_feat = inv(ra, "aaaaaaaa11111111")
    j = args_feat.index('feature="default"')
    args_feat[j] = 'feature="other"'
    uf = parse_unit(args_feat, parse_remaps(env_a))
    check("a different feature changes identity", identity(uf, "") != ida)

    # The incoming metadata alone (same root) must not change identity: proves
    # the host-tainted value is never folded in.
    ud = parse_unit(inv(ra, "cccccccc33333333"), parse_remaps(env_a))
    check("incoming metadata is ignored", identity(ud, "") == ida)

    # extra-filename is preserved verbatim while metadata is replaced.
    rewritten = rewrite_metadata(inv(ra, "aaaaaaaa11111111"), "NEWMETA")
    has_new = any(x == "metadata=NEWMETA" for x in rewritten)
    keeps_extra = any(x == "extra-filename=-aaaaaaaa11111111" for x in rewritten)
    check("metadata replaced", has_new)
    check("extra-filename preserved", keeps_extra)

    # Sidecar round-trip: a dependency's host-neutral id flows to its dependent.
    import tempfile as _tf
    d = _tf.mkdtemp()
    write_sidecar(d, "-aaaaaaaa11111111", "DEPID")
    did = dep_id("nonos_libc", ra + "/deps/libnonos_userland_libc-aaaaaaaa11111111.rlib", d)
    check("dependency identity comes from the sidecar", did == "nonos_libc=DEPID")

    # A missing sidecar falls back to the name with the hash stripped, never the
    # raw (host-tainted) hash.
    did2 = dep_id("nonos_libc", ra + "/deps/libnonos_userland_libc-deadbeef.rlib", d + "/none")
    check("missing sidecar strips the hash", "deadbeef" not in did2)

    # Argfile expansion round-trips.
    import os as _os
    fd, p = _tf.mkstemp()
    with _os.fdopen(fd, "w") as f:
        f.write("--crate-name\nproof_io\n")
    expanded, used = expand_argfiles(["@" + p])
    check("argfile expands", used and expanded == ["--crate-name", "proof_io"])

    print("SELF-TEST " + ("OK" if ok else "FAILED"))
    return 0 if ok else 1


def main():
    if len(sys.argv) == 2 and sys.argv[1] == "--self-test":
        sys.exit(_self_test())
    if len(sys.argv) < 2:
        sys.stderr.write("nonos-rustc: expected a rustc path as the first argument\n")
        sys.exit(2)
    run(sys.argv, os.environ)


if __name__ == "__main__":
    main()
