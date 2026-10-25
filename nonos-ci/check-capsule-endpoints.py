#!/usr/bin/env python3
# Every capsule's IPC numbers and inbox names, from both places they are
# written: its Capsule.mk (what the seal records) and the kernel's spawn
# constants (what spawn registers). check-capsule-ports.sh sees only the
# second, and only `const ..._PORT` lines, so two drivers could share a reply
# inbox, or a Capsule.mk could drift from its spawn file after a renumber, and
# nothing failed until a spawn hit EndpointCollision on a booted machine.
# A capsule is known by its service name, which both places spell the same.
import pathlib
import re
import sys

ROOT = pathlib.Path(sys.argv[1] if len(sys.argv) > 1 else ".")
MK_TOKEN = re.compile(r"\b(service|reply):(\d+):(\S+)")
RS_NAME = re.compile(r"const SERVICE_NAME\s*:\s*&str\s*=\s*\"([^\"]+)\"")
RS_PORT = re.compile(r"const (SERVICE|REPLY)_PORT\s*:\s*u32\s*=\s*(\d+)\s*;")
RS_INBOX = re.compile(r"const REPLY_INBOX\s*:\s*&str\s*=\s*\"([^\"]+)\"")
FIELDS = ("CAPSULE_SERVICE_ENDPOINT", "CAPSULE_REPLY_ENDPOINT", "CAPSULE_INSTANCE_ENDPOINTS")

claims, errors, declared, matched = {}, [], {}, set()


def claim(kind, value, owner):
    first = claims.setdefault((kind, value), owner)
    if first != owner:
        errors.append(f"{kind} {value} is held by {first} and by {owner}")


for mk in sorted(ROOT.glob("userland/*/Capsule.mk")):
    text = mk.read_text().replace("\\\n", " ")
    fields = dict(re.findall(r"^(CAPSULE_\w+)\s*:=\s*(.*)$", text, re.M))
    tokens = MK_TOKEN.findall(" ".join(fields.get(k, "") for k in FIELDS))
    owner = str(mk.relative_to(ROOT))
    for role, port, name in tokens:
        claim("port", int(port), owner)
        claim("name", name, owner)
    # The capsule's own pair is its first service token and first reply token.
    service = next((t for t in tokens if t[0] == "service"), None)
    reply = next((t for t in tokens if t[0] == "reply"), None)
    if service and reply:
        declared[service[2]] = (owner, int(service[1]), int(reply[1]), reply[2])

for rs in sorted(ROOT.glob("src/**/*.rs")):
    text = rs.read_text()
    name = RS_NAME.search(text)
    ports = dict(RS_PORT.findall(text))
    inbox = RS_INBOX.search(text)
    if not name or "SERVICE" not in ports:
        continue
    where = str(rs.relative_to(ROOT))
    spawn = (int(ports["SERVICE"]), int(ports.get("REPLY", -1)), inbox and inbox.group(1))
    if name.group(1) in declared:
        owner, *mk = declared[name.group(1)]
        for label, ours, theirs in zip(("service port", "reply port", "inbox"), spawn, mk):
            if ours not in (theirs, -1, None):
                errors.append(f"{where}: {label} {ours}, but {owner} says {theirs}")
        matched.add(name.group(1))
        continue
    # A capsule the kernel spawns with no Capsule.mk still shares the numbers.
    claim("port", spawn[0], where)
    if spawn[1] != -1:
        claim("port", spawn[1], where)
    if spawn[2]:
        claim("name", spawn[2], where)
    claim("name", name.group(1), where)

for line in errors:
    print(f"check-capsule-endpoints: {line}", file=sys.stderr)
if errors:
    sys.exit(1)
print(f"check-capsule-endpoints: {len(claims)} numbers and names, each held once; "
      f"{len(matched)} spawn files agree with their Capsule.mk")
