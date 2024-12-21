import argparse
from pathlib import Path

import numpy as np

args = argparse.ArgumentParser()
args.add_argument("--filename", type=str, help="input file")
parsed_args = args.parse_args()


def printmat(mat, heading=""):
    for _ in range((mat.shape[0] + 3)):
        print("\033[F", end="")
        print("\033[2K", end="\r")
    map = {-1: "#", 0: ".", 1: "O"}
    print("$$$$$$$$$$$$$$$$$$$$$$$")
    print("$$$$$$ ", heading, "   ", weight(mat))
    for row in mat:
        for col in row:
            print(map[col], end="")
        print("\n", end="")
    print("#-------------------------")


def rollblocks(arr, order=-1):
    edg = np.where(np.diff(np.hstack(([False], (arr == 0) | (arr == 1), [False]))))[0]
    for i in range(0, len(edg), 2):
        arr[edg[i] : edg[i + 1]] = np.sort(arr[edg[i] : edg[i + 1]])[::order]


def weight(mat):
    xx = (np.arange(mat.shape[0]) + 1)[::-1]
    cmat = np.multiply(mat.copy(), xx[:, np.newaxis])
    cmat[cmat < 0] = 0
    return cmat.sum()


def spin(mat):
    # north
    for column in range(mat.shape[1]):
        rollblocks(mat[:, column], order=-1)

    # west
    for row in range(mat.shape[0]):
        rollblocks(mat[row, :], order=-1)

    # south
    for column in range(mat.shape[1]):
        rollblocks(mat[:, column], order=1)

    # east
    for row in range(mat.shape[0]):
        rollblocks(mat[row, :], order=1)


lines = Path(parsed_args.filename).read_text().splitlines()
map = {"#": -1, ".": 0, "O": 1}
mat = np.array([[map[x] for x in line] for line in lines])

cmat = mat.copy()
for column in range(cmat.shape[1]):
    rollblocks(cmat[:, column], order=-1)
print("part 1 =", weight(cmat))

printmat(mat, "origin")
res = []
res.append(mat.copy())
period = (0, 0, 0)
for i in range(1, 10000000):
    spin(mat)
    printmat(mat, str(i))
    period = (0, 0, 0)
    for n, origmat in enumerate(res, start=1):
        if np.all(mat == origmat):
            period = (i - n + 1, n, i)
            break
    if period[0] > 0:
        break
    res.append(mat.copy())
print(period)

residuum = (1_000_000_000 - period[2]) % period[0]
print(f"{residuum=}")
for i in range(residuum):
    spin(mat)
print("part 2=", weight(mat))
