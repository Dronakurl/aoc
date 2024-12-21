import argparse
import re
from pathlib import Path
from typing import NamedTuple

import networkx as nx

args = argparse.ArgumentParser()
args.add_argument("filename", type=str, help="input file")
args.add_argument("--debug", action="store_true", help="debug mode")
args.add_argument("--part1", action="store_true", help="part1")
args.add_argument("--part2", action="store_true", help="part2")
parsed_args = args.parse_args()

mapxmas = dict(x=0, m=1, a=2, s=3)
mapxmas_inv = {v: k for k, v in mapxmas.items()}
MAX = 99999


class Part(NamedTuple):
    x: int
    m: int
    a: int
    s: int

    @classmethod
    def from_string(cls, mystr):
        return cls(*map(int, re.findall(r"\d+", mystr)))

    def is_fitting(self, rule: "Rule") -> bool:
        return rule.is_fitting(self)


class Rule(NamedTuple):
    rulestr: str = ""
    lower: Part = Part(*([-MAX] * 4))
    upper: Part = Part(*([MAX] * 4))
    target: str = ""

    def __str__(self) -> str:
        res = f"Rule -> {self.target:3}: "
        res += ", ".join(
            [
                f"{self.lower[i]} < {mapxmas_inv[i]} < {self.upper[i]}"
                for i in range(4)
                if self.upper[i] != MAX or self.lower[i] != -MAX
            ]
        )
        return res

    @classmethod
    def from_string(cls, mystr: str):
        pattern = re.compile(r"([xmas]{1})([<>])(\d+):(\w+)")
        match = pattern.match(mystr)
        if not match or len(match.groups()) < 4:
            raise ValueError("String format is incorrect", mystr)
        bound = match.group(3)
        xmas = match.group(1)
        gtlt = match.group(2)
        lower = Part(*[int(bound) if i == mapxmas[xmas] and gtlt == ">" else -MAX for i in range(4)])
        upper = Part(*[int(bound) if i == mapxmas[xmas] and gtlt == "<" else MAX for i in range(4)])
        return cls(target=match.group(4), rulestr=mystr, lower=lower, upper=upper)

    def __add__(self, other: "Rule"):
        return Rule(
            rulestr=self.rulestr,
            lower=Part(*[max((self.lower[i], other.lower[i])) for i in range(4)]),
            upper=Part(*[min((self.upper[i], other.upper[i])) for i in range(4)]),
            target=self.target,
        )

    def __invert__(self):
        return Rule(
            rulestr=self.rulestr,
            lower=Part(*[self.upper[i] - 1 if self.upper[i] != MAX else -MAX for i in range(4)]),
            upper=Part(*[self.lower[i] + 1 if self.lower[i] != -MAX else MAX for i in range(4)]),
            target=self.target,
        )

    def possible(self):
        for i in range(4):
            if self.upper[i] - self.lower[i] <= 1:
                return False
        return True

    def is_fitting(self, part: Part) -> bool:
        for i in range(4):
            if self.upper[i] <= part[i] or self.lower[i] >= part[i]:
                return False
        return True

    def count_poss(self) -> int:
        total = 1
        for i in range(4):
            total *= max((min((self.upper[i], 4001)) - max((self.lower[i], 0)) - 1, 0))
        return total


class Workflow(NamedTuple):
    name: str
    rules: tuple[Rule, ...]
    defstr: str

    @classmethod
    def from_string(cls, mystr: str, cumulate=False):
        pattern = re.compile(r"(\w+)\{(.+),(\w+)}")
        match = pattern.match(mystr)
        if not match or len(match.groups()) < 3:
            raise ValueError("Workflow string format is incorrect", mystr)
        rules = []
        for ss in match.group(2).split(","):
            ner = Rule.from_string(ss)
            rules.append(ner)
        rules.append(Rule(target=match.group(3), rulestr="fallback"))
        newrules = rules.copy()
        if cumulate:
            for i in range(len(newrules)):
                for k in range(i):
                    newrules[i] = newrules[i] + ~rules[k]
        return cls(
            name=match.group(1),
            rules=tuple(newrules),
            defstr=mystr,
        )

    def __str__(self) -> str:
        res = f"Workflow {self.name} \n"
        res += "   " + self.defstr + "\n"
        for r in self.rules:
            res += f"    {r}\n"
        return res

    def process(self, part: Part) -> str:
        for r in self.rules:
            if r.is_fitting(part):
                return r.target
        print(self)
        raise ValueError("The fallback rule did not work")


def test_rule():
    scan = Rule.from_string("a<2006:qkq")
    cmp = Rule(
        rulestr="a<2006:qkq",
        lower=Part(*([-MAX] * 4)),
        upper=Part(MAX, MAX, 2006, MAX),
        target="qkq",
    )
    assert scan == cmp
    cmp = Rule(
        rulestr="a<2006:qkq",
        lower=Part(-MAX, -MAX, 2005, -MAX),
        upper=Part(*([MAX] * 4)),
        target="qkq",
    )
    assert ~scan == cmp
    cmp = Rule(
        rulestr="a<2006:qkq",
        lower=Part(MAX, MAX, MAX, MAX),
        upper=Part(*([-MAX] * 4)),
        target="qkq",
    )
    print(cmp)
    print(cmp.possible())
    assert cmp.count_poss() == 0, f"{cmp.count_poss()=} "
    cmp = Rule(
        rulestr="a<2006:qkq",
        lower=Part(-MAX, -MAX, -MAX, -MAX),
        upper=Part(*([MAX] * 4)),
        target="qkq",
    )
    assert cmp.count_poss() == 4000**4, f"{cmp.count_poss()=} "

    cmp = Rule(
        rulestr="a<2006:qkq",
        lower=Part(0, 0, 0, 0),
        upper=Part(1, 1, 1, 1),
        target="qkq",
    )
    assert cmp.count_poss() == 0, f"{cmp.count_poss()=} "

    cmp = Rule(
        rulestr="a<2006:qkq",
        lower=Part(0, 0, 0, 0),
        upper=Part(2, 2, 2, 2),
        target="qkq",
    )
    assert cmp.count_poss() == 1, f"{cmp.count_poss()=} "


def test_part():
    part = Part.from_string("{x=787,m=2655,a=1222,s=2876}")
    rule = Rule.from_string("a<2006:qkq")
    assert rule.is_fitting(part)
    assert part.is_fitting(rule)


def test_workflow():
    st = "mq{a>414:A,s<617:R,a>253:R,A}"
    ps600 = Part(a=400, s=600, x=1, m=1)
    ps1000 = Part(a=400, s=1600, x=1, m=1)
    wf = Workflow.from_string(st)
    print(wf)
    print(f"ps600 => {wf.process(ps600)}")
    print(f"ps1000 => {wf.process(ps1000)}")
    wf = Workflow.from_string(st, cumulate=True)
    print(wf)
    print(f"ps600 => {wf.process(ps600)}")
    print(f"ps1000 => {wf.process(ps1000)}")


def main():
    fullstr = Path(parsed_args.filename).read_text().split("\n\n")
    wfstr = fullstr[0]
    partstr = fullstr[1]
    parts = [Part.from_string(ps) for ps in partstr.splitlines()]

    workflows = {}
    for wfs in wfstr.splitlines():
        wf = Workflow.from_string(wfs, cumulate=True)
        workflows[wf.name] = wf

    if parsed_args.part1:
        part1(workflows, parts)

    if parsed_args.part2:
        part2(workflows, parts)


def part1(workflows, parts):
    total = 0
    for p in parts:
        print(p, end="")
        st = "in"
        while st not in ("R", "A"):
            print("  " + st + " -> ", end="")
            w = workflows[st]
            st = w.process(p)
        print(st, end="")
        print("")
        if st == "A":
            total += sum(p)
    print("part 1: ", total)  # 509597


def part2(workflows, parts):
    G = nx.DiGraph()
    for sou in workflows["in"].rules:
        G.add_edge(("in", Rule(rulestr="in", target="in")), ("in", sou))
    for wfid in workflows:
        for sou in workflows[wfid].rules:
            if sou.target not in "RA":
                for tar in workflows[sou.target].rules:
                    G.add_edge((wfid, sou), (sou.target, tar))
            else:
                G.add_edge((wfid, sou), (sou.target, Rule(rulestr="RA", target=sou.target)))
    G.add_edge(("R", Rule(rulestr="RA", target="R")), ("RA", Rule(rulestr="RA", target="RA")))
    G.add_edge(("A", Rule(rulestr="RA", target="A")), ("RA", Rule(rulestr="RA", target="RA")))
    print(f"{len(G)=} {len(G.edges)=}\n")

    print("################### find all paths")
    ntotal = 0
    arules = []
    for path in nx.all_simple_paths(
        G,
        ("in", Rule(rulestr="in", target="in")),
        ("RA", Rule(rulestr="RA", target="RA")),
    ):
        print(path[-2][0], end="  :")
        totalrule = Rule(rulestr=str(path)[0:20], target="total")
        for _, rule in path:
            totalrule = totalrule + rule
        if path[-2][0] == "A":
            arules.append(totalrule)
            ntotal += totalrule.count_poss()
        print(totalrule, totalrule.count_poss())
    print("part2 = ", ntotal)

    print("part1chck", sum([sum(part) for part in parts if any([arule.is_fitting(part) for arule in arules])]))


test_rule()
test_part()
test_workflow()
main()
