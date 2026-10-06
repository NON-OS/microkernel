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
"""Every place the kernel asks for a capability bit, read from the kernel.
Read by cap_audit.

Four kinds of check decide what a capsule may do:

  syscall   the cap table, through the parsers check_syscall_caps already
            keeps honest against abi/syscalls.toml.
  handler   a handler asking again after the table admitted the call, for a
            narrower case: killing a process one did not start, registering a
            name one was not spawned with. These are listed here by hand, each
            with the source text it rests on; when that text moves, the audit
            stops rather than reporting against a kernel that no longer exists.
  service   the registry: reaching a network service takes Network, and owning
            a service endpoint takes what reaching it takes.
  spawn     the capsule's own kernel mirror, the grant the spawn site passes,
            which must stay inside the manifest or the spawn is refused.
"""

import re
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
import check_syscall_abi as abi  # noqa: E402
import check_syscall_caps as table  # noqa: E402
from check_caps_abi import read_kernel  # noqa: E402
from cap_audit_rust import Source, match_brace  # noqa: E402

POLICY = Path("src/services/registry/policy.rs")
REGISTRY = Path("src/services/registry.rs")
INSTALL = Path("src/kernel_core/process_spawn/capsule_spawn/runner/install/install.rs")
GRANT_RULE = Path("src/security/capsule_manifest/verify/caps_bits.rs")
SANDBOX = Path("src/userspace/tool_capsules/spec.rs")
TOOL_REGISTRY = Path("src/userspace/tool_capsules/registry.rs")

HARD, CONDITIONAL, SOFT, DIAGNOSTIC = "hard", "conditional", "soft", "diagnostic"


class SourceMoved(Exception):
    """A hand-listed check no longer matches the kernel text it was read from."""


class Gate:
    """One kernel check: which bits satisfy it and where it is written.

    `kind` is "all" (every name needed) or "any" (one is enough). `strength`
    says what a refusal costs: HARD the call fails, CONDITIONAL it fails only
    in a case the audit cannot decide from source, SOFT it succeeds with less,
    DIAGNOSTIC a log line is dropped. `refuses` marks a check that refuses a
    holder rather than admitting one. `literal`, when set, is the set of
    names of which a capsule must write one, other than its own, for the
    check to apply to it.
    """

    def __init__(self, what, kind, names, strength, cite, why="", refuses=False,
                 literal=None, ports=frozenset()):
        self.what = what
        self.kind = kind
        self.names = frozenset(names)
        self.strength = strength
        self.cite = cite
        self.why = why
        self.refuses = refuses
        self.literal = literal
        self.ports = frozenset(ports)

    def as_dict(self):
        return {"what": self.what, "kind": self.kind, "names": sorted(self.names),
                "strength": self.strength, "cite": self.cite, "why": self.why,
                "refuses": self.refuses}


def bits(root):
    """Kernel capability name -> bit, from src/capabilities/types/defs.rs."""
    return {name: bit for name, bit in read_kernel(root).values()}


def _cite(root, directory, needle):
    """file:line of the first line under `directory` matching `needle`."""
    rx = re.compile(needle)
    for path in sorted((root / directory).rglob("*.rs")):
        for i, line in enumerate(path.read_text().splitlines(), 1):
            if rx.search(line):
                return f"{path.relative_to(root)}:{i}"
    return str(directory)


def _anchor(root, path, text):
    """file:line where `text` sits, or SourceMoved if it no longer does."""
    full = root / path
    if not full.is_file():
        raise SourceMoved(f"{path} is gone")
    for i, line in enumerate(full.read_text().splitlines(), 1):
        if text in line:
            return f"{path}:{i}"
    raise SourceMoved(f"{path} no longer contains {text!r}")


def syscall_gates(root):
    """tag -> Gate for every syscall the kernel enum declares."""
    kernel = abi.read_kernel(root)
    demanded, problems = table.demanded(root, abi.CAP_TABLE)
    if problems:
        raise SourceMoved("; ".join(problems))
    out = {}
    for tag, variant in kernel["enum"].items():
        kind, names = demanded.get(variant, ("all", frozenset({"refused"})))
        names = frozenset(n for n in names if n != "valid_token")
        cite = _cite(root, abi.CAP_TABLE, rf"SyscallNumber::{variant}\b")
        out[tag] = Gate(f"{variant} ({tag})", kind, names, HARD, cite)
    # MkDebug admits one thing, a line on the serial log; refused, the caller
    # runs on without it. That is what the libc panic handler expects.
    if "MDBG" in out:
        out["MDBG"].strength = DIAGNOSTIC
        out["MDBG"].why = "writes a line to the serial log; refused, the line is dropped"
    return out


def handler_gates(root):
    """tag -> [Gate] for the checks a handler makes after the table admits."""
    listed = [
        ("MKIL", "any", {"ProcessControl", "Admin"}, CONDITIONAL,
         "src/syscall/microkernel/kill.rs",
         "Capability::ProcessControl.bit() | Capability::Admin.bit()",
         "only to end a process that is neither the caller's child nor a guest it hosts",
         False, None),
        ("MSOW", "all", {"Debug"}, SOFT,
         "src/syscall/microkernel/stdout_write.rs", "can_debug()",
         "stdout is copied to the serial line only for a holder of Debug", False, None),
        ("MPST", "any", {"AttestRead", "ProcessControl"}, SOFT,
         "src/syscall/microkernel/procstat_redact.rs",
         "token.grants(Capability::AttestRead) || token.grants(Capability::ProcessControl)",
         "other processes' masks and counters are shown only to these", False, None),
        ("MCLD", "all", {"SpawnBroker"}, CONDITIONAL,
         "src/kernel_core/process_spawn/capsule_spawn/attested_parent.rs",
         "Capability::SpawnBroker.bit()",
         "only to name another live process as the new capsule's parent", False, None),
        ("MSVR", "any", {"RegisterService", "Admin"}, CONDITIONAL,
         "src/services/registry/auth/caller_has_register_right.rs",
         "token.can_register_service() || token.is_admin()",
         "only to claim a runtime-registrable name the spawn did not give the caller",
         False, runtime_registrable(root)),
        ("MPCR", "all", {"Driver"}, HARD, "src/syscall/microkernel/pci.rs",
         "caps::has(pid, Capability::Driver.bit())",
         "the handler asks for Driver itself; Admin passes the table but not this",
         False, None),
        ("MPCW", "all", {"Driver"}, HARD, "src/syscall/microkernel/pci.rs",
         "caps::has(pid, Capability::Driver.bit())",
         "the handler asks for Driver itself; Admin passes the table but not this",
         False, None),
        ("MADC", "all", {"Network"}, HARD, "src/syscall/caps/checks/system.rs",
         "!self.reaches_network()",
         "the TPM quote is refused to any holder of Network", True, None),
        ("MTRN", "all", {"Network", "FileSystem"}, HARD,
         "src/userspace/tool_capsules/model_fetch/spawn.rs",
         "caller.can_network() && caller.can_open_files()",
         "running tool.model-fetch takes Network and FileSystem in the caller",
         False, {"tool.model-fetch"}),
    ]
    out = {}
    for tag, kind, names, strength, path, anchor, why, refuses, literal in listed:
        cite = _anchor(root, Path(path), anchor)
        out.setdefault(tag, []).append(
            Gate(f"handler for {tag}", kind, names, strength, cite, why, refuses,
                 None if literal is None else frozenset(literal)))
    return out


def runtime_registrable(root):
    """The names a capsule may claim at runtime beyond its own endpoints."""
    path = Path("src/services/registry/reserved.rs")
    m = re.search(r"const RUNTIME_REGISTRABLE\s*:\s*\[&str;\s*\d+\]\s*=\s*\[(.*?)\];",
                  (root / path).read_text(), re.S)
    if not m:
        raise SourceMoved(f"{path} no longer defines RUNTIME_REGISTRABLE")
    return set(re.findall(r'"([^"]+)"', m.group(1)))


def network_services(root):
    """(names, cite) of the services whose endpoints take Network to reach."""
    text = (root / POLICY).read_text()
    m = re.search(r"const NETWORK_SERVICES\s*:\s*\[&str;\s*\d+\]\s*=\s*\[(.*?)\];", text, re.S)
    if not m:
        raise SourceMoved(f"{POLICY} no longer defines NETWORK_SERVICES")
    names = re.findall(r'"([^"]+)"', m.group(1))
    line = text[:m.start()].count("\n") + 1
    return names, f"{POLICY}:{line}"


def endpoint_owner_cite(root):
    """Where registering an endpoint refuses an owner lacking its requirement."""
    return (_anchor(root, REGISTRY, "owner_has_required(pid, caps)") + ", " +
            _anchor(root, INSTALL, "register_endpoint(params.name"))


def _asked_bit(root, path, const, anchor, name):
    """The cite of a service's MkCapCheck, once its constant is checked to be
    the kernel's bit of that name."""
    cite = _anchor(root, path, anchor)
    shift = re.search(const + r": u64 = 1 << (\d+);", (root / path).read_text())
    if not shift or 1 << int(shift.group(1)) != bits(root).get(name):
        raise SourceMoved(f"{path}: {const} is no longer the {name} bit")
    return cite


def service_acls(root, handle_of):
    """Checks a userland service makes on its caller through MkCapCheck.

    virtio_blk and vfs do this. Each rule lives in the service's own source
    and is read from there, so a change to the bit it asks for shows up here.
    vfs is reached by name, through the vfs client module, or by its fixed
    port, read from its Capsule.mk."""
    gates = []
    handle = handle_of.get("driver-virtio-blk")
    if handle:
        cite = _asked_bit(root, Path("userland/capsule_driver_virtio_blk/src/server/acl.rs"),
                          "CAP_STORE_WRITE", "mk_cap_check(sender_pid, CAP_STORE_WRITE)",
                          "StoreWrite")
        gates.append(Gate(f"service {handle}", "all", {"StoreWrite"}, CONDITIONAL, cite,
                          "virtio_blk serves the write path only to a holder of StoreWrite",
                          literal=frozenset({handle})))
    mk = root / "userland/capsule_vfs/Capsule.mk"
    if "vfs" in handle_of and mk.is_file():
        cite = _asked_bit(root, Path("userland/capsule_vfs/src/server/fs_gate.rs"),
                          "CAP_FILE_SYSTEM", "mk_cap_check(pid, CAP_FILE_SYSTEM)", "FileSystem")
        served = re.findall(r"service:(\d+):(\S+)", mk.read_text())
        if not served:
            raise SourceMoved(f"{mk.relative_to(root)} names no service endpoint")
        gates.append(Gate(f"service {served[0][1]}", "all", {"FileSystem"}, HARD, cite,
                          "vfs answers only the kernel and a holder of FileSystem",
                          literal=frozenset(n for _, n in served),
                          ports=frozenset(int(p) for p, _ in served)))
    return gates


# ---------------------------------------------------------------------------
# The kernel spawn mirror


class Mirror:
    """What a spawn site grants: `mask`, the part of it that a build feature
    decides (`conditional`), and every term that could not be read."""

    def __init__(self):
        self.mask = 0
        self.conditional = 0
        self.unresolved = []
        self.sites = []


def tool_sandbox(root, bit_of):
    """(mask, cite) of SANDBOX_CAPS, the grant the tool registry gives every
    crates.io tool that names no set of its own."""
    text = (root / SANDBOX).read_text()
    m = re.search(r"const SANDBOX_CAPS\s*:\s*u64\s*=\s*(.*?);", text, re.S)
    if not m:
        raise SourceMoved(f"{SANDBOX} no longer defines SANDBOX_CAPS")
    names = re.findall(r"Capability::(\w+)\.bit\(\)", m.group(1))
    rest = re.sub(r"Capability::\w+\.bit\(\)|\|", "", m.group(1)).strip()
    if rest or not names or any(n not in bit_of for n in names):
        raise SourceMoved(f"{SANDBOX}: SANDBOX_CAPS is no longer a union of capability bits")
    mask = 0
    for n in names:
        mask |= bit_of[n]
    return mask, f"{SANDBOX}:{text[:m.start()].count(chr(10)) + 1}"


def sandboxed_tools(root):
    """The binary names of the registry's tools spawned with SANDBOX_CAPS:
    every tool_capsule! entry that names no capability set of its own."""
    text = (root / TOOL_REGISTRY).read_text()
    out = set()
    for m in re.finditer(r"\btool_capsule!\(", text):
        depth, i = 1, m.end()
        while i < len(text) and depth:
            depth += {"(": 1, ")": -1}.get(text[i], 0)
            i += 1
        args = _split_top(re.sub(r"//[^\n]*", "", text[m.end():i - 1]), ",")
        if len(args) == 6 and args[5].startswith('"'):
            out.add(args[5].strip('"'))
    if not out:
        raise SourceMoved(f"{TOOL_REGISTRY} registers no tool with the sandbox set")
    return out


def _split_top(expr, sep):
    parts, depth, cur = [], 0, []
    for c in expr:
        if c in "([{":
            depth += 1
        elif c in ")]}":
            depth -= 1
        if c == sep and depth == 0:
            parts.append("".join(cur))
            cur = []
        else:
            cur.append(c)
    parts.append("".join(cur))
    return [p.strip() for p in parts if p.strip()]


def _expr_after(code, start):
    """The expression from `start` to the first top-level `,`, `;` or `}`."""
    depth = 0
    for i in range(start, len(code)):
        c = code[i]
        if c in "([":
            depth += 1
        elif c in ")]":
            depth -= 1
        elif depth == 0 and c in ",;}":
            return code[start:i]
    return code[start:]


def read_mirror(root, directory, kernel_bits):
    """Evaluate every `requested_caps:` under a mirror directory."""
    out = Mirror()
    base = root / directory
    if not base.is_dir():
        out.unresolved.append(f"{directory} does not exist")
        return out
    sources = [Source.read(p) for p in sorted(base.rglob("*.rs"))]
    scope = sources + [Source.read(p) for p in sorted(base.parent.glob("*.rs"))]

    def const(name):
        rx = re.compile(rf"\bconst\s+{name}\s*:\s*u64\s*=")
        for s in scope:
            m = rx.search(s.code)
            if m:
                return [_expr_after(s.code, m.end())]
        return None

    def function(name):
        """The tail expression of `fn name() -> u64 { ... }`."""
        rx = re.compile(rf"\bfn\s+{name}\s*\(\s*\)\s*->\s*u64\s*\{{")
        for s in scope:
            m = rx.search(s.code)
            if m:
                body = s.code[m.end():match_brace(s.code, m.end() - 1) - 1]
                return [body.rsplit(";", 1)[-1]]
        return None

    def field(name):
        rx = re.compile(rf"\b{name}\s*:")
        found = []
        for s in sources:
            for m in rx.finditer(s.code):
                value = _expr_after(s.code, m.end()).strip()
                if value and not re.fullmatch(r"u\d+|usize", value):
                    found.append(value)
        return found or None

    def evaluate(expr, depth=0):
        mask, cond = 0, 0
        for term in _split_top(expr, "|"):
            while term.startswith("(") and term.endswith(")"):
                term = term[1:-1].strip()
            cap = re.fullmatch(r"(?:[\w:]*::)?Capability::(\w+)\.bit\(\)", term)
            num = re.fullmatch(r"0[xX][0-9a-fA-F_]+|\d[\d_]*", term)
            if cap and cap.group(1) in kernel_bits:
                mask |= kernel_bits[cap.group(1)]
            elif re.fullmatch(r"(?:[\w:]*::)?serial_debug_cap\(\)", term):
                cond |= kernel_bits["Debug"]
            elif num:
                mask |= int(term.replace("_", ""), 0)
            elif depth < 4 and re.fullmatch(r"\w+", term) and const(term):
                m, c = evaluate(const(term)[0], depth + 1)
                mask, cond = mask | m, cond | c
            elif depth < 4 and re.fullmatch(r"\w+\(\)", term) and function(term[:-2]):
                m, c = evaluate(function(term[:-2])[0], depth + 1)
                mask, cond = mask | m, cond | c
            elif depth < 4 and re.fullmatch(r"\w+\.(\w+)", term) and field(term.split(".")[1]):
                for value in field(term.split(".")[1]):
                    m, c = evaluate(value, depth + 1)
                    mask, cond = mask | m, cond | c
            else:
                out.unresolved.append(term)
        return mask, cond

    for s in sources:
        for m in re.finditer(r"\brequested_caps\s*:", s.code):
            expr = _expr_after(s.code, m.end())
            if re.fullmatch(r"\s*u64\s*", expr):
                continue
            mask, cond = evaluate(expr)
            out.mask |= mask
            out.conditional |= cond
            out.sites.append(f"{s.path.relative_to(root)}:{s.line(m.start())}")
    if not out.sites:
        out.unresolved.append(f"no requested_caps under {directory}")
    return out


def grant_rule_cite(root):
    """Where the spawn gate refuses a grant wider than the manifest."""
    return _anchor(root, GRANT_RULE, "granted & !(required | optional) == 0")
