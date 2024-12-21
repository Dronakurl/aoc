import argparse
from pathlib import Path
from typing import List

import numpy as np


def loadfile(filename: str = "test.txt") -> List[np.ndarray]:
    """Load an input file into a list of numpy arrays"""
    fullstr = Path(filename).read_text()
    inputs = [
        np.array([int(x) for x in line.split()] + [np.NaN])
        for line in fullstr.splitlines()
    ]
    return inputs


def calc_diffs(arr: np.ndarray) -> np.ndarray:
    """Calculate the differences between numbers"""
    cur_diff = arr
    res = np.array(arr)
    for _ in range(len(arr) - 1):
        cur_diff = np.diff(cur_diff, n=1)
        cur_diff = np.append(cur_diff, [np.nan] * (len(arr) - len(cur_diff)))
        res = np.vstack((res, cur_diff))
    return res


def display_mat(matrix: np.ndarray, offset: bool = True) -> None:
    """Display the matrix in this nice format from the puzzle input

    Args:
        matrix: matrix
        offset: if true, include an offset, so that the next numbers are shown between the previous line
    """
    n = 0
    for row in matrix:
        if offset:
            print("   " * n, end="")
        for num in row:
            if not np.isnan(num):
                print("{:6}".format(num), end="")
        print("")
        n += 1


def fill_up(matrix: np.ndarray, n: int = 0):
    """Fill up the matrix recursively, extrapolating beginning with n, return the extrapolated value"""
    nth_line = matrix[n]
    last_notnan_index = next(
        (i for i in range(len(nth_line) - 1, -1, -1) if not np.isnan(nth_line[i])), None
    )
    if last_notnan_index is None:
        return 0
    if n + 1 < matrix.shape[0] - 1:
        next_line = fill_up(matrix, n + 1)
    else:
        next_line = 0
    res = matrix[n, last_notnan_index] + next_line
    matrix[n, last_notnan_index + 1] = res

    return res


def flip_time(matrix: np.ndarray) -> np.ndarray:
    """Flip the time direction in the matrix to extrapolate backwards in time"""
    matrix = np.flip(matrix, axis=1)
    for n in range(len(matrix)):
        matrix[n] = np.concatenate(
            [matrix[n][~np.isnan(matrix[n])], matrix[n][np.isnan(matrix[n])]]
        ) * (-1 if n % 2 != 0 else 1)
    matrix = np.concatenate((matrix, np.full((matrix.shape[0], 1), np.nan)), axis=1)
    return matrix


args = argparse.ArgumentParser()
args.add_argument("--debug", action="store_true", help="Enable debug mode")
parsed_args = args.parse_args()
inputs = loadfile("test.txt" if parsed_args.debug else "day9.txt")
total = [0, 0]
for inp in inputs:
    res = calc_diffs(inp)
    total[0] += fill_up(res)
    if parsed_args.debug:
        display_mat(res, offset=False)
    res = flip_time(res)
    total[1] += fill_up(res)
    if parsed_args.debug:
        display_mat(res, offset=False)


print("part 1 = ", total[0])
print("part 2 = ", total[1])
