from grid import Grid
from node import Node, Point


def test_node():
    node1 = Node(Point(2, 3), ((-1, 0), (0, 1), (1, 0)))
    node2 = Node(Point(0, 3), ((-1, 0), (-1, 0), (-1, 0)))
    assert node1 != node2
    assert node1 + (0, 1) == Node(Point(2, 4), ((0, 1), (1, 0), (0, 1)))
    assert not node1 > node2
    assert node1 > node1 + (0, 1)
    assert node1 + (0, 1) < node1
    assert node1 + (0, 1) + (0, 1) < node1 >> (0, 1)
    assert not node1 > node1 >> (-1, 1)
    assert not node2 > node2 + (-1, 0)
    assert node2 > node2 + (1, 0)


def test_point():
    x = Point(2, 34)
    assert x == (2, 34)


def test_grid():
    g = Grid.from_file("simple.txt", hist=[0, 0])
    g.append((1, 2))
    g.append((3, 2))
    print(g.get_maximum())
    assert g.index_of_point((1, 2)) == 0


if __name__ == "__main__":
    test_node()
    test_point()
