fn main() {
    let input = include_str!("../../inputs/day11.txt");

    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str) -> i32 {
    let (mut monkeys, _) = parse_input(input);

    let mut results = vec![0; monkeys.len()];
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let monkey = &monkeys[i];
            let monkey_results = monkey.inspect_all();
            results[i] += monkey_results.len() as i32;
            for (item, m) in monkey_results.into_iter() {
                monkeys[m].add_item(item);
            }
            monkeys[i].clear_items();
        }
    }

    results.sort_unstable();
    results[results.len() - 2..].iter().product()
}

fn part_2(input: &str) -> u64 {
    let (mut monkeys, modulus) = parse_input(input);

    let mut results = vec![0u64; monkeys.len()];
    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            let monkey = &monkeys[i];
            let monkey_results = monkey.inspect_all_part_2(modulus);
            results[i] += monkey_results.len() as u64;
            for (item, m) in monkey_results.into_iter() {
                monkeys[m].add_item(item);
            }
            monkeys[i].clear_items();
        }
    }

    results.sort_unstable();
    results[results.len() - 2..].iter().product()
}

fn parse_input(input: &str) -> (Vec<Monkey>, u64) {
    let mut monkeys: Vec<Monkey> = Vec::new();
    let lines: Vec<&str> = input.lines().collect();
    let monkey_description: Vec<&[&str]> = lines.split(|&l| l.is_empty()).collect();
    let mut divisors = Vec::new();
    for m in monkey_description {
        let items = m[1][18..]
            .split(", ")
            .map(|i| i.parse::<u64>().unwrap())
            .collect();
        let operation = match m[2][23..]
            .split_once(char::is_whitespace)
            .map(|(op, n)| (op, n.parse::<u64>()))
            .unwrap()
        {
            ("+", Ok(n)) => Operation::Add(n),
            ("*", Ok(n)) => Operation::Multiply(n),
            ("*", _) => Operation::Square,
            _ => panic!("Unknown operation!"),
        };
        let divisor = m[3][21..].parse::<u64>().unwrap();
        divisors.push(divisor);
        let success_monkey = m[4][29..].parse::<usize>().unwrap();
        let failure_monkey = m[5][30..].parse::<usize>().unwrap();

        monkeys.push(Monkey {
            items,
            operation,
            divisor,
            success_monkey,
            failure_monkey,
        });
    }
    let modulus = divisors.iter().product();
    (monkeys, modulus)
}

#[derive(Debug)]
enum Operation {
    Multiply(u64),
    Add(u64),
    Square,
}

impl Operation {
    fn apply(&self, item: u64) -> u64 {
        match self {
            Operation::Multiply(n) => item * n,
            Operation::Add(n) => item + n,
            Operation::Square => item * item,
        }
    }
}

#[derive(Debug)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    divisor: u64,
    success_monkey: usize,
    failure_monkey: usize,
}

impl Monkey {
    fn add_item(&mut self, item: u64) {
        self.items.push(item);
    }

    fn clear_items(&mut self) {
        self.items.clear();
    }

    fn inspect_all(&self) -> Vec<(u64, usize)> {
        let mut results = Vec::with_capacity(self.items.len());
        for item in &self.items {
            let item = self.operation.apply(*item) / 3;
            results.push((item.clone(), self.test_item(item)));
        }
        results
    }

    fn inspect_all_part_2(&self, modulus: u64) -> Vec<(u64, usize)> {
        let mut results = Vec::with_capacity(self.items.len());
        for item in &self.items {
            let item = self.operation.apply(*item);
            results.push((item % modulus, self.test_item(item)));
        }
        results
    }

    fn test_item(&self, item: u64) -> usize {
        if item % self.divisor == 0 {
            self.success_monkey
        } else {
            self.failure_monkey
        }
    }
}
