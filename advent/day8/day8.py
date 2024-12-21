import argparse
import math
from pathlib import Path

import pandas as pd

args = argparse.ArgumentParser()
args.add_argument("--debug", action="store_true", help="Enable debug mode")
parsed_args = args.parse_args()


def loadfile(debugfile: str):
    fullstr = Path(debugfile if parsed_args.debug else "day8.txt").read_text()
    lines = fullstr.splitlines()
    instr = list(lines.pop(0))
    lines.pop(0)
    strings = pd.Series(lines)
    df: pd.DataFrame = (
        strings.str.extract(r"([\w]{3}) = \(([\w]{3}), ([\w]{3})\)")
        .rename(columns={0: "nodeid", 1: "L", 2: "R"})
        .set_index("nodeid")
    )
    df["ghoststart"] = df.index.to_series().str.match(r"\w{2}A")
    return df, instr


def find_step_count(start_node: str, stopcond=lambda x: x != "ZZZ"):
    curr_node = start_node
    n = 0
    while stopcond(curr_node):
        for i in instr:
            n += 1
            if parsed_args.debug:
                print(i, curr_node)
            curr_node = df.loc[curr_node, i]
    return n


df, instr = loadfile("test2.txt")
print("part 1:", find_step_count("AAA"))
df, instr = loadfile("ghost.txt")
allpaths = (
    df.loc[df.ghoststart]
    .index.to_series()
    .apply(find_step_count, stopcond=lambda x: x[2] != "Z")
)
print("part 2:", math.lcm(*allpaths))
