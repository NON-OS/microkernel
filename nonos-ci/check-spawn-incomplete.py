#!/usr/bin/env python3
# NONOS Operating System (AGPL-3.0-or-later)
"""A boot whose capsules did not all spawn prints "[INIT] capsule spawn
incomplete" in place of "[INIT] Capsules spawned". Each boot check is run on a
complete log and an incomplete one: the evidence script, the matrix verdict and
the QEMU runner's watch must pass the first and fail the second.

Usage: check-spawn-incomplete.py              check the tree
       check-spawn-incomplete.py --selftest   also show each check, mutated, fails it
"""
import importlib, os, shutil, subprocess, sys, tempfile

TOP = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
COMPLETE = "[NONOS] Handoff OK\n[RAMFS] capsule spawned\n[INIT] Capsules spawned\n"
INCOMPLETE = ("[NONOS] Handoff OK\n[ERROR] RAMFS: capsule ZK attestation failed\n"
              "[INIT] capsule spawn incomplete: 1 failed\n")
MUTANTS = [("scripts/qemu_boot_evidence.sh", "|\\[INIT\\] capsule spawn incomplete", ""),
           ("scripts/bootmatrix/verdict.py", ',\n         "[INIT] capsule spawn incomplete"', ""),
           ("tools/nonos_qemu/__main__.py", '"[INIT] capsule spawn incomplete" in text', "False")]


def evidence(root, log, tmp):
    env = dict(os.environ, NONOS_BOOT_EVIDENCE_OUT=tmp, NONOS_BOOT_EVIDENCE_LOG=tmp + "/serial.log",
               NONOS_BOOT_EVIDENCE_TIMEOUT="20", NONOS_BOOT_EVIDENCE_CMD=f"cp {log} {tmp}/serial.log")
    return subprocess.run(["sh", f"{root}/scripts/qemu_boot_evidence.sh"], env=env,
                          capture_output=True).returncode


def failures(root):
    """What each check got wrong, in words; empty when every one held."""
    bad, tmp = [], tempfile.mkdtemp()
    sys.path[:0] = [f"{root}/scripts", f"{root}/tools", f"{TOP}/tools"]
    for k in [m for m in sys.modules if m.split(".")[0] in ("bootmatrix", "nonos_qemu")]:
        del sys.modules[k]
    verdict, cell = (importlib.import_module(f"bootmatrix.{m}") for m in ("verdict", "cell"))
    watch = importlib.import_module("nonos_qemu.__main__").watch
    proc = type("Running", (), {"poll": lambda self: None, "returncode": None})()
    for name, text, want in (("complete", COMPLETE, 0), ("incomplete", INCOMPLETE, 1)):
        log = f"{tmp}/{name}.log"
        with open(log, "w") as f:
            f.write(text)
        if evidence(root, log, tmp) != want:
            bad.append(f"qemu_boot_evidence.sh on a {name} log")
        q35 = cell.Cell("q35-up", "up", "q35", 1)
        said = any("capsule spawn incomplete" in r for r in verdict.judge(q35, text, True, ""))
        if said != bool(want):
            bad.append(f"bootmatrix verdict on a {name} log")
        if watch(proc, log, 4, ["Capsules spawned"] if not want else ["Handoff OK"]) != want:
            bad.append(f"nonos_qemu watch on a {name} log")
    del sys.path[:3]
    shutil.rmtree(tmp)
    return bad


def selftest():
    for rel, old, new in MUTANTS:
        root = tempfile.mkdtemp()
        for d in ("scripts", "tools/nonos_qemu"):
            shutil.copytree(f"{TOP}/{d}", f"{root}/{d}", ignore=shutil.ignore_patterns("__pycache__"))
        text = open(f"{root}/{rel}").read()
        assert old in text, f"mutant site gone from {rel}"
        open(f"{root}/{rel}", "w").write(text.replace(old, new, 1))
        if not failures(root):
            sys.exit(f"selftest: {rel} without its incomplete check still passes")
        shutil.rmtree(root)
        print(f"caught: {rel} without its incomplete check")


if __name__ == "__main__":
    if "--selftest" in sys.argv:
        selftest()
    wrong = failures(TOP)
    sys.exit("spawn incomplete: " + "; ".join(wrong) if wrong else 0)
