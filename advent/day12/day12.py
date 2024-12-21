import argparse
from functools import cache, partial
from multiprocessing import Pool
from pathlib import Path

args = argparse.ArgumentParser()
args.add_argument("--filename", type=str, help="input file")
parsed_args = args.parse_args()


@cache
def noo(row: str, blocks: tuple):
    bl = [x for x in row.split(".") if x]

    if len(blocks) == 0 and all(c == "?" for c in row):
        return 1
    elif len(blocks) == 0 and "#" in row:
        return 0
    elif "?" not in row:
        return int(blocks == tuple(map(len, bl)))
    elif "?" not in bl[-1]:
        if blocks[-1] == len(bl[-1]):
            return noo(".".join(bl[:-1]), blocks[:-1])
        else:
            return 0
    elif "?" not in bl[0]:
        if blocks[0] == len(bl[0]):
            return noo(".".join(bl[1:]), blocks[1:])
        else:
            return 0
    else:
        return noo(row.replace("?", ".", 1), blocks) + noo(row.replace("?", "#", 1), blocks)
    return 0


def process_line(line, multiplier):
    parts = line.split(" ")
    blocks = [int(block) for block in parts[1].split(",")] * multiplier
    row = "?".join([parts[0]] * multiplier)
    res = noo(row, tuple(blocks))
    print(f"line {parts[0]:20} {str(parts[1]):20} {res=}")
    return res


def countit(multiplier=1):
    total = 0
    lines = Path(parsed_args.filename).read_text().splitlines()
    fixed_multiplier_process_line = partial(process_line, multiplier=multiplier)
    with Pool() as pool:
        results = pool.map(fixed_multiplier_process_line, lines)
    for res in results:
        total += res
    return total


print("part 1 =", countit(1))
print("part 2 =", countit(5))
