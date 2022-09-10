from typing import NamedTuple


class Point(NamedTuple):
    x: int
    y: int

    def __mul__(self, weight):
        return Point(self.x * weight, self.y * weight)

    def __add__(self, other):
        return Point(self.x + other.x, self.y + other.y)


def Bezier2(t, w):
    t2 = t * t
    mt = 1 - t
    mt2 = mt * mt
    return w[0] * mt2 + w[1] * 2 * mt * t + w[2] * t2


def Bezier3(t, w):
    t2 = t * t
    t3 = t2 * t
    mt = 1 - t
    mt2 = mt * mt
    mt3 = mt2 * mt
    return w[0] * mt3 + w[1] * mt2 * t * 3 + w[2] * mt * t2 * 3 + w[3] * t3


quad = (Point(110, 150), Point(25, 190), Point(210, 250))
cubic = (Point(110, 150), Point(25, 190), Point(210, 250), Point(210, 30))


def main():
    for i in range(0, 10):
        t = i / 10.0
        print("quad: {}".format(Bezier2(t, quad)))
    for i in range(0, 10):
        t = i / 10.0
        print("cubic: {}".format(Bezier3(t, cubic)))


if __name__ == "__main__":
    main()
