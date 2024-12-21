from typing import NamedTuple, Optional

dirs = {(1, 0), (0, 1), (0, -1), (-1, 0)}


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


class Node(NamedTuple):
    point: Point
    dirhist: tuple[Optional[tuple[int, int]], Optional[tuple[int, int]], Optional[tuple[int, int]]]

    @property
    def row(self) -> int:
        return self.point.row

    @property
    def col(self) -> int:
        return self.point.col

    def __rshift__(self, other: tuple[int, int]):
        newpoint = Point(self.point.row + other[0], self.point.col + other[1])
        return Node(newpoint, (*self.dirhist[1:], other))

    def __add__(self, other: tuple[int, int]):
        if other not in dirs:
            raise ValueError(f"only value in {dirs} allowed, you provided {other}")
        return self >> other

    def __sub__(self, other: "Node") -> tuple[int, int]:
        return self.point - other.point

    def __gt__(self, other: "Node") -> bool:
        """self>other Determine of self leads to other node"""
        delta = other - self
        if delta not in dirs:
            return False
        elif all(d == self.dirhist[0] for d in self.dirhist):
            if self.dirhist[0] == delta:
                return False
        elif (-delta[0], -delta[1]) == self.dirhist[-1]:
            return False
        return other.dirhist[2] == delta and other.dirhist[0:2] == self.dirhist[1:3]

    def __lt__(self, other: "Node") -> bool:
        return other > self

    def __getitem__(self, key):
        return self.point[key]


def generate_path(points, initdirhist=((1, 0), (0, -1), (1, 0))):
    res = [Node(Point(*points[0]), initdirhist)]
    for n in range(1, len(points)):
        delta = Point(*points[n]) - Point(*points[n - 1])
        node = res[n - 1] + delta
        res.append(node)
    return res
