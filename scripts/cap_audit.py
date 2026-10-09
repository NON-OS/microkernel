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
"""Hold every capsule's capability mask against what its code can be shown
to need.

A mask is three numbers that must agree: CAPSULE_REQUIRED_CAPS in Capsule.mk,
the `.nonos.caps` section the source declares, and the grant the kernel spawn
mirror passes. Checks elsewhere keep them equal to each other. Nothing asked
whether the number they agree on is the right one, so a bit granted once for
a reason that went away stayed granted, and a bit a new call needs was found
only when the call failed on a booted machine.

For each granted bit this looks for evidence: a syscall the capsule or a crate
it links can reach whose gate names the bit, a network service it names, a
service endpoint it owns, a handler check (kernel side, cap_audit_kernel), or
a service that asks MkCapCheck of its caller: virtio_blk for StoreWrite, and
vfs for FileSystem, reached by the vfs_pool name, by port 4104, or through
app_skeleton's vfs client. A name written as one entry of a table of names
is not a message to that service. A crates.io tool is held to the tool
sandbox, SANDBOX_CAPS, which its manifest must name, and to the bits vfs
asks, which it must be spawned with.
Each bit is then reported as used (with the file and line of one use), as no
evidence found, or, for a bit something needs and the mask lacks, as one that
would fail at runtime.

No evidence is not proof of no use. The audit is wrong when:
  - a wrapper is reached through a function pointer, trait object or macro,
    or imported under a name it does not resolve;
  - a syscall number is computed rather than written as a tag;
  - a service name is assembled at runtime or read from a message;
  - a kernel check is added and not listed in cap_audit_kernel.
It over-counts the other way on purpose: a linked crate is read whole, code
behind every cfg is read, and a crates.io tool's lockfile names crates for
every target. So "used" may be generous, and "no evidence" must be read by a
person before a bit is dropped.
"""

import argparse
import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
import cap_audit_kernel as kern  # noqa: E402
import cap_audit_userland as user  # noqa: E402

RANK = {kern.HARD: 0, kern.CONDITIONAL: 1, kern.SOFT: 2, kern.DIAGNOSTIC: 3}
STATUS = {kern.HARD: "used", kern.CONDITIONAL: "conditional", kern.SOFT: "soft",
          kern.DIAGNOSTIC: "diagnostic"}
KEEP = 4

# Capsules --strict does not hold to its rules, each with the reason.
EXEMPT = {
    "attack": "the test-only attack suite: its calls are meant to be refused, and its "
              "verdict lines, written with mk_debug, are the evidence, so it keeps "
              "Debug required",
    "toolkit": "its render op attaches surfaces it holds no bit for, and no capsule sends it",
}

# Capsules that may keep Debug in CAPSULE_REQUIRED_CAPS. Every other capsule
# holds it as optional, which only a kernel built with `capsule-serial-debug`
# grants (serial_debug_cap()).
DEBUG_REQUIRED = {
    "linux": "the guest console: a Linux write to fd 1 or 2 is mk_debug, and EIO "
             "without it",
}


class Kernel:
    """Everything the kernel checks, read once."""

    def __init__(self, root, handle_of):
        self.bits = kern.bits(root)
        self.syscalls = kern.syscall_gates(root)
        self.handlers = kern.handler_gates(root)
        self.network, self.network_cite = kern.network_services(root)
        self.owner_cite = kern.endpoint_owner_cite(root)
        self.acls = kern.service_acls(root, handle_of)
        self.grant_cite = kern.grant_rule_cite(root)
        self.sandbox, self.sandbox_cite = kern.tool_sandbox(root, self.bits)
        self.sandboxed = kern.sandboxed_tools(root)
        self.vfs_bits = 0
        for acl in self.acls:
            if acl.literal and "vfs_pool" in acl.literal:
                for n in acl.names:
                    self.vfs_bits |= self.bits[n]

    def names(self, mask):
        return [n for n, b in sorted(self.bits.items(), key=lambda kv: kv[1]) if mask & b]


class Finding:
    def __init__(self, gate, use, strength):
        self.gate = gate
        self.use = use
        self.strength = strength

    def as_dict(self):
        return {"strength": self.strength, "what": self.gate.what, "why": self.gate.why,
                "gate": self.gate.cite, "at": self.use.cite, "via": self.use.via,
                "crate": self.use.crate}


def assess(capsule, uses, kernel, root_crate):
    """Credit each gate a capsule's uses reach to the bits it holds, and
    record every gate it cannot satisfy."""
    granted = capsule.granted
    credit = {}
    missing, refused, notes = [], [], []
    literals = {u.literal for u in uses if u.literal} - set(capsule.own_names)

    def holds(name):
        return bool(granted & kernel.bits.get(name, 0))

    def apply(gate, use):
        strength = gate.strength
        if use.crate.endswith("(lockfile only)") and strength == kern.HARD:
            strength = kern.CONDITIONAL
        finding = Finding(gate, use, strength)
        # A dependency is read whole, so a call in it proves the bit is
        # reachable, not that this capsule reaches it. That is enough to keep
        # a bit; it is not enough to say the capsule fails without one.
        shortfall = finding
        if use.crate != root_crate and strength == kern.HARD:
            shortfall = Finding(gate, use, kern.CONDITIONAL)
        if gate.refuses:
            for n in sorted(gate.names):
                if holds(n):
                    refused.append((n, shortfall))
            return
        if gate.kind == "all":
            for n in sorted(gate.names):
                if holds(n):
                    credit.setdefault(n, []).append(finding)
                else:
                    missing.append((n, shortfall))
            return
        held = [n for n in sorted(gate.names) if holds(n) and n != "Admin"]
        if not held and "Admin" in gate.names and holds("Admin"):
            held = ["Admin"]
        for n in held:
            credit.setdefault(n, []).append(finding)
        if not held:
            missing.append(("one of " + ", ".join(sorted(gate.names)), shortfall))

    for use in uses:
        for tag in sorted(use.tags):
            gate = kernel.syscalls.get(tag)
            if gate is None:
                if use.raw:
                    notes.append(f"{use.cite}: syscall tag {tag} is not in the kernel "
                                 f"enum; the call answers ENOSYS")
                continue
            if gate.names:
                apply(gate, use)
            for extra in kernel.handlers.get(tag, ()):
                if extra.literal is None or extra.literal & literals:
                    apply(extra, use)
        if use.literal and use.literal not in capsule.own_names:
            if use.literal in kernel.network:
                apply(kern.Gate(f"names network service {use.literal}", "all", {"Network"},
                                kern.CONDITIONAL, kernel.network_cite,
                                "sending to it takes Network; naming it alone does not"), use)
            for acl in kernel.acls:
                if use.literal in acl.literal and not use.listed:
                    apply(acl, use)
        if use.port:
            for acl in kernel.acls:
                if use.port in acl.ports and not acl.literal & set(capsule.own_names):
                    apply(acl, use)

    spawn = user.Use(root_crate, str(capsule.mk))
    if capsule.service_names:
        apply(kern.Gate("owns its service endpoint", "all", {"IPC"}, kern.HARD,
                        kernel.owner_cite,
                        "spawn registers the endpoint and refuses an owner without IPC"),
              spawn)
    for name in capsule.service_names:
        if name in kernel.network:
            apply(kern.Gate(f"owns network service {name}", "all", {"Network"}, kern.HARD,
                            kernel.network_cite,
                            "spawn refuses an owner of a network service without Network"),
                  spawn)
    return credit, missing, refused, notes


def _best(findings, root_crate):
    """The findings worth printing first: strongest, then the capsule's own
    code before a dependency's, one per distinct call site."""
    seen, out = set(), []
    for f in sorted(findings, key=lambda f: (RANK[f.strength], f.use.crate != root_crate,
                                             f.use.cite)):
        if f.use.cite in seen:
            continue
        seen.add(f.use.cite)
        out.append(f)
    return out


def sandbox_report(capsule, kernel, grant):
    """For a crates.io tool: the grant it is spawned with, the sandbox bits
    its manifest lacks (the spawn gate refuses the grant), and the bits vfs
    asks that it is not installed with (it cannot open a file). None for a
    capsule that is not a crates.io tool."""
    registry = capsule.bin_name in kernel.sandboxed
    if not registry and capsule.fields.get("CAPSULE_DOMAIN") != "crates.io":
        return None
    if registry:
        grant = kernel.sandbox
    installed = capsule.required | (capsule.optional & (grant or 0))
    return {"grant": f"0x{(grant or 0):X}",
            "from": kernel.sandbox_cite if registry else (capsule.mirror or "its manifest"),
            "lacks": kernel.names(kernel.sandbox & ~capsule.granted),
            "vfs_unheld": kernel.names(kernel.vfs_bits & ~installed)}


def audit_one(root, capsule, kernel, wrappers, cache):
    linked = user.link(root, capsule, wrappers, cache)
    root_crate = (root / capsule.dir).resolve().name
    credit, missing, refused, notes = assess(capsule, linked.uses, kernel, root_crate)
    bits_out = []
    for name in kernel.names(capsule.granted):
        found = _best(credit.get(name, []), root_crate)
        strengths = {f.strength for f in found}
        status = next((STATUS[s] for s in sorted(strengths, key=RANK.get)), "no evidence")
        bits_out.append({
            "name": name, "bit": f"0x{kernel.bits[name]:X}",
            "source": "required" if capsule.required & kernel.bits[name] else "optional",
            "status": status, "evidence_count": len(found),
            "evidence": [f.as_dict() for f in found[:KEEP]]})
    grouped = {}
    for needs, f in missing:
        grouped.setdefault(needs, []).append(f)
    missing_out = []
    for needs, fs in sorted(grouped.items()):
        best = _best(fs, root_crate)
        missing_out.append({"needs": needs, "strength": best[0].strength,
                            "evidence_count": len(best),
                            "evidence": [f.as_dict() for f in best[:KEEP]]})
    refused_out = [{"holds": n, **f.as_dict()} for n, f in refused]
    notes = sorted(set(notes))
    notes += linked.gaps
    mirror = None
    grant = None
    if capsule.mirror:
        m = kern.read_mirror(root, capsule.mirror, kernel.bits)
        grant = m.mask | m.conditional
        outside = grant & ~capsule.granted
        mirror = {"dir": capsule.mirror, "mask": f"0x{m.mask:X}",
                  "conditional": f"0x{m.conditional:X}", "sites": m.sites,
                  "unresolved": m.unresolved,
                  "installs": f"0x{capsule.required | (capsule.optional & grant):X}"}
        if outside:
            notes.append(f"the kernel mirror grants {', '.join(kernel.names(outside))}, which the "
                         f"manifest does not list: the spawn gate refuses it ({kernel.grant_cite})")
        if m.conditional & capsule.required & kernel.bits["Debug"]:
            notes.append("the mirror grants Debug through serial_debug_cap(), which cannot "
                         "withhold a required bit: required bits install whatever the grant")
        for term in m.unresolved:
            notes.append(f"kernel mirror term not read: {term}")
    elif capsule.bin_name in kernel.sandboxed:
        notes.append(f"spawned by the tool registry with SANDBOX_CAPS "
                     f"0x{kernel.sandbox:X} ({kernel.sandbox_cite})")
    elif not capsule.prebuilt:
        notes.append("no CAPSULE_KERNEL_MIRROR: spawned from the store or as a tool, "
                     "with the manifest's required caps")
    sandbox = sandbox_report(capsule, kernel, grant)
    if capsule.required & kernel.bits["Debug"]:
        why = DEBUG_REQUIRED.get(capsule.slug) or EXEMPT.get(capsule.slug)
        notes.append("Debug is in CAPSULE_REQUIRED_CAPS, so every build installs it, "
                     "a release build included"
                     + (f"; kept for {why}" if why else "; it belongs in "
                        "CAPSULE_OPTIONAL_CAPS"))
    if capsule.slug in EXEMPT:
        notes.append(f"exempt from --strict: {EXEMPT[capsule.slug]}")
    declared, where = user.declared_caps(root, capsule)
    if where and declared is None:
        notes.append(where)
    if declared is not None and declared != capsule.required:
        notes.append(f".nonos.caps declares 0x{declared:X} at {where}, the manifest "
                     f"requires 0x{capsule.required:X}")
    if capsule.ceiling is not None and capsule.granted & ~capsule.ceiling:
        notes.append(f"required or optional caps exceed the ceiling 0x{capsule.ceiling:X}")
    return {
        "capsule": capsule.slug, "handle": capsule.handle, "dir": str(capsule.dir),
        "mk": str(capsule.mk), "required": f"0x{capsule.required:X}",
        "optional": f"0x{capsule.optional:X}",
        "ceiling": None if capsule.ceiling is None else f"0x{capsule.ceiling:X}",
        "mirror": mirror,
        "declared": None if declared is None else {"value": f"0x{declared:X}", "at": where},
        "bits": bits_out, "missing": missing_out, "refused": refused_out,
        "sandbox": sandbox, "notes": notes, "crates": linked.crates,
    }


def strict_failures(reports):
    """What --strict refuses, one line per capsule and rule: a call the
    capsule's own code makes with no bit for it, Debug required outside
    DEBUG_REQUIRED, and a crates.io tool whose manifest lacks part of the
    tool sandbox or that is spawned without the bits vfs asks. A capsule in
    EXEMPT is not held to these."""
    out = []
    for r in reports:
        if r["capsule"] in EXEMPT:
            continue
        box = r.get("sandbox")
        if box and box["lacks"]:
            out.append(f"{r['capsule']}: its manifest lacks {', '.join(box['lacks'])} of the "
                       f"tool sandbox, so the spawn gate refuses its grant")
        if box and box["vfs_unheld"]:
            out.append(f"{r['capsule']}: a crates.io tool spawned without "
                       f"{', '.join(box['vfs_unheld'])}, which vfs asks of every request, "
                       f"cannot open a file")
        hard = [m["needs"] for m in r["missing"] if m["strength"] == kern.HARD]
        if hard:
            out.append(f"{r['capsule']}: would fail without {'; '.join(hard)}")
        if r["capsule"] not in DEBUG_REQUIRED and any(
                b["name"] == "Debug" and b["source"] == "required" for b in r["bits"]):
            out.append(f"{r['capsule']}: Debug is required; it belongs in "
                       f"CAPSULE_OPTIONAL_CAPS")
    return out


def audit(root, only=()):
    root = root.resolve()
    caps = user.capsules(root)
    handle_of = {c.slug: c.handle for c in caps}
    kernel = Kernel(root, handle_of)
    wrappers = user.Wrappers(root)
    cache = {}
    return [audit_one(root, c, kernel, wrappers, cache) for c in caps
            if not only or c.slug in only or c.dir.name in only]


# ---------------------------------------------------------------------------
# Reports


def _short(ev):
    via = f" via `{ev['via']}`" if ev.get("via") else ""
    return f"{ev['what']}{via} at `{ev['at']}`"


def _cell(text):
    return text.replace("|", "\\|")


def _missing(r, strengths):
    return [m for m in r["missing"] if m["strength"] in strengths]


def markdown(reports):
    out = ["# Capability audit", "",
           "Generated by `scripts/cap_audit.py --markdown`. A bit marked no evidence is "
           "a candidate to drop, not a proven unused one: read the module docstring for "
           "the ways the audit can miss a use. Would fail means the capsule's own code "
           "makes a call it holds no bit for; to review means a linked crate does, or the "
           "kernel refuses only in a case the audit cannot decide.", "",
           "| capsule | required | no evidence | would fail | to review |",
           "|---|---|---|---|---|"]
    for r in reports:
        none = ", ".join(b["name"] for b in r["bits"] if b["status"] == "no evidence") or "none"
        hard = ", ".join(m["needs"] for m in _missing(r, {kern.HARD})) or "none"
        review = ", ".join(m["needs"] for m in _missing(r, {kern.CONDITIONAL})) or "none"
        out.append(f"| {r['capsule']} | {r['required']} | {none} | {_cell(hard)} | "
                   f"{_cell(review)} |")
    for r in reports:
        out += ["", f"## {r['capsule']}", "",
                f"`{r['mk']}`: required {r['required']}, optional {r['optional']}"
                + (f", ceiling {r['ceiling']}" if r["ceiling"] else "")
                + (f"; kernel mirror `{r['mirror']['dir']}` grants {r['mirror']['mask']}"
                   + (f" (+{r['mirror']['conditional']} by build feature)"
                      if r["mirror"]["conditional"] != "0x0" else "")
                   if r["mirror"] else "")
                + ".", "", "| bit | status | evidence |", "|---|---|---|"]
        for b in r["bits"]:
            ev = _short(b["evidence"][0]) if b["evidence"] else "none found"
            more = f" (+{b['evidence_count'] - 1} more)" if b["evidence_count"] > 1 else ""
            out.append(f"| {b['name']} | {b['status']} | {_cell(ev)}{more} |")
        for m in _missing(r, {kern.HARD, kern.CONDITIONAL}):
            label = "would fail" if m["strength"] == kern.HARD else "to review"
            out.append(f"| {_cell(m['needs'])} (not granted) | {label} | "
                       f"{_cell(_short(m['evidence'][0]))} |")
        for f in r["refused"]:
            out.append(f"| {f['holds']} (refuses) | {f['strength']} | {_cell(_short(f))} |")
        less = _missing(r, {kern.SOFT, kern.DIAGNOSTIC})
        if less:
            out.append("\nRuns with less, not refused: " + "; ".join(
                f"without {m['needs']}, {m['evidence'][0]['why'] or m['evidence'][0]['what']}"
                for m in less) + ".")
        for n in r["notes"]:
            out.append(f"\n{n}")
    return "\n".join(out) + "\n"


def text_summary(reports):
    lines = []
    for r in reports:
        none = [b["name"] for b in r["bits"] if b["status"] == "no evidence"]
        hard = [m["needs"] for m in r["missing"] if m["strength"] == kern.HARD]
        lines.append(f"{r['capsule']}: required {r['required']}"
                     + (f"; no evidence for {', '.join(none)}" if none else "")
                     + (f"; WOULD FAIL without {'; '.join(hard)}" if hard else ""))
    dropped = sum(1 for r in reports for b in r["bits"] if b["status"] == "no evidence")
    failing = sum(1 for r in reports if any(m["strength"] == kern.HARD for m in r["missing"]))
    strict = strict_failures(reports)
    lines += [f"strict: {f}" for f in strict]
    lines.append(f"cap-audit: {len(reports)} capsules, {dropped} granted bits with no "
                 f"evidence, {failing} capsules missing a bit a call needs, "
                 f"{len(strict)} strict failures ({', '.join(sorted(EXEMPT))} exempt)")
    return "\n".join(lines)


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    ap.add_argument("--root", type=Path, default=Path("."), help="repository root")
    ap.add_argument("--capsule", action="append", default=[],
                    help="audit only this slug or directory name (repeatable)")
    ap.add_argument("--markdown", action="store_true", help="print the markdown report")
    ap.add_argument("--json", action="store_true", help="print the JSON report")
    ap.add_argument("--strict", action="store_true",
                    help="exit 1 when a capsule lacks a bit a call it makes needs, requires "
                         "Debug outside DEBUG_REQUIRED, or is a crates.io tool outside the "
                         "sandbox or without what vfs asks (EXEMPT capsules aside)")
    ap.add_argument("--selftest", action="store_true", help="run the parser self-test")
    args = ap.parse_args()

    if args.selftest:
        from cap_audit_selftest import self_test
        return self_test(args.root)
    try:
        reports = audit(args.root, set(args.capsule))
    except kern.SourceMoved as e:
        print(f"cap-audit: {e}; update cap_audit_kernel before trusting the audit",
              file=sys.stderr)
        return 2
    except FileNotFoundError as e:
        print(f"cap-audit: {e}", file=sys.stderr)
        return 2
    if not reports:
        print("cap-audit: no capsules found", file=sys.stderr)
        return 2
    if args.json:
        print(json.dumps(reports, indent=1))
    elif args.markdown:
        print(markdown(reports), end="")
    else:
        print(text_summary(reports))
    if args.strict:
        failing = strict_failures(reports)
        for f in failing:
            print(f"cap-audit --strict: {f}", file=sys.stderr)
        return 1 if failing else 0
    return 0


if __name__ == "__main__":
    sys.exit(main())
