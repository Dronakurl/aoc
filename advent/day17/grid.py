import sys
import time
from itertools import product
from pathlib import Path
from queue import PriorityQueue
from typing import Optional

import matplotlib.pyplot as plt
import networkx as nx
import numpy as np
from node import Node, Point, dirs

dcs = [dc for dc in product(dirs, repeat=3) if not (dc[1] == (-dc[0][0], -dc[0][1]) or dc[2] == (-dc[1][0], -dc[1][1]))]
dir_combs: set[tuple[tuple[int, int], tuple[int, int], tuple[int, int]]] = set(dcs)  # type: ignore


class Grid:
    def __init__(self, mat: np.ndarray, hist=None) -> None:
        self.mat = mat
        self.hist = hist
        self.G: nx.DiGraph = nx.DiGraph()

    def append(self, node):
        if self.hist is None:
            self.hist = []
        if type(node) == Node:
            if node.point.outside(self.mat):
                raise KeyError("Try to add node outside the matrix")
            self.hist.append(node)  # type: ignore
        elif type(node) == tuple:
            self.hist.append(Node(Point(row=node[0], col=node[1]), (None, None, None)))  # type: ignore
        else:
            raise TypeError("provide either Node or tuple", type(node))

    def to_networkx_graph_simple(self):
        self.G = nx.DiGraph()
        rows, cols = self.mat.shape
        for row in range(rows):
            print(f"to_networkx_graph: {row=}")
            for col in range(cols):
                p = Point(row=row, col=col)
                self.G.add_node(p)
                for d in dirs:
                    if not (p + d).outside(self.mat.shape):
                        self.G.add_node(p + d)
                        self.G.add_edge(p, p + d, weight=self.mat[p + d])
                        self.G.add_edge(p + d, p, weight=self.mat[p])

    def shortest_simple(self, minimum=0, maximum=0):
        w = 1_000_000_000
        path = None
        for p in nx.all_simple_paths(
            self.G,
            source=Point(0, 0),
            target=Point(self.mat.shape[0] - 1, self.mat.shape[1] - 1),
            cutoff=2 * (self.mat.shape[0] + self.mat.shape[1]),
        ):
            weight = nx.path_weight(self.G, p, weight="weight")
            if weight < w and self.check_path_viable(path=p, minimum=minimum, maximum=maximum):
                print(weight)
                w = weight
                path = p
        if path:
            self.hist = path
            # self.hist = [Node(p, ((0, 0), (0, 0), (0, 0))) for p in path]
        return path, w

    def check_path_viable(self, path=None, minimum=0, maximum=0):
        if path is None:
            path = self.hist
        if path is None:
            print("check_path_viable no path given")
            return
        differences = [path[i + 1] - path[i] for i in range(len(path) - 1)]

        min_repeats = float("inf")
        max_repeats = 0
        current_count = 1
        for i in range(1, len(differences)):
            if differences[i] == differences[i - 1]:
                current_count += 1
            else:
                min_repeats = min(min_repeats, current_count)
                max_repeats = max(max_repeats, current_count)
                current_count = 1
        return min_repeats >= minimum and max_repeats <= maximum

    def to_networkx_graph(self):
        self.G = nx.DiGraph()
        rows, cols = self.mat.shape
        for row in range(rows):
            print(f"to_networkx_graph: {row=}")
            for col in range(cols):
                p = Point(row=row, col=col)
                for dirc in dir_combs:
                    node = Node(p, dirhist=dirc)
                    self.G.add_node(node)
                    for d in dirs:
                        if not (p + d).outside(self.mat.shape):
                            # if not self.G.has_node(Node(p + d, dirhist=next(iter(dir_combs)))):
                            for dh in dir_combs:
                                other_node = Node(p + d, dirhist=dh)
                                self.G.add_node(other_node)
                                if node > other_node:
                                    self.G.add_edge(node, other_node, weight=self.mat[p + d])
                                if node < other_node:
                                    self.G.add_edge(other_node, node, weight=self.mat[p])

    def shortest_path(self, source, target):
        path = nx.shortest_path(
            self.G,
            source=source,
            target=target,
            weight="weight",
        )
        weight = None if path is None else nx.path_weight(self.G, path, "weight")
        return path, weight

    def shortest(self):
        w = None
        for dc in dir_combs:
            target: Node = Node(Point(self.mat.shape[0] - 1, self.mat.shape[1] - 1), dc)
            if len(self.G.in_edges(target)) > 0:  # type:ignore
                for dc2 in [((1, 0), (1, 0), (0, 1)), ((1, 0), (0, -1), (1, 0))]:
                    path = None
                    source = Node(Point(0, 0), dc2)
                    try:
                        path, weight = self.shortest_path(source, target)
                    except nx.NetworkXNoPath:
                        pass
                        # print("nopath", source, target)
                    if path:
                        weight = nx.path_weight(self.G, path, "weight")
                        # print("path with ", dc, weight)
                        # if path[1].point == (1, 0):
                        #     for p in path:
                        #         print(p.point, end="")
                        #     print("")
                        if not w or weight < w:
                            self.hist = path
                            w = weight
        return w

    def weight(self):
        if self.hist is not None:
            return nx.path_weight(self.G, self.hist, "weight")
        else:
            print("no path")
            return -999

    def is_path(self, path=None):
        return nx.is_path(self.G, self.hist if path is None else path)

    def print_path(self):
        if not self.hist:
            print("no path given")
            return
        for x in self.hist:
            print(x)

    def print_nodes(self):
        print(len(self.G.nodes), " nodes")
        if len(self.G.nodes) < 500:
            for x in self.G:
                print(x)
                print("   ", end="")
                for y in self.G.out_edges(x):
                    assert y[1] - x == y[1].dirhist[-1]
                    print(y[1].point, end=" ")
                print("inedges:", len(self.G.in_edges(x)), [v[0].dirhist for v in self.G.in_edges(x)])
                print("")

    def index_of_point(self, tup):
        if self.hist is None:
            return None
        res = [i for (i, v) in enumerate(self.hist) if v[0] == tup[0] and v[1] == tup[1]]
        if res:
            return res[0]
        else:
            return None

    @classmethod
    def from_file(cls, filename: str, hist: Optional[list] = None) -> "Grid":
        lines = Path(filename).read_text().splitlines()
        mat = np.array([[int(x) for x in line] for line in lines])
        return cls(mat=mat, hist=None)

    def __str__(self, heading="") -> str:
        # hist = None
        # if self.hist is not None:
        hist = self.hist
        # elif self.pointhist is not None:
        #     hist = self.pointhist
        res = ""
        res += "$$$$$$$$" + heading
        res += "\n"
        # calc = "" if hist is None else " calc = " + str(np.array([self.mat[x.point] for x in hist]).sum())
        # res += "$$$$$$ " + calc + ("" if hist is None else " len = " + str(len(hist)))
        res += "$$$$$$ " + ("" if hist is None else " len = " + str(len(hist)))
        res += "\n"
        for row in range(self.mat.shape[0]):
            for col in range(self.mat.shape[1]):
                i = self.index_of_point((row, col))
                if hist and len(hist) > 0 and (hist[-1][0], hist[-1][1]) == (row, col):
                    res += "🥿"
                elif hist and i is not None:
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
                    res += f"\033[31m{c:^3}\033[0m"
                else:
                    res += f"{self.mat[row, col]:^3}"
            res += "\n"
        res += "-------------------------"
        return res

    def print(self, clear=True):
        if clear:
            for _ in range(len(self.__str__().splitlines())):
                sys.stdout.write("\033[F")
                sys.stdout.write("\033[K")
        print(self)

    def make_room_to_print(self):
        for _ in range(len(self.__str__().splitlines())):
            print("")

    def get_maximum(self):
        raise NotImplementedError("this only works for quadratic matrices")
        return np.trace(self.mat) + np.trace(self.mat, offset=1) - self.mat[0, 0]

    def draw(self):
        A = nx.nx_agraph.to_agraph(self.G)
        A.layout()
        A.draw("file.png")
        nx.draw(self.G)
        plt.show()

    def shortest_simple2(self, minval=1, maxval=3):
        q = PriorityQueue()
        grid = self.mat
        max_y, max_x = (v - 1 for v in grid.shape)
        goal = max_y, max_x
        q.put((0, (0, 0, 0)))
        q.put((0, (0, 0, 1)))
        seen = set()
        cost = 0
        pred = np.full(self.mat.shape, fill_value=np.array((0, 0), dtype=[("f0", "i"), ("f1", "i")]))
        costs = np.full(self.mat.shape, 10000000)
        costs[0, 0] = 0

        # Queue of next parts of the graph to be viewed
        while q:
            # Direction = Direction after the turn on this location
            cost, (row, col, direction) = q.get()
            if (row, col) == goal:
                break
            if (row, col, direction) in seen:
                continue
            seen.add((row, col, direction))
            original_cost = cost
            for sign in [-1, 1]:
                cost = original_cost
                new_row, new_col = row, col
                for i in range(1, maxval + 1):
                    if direction == 1:
                        new_col = col + i * sign
                    else:
                        new_row = row + i * sign
                    if new_col < 0 or new_row < 0 or new_col > max_x or new_row > max_y:
                        # break no continue, because we are already on the edge and do not
                        # need to search further
                        break
                    cost += grid[new_row, new_col]
                    # Don't check in the other direction, since it is only needed for turning
                    # if ((new_row, new_col, 1 - direction)) in seen:
                    #     continue
                    if i >= minval:
                        if (new_row, new_col) == (max_y, max_x):
                            print(row, col, cost)
                        if cost < costs[new_row, new_col]:
                            pred[new_row, new_col] = (row, col)
                            costs[new_row, new_col] = cost
                        q.put((cost, (new_row, new_col, 1 - direction)))

        # for row, col, direction in seen:
        #     try:
        #         if not (row, col) == (max_y, max_x):
        #             pred[row + (1 if direction == 0 else 0), col + (1 if direction == 1 else 0)] = (row, col)
        #     except IndexError:
        #         pass
        self.hist = []
        print(tuple(pred[0, 0]))
        cur = (max_y, max_x)
        self.hist.append(Point(*cur))
        # while cur != (0, 0):
        for _ in range(20):
            cur = tuple(pred[cur])
            self.hist.append(Point(*cur))
        for col in pred:
            for row in col:
                print(f"{row[0]:2}-{row[1]:2}", end="")
            print("")
        print(self.hist)
        self.hist = self.hist[::-1]
        return cost, costs


if __name__ == "__main__":
    g = Grid.from_file("simple.txt", hist=[0, 0])
    g.make_room_to_print()
    for i in range(10):
        g.print(clear=True)
        g.append((i % g.mat.shape[0], (i - i % g.mat.shape[0]) & g.mat.shape[1]))
        time.sleep(0.5)
