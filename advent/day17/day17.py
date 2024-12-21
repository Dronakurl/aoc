import argparse
import time
# from functools import cache, cmp_to_key
from pathlib import Path

import numpy as np

# from typing import Optional


args = argparse.ArgumentParser()
args.add_argument("filename", type=str, help="input file")
args.add_argument("--debug", action="store_true", help="debug mode")
args.add_argument("--step", action="store_true", help="step mode")
args.add_argument("--sleep", action="store_true", help="sleep")
parsed_args = args.parse_args()


class Point(tuple[int, int]):
    def __new__(cls, row, col):
        return super(Point, cls).__new__(cls, (row, col))

    @property
    def col(self) -> int:
        return self[1]

    @property
    def row(self) -> int:
        return self[0]

    def __add__(self, other):
        if type(other) == Point:
            return (self.row + other.row, self.col + other.col)
        elif type(other) == tuple:
            return Point(self.row + other[0], self.col + other[1])
        else:
            raise TypeError

    def __sub__(self, other):
        if isinstance(other, Point):
            return (self.row - other.row, self.col - other.col)
        elif isinstance(other, tuple):
            return Point(self.row - other[0], self.col - other[1])
        else:
            raise TypeError("Operands must be a Point or a tuple")

    def outside(self, shape) -> bool:
        return self.col < 0 or self.row < 0 or self.row >= shape[0] or self.col >= shape[1]


def printmat(mat, hist=[], heading="", clear=False):
    if clear:
        for _ in range((mat.shape[0] + 3)):
            print("\033[F", end="")
            print("\033[2K", end="\r")
    print("$$$$$$$$", heading)
    print(
        "$$$$$$ ",
        " calc = " + str(np.array([mat[x] for x in hist]).sum()),
        " len = " + str(len(hist)),
        " histlen = " + str(len(hists)),
        "                    ",
    )
    for row in range(mat.shape[0]):
        for col in range(mat.shape[1]):
            if len(hist) > 0 and (row, col) == hist[-1]:
                print("👠 ", end="")
            elif (row, col) in hist:
                i = hist.index((row, col))
                c = str(i)
                if len(hist) > 1:
                    diff = hist[i] - hist[i - 1]
                    if diff == (0, 1):
                        # c = "▶️"
                        c = ">"
                    elif diff == (0, -1):
                        # c = "◀️"
                        c = "<"
                    elif diff == (1, 0):
                        # c = "🔽"
                        c = "V"
                    elif diff == (-1, 0):
                        # c = "🔼"
                        c = "^"
                print(f"\033[31m{c:^3}\033[0m", end="")
            else:
                print(f"{mat[(row, col)]:^3}", end="")
        print("\n", end="")
    print("-------------------------")


# @cache
def get_loss(cur: Point, curtotal: int, dirhist: tuple) -> int:
    if parsed_args.debug:
        print("\n")
    if parsed_args.step:
        input("walk")
    global maximum
    global hist
    res: int = mat[cur]
    submatrix = mat[cur.row + 1 :, cur.col + 1 :]
    if mat.shape[0] - cur.row == mat.shape[1] - cur.col:
        # submatrix = mat[mat.shape[0]-cur.row:, mat.shape[1]-cur.col:]
        theory = np.trace(submatrix) + np.trace(submatrix, offset=1) + curtotal
        if theory < maximum and len(submatrix) > 0:
            if parsed_args.debug:
                print("submatrix found", submatrix)
            theory = maximum
    if curtotal > maximum:
        if parsed_args.debug:
            print("worse than the rest", cur, curtotal)
        return curtotal
    if np.sort(submatrix.flatten())[0 : submatrix.shape[0] + submatrix.shape[1] + 1].sum() + curtotal > maximum:
        return curtotal
    if len(set(hist)) < len(hist):
        raise KeyError("found a loop", cur, res)
    if cur == (mat.shape[0] - 1, mat.shape[1] - 1):
        if parsed_args.debug:
            print("reached the end", cur, res)
        # printmat(mat, hist, heading=f"maximum {maximum}", clear=True)
        # time.sleep(0.2)
        # print(f"\033[2K maximum {maximum:40}", end="\r")
        maximum = min([maximum, curtotal])
        # maximum = res
        hists.append(dict(res=curtotal, hist=hist.copy()))
        return curtotal  # , Point(mat.shape[0] - 1, mat.shape[1] - 1)

    searchdir = dirs
    if len(dirhist) > 0:
        searchdir = searchdir - {(-dirhist[-1][0], -dirhist[-1][1])}
    if len(dirhist) == 3:
        myhist = []
        for n in range(min([3, len(hist) - 1])):
            myhist.insert(0, (hist[-1 - n][0] - hist[-1 - n - 1][0], hist[-1 - n][1] - hist[-1 - n - 1][1]))
        # if tuple(myhist) != dirhist:
        #     breakpoint()
        if all(d == dirhist[0] for d in dirhist):
            searchdir = dirs - {dirhist[0]}
    elif len(dirhist) > 3:
        raise KeyError(dirhist)
    if parsed_args.debug:
        print(f"{dirhist=}")
        print(f"{searchdir=}")
    searchdirlst = list(searchdir)

    # colorrow = cur.col > cur.row
    # def custom_compare(item1, item2):
    #     if (item1[0] < 0 or item1[1] < 0) and (item2[0] >= 0 and item2[1] >= 0):
    #         return 1
    #     elif (item2[0] < 0 or item2[1] < 0) and (item1[0] >= 0 and item1[1] >= 0):
    #         return -1
    #     else:
    #         if not colorrow:
    #             if item1[0] < item2[0]:
    #                 return -1
    #             elif item1[0] > item2[0]:
    #                 return 1
    #         else:
    #             if item1[1] < item2[1]:
    #                 return -1
    #             elif item1[1] > item2[1]:
    #                 return 1
    #     return 0

    # searchdirlst = sorted(searchdirlst, key=cmp_to_key(custom_compare))
    # print(cur, searchdirlst)
    searchdirlst = [x for x in searchdirlst if not (cur + x).outside(mat.shape)]
    searchdirlst = sorted(searchdirlst, key=lambda xx: int(mat[cur + xx]))
    # if cur.row == 0 and cur.col == 6:
    #     breakpoint()

    losses = []
    for newdir in searchdirlst:
        nextcur = cur + newdir
        if nextcur in hist:
            continue
        if len(hist) > 0:
            hist = hist[: hist.index(cur) + 1]
        else:
            hist = [cur]
        hist.append(nextcur)
        if len(dirhist) == 3:
            newdirhist = (*dirhist[1:], newdir)
            if all(d == (*dirhist, newdir)[0] for d in (*dirhist, newdir)):
                raise Exception
        else:
            newdirhist = (*dirhist, newdir)
        if nextcur == (mat.shape[0] - 1, mat.shape[1] - 1) or parsed_args.sleep or parsed_args.debug:
            printmat(
                mat,
                hist,
                heading=f"cur {cur} maximum {maximum:2} curtotal {curtotal+mat[nextcur]:3} dirhist {newdirhist}",
                clear=not parsed_args.debug,
            )

        # if res + mat[nextcur] != np.array([mat[x] for x in hist]).sum():
        #     breakpoint()
        if parsed_args.sleep:
            time.sleep(0.02)
        rec = get_loss(nextcur, curtotal + int(mat[nextcur]), newdirhist)
        if rec:
            losses.append(rec)
    if len(losses) > 0:
        min_loss = min([x for x in losses])
        # print(f"{min_loss=} {losses=}")
        # point = [x[1] for x in losses if x[0] == min_loss][0]
        return curtotal + min_loss
    else:
        return 9999999


lines = Path(parsed_args.filename).read_text().splitlines()
mat = np.array([[int(x) for x in line] for line in lines])

maximum = np.trace(mat) + np.trace(mat, offset=1)
dirs = {(1, 0), (0, 1), (0, -1), (-1, 0)}
hist = []
hists = []

get_loss(Point(0, 0), 0, dirhist=())
print(" ")
hist = []
for h in hists:
    if h["res"] == maximum:
        hist = h["hist"]
        break
print(mat, maximum, mat.shape)
printmat(mat, hist, heading=f"maximum {maximum} ", clear=False)
print(hist)
print("part 1 = ", maximum)
