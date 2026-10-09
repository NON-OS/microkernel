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
"""The audit's parsers held to small trees whose answers are known, then the
real tree read end to end. Read by cap_audit --selftest.

Each case is one way the audit was wrong while it was written: a `;` inside
`[u8; 32]` taken for the end of a signature, a module list kept in an
include!d file, a binary calling its own package's library with no
dependency line, a comment naming a wrapper counted as a call."""

import tempfile
from pathlib import Path

import cap_audit
import cap_audit_kernel as kern
import cap_audit_userland as user
from cap_audit_rust import Source, cargo_manifest, module_files


def _write(root, files):
    for rel, text in files.items():
        path = root / rel
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(text)


def check_scan():
    text = ('fn a() { // mk_debug(\n let s = b"net.tcp"; /* mk_exit( */ mk_ipc_call(1);\n'
            " let c = '{'; let r = r#\"x\"y\"#; x.mk_kill(2); }\nfn b<'a>(x: &'a str);\n"
            "pub fn c(out: &mut [u8; 32]) -> i64 { mk_device_secret(out) }\n")
    s = Source(Path("x.rs"), text)
    assert len(s.code) == len(text) and len(s.plain) == len(text), "offsets moved"
    assert "mk_debug" not in s.code and "mk_exit" not in s.code, "a comment was read as code"
    assert [lit for _, lit in s.literals] == ["net.tcp", 'x"y'], s.literals
    names = [n for _, n, _ in s.calls()]
    assert "mk_ipc_call" in names and "mk_device_secret" in names, names
    assert "mk_kill" not in names, "a method call was read as a free function"
    fns = {n for n, _, _ in s.functions()}
    assert fns == {"a", "c"}, f"declaration or array signature misread: {fns}"
    assert s.line(text.index("mk_ipc_call")) == 2


def check_cargo(tmp):
    _write(tmp, {
        "app/Cargo.toml": (
            '[package]\nname = "app"\n[[bin]]\nname = "app"\npath = "src/main.rs"\n'
            '[features]\ndefault = ["rt"]\nrt = ["libc/heap"]\n'
            '[dependencies]\nlibc = { package = "nonos_userland_libc", path = "../libc", '
            'default-features = false, features = ["panic-handler"] } # a "#" comment\n'
            '[dependencies.helper]\npath = "../helper"\n'
            '[dev-dependencies]\nproof = { path = "../proof" }\n'),
    })
    deps, lib, bins, default, feats = cargo_manifest(tmp / "app/Cargo.toml")
    by = {d.name: d for d in deps}
    assert set(by) == {"libc", "helper"}, f"dev-dependency read, or a table missed: {by}"
    assert not by["libc"].default_features and by["libc"].features == {"panic-handler"}
    assert bins == {"app": tmp / "app/src/main.rs"} and lib is None
    assert default == {"rt"} and feats["rt"] == {"libc/heap"}


def check_modules(tmp):
    _write(tmp, {
        "m/src/lib.rs": 'include!("mods.rs");\n#[cfg(test)]\nmod tests;\n'
                        '#[path = "elsewhere/odd.rs"]\nmod odd;\n',
        "m/src/mods.rs": "pub mod a;\n",
        "m/src/a.rs": "mod b;\n",
        "m/src/a/b.rs": "",
        "m/src/tests.rs": "",
        "m/src/elsewhere/odd.rs": "",
    })
    got = {p.relative_to(tmp / "m/src").as_posix() for p in module_files(tmp / "m/src/lib.rs")}
    assert got == {"lib.rs", "mods.rs", "a.rs", "a/b.rs", "elsewhere/odd.rs"}, got


def check_wrappers(tmp):
    """A wrapper reached through another wrapper, and a renamed export."""
    _write(tmp, {
        "userland/libc/Cargo.toml": '[package]\nname = "nonos_userland_libc"\n',
        "userland/libc/src/lib.rs": (
            "mod numbers;\npub mod debug;\npub mod heap;\nmod panic;\n"
            "pub use heap::{init as heap_init};\npub use debug::mk_debug;\n"),
        "userland/libc/src/numbers.rs": (
            'pub(crate) const N_MK_DEBUG: i64 = tag4(b"MDBG");\n'
            'pub(crate) const N_MK_MMAP: i64 = tag4(b"MMAP");\n'),
        "userland/libc/src/debug.rs": (
            "pub fn mk_debug(p: *const u8, n: usize) -> i64 { call_raw(N_MK_DEBUG, [0; 6]) }\n"),
        "userland/libc/src/heap/mod.rs": "mod grow;\npub use grow::init;\n",
        "userland/libc/src/heap/grow.rs": (
            "pub fn init() -> Result<(), ()> { map(4096); Ok(()) }\n"
            "fn map(n: usize) -> i64 { call_raw(N_MK_MMAP, [0; 6]) }\n"),
        "userland/libc/src/panic.rs": "fn panic() -> ! { mk_debug(0 as _, 0); loop {} }\n",
        "userland/nonos_abi/Cargo.toml": '[package]\nname = "nonos_abi"\n',
        "userland/nonos_abi/src/lib.rs": "",
    })
    w = user.Wrappers(tmp)
    assert w.tags_of("heap_init") == {"MMAP"}, w.tags_of("heap_init")
    assert w.tags_of("mk_debug") == {"MDBG"}
    assert "heap_init" in w.exports["libc"] and "mk_debug" in w.exports["libc"]
    assert {"MDBG"} <= set().union(*(w.tags_of(n) for n in w.implicit["panic-handler"]))
    _write(tmp, {"userland/app/src/main.rs": (
        "use nonos_libc::heap_init;\n// mk_debug(\nfn main() { heap_init(); "
        "nonos_libc::debug::mk_debug(0 as _, 0); }\n")})
    uses = user.crate_uses(tmp, "app", [tmp / "userland/app/src/main.rs"], w,
                           {"nonos_libc": "libc"})
    found = {(u.via, tuple(sorted(u.tags))) for u in uses if u.tags}
    assert found == {("heap_init", ("MMAP",)), ("mk_debug", ("MDBG",))}, found
    none = user.crate_uses(tmp, "app", [tmp / "userland/app/src/main.rs"], w, {})
    assert not [u for u in none if u.tags], "a crate without the libc reached into it"
    return w


def check_link(tmp, w):
    """A binary that reaches the libc only through its own package's library,
    and a raw syscall written as a bare constant."""
    _write(tmp, {
        "userland/capsule_x/Capsule.mk": (
            "CAPSULE_SLUG := x\nCAPSULE_HANDLE := app.x\nCAPSULE_DIR := userland/capsule_x\n"
            "CAPSULE_BIN_NAME := x\nCAPSULE_SERVICE_ENDPOINT := service:1:app.x\n"
            "CAPSULE_REQUIRED_CAPS := 0x19\n"),
        "userland/capsule_x/Cargo.toml": (
            '[package]\nname = "x"\n[[bin]]\nname = "x"\npath = "src/main.rs"\n'
            '[dependencies]\nnonos_libc = { package = "nonos_userland_libc", path = "../libc", '
            "default-features = false }\n"),
        "userland/capsule_x/src/main.rs": "fn main() { x::start(); }\n",
        "userland/capsule_x/src/lib.rs": (
            "pub fn start() { nonos_libc::heap_init(); poke(); }\n"
            "fn poke() { nonos_libc::mk_syscall_raw(0x504D_4D4D, [0; 6]); }\n"),
    })
    capsule = user.Capsule(tmp / "userland/capsule_x/Capsule.mk", tmp)
    linked = user.link(tmp, capsule, w, {})
    tags = {t for u in linked.uses for t in u.tags}
    assert tags == {"MMAP", "MMMP"}, f"own library or raw constant missed: {tags}"
    assert not linked.libc_features, linked.libc_features


def check_mirror(tmp):
    _write(tmp, {
        "src/capabilities/types/defs.rs": "",
        "src/k/cap/spawn.rs": (
            "const BASE: u64 = Capability::IPC.bit() | Capability::Memory.bit();\n"
            "fn extra() -> u64 { Capability::Crypto.bit() }\n"
            "fn one() { Spec { requested_caps: BASE | extra()\n"
            "    // a comment | Capability::Admin.bit()\n"
            "    | crate::capabilities::serial_debug_cap(), x: 1 }; }\n"
            "fn two(role: &Role) { Spec { requested_caps: BASE | role.extra_caps, y: 2 }; }\n"),
        "src/k/cap/roles.rs": (
            "pub struct Role { pub extra_caps: u64 }\n"
            "const A: Role = Role { extra_caps: Capability::Network.bit() };\n"
            "const B: Role = Role { extra_caps: 0 };\n"),
    })
    bits = {"Network": 4, "IPC": 8, "Memory": 16, "Crypto": 32, "Debug": 256, "Admin": 512}
    m = kern.read_mirror(tmp, Path("src/k/cap"), bits)
    assert m.unresolved == [], m.unresolved
    assert m.mask == 8 | 16 | 32 | 4, hex(m.mask)
    assert m.conditional == 256 and len(m.sites) == 2


class _Capsule:
    def __init__(self, granted, own=("me",)):
        self.granted = granted
        self.own_names = list(own)
        self.service_names = list(own)
        self.mk = Path("userland/me/Capsule.mk")


class _Kernel:
    """Just enough of cap_audit.Kernel for assess."""

    def __init__(self):
        self.bits = {"CoreExec": 1, "Network": 4, "IPC": 8, "Memory": 16, "Debug": 256,
                     "Admin": 512, "DeviceEnum": 0x8000, "AttestRead": 0x80000000}
        self.syscalls = {
            "MMAP": kern.Gate("MkMmap", "all", {"Memory"}, kern.HARD, "t"),
            "MDLS": kern.Gate("MkDeviceList", "any", {"DeviceEnum", "Admin"}, kern.HARD, "t"),
            "MDBG": kern.Gate("MkDebug", "all", {"Debug"}, kern.DIAGNOSTIC, "t"),
            "MADC": kern.Gate("MkAttestDoc", "all", {"AttestRead"}, kern.HARD, "t"),
        }
        self.handlers = {"MADC": [kern.Gate("h", "all", {"Network"}, kern.HARD, "t",
                                            refuses=True)]}
        self.network, self.network_cite = ["net.tcp"], "t"
        self.owner_cite, self.acls, self.grant_cite = "t", [], "t"


def check_assess():
    k = _Kernel()
    own = "capsule_me"
    uses = [user.Use(own, "a:1", {"MMAP"}), user.Use(own, "a:2", {"MDLS"}),
            user.Use(own, "a:3", {"MDBG"}), user.Use("dep", "b:1", {"MADC"}),
            user.Use(own, "a:4", literal="net.tcp"), user.Use(own, "a:5", literal="me")]
    credit, missing, refused, _ = cap_audit.assess(_Capsule(1 | 8 | 16 | 512 | 4), uses, k, own)
    assert set(credit) == {"IPC", "Memory", "Admin", "Network"}, set(credit)
    assert [n for n, _ in missing] == ["Debug", "AttestRead"], missing
    assert missing[0][1].strength == kern.DIAGNOSTIC
    assert missing[1][1].strength == kern.CONDITIONAL, "a dependency's call said fail"
    assert [n for n, _ in refused] == ["Network"]
    # With DeviceEnum held, Admin is not what the enumeration is credited to.
    credit, _, _, _ = cap_audit.assess(_Capsule(8 | 512 | 0x8000), uses[1:2], k, own)
    assert "Admin" not in credit and "DeviceEnum" in credit


def check_vfs_reach(tmp):
    """vfs is reached by its name, by its fixed port, or through the client
    module that writes the name once. A name in a table of names is not a
    message, the client module's own name is its definition, and the bit is
    owed by a capsule whose own code reaches vfs, not by every crate that
    links the client."""
    _write(tmp, {
        "userland/app_skeleton/src/clients/vfs/types.rs": 'pub const NAME: &[u8] = b"vfs_pool";\n',
        "userland/app_skeleton/src/clients/vfs/resolve.rs": "const VFS_FIXED_PORT: u32 = 4104;\n",
        "userland/me/src/a.rs": (
            "use nonos_app_skeleton::clients::{\n    ui, vfs,\n};\n"
            'const CRITICAL: &[&[u8]] = &[b"init", b"vfs_pool"];\n'),
        "userland/me/src/b.rs": "pub(super) const VFS_PORT: u32 = 4104;\n",
        "userland/me/src/c.rs": 'fn f() { lookup(b"vfs_pool", 0); }\n',
    })
    files = sorted((tmp / "userland/me").rglob("*.rs")) + sorted(
        (tmp / "userland/app_skeleton").rglob("*.rs"))
    uses = {(Path(u.cite).name.split(":")[0], u.literal, u.port, u.listed)
            for u in user.crate_uses(tmp, "me", files, user.Wrappers.EMPTY, {})
            if u.literal == "vfs_pool" or u.port}
    assert uses == {("a.rs", "vfs_pool", None, False), ("a.rs", "vfs_pool", None, True),
                    ("b.rs", None, 4104, False), ("c.rs", "vfs_pool", None, False)}, uses

    k = _Kernel()
    k.bits["FileSystem"] = 0x40
    k.acls = [kern.Gate("service vfs_pool", "all", {"FileSystem"}, kern.HARD, "t",
                        literal=frozenset({"vfs_pool"}), ports=frozenset({4104}))]
    named = user.Use("capsule_me", "a:1", literal="vfs_pool")
    listed = user.Use("capsule_me", "a:2", literal="vfs_pool", listed=True)
    port = user.Use("capsule_me", "b:1", via="VFS_PORT", port=4104)
    dep = user.Use("lib", "l:1", literal="vfs_pool")
    for use in (named, port):
        _, missing, _, _ = cap_audit.assess(_Capsule(0x19), [use], k, "capsule_me")
        assert [(n, f.strength) for n, f in missing] == [("FileSystem", kern.HARD)], missing
    _, missing, _, _ = cap_audit.assess(_Capsule(0x19), [dep], k, "capsule_me")
    assert [(n, f.strength) for n, f in missing] == [("FileSystem", kern.CONDITIONAL)]
    for granted, uses_, own in ((0x19, [listed], ("me",)), (0x59, [named, port], ("me",)),
                                (0x19, [named, port], ("vfs_pool",))):
        credit, missing, _, _ = cap_audit.assess(_Capsule(granted, own), uses_, k, "capsule_me")
        assert not missing, (granted, own, missing)


def check_sandbox(tmp):
    """The tool sandbox is read from its own constant, a registry entry with a
    set of its own is not in it, and --strict holds every crates.io tool to
    the sandbox and to what vfs asks, so the two cannot drift apart."""
    _write(tmp, {
        "src/userspace/tool_capsules/spec.rs": (
            "pub(super) const SANDBOX_CAPS: u64 = Capability::CoreExec.bit()\n"
            "    | Capability::IPC.bit() | Capability::Memory.bit()\n"
            "    | Capability::FileSystem.bit();\n"),
        "src/userspace/tool_capsules/registry.rs": (
            'tool_capsule!(\n    "tool.a",\n    1,\n    "endpoint.tool.a.reply",\n    2,\n'
            '    "../a",\n    "a"\n),\n'
            '// a comment, with (parens)\n'
            'tool_capsule!("tool.b", 3, "r", 4, concat!("x", env!("Y"), "/b"), "b", CLI_CAPS),\n'),
    })
    bits = {"CoreExec": 1, "IPC": 8, "Memory": 16, "FileSystem": 0x40}
    mask, _ = kern.tool_sandbox(tmp, bits)
    assert mask == 0x59, hex(mask)
    assert kern.sandboxed_tools(tmp) == {"a"}, kern.sandboxed_tools(tmp)

    def report(name, lacks=(), unheld=()):
        return {"capsule": name, "bits": [], "missing": [],
                "sandbox": {"grant": "0x59", "from": "t", "lacks": list(lacks),
                            "vfs_unheld": list(unheld)}}
    fails = cap_audit.strict_failures([report("ok"), report("old", lacks=["FileSystem"]),
                                       report("bare", unheld=["FileSystem"])])
    assert [f.split(":")[0] for f in fails] == ["old", "bare"], fails


def check_strict():
    """--strict holds a capsule to its own calls and to an optional Debug;
    EXEMPT and DEBUG_REQUIRED are the only ways out, and a dependency's call
    is not a failure."""
    def report(name, debug=None, needs=None, strength=kern.HARD):
        return {"capsule": name,
                "bits": [{"name": "Debug", "source": debug}] if debug else [],
                "missing": [{"needs": needs, "strength": strength}] if needs else []}
    fails = cap_audit.strict_failures([
        report("app", debug="required"), report("app2", debug="optional"),
        report("tool", needs="Memory"), report("dep", needs="Admin",
                                               strength=kern.CONDITIONAL),
        report("attack", debug="required", needs="Network"),
        report("linux", debug="required")])
    assert fails == ["app: Debug is required; it belongs in CAPSULE_OPTIONAL_CAPS",
                     "tool: would fail without Memory"], fails
    assert "attack" in cap_audit.EXEMPT and "linux" in cap_audit.DEBUG_REQUIRED


def check_tree(root):
    """The real tree parses, and its parsers agree with what the kernel says."""
    gates = kern.syscall_gates(root)
    assert gates["MMAP"].kind == "all" and gates["MMAP"].names == {"Memory"}
    assert gates["MDLS"].kind == "any" and gates["MDLS"].names == {"DeviceEnum", "Admin"}
    assert gates["MDBG"].strength == kern.DIAGNOSTIC
    kern.handler_gates(root)
    names, _ = kern.network_services(root)
    assert "net.tcp" in names and "net.socks5" in names, names
    reports = cap_audit.audit(root)
    assert len(reports) >= 90, len(reports)
    for r in reports:
        for b in r["bits"]:
            if b["status"] != "no evidence":
                assert b["evidence"], f"{r['capsule']} {b['name']} credited with no site"
    acls = kern.service_acls(root, {"vfs": "vfs_pool", "driver-virtio-blk": "driver.virtio_blk0"})
    assert any(4104 in a.ports and a.names == {"FileSystem"} for a in acls), "vfs gate not read"
    bits = kern.bits(root)
    sandbox, _ = kern.tool_sandbox(root, bits)
    assert sandbox & bits["FileSystem"], "the tool sandbox no longer holds FileSystem"
    tools = [r for r in reports if r["sandbox"]]
    assert len(tools) >= 7 and all(not r["sandbox"]["lacks"] and not r["sandbox"]["vfs_unheld"]
                                   for r in tools), [r["sandbox"] for r in tools]
    pm = next(r for r in reports if r["capsule"] == "process-manager")
    assert not [m for m in pm["missing"] if m["needs"] == "FileSystem"], \
        "process-manager's table of critical names was read as a call to vfs"
    return len(reports)


def self_test(root):
    with tempfile.TemporaryDirectory() as d:
        tmp = Path(d)
        check_scan()
        check_cargo(tmp)
        check_modules(tmp)
        check_link(tmp, check_wrappers(tmp))
        check_mirror(tmp)
        check_vfs_reach(tmp)
        check_sandbox(tmp)
    check_assess()
    check_strict()
    count = check_tree(Path(root).resolve())
    print(f"cap-audit: self-test passed, the parsers read every planted case and "
          f"{count} real capsules")
    return 0
