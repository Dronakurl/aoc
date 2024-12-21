import argparse

from grid import Grid

args = argparse.ArgumentParser()
args.add_argument("filename", type=str, help="input file")
args.add_argument("--debug", action="store_true", help="debug mode")
args.add_argument("--part1", action="store_true", help="part 1")
args.add_argument("--part2", action="store_true", help="part 2")
parsed_args = args.parse_args()

grid = Grid.from_file(parsed_args.filename)
grid.print(clear=False)

if parsed_args.part1:
    grid.to_networkx_graph()
    grid.shortest()
    grid.print(clear=False)
    print("part 1 = ", grid.weight())

if parsed_args.part2:
    w, costs = grid.shortest_simple2(minval=1, maxval=3)
    print("part 1 = ", w)
    grid.print(clear=False)
    # w, costs = grid.shortest_simple2(minval=4, maxval=10)
    # print("part 2 = ", w)
    print(costs)
