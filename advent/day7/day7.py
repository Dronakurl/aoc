import argparse
from collections import Counter

import pandas as pd

args = argparse.ArgumentParser()
args.add_argument("--debug", action="store_true", help="Enable debug mode")
args.add_argument("--part", type=str, help="Specify a part", default="all")

parsed_args = args.parse_args()

if parsed_args.debug:
    fn = "test.txt"
else:
    fn = "day7.txt"


def get_rank(cards: str, part2: bool = False):
    if len(cards) != 5:
        raise ValueError(f"Invalid cards string {cards}")
    card_counts = Counter(cards)
    cnt = sorted(list(card_counts.values()), reverse=True)
    joker_cnt = card_counts["J"]
    if cnt[0] == 5:
        # five of a kind
        return 1
    elif cnt[0] == 4:
        # four of a kind
        if part2 and joker_cnt in (1, 4):
            return 1
        elif part2 and joker_cnt > 1 and joker_cnt < 4:
            raise ValueError(f"{cards=}")
        return 2
    elif cnt[0] == 3 and cnt[-1] == 2:
        # fullhouse
        if part2 and joker_cnt >= 2:
            return 1
        elif part2 and joker_cnt > 3:
            raise ValueError(f"{cards=}")
        elif part2 and joker_cnt == 1:
            raise ValueError(f"{cards=}")
        return 3
    elif cnt[0] == 3:
        # three of a kind
        if part2 and joker_cnt == 1:
            # one joker, four of a kind
            return 2
        elif part2 and joker_cnt == 2:
            # that would mean a full house
            raise ValueError(f"{cards=}")
        elif part2 and joker_cnt == 3:
            return 2
        return 4
    elif cnt[0] == 2 and cnt[1] == 2:
        # two pairs
        if part2 and joker_cnt == 2:
            # one of the pairs are jokers> 4 of a kind
            return 2
        elif part2 and joker_cnt == 1:
            return 3
        elif part2 and joker_cnt > 2:
            raise ValueError(f"{cards=}")
        return 5
    elif cnt[0] == 2:
        # one pair
        if part2 and joker_cnt == 2:
            # the pair is a joker, 3 of a kind
            return 4
        elif part2 and joker_cnt == 1:
            return 4
        elif part2 and joker_cnt > 2:
            raise ValueError(f"{cards=}")
        return 6
    elif cnt[0] == 1:
        # high card
        if part2 and joker_cnt == 1:
            return 6
        elif part2 and joker_cnt > 1:
            raise ValueError(f"{cards=}")
        return 7


def doit(part2: bool = False):
    df = pd.read_csv(fn, sep=" ", names=["cards", "value"])
    df["rank"] = df.cards.apply(get_rank, part2=part2)
    cards = ["A", "K", "Q", "J", "T", "9", "8", "7", "6", "5", "4", "3", "2"]
    card_ranks = {cards[i]: i for i in range(len(cards))}
    if part2:
        card_ranks["J"] = 99
    df["card_ranks"] = df.cards.apply(lambda x: tuple([card_ranks[c] for c in x]))
    df = df.sort_values(["rank", "card_ranks"], axis=0, ascending=False)
    df = df.reset_index(drop=True).reset_index(names="total_rank")
    df.total_rank += 1
    df["new_value"] = df.total_rank * df.value
    df["joker_cnt"] = df.cards.apply(lambda x: Counter(x)["J"])
    print(df.groupby(["joker_cnt", "rank"]).count())
    print(df[df["cards"].str.contains("J")].sample(5))
    if parsed_args.debug:
        print(df.to_string())
    else:
        print(df)
    print(sum(df.new_value))


if parsed_args.part in ["part1", "all"]:
    doit(part2=False)
if parsed_args.part in ["part2", "all"]:
    doit(part2=True)
