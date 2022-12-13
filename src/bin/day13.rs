use std::cmp::Ordering;

fn main() {
    let input = include_str!("../../inputs/day13.txt");

    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str) -> usize {
    let parsed_lines = parse_input(input);
    let mut pair_indices = Vec::new();
    for (index, lines) in parsed_lines.chunks(2).enumerate() {
        if lines[0] < lines[1] {
            pair_indices.push(index + 1);
        }
    }
    pair_indices.into_iter().sum()
}

fn part_2(input: &str) -> usize {
    let parsed_lines = parse_input(input);
    let mut sorted = parsed_lines
        .into_iter()
        .map(|l| Element::List(l))
        .collect::<Vec<_>>();
    // Add marker elements
    let markers = [2, 6].map(|n| Element::List(vec![Element::Number(n)]));
    markers.iter().for_each(|m| sorted.push(m.clone()));
    sorted.sort();
    // Find marker elements and multiply their indices
    markers
        .into_iter()
        .map(|m| sorted.iter().position(|e| e == &m).unwrap() + 1)
        .product()
}

fn parse_input(input: &str) -> Vec<Vec<Element>> {
    let lines: Vec<&str> = input.lines().filter(|l| !l.is_empty()).collect();

    let mut parsed_lines: Vec<Vec<Element>> = Vec::new();

    for line in lines {
        let mut vec_stack: Vec<_> = Vec::new();

        let replaced_line = line.replace("[", " [ ").replace("]", " ] ");
        let parts: Vec<&str> = replaced_line
            .split(&[' ', ','][..])
            .filter(|p| !p.is_empty())
            .collect();

        for p in parts {
            match p {
                "[" => vec_stack.push(Vec::new()),
                "]" => {
                    let vec = vec_stack.pop().unwrap();
                    match vec_stack.last_mut() {
                        Some(v) => v.push(Element::List(vec)),
                        None => parsed_lines.push(vec),
                    }
                }
                _ => vec_stack
                    .last_mut()
                    .unwrap()
                    .push(Element::Number(p.parse().unwrap())),
            }
        }
    }
    parsed_lines
}

#[derive(Debug, Clone, PartialEq, Eq, Ord)]
enum Element {
    List(Vec<Element>),
    Number(i32),
}

impl PartialOrd for Element {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            // Compare two numbers
            (Element::Number(a), Element::Number(b)) => a.partial_cmp(b),
            // Compare two lists
            (Element::List(a), Element::List(b)) => {
                for (a, b) in a.iter().zip(b.iter()) {
                    match a.partial_cmp(b) {
                        Some(Ordering::Equal) => continue,
                        other_order => return other_order,
                    }
                }
                a.len().partial_cmp(&b.len())
            }
            // Compare list with number (number is converted to list with one element)
            (n @ Element::Number(_), l @ Element::List(_)) => {
                Element::List(vec![n.clone()]).partial_cmp(l)
            }
            (l @ Element::List(_), n @ Element::Number(_)) => {
                l.partial_cmp(&Element::List(vec![n.clone()]))
            }
        }
    }
}
