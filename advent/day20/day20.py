# ruff: noqa:
import argparse
import math
import re
from enum import Enum
from pathlib import Path
from queue import Queue
from typing import NamedTuple

args = argparse.ArgumentParser()
args.add_argument("filename", type=str, help="input file")
args.add_argument("--debug", action="store_true", help="debug mode")
args.add_argument("--part1", action="store_true", help="part1")
args.add_argument("--part2", action="store_true", help="part2")
parsed_args = args.parse_args()


class HiLo(Enum):
    HIGH = -1
    LOW = 1
    NONE = 0

    def __invert__(self):
        return HiLo(-self.value)

    def __bool__(self) -> bool:
        return self.value != 0

    def __str__(self) -> str:
        if self == HiLo.HIGH:
            return "high"
        elif self == HiLo.LOW:
            return "low"
        else:
            return "none"

    def __repr_(self) -> str:
        return self.__str__()


class Pulse(NamedTuple):
    hilo: HiLo
    mod: "Module"
    source: str = ""

    def __call__(self):
        return self.mod.process_all(hilo=self.hilo, source=self.source)

    def __str__(self) -> str:
        return f"f{self.mod.name} ({str(self.hilo)}) -> " + ", ".join([f"{t.name}" for t in self.mod.targets])

    def __repr__(self) -> str:
        return self.__str__()


class CntQueue(Queue):
    def __init__(self):
        super().__init__()
        self.nlow = 0
        self.nhig = 0

    def get(self):
        signal: Pulse = super().get()
        if signal.mod.name == "button":
            return signal
        self.nlow += signal.hilo == HiLo.LOW
        self.nhig += signal.hilo == HiLo.HIGH
        return signal

    def put(self, signal):
        if type(signal) != Pulse:
            raise TypeError("Can only process Pulse objects provided", type(signal))
        super().put(signal)

    def putlist(self, lst: list[Pulse]):
        for ll in lst:
            self.put(ll)


class Module:
    def __init__(self, name: str, targets: list["Module"] = []) -> None:
        self.name = name
        self.targets = targets

    def process(self, hilo: HiLo, source: str) -> HiLo:
        return HiLo.NONE

    def process_all(self, hilo: HiLo, source: str) -> list[Pulse]:
        res = []
        ret = self.process(hilo=hilo, source=source)
        if ret == HiLo.NONE:
            return []
        for target in self.targets:
            if parsed_args.debug:
                print(self.name + " -" + str(ret) + " -> " + target.name)
            res.append(Pulse(mod=target, hilo=ret, source=self.name))
        return res

    def __eq__(self, string: str) -> bool:
        return self.name == string

    def __str__(self):
        res = str(self.__class__.__name__) + "(" + self.name + " -> "
        res += ", ".join([t.name for t in self.targets]) + " )"
        return res


class FlipFlop(Module):
    """
    Flip-flop modules (prefix %) are either on or off; they are initially off.
    If a flip-flop module receives a high pulse, it is ignored and nothing
    happens. However, if a flip-flop module receives a low pulse, it flips
    between on and off. If it was off, it turns on and sends a high pulse. If
    it was on, it turns off and sends a low pulse.
    """

    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.on = False

    def process(self, hilo: HiLo = HiLo.NONE, source: str = ""):
        if hilo == HiLo.LOW:
            self.on = not self.on
            if self.on:
                return HiLo.HIGH
            else:
                return HiLo.LOW
        return HiLo.NONE


class Conjunction(Module):
    """
    Conjunction modules (prefix &) remember the type of the most recent pulse
    received from each of their connected input modules; they initially default
    to remembering a low pulse for each input. When a pulse is received, the
    conjunction module first updates its memory for that input. Then, if it
    remembers high pulses for all inputs, it sends a low pulse; otherwise, it
    sends a high pulse.
    """

    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.memory = dict()

    def process(self, hilo: HiLo = HiLo.NONE, source: str = ""):
        self.memory[source] = hilo
        if all([v == HiLo.HIGH for v in self.memory.values()]):
            return HiLo.LOW
        return HiLo.HIGH


class Broadcaster(Module):
    def process(self, hilo: HiLo, source: str) -> HiLo:
        return hilo


def save_state(mods: dict[str, Module]):
    state = []
    for mod in mods.values():
        if isinstance(mod, FlipFlop):
            state.append((mod.name, mod.on))
        elif isinstance(mod, Conjunction):
            state.append((mod.name, tuple(mod.memory.items())))
    return tuple(state)


def set_state(mods: dict, state: tuple = ()):
    for mod_name, mod_state in state:
        if mod_name in mods:
            if isinstance(mods[mod_name], FlipFlop):
                mods[mod_name].on = mod_state
            elif isinstance(mods[mod_name], Conjunction):
                mods[mod_name].memory = dict(mod_state)
        elif isinstance(mods[mod_name], FlipFlop) or isinstance(mods[mod_name], Conjunction):
            raise KeyError(f"modname {mod_name} is an instance with a memory and is not set as expected")


def compare_state(mods: dict, state1: tuple = (), state2: tuple = ()) -> bool:
    if len(state1) != len(state2):
        raise ValueError("You are comparing states of different machinery")
    falses = []
    state2_dict = dict(state2)
    for mod_name, mod_state in state1:
        state2_state = state2_dict[mod_name]
        if isinstance(mods[mod_name], Conjunction):
            if mod_state == state2_state:
                continue
            if any([v[1] != HiLo.LOW for v in mod_state]) or all([v[1] != HiLo.LOW for v in state2_state]):
                falses.append((mod_name, mod_state, state2_state))
        elif mod_state != state2_state:
            falses.append((mod_name, mod_state, state2_state))
    if parsed_args.debug:
        print("\n".join(falses))
    return len(falses) == 0


def init_mods():
    fullstr = Path(parsed_args.filename).read_text()
    modsraw = dict()
    for line in fullstr.splitlines():
        match = re.match(r"^([%&])*(\w+) -> (.+)$", line)
        if not match:
            raise ValueError("input not formatted right", line)
        type_symbol, name, targets_str = match.groups()
        targets = [target.strip() for target in targets_str.split(",")]
        modsraw[name] = {"type": type_symbol, "name": name, "targets": targets}
    mods: dict[str, Module] = dict()
    for bl in modsraw.values():
        if bl["type"] == "%":
            mods[bl["name"]] = FlipFlop(name=bl["name"], targets=[])
        elif bl["type"] == "&":
            mods[bl["name"]] = Conjunction(name=bl["name"], targets=[])
        if bl["name"] == "broadcaster":
            mods[bl["name"]] = Broadcaster(name=bl["name"], targets=[])

    extra = []
    for k, v in mods.items():
        for t in modsraw[k]["targets"]:
            if t not in mods:
                extra.append((k, t))
                continue
            if isinstance(mods[t], Conjunction):
                mods[t].memory[v.name] = HiLo.LOW  # type: ignore
            v.targets.append(mods[t])
    for ee in extra:
        mods[ee[1]] = Module(name=ee[1], targets=[])
        mods[ee[0]].targets.append(mods[ee[1]])

    if parsed_args.debug:
        print("The modules:")
        for v in mods.values():
            print(v)
        print("----------\n")

    return mods


def main():
    if parsed_args.part1:
        part1()

    if parsed_args.part2:
        part2()


def part1():
    mods = init_mods()

    inistat = save_state(mods)

    def countit(state):
        q = CntQueue()
        set_state(mods, state)
        q.put(Pulse(mod=Broadcaster(name="button", targets=[mods["broadcaster"]]), hilo=HiLo.LOW))
        while not q.empty():
            pulse = q.get()
            # print(pulse)
            q.putlist(pulse())

        if parsed_args.debug:
            print(f"{q.nhig=} {q.nlow=}")
        # print(save_state(mods))
        return save_state(mods), q.nhig, q.nlow

    different = True
    state = inistat
    cycle = (0, 0)
    n = 0
    while different and n < 1000:
        state, nhig, nlow = countit(state)
        cycle = (cycle[0] + nhig, cycle[1] + nlow)
        different = not compare_state(mods, inistat, state)
        print(n)
        n = n + 1
        if parsed_args.debug:
            print("")

    k = 1000 % n
    print("k=", k)
    total = (cycle[0] * (1000 - k) / n, cycle[1] * (1000 - k) / n)
    print("total=", total)
    for _ in range(k):
        _, nhig, nlow = countit(state)
        total = (total[0] + nhig, total[1] + nlow)
    print(total, n, cycle)
    print("part 1 = ", total[0] * total[1])


def part2():
    mods = init_mods()

    q = Queue()

    def tryit(target="rx"):
        q.put(Pulse(mod=Broadcaster(name="button", targets=[mods["broadcaster"]]), hilo=HiLo.LOW))
        while not q.empty():
            pulse = q.get()
            pl = pulse()
            for p in pl:
                if p.mod.name == target and p.hilo == HiLo.LOW:
                    return True
                q.put(p)

        return False

    ns = []
    print("Find the 4 output modules for rx manually!")
    for target in ["kk", "vt", "xr", "fv"]:
        mods = init_mods()
        foundit = False
        n = 0
        while not foundit:
            n += 1
            foundit = tryit(target)
            if foundit:
                print(save_state(dict(target=mods[target])))
        print(target, n)
        ns.append(n)

    print(ns)
    print("part 2 = ", math.lcm(*ns))


def test_basic():
    hl = HiLo.LOW
    assert hl == HiLo.LOW
    assert ~hl == HiLo.HIGH
    assert not HiLo.NONE

    q = CntQueue()
    q.put(Pulse(mod=Module("first"), hilo=HiLo.LOW))
    q.put(Pulse(mod=Module("second"), hilo=HiLo.HIGH))
    assert q.get().mod == "first"
    assert q.get().mod == "second"
    assert q.nlow == 1
    assert q.nhig == 1
    x = FlipFlop("second")
    x.on = True
    assert x.on
    q.put(Pulse(mod=x, hilo=HiLo.HIGH))
    x.on = False
    y = q.get()
    assert type(y.mod) == FlipFlop
    assert not y.mod.on


test_basic()
main()
