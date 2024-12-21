import argparse
from collections import namedtuple
from pathlib import Path
from queue import Queue
from typing import NamedTuple

from PIL import Image

args = argparse.ArgumentParser()
args.add_argument("filename", type=str, help="input file")
args.add_argument("--debug", action="store_true", help="debug mode")
parsed_args = args.parse_args()


Instr = namedtuple("Instr", ["dir", "len", "col"])


class Coord(NamedTuple):
    x: int
    y: int

    def __add__(self, ins: Instr):
        mo = dict(U=(0, -1), D=(0, 1), L=(-1, 0), R=(1, 0)).get(ins.dir, (0, 0))
        return Coord(self[0] + mo[0] * ins.len, self[1] + mo[1] * ins.len)

    def __rshift__(self, dir):
        return Coord(self[0] + dir[0], self[1] + dir[1])

    def paint(self, ins: Instr, canvas: list[list], path: list = [], dirs: list = [], col: str = "#"):
        mo = dict(U=(0, -1), D=(0, 1), L=(-1, 0), R=(1, 0)).get(ins.dir, (0, 0))
        for k in range(ins.len):
            canvas[self.y + k * mo[1]][self.x + k * mo[0]] = ins.col if col == "" else col
            path.append((self.x + k * mo[0], self.y + k * mo[1]))
            dirs.append(ins.dir if k != 0 else "U")
        return self + ins


def field_to_image(field):
    def hex_to_rgb(hex_color):
        color_mapping = {
            "#": (255, 255, 255),
            ".": (73, 0, 0),
        }
        if color_mapping.get(hex_color):
            return color_mapping.get(hex_color)
        hex_color = hex_color.lstrip("#")
        return tuple(int(hex_color[i : i + 2], 16) for i in (0, 2, 4))

    height = len(field)
    width = len(field[0]) if height > 0 else 0
    image = Image.new("RGB", (width, height))
    pixels = image.load()
    for y in range(height):
        for x in range(width):
            pixels[x, y] = hex_to_rgb(field[y][x])
    image.save("image.png")
    image.show()


instr = []
for row in Path(parsed_args.filename).read_text().splitlines():
    sp = row.split(" ")
    instr.append(Instr(sp[0], int(sp[1]), sp[2][1:-1]))
print("number of instructions = ", len(instr))

cur = Coord(0, 0)
dig = [cur]
for ins in instr:
    cur = cur + ins
    dig.append(cur)

minc = list(map(min, zip(*dig)))
maxc = map(max, zip(*dig))
shape = tuple(map(lambda x, y: x - y + 1, maxc, minc))

dig = list(map(lambda d: d >> [-x for x in minc], dig))
canvas = [["." for _ in range(shape[0])] for _ in range(shape[1])]
d = dig[0]
path = []
dirs = []
for ins in instr:
    d = d.paint(ins, canvas, path, dirs, col="")

x = 0
for x in range(1, shape[0]):
    if canvas[5][x] == "." and canvas[5][x - 1] != ".":
        break
seed = (x, 5)
print(seed)

q = Queue()
q.put(seed)
while not q.empty():
    (x, y) = q.get()
    if x < 0 or y < 0 or x >= shape[0] or y >= shape[1]:
        continue
    if canvas[y][x] == ".":
        canvas[y][x] = "#00FF00"
        q.put((x, y + 1))
        q.put((x, y - 1))
        q.put((x + 1, y))
        q.put((x - 1, y))


# for y in range(shape[1]):
#     inside = False
#     for x in range(shape[0]):
#         if canvas[y][x] != ".":
#             if dirs[path.index((x, y))] in ("U", "D"):
#                 inside = not inside
#         elif inside:
#             canvas[y][x] = "#FF0000"

cnt = 0
for y in range(shape[1]):
    for x in range(shape[0]):
        if canvas[y][x] != ".":
            cnt += 1

print("part 1 : ", cnt)


field_to_image(canvas)


# fn fill(x, y):
#     if not Inside(x, y) then return
#     let s = new empty stack or queue
#     Add (x, y) to s
#     while s is not empty:
#         Remove an (x, y) from s
#         let lx = x
#         while Inside(lx - 1, y):
#             Set(lx - 1, y)
#             lx = lx - 1
#         while Inside(x, y):
#             Set(x, y)
#             x = x + 1
#       scan(lx, x - 1, y + 1, s)
#       scan(lx, x - 1, y - 1, s)
#
# fn scan(lx, rx, y, s):
#     let span_added = false
#     for x in lx .. rx:
#         if not Inside(x, y):
#             span_added = false
#         else if not span_added:
#             Add (x, y) to s
#             span_added = true

# for row in field:
#     print("".join(row))


# instr["up"] = instr.apply(lambda row: row["leng"] if row["direction"] == "U" else 0, axis=1)
# instr[instr.direction == "U"].leng.sum() - instr[instr.direction == "U"].leng.sum()
# sums = instr.groupby("direction").leng.sum()
# max_v = sums.loc[["U", "D"]].max()
# max_h = sums.loc[["L", "R"]].max()
