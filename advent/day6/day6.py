from pathlib import Path
from typing import List, Tuple

import numpy as np


def scan_input(oneortwo=1) -> List[Tuple[int, int]]:
    fst = Path("day6.txt").read_text()
    if oneortwo == 2:
        fst = fst.replace(" ", "")
    tims = list(map(int, fst.splitlines()[0].split(":")[1].split()))
    dist = list(map(int, fst.splitlines()[1].split(":")[1].split()))
    return list(zip(tims, dist))


racs = scan_input()
print(racs)

res = 1
for r in racs:
    tim = r[0]
    dis = r[1]
    buthold = np.arange(0, tim)
    dist = buthold * (tim - buthold)
    res *= ((dist - dis) > 0).sum()
print(f"part 1 = {res}")

racs = scan_input(2)
res = 1
print(racs)
r = racs[0]
tim = r[0]
dis = r[1]
buthold = np.arange(0, tim)
dist = buthold * (tim - buthold)
res *= ((dist - dis) > 0).sum()
print(f"part 2 = {res}")
