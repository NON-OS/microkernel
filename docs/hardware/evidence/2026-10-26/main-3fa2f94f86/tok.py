import re,sys,collections
TAGS=("[EXIT]","[DMA]","[CAP-DENY]","[IPC-DENY]")
def toks(p):
    c=collections.Counter()
    for line in open(p,errors="replace"):
        for t in TAGS:
            i=line.find(t)
            if i<0: continue
            s=line[i:].strip()
            s=re.sub(r"\b[0-9a-f]{1,4}:[0-9a-f]{1,2}(:[0-9a-f]{2})?\.[0-7]\b","<bdf>",s)
            s=re.sub(r"0x[0-9a-fA-F]+|\b\d+\b","#",s)
            c[s[:110]]+=1
    return c
a,b=toks(sys.argv[1]),toks(sys.argv[2])
for t in TAGS: print(t, sum(v for k,v in a.items() if k.startswith(t)), "->", sum(v for k,v in b.items() if k.startswith(t)))
for k in sorted(set(a)|set(b)):
    if a[k]!=b[k]: print(" DIFF", a[k], b[k], k)
