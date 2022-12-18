use std::{
    cell::RefCell,
    collections::{BTreeSet, HashMap},
};

type Params = (String, BTreeSet<String>, i32);

thread_local! {
    static CACHE: RefCell<HashMap<i32, HashMap<Params, i32>>> = RefCell::new(HashMap::new());
}

fn main() {
    let input = include_str!("../../inputs/day16.txt");

    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str) -> i32 {
    let valves = parse_input(input);
    get_total_pressure_cached(
        get_total_pressure,
        1,
        &valves,
        String::from("AA"),
        &BTreeSet::new(),
        30,
    )
}

fn part_2(input: &str) -> i32 {
    let valves = parse_input(input);
    get_total_pressure_cached(
        get_total_pressure_2,
        2,
        &valves,
        String::from("AA"),
        &BTreeSet::new(),
        26,
    )
}

fn parse_input(input: &str) -> HashMap<String, Valve> {
    let valves = input.lines().fold(HashMap::new(), |mut map, line| {
        let parts: Vec<&str> = line.split(" ").collect();
        map.insert(
            parts[1].to_string(),
            Valve {
                flow: parts[4][5..]
                    .strip_suffix(";")
                    .unwrap()
                    .parse::<i32>()
                    .unwrap(),
                connections: parts[9..]
                    .iter()
                    .map(|s| s.trim_end_matches(",").to_string())
                    .collect(),
            },
        );
        map
    });
    valves
}

fn get_total_pressure_cached(
    function: fn(&HashMap<String, Valve>, String, &BTreeSet<String>, i32) -> i32,
    cache_id: i32,
    valves: &HashMap<String, Valve>,
    current_valve: String,
    open_valves: &BTreeSet<String>,
    time_remaining: i32,
) -> i32 {
    CACHE.with(|cache| {
        let key = (current_valve.clone(), open_valves.clone(), time_remaining);
        if let Some(r) = cache
            .borrow_mut()
            .entry(cache_id)
            .or_default()
            .get(&key)
            .cloned()
        {
            return r;
        }
        let pressure = function(valves, current_valve, open_valves, time_remaining);

        let mut cache = cache.borrow_mut();
        cache.get_mut(&cache_id).unwrap().insert(key, pressure);
        pressure
    })
}

fn get_total_pressure(
    valves: &HashMap<String, Valve>,
    current_valve: String,
    open_valves: &BTreeSet<String>,
    time_remaining: i32,
) -> i32 {
    let valve = valves.get(&current_valve).unwrap();
    if time_remaining <= 0 || open_valves.len() == valves.len() {
        return 0;
    }
    let mut pressure = valve
        .connections
        .iter()
        .map(|connection| {
            get_total_pressure_cached(
                get_total_pressure,
                1,
                valves,
                connection.clone(),
                open_valves,
                time_remaining - 1,
            )
        })
        .max()
        .unwrap();

    if !open_valves.contains(&current_valve) && valve.flow > 0 {
        let mut open_valves = open_valves.clone();
        open_valves.insert(current_valve);
        let flow = valve.flow * (time_remaining - 1);

        pressure = pressure.max(
            valve
                .connections
                .iter()
                .map(|connection| {
                    flow + get_total_pressure_cached(
                        get_total_pressure,
                        1,
                        valves,
                        connection.clone(),
                        &open_valves,
                        time_remaining - 2,
                    )
                })
                .max()
                .unwrap(),
        );
    }

    pressure
}

fn get_total_pressure_2(
    valves: &HashMap<String, Valve>,
    current_valve: String,
    open_valves: &BTreeSet<String>,
    time_remaining: i32,
) -> i32 {
    let valve = valves.get(&current_valve).unwrap();
    if time_remaining <= 0 || open_valves.len() == valves.len() {
        return get_total_pressure_cached(
            get_total_pressure,
            1,
            valves,
            String::from("AA"),
            open_valves,
            26,
        );
    }
    let mut pressure = valve
        .connections
        .iter()
        .map(|connection| {
            get_total_pressure_cached(
                get_total_pressure_2,
                2,
                valves,
                connection.clone(),
                open_valves,
                time_remaining - 1,
            )
        })
        .max()
        .unwrap();

    if !open_valves.contains(&current_valve) && valve.flow > 0 {
        let mut open_valves = open_valves.clone();
        open_valves.insert(current_valve);
        let flow = valve.flow * (time_remaining - 1);

        pressure = pressure.max(
            valve
                .connections
                .iter()
                .map(|connection| {
                    flow + get_total_pressure_cached(
                        get_total_pressure_2,
                        2,
                        valves,
                        connection.clone(),
                        &open_valves,
                        time_remaining - 2,
                    )
                })
                .max()
                .unwrap(),
        );
    }

    pressure
}

struct Valve {
    flow: i32,
    connections: BTreeSet<String>,
}
