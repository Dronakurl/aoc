import argparse
from pathlib import Path

import numpy as np

args = argparse.ArgumentParser()
args.add_argument("--filename", type=str, help="input file")
parsed_args = args.parse_args()


def checksym(xx: np.ndarray, smudgefix=False, debug=False) -> int:
    """
    0 xxxx
    1 xxxx > xx[:2*row] > :2
    2 xxxx > xx[:2*row] > :4
    3 xxxx > xx[ln - (ln-row)*2 - 1:] > 1:
    4 xxxx > xx[ln - (ln-row)*2 - 1:] > 3:

    2*row>=len
    """
    ln = xx.shape[0] - 1
    if debug:
        print(xx, ln)
    for row in range(1, ln + 1):
        if debug:
            print(f"{row = }")
        if 2 * row > ln:
            xxx = xx[ln - (ln - row) * 2 - 1 :]
        else:
            xxx = xx[: 2 * row]
        dif = xxx - np.flip(xxx, axis=0)
        if debug:
            print("xxx=")
            print(xxx)
            print("dif=")
            print(dif)
        if np.sum(dif == 1) == 1 and smudgefix:
            return row
        if not dif.any() and not smudgefix:
            return row
    return 0


def doit(smudgefix: bool):
    lines = Path(parsed_args.filename).read_text()
    fields = lines.split("\n\n")
    total = 0

    for field in fields:
        xx = np.array([[1 if char == "#" else 0 for char in line] for line in field.splitlines()])
        print(f"{checksym(xx, smudgefix)=} {checksym(xx.T, smudgefix)=}")

        total += 100 * checksym(xx, smudgefix)
        total += checksym(xx.T, smudgefix)

    return total


print("part 1=", doit(False))
print("part 2=", doit(True))
