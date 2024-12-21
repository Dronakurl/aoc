import io
import re
from dataclasses import dataclass
from typing import List


@dataclass
class Draw:
    red: int = 0
    green: int = 0
    blue: int = 0

    @classmethod
    def parse(cls, datastring: str = "2233 green, 4 blue, 1 red"):
        cmd = {}
        for tpl in re.findall(r"(\d*) (green|blue|red)", datastring):
            cmd[tpl[1]] = int(tpl[0])
        return cls(**cmd)

    def power(self):
        return self.red * self.green * self.blue


@dataclass
class Game:
    """Representing a whole game"""

    game_id: int
    draws: List[Draw]

    @classmethod
    def parse(cls, datastring: str = "Game 11: 3 blue, 4 red; 1 red, 2 green, 6 blue"):
        """Parse a game

        Args:
            datastring: string Representing the

        Raises:
            ValueError: In case something goes

        Returns: Game object

        """
        m = re.match(r"^Game (\d*): (.*)$", datastring)
        if m is None:
            raise ValueError(f"{datastring=}")
        draws = []
        for draw in m.group(2).split(";"):
            draws.append(Draw.parse(draw))
        game = cls(game_id=int(m.group(1)), draws=draws)
        return game

    def is_possible(self, draw: Draw):
        for d in self.draws:
            if draw.red < d.red or draw.green < d.green or draw.blue < d.blue:
                return False
        return True

    def get_color_list(self, rgb: str) -> List[str]:
        """Get a list of number of draws for a color

        Args:
            rgb: color name

        Returns: List with numbers containing the draws

        """
        res = []
        for d in self.draws:
            res.append(getattr(d, rgb))
        return res

    def getmax(self):
        colors = ["red", "green", "blue"]
        maxdraw = Draw()
        for c in colors:
            setattr(maxdraw, c, max(self.get_color_list(c)))
        return maxdraw


with io.open("day2.txt", "r") as f:
    lines = f.readlines()

# round 1
res = 0
for lin in lines:
    game = Game.parse(lin)
    if game.is_possible(Draw(red=12, green=13, blue=14)):
        res += game.game_id
print(res)
# round 2
res = 0
for lin in lines:
    game = Game.parse(lin)
    res += game.getmax().power()
print(res)
