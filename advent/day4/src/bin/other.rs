use std::time::Instant;

struct Card {
    matches: usize,
}

impl Card {
    fn new(data: &str) -> Card {
        let (want, have) = data.split_once(": ").unwrap().1.split_once(" | ").unwrap();

        let want: Vec<usize> = want
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        let have: Vec<usize> = have
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        let matches = want.iter().filter(|n| have.contains(n)).count();

        Card { matches }
    }
}

fn part_one(cards: &[Card]) -> usize {
    cards
        .iter()
        .map(|c| {
            if c.matches == 0 {
                return 0;
            }

            usize::pow(2, (c.matches - 1) as u32)
        })
        .sum()
}

fn part_two(cards: &[Card]) -> usize {
    // initialize counts vec to the size of cards, with values of `1`
    let mut counts = vec![1; cards.len()];
    let len = counts.len() - 1;

    for (i, card) in cards.iter().enumerate() {
        let j = if card.matches > len {
            len
        } else {
            card.matches + i
        };

        for k in i + 1..j + 1 {
            counts[k] += counts[i];
        }
    }

    counts.iter().sum()
}

fn get_cards(data: &str) -> Vec<Card> {
    data.lines().map(Card::new).collect()
}

fn main() {
    let two = true;
    let one = true;
    let start = Instant::now();
    let contents = include_str!("/home/konrad/quick/advent/day4/day4.txt");

    let cards = get_cards(contents);

    if one {
        let now = Instant::now();
        let ans = part_one(&cards);
        println!("Part one: {:?} {:?}", ans, now.elapsed());
    }

    if two {
        let now = Instant::now();
        let ans = part_two(&cards);
        println!("Part two: {:?} {:?}", ans, now.elapsed());
    }

    println!("Time: {:?}", start.elapsed())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("./example.txt");

    #[test]
    fn test_cards() {
        let cards = get_cards(EXAMPLE);

        assert_eq!(cards[0].matches, 4);
    }

    #[test]
    fn test_part_one() {
        let cards = get_cards(EXAMPLE);

        let ans = part_one(&cards);

        assert_eq!(ans, 13);
    }

    #[test]
    fn test_part_two() {
        let cards = get_cards(EXAMPLE);

        let ans = part_two(&cards);

        assert_eq!(ans, 30);
    }
}
