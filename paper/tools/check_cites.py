#!/usr/bin/env python3
"""Check every source citation in paper/sections: the file exists in the
tree, and a cited line range lies inside the file. Run from the repo root.
Looks at \\src, \\srcs, \\srcss, \\srcsss and \\path arguments."""
import glob
import os
import re
import sys

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", ".."))
bad = 0
total = 0
for p in sorted(glob.glob(os.path.join(ROOT, "paper/sections/*.tex"))):
    t = open(p).read()
    groups = [m.group(1) for m in re.finditer(r"\\path\{([^{}]*)\}", t)]
    for m in re.finditer(r"\\srcs{0,3}((?:\{[^{}]*\})+)", t):
        groups += re.findall(r"\{([^{}]*)\}", m.group(1))
    for g in groups:
        for item in re.split(r";\s*", g):
            item = item.strip()
            if not item or item.startswith("http"):
                continue
            total += 1
            mm = re.match(r"^(.*?):(\d+)(?:-(\d+))?$", item)
            path, a, b = (mm.group(1), int(mm.group(2)), int(mm.group(3) or mm.group(2))) if mm else (item, None, None)
            full = os.path.join(ROOT, path)
            if not os.path.exists(full):
                print("MISSING", os.path.basename(p), path)
                bad += 1
                continue
            if a is not None and os.path.isfile(full):
                n = len(open(full, errors="replace").read().split("\n"))
                if b > n or a > b:
                    print("RANGE", os.path.basename(p), item, "file has", n)
                    bad += 1
print(total, "citations,", bad, "problems")
sys.exit(1 if bad else 0)
