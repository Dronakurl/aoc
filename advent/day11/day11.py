import argparse

import numpy as np

args = argparse.ArgumentParser()
args.add_argument("--filename", type=str, help="input file")
parsed_args = args.parse_args()

with open(parsed_args.filename, "r") as f:
    lines = f.readlines()

mapping = str.maketrans({".": "0", "#": "1"})
arr = np.array([list(line.strip().translate(mapping)) for line in lines]).astype(int)

row_indices = np.where(~arr.any(axis=1))[0]
col_indices = np.where(~arr.any(axis=0))[0]

if parsed_args.filename.startswith("test"):
    print(arr)
galaxies = np.argwhere(arr != 0)
galaxy_cnt = len(galaxies)
x = galaxies[:, 0].reshape(-1, 1)
y = galaxies[:, 1].reshape(-1, 1)
distances = np.triu(np.abs(x - x.T) + np.abs(y - y.T))
row_indices_matrix = np.triu(
    np.vectorize(lambda i, j: np.sum((row_indices > min(i, j)) & (row_indices < max(i, j))))(
        x.flatten()[:, None], x.flatten()
    )
)
col_indices_matrix = np.triu(
    np.vectorize(lambda i, j: np.sum((col_indices > min(i, j)) & (col_indices < max(i, j))))(
        y.flatten()[:, None], y.flatten()
    )
)
print("part 1 = ", np.sum(distances + row_indices_matrix + col_indices_matrix))
print("part 2 = ", np.sum(distances + 999_999 * (row_indices_matrix + col_indices_matrix)))

if parsed_args.filename.startswith("test"):
    print(f"Rows with all zeros: {row_indices}")
    print(f"Columns with all zeros: {col_indices}")
    print(galaxies)
    print(row_indices_matrix)
    print(col_indices_matrix)
    print(distances)
