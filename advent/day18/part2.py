import argparse
from collections import namedtuple
from pathlib import Path
from typing import NamedTuple

args = argparse.ArgumentParser()
args.add_argument("filename", type=str, help="input file")
args.add_argument("--debug", action="store_true", help="debug mode")
args.add_argument("--part2", action="store_true", help="part2")

parsed_args = args.parse_args()


Instr = namedtuple("Instr", ["dir", "len"])


class Coord(NamedTuple):
    x: int
    y: int

    def __add__(self, ins: Instr):
        mo = dict(U=(0, -1), D=(0, 1), L=(-1, 0), R=(1, 0)).get(ins.dir, (0, 0))
        return Coord(self[0] + mo[0] * ins.len, self[1] + mo[1] * ins.len)

    def __rshift__(self, dir):
        return Coord(self[0] + dir[0], self[1] + dir[1])


instr = []
leny = 0
for row in Path(parsed_args.filename).read_text().splitlines():
    sp = row.split(" ")
    if not parsed_args.part2:
        instr.append(Instr(sp[0], int(sp[1])))
    else:
        instr.append(Instr({"0": "R", "1": "D", "2": "L", "3": "U"}.get(sp[2][-2], ""), int(sp[2][2:-2], 16)))
    leny += instr[-1].len
print("number of instructions = ", len(instr))

cur = Coord(0, 0)
dig = [cur]
for ins in instr:
    cur = cur + ins
    dig.append(cur)
print(dig)

total = leny / 2 + 1
print(leny)
for i in range(1, len(dig)):
    trig = (dig[i - 1].x * dig[i].y - dig[i - 1].y * dig[i].x) / 2
    print(i, trig)
    total += trig


print("")
print(total)
