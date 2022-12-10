fn main() {
    let input = include_str!("../../inputs/day10.txt");

    println!("Part 1: {}", part_1(input));
    part_2(input);
}

fn part_1(input: &str) -> i32 {
    const INTERESTING_CYCLES: [i32; 6] = [20, 60, 100, 140, 180, 220];
    let mut xregister = 1;
    let mut signal_sum = 0;

    let instructions: Vec<&str> = input.lines().collect();
    let mut ip: usize = 0;
    let mut waiting = false;

    for cycle in 1.. {
        if let Some(instruction) = instructions.get(ip) {
            if INTERESTING_CYCLES.contains(&cycle) {
                let signal = xregister * cycle;
                signal_sum += signal;
            }

            let parts: Vec<&str> = instruction.split_whitespace().collect();

            match parts[..] {
                ["addx", x] => {
                    if !waiting {
                        waiting = true;
                        continue;
                    }
                    waiting = false;
                    let x = x.parse::<i32>().unwrap();
                    xregister += x;
                }
                ["noop"] => {}
                _ => panic!("Unknown instruction: {}", instruction),
            }

            ip += 1;
        } else {
            break;
        }
    }
    signal_sum
}

fn part_2(input: &str) {
    let mut xregister = 1;
    let instructions: Vec<&str> = input.lines().collect();
    let mut ip: usize = 0;
    let mut waiting = false;

    for cycle in 1.. {
        if let Some(&instruction) = instructions.get(ip) {
            if (cycle % 40 as i32).abs_diff(xregister + 1) < 2 {
                print!("█");
            } else {
                print!("░");
            }

            if cycle % 40 == 0 {
                println!()
            }

            let parts: Vec<&str> = instruction.split_whitespace().collect();

            match parts[..] {
                ["addx", x] => {
                    if !waiting {
                        waiting = true;
                        continue;
                    }
                    waiting = false;
                    let x = x.parse::<i32>().unwrap();
                    xregister += x;
                }
                ["noop"] => {}
                _ => panic!("Unknown instruction: {}", instruction),
            }

            ip += 1;
        } else {
            break;
        }
    }
}
