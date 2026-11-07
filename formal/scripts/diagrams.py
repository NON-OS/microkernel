#!/usr/bin/env python3
# NONOS Operating System
# Copyright (C) 2026 NONOS Contributors
#
# This program is free software: you can redistribute it and/or modify it under
# the terms of the GNU Affero General Public License as published by the Free
# Software Foundation, either version 3 of the License, or (at your option) any
# later version. See <https://www.gnu.org/licenses/>.
#
# Draws the formal-verification figures in docs/media in the house style of
# the other kernel diagrams: the banner, logo and type are taken from
# docs/media/diagram-capability-check.svg, so the figures stay in step with
# it. Run from the repository root: python3 -I formal/scripts/diagrams.py

import pathlib
import re
import sys

ROOT = pathlib.Path(__file__).resolve().parents[2]
SOURCE = ROOT / "docs/media/diagram-capability-check.svg"
OUT = ROOT / "docs/media"

SERIF = ("'Libertinus Serif', 'Linux Libertine O', 'Palatino Linotype', Palatino, "
         "'Iowan Old Style', Georgia, 'Times New Roman', serif")
MONO = "ui-monospace, SFMono-Regular, Menlo, Consolas, 'Liberation Mono', monospace"
INK = "#111111"
SUB = "#333333"
NOTE = "#4A4F57"
FILL_KEY = "#EEF1F4"
FILL_SOFT = "#F6F8FA"
WIDTH = 1200


def esc(t):
    return t.replace("&", "&amp;").replace("<", "&lt;").replace(">", "&gt;")


def banner_parts():
    s = SOURCE.read_text()
    image = re.search(r"<image [^>]*/>", s).group(0)
    grad = re.search(r"<defs><linearGradient id=\"bg\".*?</defs>", s).group(0)
    start = s.index('<g transform="translate(20 9) scale(0.056)">')
    end = s.index("</g>", start) + 4
    logo = s[start:end]
    return image, grad, logo


class Fig:
    def __init__(self, name, title, height):
        self.name, self.title, self.h = name, title, height
        self.parts = []

    def box(self, x, y, w, h, head, sub=None, fill="#FFFFFF", bold=1.3, dashed=False):
        dash = ' stroke-dasharray="7 5"' if dashed else ""
        self.parts.append(f'<rect x="{x}" y="{y}" width="{w}" height="{h}" rx="2" fill="{fill}" '
                          f'stroke="{INK}" stroke-width="{bold}"{dash}/>')
        cx = x + w / 2
        if sub:
            lines = sub if isinstance(sub, list) else [sub]
            top = y + h / 2 - 5 - 9.5 * (len(lines) - 1)
            self.text(cx, top, head, 18, 700, INK)
            for i, line in enumerate(lines):
                self.text(cx, top + 24 + 19 * i, line, 15, 400, SUB)
        else:
            self.text(cx, y + h / 2 + 6, head, 18, 700, INK)

    def text(self, x, y, t, size=15, weight=400, fill=INK, anchor="middle", family=SERIF, italic=False):
        style = ' font-style="italic"' if italic else ""
        self.parts.append(f'<text x="{x}" y="{y}" font-family="{family}" font-size="{size}" '
                          f'font-weight="{weight}" fill="{fill}" text-anchor="{anchor}"{style}>{esc(t)}</text>')

    def mono(self, x, y, t, anchor="middle", size=12, fill=INK):
        self.text(x, y, t, size, 400, fill, anchor, MONO)

    def arrow(self, x1, y1, x2, y2, dashed=False):
        dash = ' stroke-dasharray="6 5"' if dashed else ""
        self.parts.append(f'<line x1="{x1}" y1="{y1}" x2="{x2}" y2="{y2}" stroke="#111" '
                          f'stroke-width="1.5"{dash} marker-end="url(#ah)"/>')

    def path(self, d, dashed=False):
        dash = ' stroke-dasharray="6 5"' if dashed else ""
        self.parts.append(f'<path d="{d}" fill="none" stroke="#111" stroke-width="1.5"{dash} '
                          f'marker-end="url(#ah)"/>')

    def panel(self, x, y, w, h, caption):
        self.parts.append(f'<rect x="{x}" y="{y}" width="{w}" height="{h}" rx="2" fill="#FFFFFF" '
                          f'stroke="{INK}" stroke-width="1.4"/>')
        self.text(x + 24, y + 32, caption, 14, 400, SUB, "start", italic=True)

    def num(self, x, y, n):
        self.parts.append(f'<circle cx="{x}" cy="{y}" r="13" fill="#FFFFFF" stroke="#111" stroke-width="1.3"/>')
        self.text(x, y + 5, str(n), 15, 700)

    def render(self, image, grad, logo):
        h = self.h
        head = (f'<svg xmlns="http://www.w3.org/2000/svg" width="{WIDTH}" height="{h}" '
                f'viewBox="0 0 {WIDTH} {h}" role="img" aria-label="{self.name}">'
                '<defs><marker id="ah" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="7" '
                'markerHeight="7" orient="auto-start-reverse"><path d="M0 0 L10 5 L0 10 z" '
                'fill="#111"/></marker></defs>'
                f'<rect x="0.5" y="0.5" width="{WIDTH - 1}" height="{h - 1}" fill="#FFFFFF" stroke="#CDD2D8"/>'
                f'{image}{grad}<rect x="0" y="0" width="{WIDTH}" height="46" fill="url(#bg)"/>{logo}'
                f'<text x="1178" y="29" font-family="{SERIF}" font-size="13" fill="#FFFFFF" '
                f'text-anchor="end" letter-spacing="3">{esc(self.title.upper())}</text>')
        return head + "".join(self.parts) + "</svg>\n"


def fig_layers():
    f = Fig("formal-layers", "What is proven, and about what", 668)
    rows = [
        ("formal/", "one state machine for the kernel core", "24 headline theorems over every reachable state", FILL_KEY),
        ("verification/lean", "one model per mechanism", "201 modules, 1,604 theorems", FILL_SOFT),
        ("Verus, Kani, extraction", "the Rust operations themselves", "393 extracted crates, 1,176 functions", "#FFFFFF"),
        ("userland/*_proofs", "the kernel source, included by path", "run on the host with cargo test", "#FFFFFF"),
    ]
    y = 80
    for i, (head, sub, note, fill) in enumerate(rows):
        f.box(160, y, 520, 86, head, sub, fill, 2.0 if i == 0 else 1.3)
        f.text(720, y + 49, note, 15, 400, NOTE, "start")
        if i < len(rows) - 1:
            f.arrow(420, y + 86, 420, y + 112)
        y += 112
    f.box(160, y, 520, 70, "src/", "the kernel that ships", "#FFFFFF")
    f.path(f"M 160 {y + 35} C 118 {y + 35}, 118 123, 156 123", dashed=True)
    f.text(62, 340, "tables", 14, 400, NOTE, italic=True)
    f.text(62, 358, "read from", 14, 400, NOTE, italic=True)
    f.text(62, 376, "the code by", 14, 400, NOTE, italic=True)
    f.mono(62, 396, "gen_tables", size=11)
    f.text(600, 644, "A dashed arrow is data the build reads from the code; solid arrows point from the claim to what it rests on.",
           14, 400, NOTE, italic=True)
    return f


def fig_state():
    f = Fig("formal-state-machine", "The kernel as one state machine", 616)
    f.panel(48, 70, 1104, 222, "State: what the model keeps, and where it lives in the kernel")
    cells = [
        ("capsules", ["admitted, held, envelope", "token, epoch, allowlist"], "pcb.caps_bits, capability_token"),
        ("mappings", ["per pid: address", "to frame and permission"], "page tables, map_page"),
        ("frames", ["free, one process,", "or one surface slot"], "frame allocator, SLOTS"),
        ("surfaces", ["owner, named, consented,", "attached, released"], "surface_registry"),
        ("inboxes", ["bytes only,", "per receiver"], "IpcMessage"),
    ]
    x = 72
    for head, sub, code in cells:
        f.box(x, 120, 196, 120, head, sub, FILL_SOFT)
        f.mono(x + 98, 270, code, size=11)
        x += 212
    f.panel(48, 312, 1104, 262, "Steps: the only ways the state changes")
    steps = [
        ("spawn", "admission decision passes"), ("load", "ELF and stacks, W^X checked"),
        ("call", "gate passes, then a handler"), ("deny", "EPERM, or the caller killed"),
        ("protect", "W^X checked"), ("quarantine", "kernel revoke"),
        ("teardown", "everything given back"), ("surfaceFree", "last view gone"),
    ]
    for i, (head, sub) in enumerate(steps):
        col, row = i % 4, i // 4
        f.box(72 + col * 268, 362 + row * 102, 248, 84, head, sub, "#FFFFFF")
    f.text(600, 600, "Reachable is every state these steps lead to from boot. Each theorem is stated over Reachable.",
           14, 400, NOTE, italic=True)
    return f


def fig_gate():
    f = Fig("formal-gate", "From a trap to a handler", 560)
    stages = [
        ("A capsule traps", "syscall number, six arguments", "#FFFFFF"),
        ("Resolve the number", "the ABI registry", "#FFFFFF"),
        ("Bind the token", "its own pid, current epoch", "#FFFFFF"),
        ("Test the allowlist", "one bit per registry slot", FILL_KEY),
        ("Run the handler", "its effect belongs to the number", FILL_SOFT),
    ]
    x = 30
    for i, (head, sub, fill) in enumerate(stages):
        f.box(x, 100, 214, 86, head, sub, fill, 2.0 if fill == FILL_KEY else 1.3)
        if i < len(stages) - 1:
            f.arrow(x + 214, 143, x + 236, 143)
        x += 236
    f.mono(373, 222, "unknown number: ENOSYS")
    f.arrow(373, 186, 373, 206, dashed=True)
    f.mono(845, 222, "refused: EPERM, or a native capsule is killed")
    f.arrow(845, 186, 845, 206, dashed=True)
    f.panel(48, 260, 1104, 270, "What Lean proves about this path, in every reachable state")
    rows = [
        ("syscall_gate_completeness", "a handler runs only past the allowlist bit, and the list is derived from the word the capsule holds"),
        ("syscall_outside_allowlist_denied", "outside the list the only outcomes are EPERM with nothing changed, or the caller torn down"),
        ("handler_requires_capability", "grant needs Admin, mmap needs Memory, IPC needs IPC, surfaces need graphics bits"),
        ("revoke_takes_effect", "a revoked bit is gone from the next decision; there is no stale token to use"),
        ("empty_word_reaches_only_tty_query_or_random", "a token with no capability reaches two syscalls: its streams and random bytes"),
    ]
    for i, (name, says) in enumerate(rows):
        y = 316 + i * 42
        f.mono(72, y, name, anchor="start", size=13)
        f.text(430, y, says, 15, 400, INK, "start")
    return f


def fig_grant():
    f = Fig("formal-grant-bound", "The grant bound, before and after", 600)
    f.panel(48, 70, 540, 380, "Before: the caller's word was the only bound")
    f.box(80, 130, 220, 86, "capsule_acpi", ["holds Admin, Driver,", "Mmio, Irq"], "#FFFFFF")
    f.box(340, 130, 220, 86, "any live pid", "manifest names IPC", "#FFFFFF")
    f.arrow(300, 173, 340, 173)
    f.text(320, 250, "MkCapGrant(Driver | Mmio | Irq | Admin)", 15, 400, INK)
    f.box(210, 290, 220, 86, "accepted", "the pid now drives hardware", FILL_KEY, 2.0)
    f.arrow(320, 262, 320, 290, dashed=True)
    f.mono(318, 420, "AdminGrant.old_rule_grant_exceeds_admitted")
    f.panel(612, 70, 540, 380, "After: the target's envelope bounds every grant")
    f.box(644, 130, 220, 86, "capsule_acpi", ["holds Admin, Driver,", "Mmio, Irq"], "#FFFFFF")
    f.box(904, 130, 220, 86, "any live pid", "envelope: IPC", "#FFFFFF")
    f.arrow(864, 173, 904, 173)
    f.text(884, 250, "mask within the target's envelope?", 15, 400, INK)
    f.box(774, 290, 220, 86, "refused", "EPERM, nothing changes", "#FFFFFF", 2.0)
    f.arrow(884, 262, 884, 290, dashed=True)
    f.mono(884, 420, "AdminGrant.fixed_rule_refuses_the_old_witness")
    f.text(600, 490, "envelope = profile_gate::caps((required | optional) & image ceiling) | installed word,", 15, 400, INK)
    f.text(600, 512, "recorded once at spawn (process::caps_envelope); zero for a process no manifest admitted.", 15, 400, INK)
    f.mono(600, 550, "capability_monotonicity: held ⊆ (required | optional) & image ceiling, in every reachable state")
    return f


def fig_drift():
    f = Fig("formal-drift", "How the model keeps up with the code", 520)
    f.box(48, 100, 250, 96, "The Rust tables", ["cap_table, capability bits,", "ABI registry, policy"], "#FFFFFF")
    f.box(346, 100, 230, 96, "gen_tables.py", ["reads them, refuses", "anything it cannot read"], FILL_SOFT)
    f.box(624, 100, 230, 96, "Formal/Data", ["190 syscalls,", "41 capability bits"], FILL_SOFT)
    f.box(902, 100, 250, 96, "lake build", ["every theorem checked", "against that data"], FILL_KEY, 2.0)
    f.arrow(298, 148, 346, 148)
    f.arrow(576, 148, 624, 148)
    f.arrow(854, 148, 902, 148)
    f.panel(48, 236, 1104, 250, "Two checks that fail when the code and the model part")
    f.num(87, 296, 1)
    f.text(116, 302, "CI regenerates the tables and fails on any difference from the committed files.", 16, 400, INK, "start")
    f.mono(116, 326, "python3 -I formal/scripts/gen_tables.py --check", anchor="start")
    f.num(87, 378, 2)
    f.text(116, 384, "kernel_proofs reads the Lean data back and asks the real cap_table and the real allowlist", 16, 400, INK, "start")
    f.text(116, 406, "the same question on every syscall, both launcher answers, hundreds of capability words.", 16, 400, INK, "start")
    f.mono(116, 430, "the_model_gate_is_the_kernel_gate, the_shipped_allowlist_is_the_model_list", anchor="start")
    f.text(600, 470, "Changing one arm of cap_table/mk_trust.rs trips both.", 14, 400, NOTE, italic=True)
    return f


def fig_isolation():
    f = Fig("formal-isolation", "Memory between capsules", 600)
    f.box(60, 90, 300, 96, "capsule A", "owner of surface s", "#FFFFFF")
    f.box(840, 90, 300, 96, "capsule B", "named by A on s", "#FFFFFF")
    f.box(60, 250, 300, 80, "A's private frames", "mapped once, by A only", FILL_SOFT)
    f.box(840, 250, 300, 80, "B's private frames", "mapped once, by B only", FILL_SOFT)
    f.box(450, 250, 300, 80, "surface s frames", "held by the slot, not a pid", FILL_KEY, 2.0)
    f.arrow(210, 186, 210, 250)
    f.arrow(990, 186, 990, 250)
    f.arrow(300, 186, 520, 250)
    f.arrow(900, 186, 680, 250)
    f.text(600, 150, "share_surface: A names B", 15, 400, INK)
    f.arrow(360, 160, 840, 160, dashed=True)
    f.text(600, 362, "views are never executable; B's view is writable only if A's share said so", 15, 400, NOTE, italic=True)
    f.panel(48, 390, 1104, 180, "What isolation_preservation and dead_holds_nothing state")
    f.text(72, 452, "Two capsules share a frame only as views of one surface, each the project or a receiver the project named.",
           16, 400, INK, "start")
    f.text(72, 482, "A private frame is in one address space at one address. Every token is bound to its own capsule.",
           16, 400, INK, "start")
    f.text(72, 512, "A pid with no capsule holds no mapping, frame, view, right or message, so a reused pid starts empty.",
           16, 400, INK, "start")
    f.text(72, 542, "Every step preserves this: spawn, load, each handler, protect, quarantine, teardown, slot free.",
           16, 400, INK, "start")
    return f


def fig_refine():
    f = Fig("formal-refine", "From the Rust to the model", 640)
    f.box(48, 90, 300, 104, "src/syscall/contract", ["cap_table, allowlist,", "the token checks"], "#FFFFFF")
    f.box(450, 90, 300, 104, "Charon, Aeneas", ["the MIR rustc compiles,", "translated to Lean"], FILL_SOFT)
    f.box(852, 90, 300, 104, "CapTable, Allowlist", ["the extracted functions,", "regenerated by CI"], FILL_SOFT)
    f.arrow(348, 142, 450, 142)
    f.arrow(750, 142, 852, 142)
    f.box(852, 280, 300, 104, "formal/", ["capTableAllows, allowed:", "what the gate theorem uses"], FILL_KEY, 2.0)
    f.path("M 1002 194 L 1002 276")
    f.text(990, 232, "proved equal,", 14, 400, NOTE, "end", italic=True)
    f.text(990, 252, "190 rows", 14, 400, NOTE, "end", italic=True)
    f.box(450, 280, 300, 104, "FunsExternal.lean", ["six library calls defined,", "clock and index opaque"], "#FFFFFF")
    f.arrow(750, 332, 852, 332, dashed=True)
    f.panel(48, 420, 1104, 190, "What the two refinements state, and what they rest on")
    f.mono(72, 476, "is_allowed_is_the_model: is_allowed t n l = ok (capTableAllows (word ps) l (tag n))", anchor="start")
    f.mono(72, 502, "set_spec, allowed_reads_its_slot: the bitmap holds and answers exactly what was set", anchor="start")
    f.text(72, 544, "Trusted: rustc, Charon and Aeneas; the six library definitions, each a few lines checked", 16, 400, INK, "start")
    f.text(72, 568, "against the Rust documentation. No axiom beyond propext, Quot.sound and Classical.choice.", 16, 400, INK, "start")
    f.text(600, 596, "A solid arrow is a translation or a proof; the dashed arrow is a definition the proof uses.",
           14, 400, NOTE, italic=True)
    return f


def main():
    image, grad, logo = banner_parts()
    figs = [fig_layers(), fig_state(), fig_gate(), fig_grant(), fig_drift(), fig_isolation(), fig_refine()]
    for fig in figs:
        (OUT / f"diagram-{fig.name}.svg").write_text(fig.render(image, grad, logo))
    return 0


if __name__ == "__main__":
    sys.exit(main())
