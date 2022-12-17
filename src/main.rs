use std::{
    cmp::Ordering,
    collections::{hash_map::RandomState, BinaryHeap, HashMap, HashSet, VecDeque},
    time::Instant,
};

use rayon::prelude::*;

fn main() {
    fn print_measured<Func>(name: &str, exercise: Func)
    where
        Func: Fn() -> String,
    {
        let start = Instant::now();
        let str = exercise();
        let duration = start.elapsed();
        println!("{}: {} [{:.2?}]", name, str, duration);
    }

    // print_measured("1s", do_1s);
    // print_measured("1g", do_1g);

    // print_measured("2s", do_2s);
    // print_measured("2g", do_2g);

    // print_measured("3s", do_3s);
    // print_measured("3g", do_3g);

    // print_measured("4s", do_4s);
    // print_measured("4g", do_4g);

    // print_measured("5s", do_5s);
    // print_measured("5g", do_5g);

    // print_measured("6s", do_6s);
    // print_measured("6g", do_6g);

    // print_measured("7s", do_7s);
    // print_measured("7g", do_7g);

    // print_measured("8s", do_8s);
    // print_measured("8g", do_8g);

    // print_measured("9s", do_9s);
    // print_measured("9g", do_9g);

    // print_measured("10s", do_10s);
    // print_measured("10g", do_10g);

    // print_measured("11s", do_11s);
    // print_measured("11g", do_11g);

    // print_measured("12s", do_12s);
    // print_measured("12g", do_12g);

    // print_measured("13s", do_13s);
    // print_measured("13g", do_13g);

    // print_measured("14s", do_14s);
    // print_measured("14g", do_14g);

    // print_measured("15s", do_15s);
    // print_measured("15g", do_15g);

    // print_measured("16s", do_16s);
    // print_measured("16g", do_16g);

    print_measured("17s", do_17s);
    print_measured("17g", do_17g);

    print_measured("18s", do_18s);
    print_measured("18g", do_18g);

    print_measured("19s", do_19s);
    print_measured("19g", do_19g);

    print_measured("20s", do_20s);
    print_measured("20g", do_20g);

    print_measured("21s", do_21s);
    print_measured("21g", do_21g);

    print_measured("22s", do_22s);
    print_measured("22g", do_22g);

    print_measured("23s", do_23s);
    print_measured("23g", do_23g);

    print_measured("24s", do_24s);
    print_measured("24g", do_24g);
}

fn do_6s() -> String {
    let mut lines = read_lines("./res/6");

    let line = lines.next().unwrap();

    for (n, window) in line.as_str().as_bytes().windows(4).enumerate() {
        let mut iter = window.iter();
        let a = iter.next().unwrap();
        let b = iter.next().unwrap();
        let c = iter.next().unwrap();
        let d = iter.next().unwrap();

        if a != b && a != c && a != d && b != c && b != d && c != d {
            return (n + 4).to_string();
        }
    }

    unreachable!()
}

fn do_6g() -> String {
    let mut lines = read_lines("./res/6");

    let line = lines.next().unwrap();

    for (n, window) in line.as_str().as_bytes().windows(14).enumerate() {
        let vec = window.to_vec();
        let set = HashSet::<u8, RandomState>::from_iter(vec);

        if set.len() == 14 {
            return (n + 14).to_string();
        }
    }

    unreachable!()
}

fn calculate_total_size(
    id: usize,
    child_id_vec: &Vec<HashMap<String, usize>>,
    own_size_vec: &Vec<i32>,
    total_size_vec: &mut Vec<Option<i32>>,
) {
    total_size_vec[id] = Some(own_size_vec[id]);

    for i in child_id_vec[id].values() {
        calculate_total_size(*i, child_id_vec, own_size_vec, total_size_vec);
        total_size_vec[id] = Some(total_size_vec[id].unwrap() + total_size_vec[*i].unwrap());
    }
}

fn do_7s() -> String {
    let lines = read_lines("./res/7");

    let mut parent_id_vec = vec![usize::MAX];
    let mut child_id_vec = vec![HashMap::<String, usize>::new()];
    let mut own_size_vec = vec![0];

    let mut current_id = 0;

    for line in lines {
        if line == "$ cd /" {
            current_id = 0;
        } else if line == "$ ls" {
        } else if line == "$ cd .." {
            current_id = parent_id_vec[current_id];
        } else if let Some(child) = line.strip_prefix("$ cd ") {
            current_id = child_id_vec[current_id][child];
        } else {
            let (a, b) = line.split_once(' ').unwrap();
            assert_eq!(
                child_id_vec[current_id].insert(b.to_owned(), parent_id_vec.len()),
                None
            );
            parent_id_vec.push(current_id);
            child_id_vec.push(HashMap::new());
            if a == "dir" {
                own_size_vec.push(0);
            } else {
                own_size_vec.push(a.parse().unwrap());
            }
        }
    }

    let mut total_size_vec = vec![None; parent_id_vec.len()];

    calculate_total_size(0, &child_id_vec, &own_size_vec, &mut total_size_vec);

    total_size_vec
        .iter()
        .map(|val| val.unwrap())
        .zip(own_size_vec)
        .filter(|(total, own)| *own == 0 && *total <= 100000)
        .map(|(total, _)| total)
        .sum::<i32>()
        .to_string()
}

fn do_7g() -> String {
    let lines = read_lines("./res/7");

    let mut parent_id_vec = vec![usize::MAX];
    let mut child_id_vec = vec![HashMap::<String, usize>::new()];
    let mut own_size_vec = vec![0];

    let mut current_id = 0;

    for line in lines {
        if line == "$ cd /" {
            current_id = 0;
        } else if line == "$ ls" {
        } else if line == "$ cd .." {
            current_id = parent_id_vec[current_id];
        } else if let Some(child) = line.strip_prefix("$ cd ") {
            current_id = child_id_vec[current_id][child];
        } else {
            let (a, b) = line.split_once(' ').unwrap();
            assert_eq!(
                child_id_vec[current_id].insert(b.to_owned(), parent_id_vec.len()),
                None
            );
            parent_id_vec.push(current_id);
            child_id_vec.push(HashMap::new());
            if a == "dir" {
                own_size_vec.push(0);
            } else {
                own_size_vec.push(a.parse().unwrap());
            }
        }
    }

    let mut total_size_vec = vec![None; parent_id_vec.len()];

    calculate_total_size(0, &child_id_vec, &own_size_vec, &mut total_size_vec);

    let threshold = -40000000 + total_size_vec[0].unwrap();

    let mut best_size = total_size_vec[0].unwrap();

    for size in total_size_vec.iter().skip(1) {
        if threshold <= size.unwrap() && size.unwrap() <= best_size {
            best_size = size.unwrap();
        }
    }

    best_size.to_string()
}

fn do_8s() -> String {
    let lines = read_lines("./res/8");

    let heights: Vec<Vec<u32>> = lines
        .map(|line| line.chars().map(|ch| ch.to_digit(10).unwrap()).collect())
        .collect();

    let m = heights.len();
    let n = heights.first().unwrap().len();

    let mut number_visible = 0;

    for a in 0..m {
        for b in 0..n {
            let mut success = true;
            for a_n in 0..a {
                if heights[a][b] <= heights[a_n][b] {
                    success = false
                }
            }
            if success {
                number_visible += 1;
                continue;
            }

            let mut success = true;
            for a_s in (a + 1)..m {
                if heights[a][b] <= heights[a_s][b] {
                    success = false
                }
            }
            if success {
                number_visible += 1;
                continue;
            }

            let mut success = true;
            for b_w in 0..b {
                if heights[a][b] <= heights[a][b_w] {
                    success = false
                }
            }
            if success {
                number_visible += 1;
                continue;
            }

            let mut success = true;
            for b_e in (b + 1)..n {
                if heights[a][b] <= heights[a][b_e] {
                    success = false
                }
            }
            if success {
                number_visible += 1;
                continue;
            }
        }
    }

    number_visible.to_string()
}

fn do_8g() -> String {
    let lines = read_lines("./res/8");

    let heights: Vec<Vec<u32>> = lines
        .map(|line| line.chars().map(|ch| ch.to_digit(10).unwrap()).collect())
        .collect();

    let m = heights.len();
    let n = heights.first().unwrap().len();

    let mut best_score = -1;

    for a in 0..m {
        for b in 0..n {
            let mut score_n = 0;
            for a_n in (0..a).rev() {
                score_n += 1;
                if heights[a][b] <= heights[a_n][b] {
                    break;
                }
            }

            let mut score_s = 0;
            for a_s in (a + 1)..m {
                score_s += 1;
                if heights[a][b] <= heights[a_s][b] {
                    break;
                }
            }

            let mut score_w = 0;
            for b_w in (0..b).rev() {
                score_w += 1;
                if heights[a][b] <= heights[a][b_w] {
                    break;
                }
            }

            let mut score_e = 0;
            for b_e in (b + 1)..n {
                score_e += 1;
                if heights[a][b] <= heights[a][b_e] {
                    break;
                }
            }

            best_score = best_score.max(score_n * score_s * score_w * score_e);
        }
    }

    best_score.to_string()
}

fn do_9s() -> String {
    let lines = read_lines("./res/9");

    let mut hx = 0;
    let mut hy = 0;
    let mut tx = 0;
    let mut ty = 0;

    let mut places = HashSet::new();
    places.insert((0, 0));

    for line in lines {
        let (dir, count_str) = line.split_once(' ').unwrap();
        let count: i32 = count_str.parse().unwrap();
        for _ in 0..count {
            match dir {
                "L" => hx -= 1,
                "R" => hx += 1,
                "D" => hy -= 1,
                "U" => hy += 1,
                _ => unreachable!(),
            }

            trace_knot(hx, hy, &mut tx, &mut ty);

            places.insert((tx, ty));
        }
    }

    places.len().to_string()
}

fn trace_knot(hx: i32, hy: i32, tx: &mut i32, ty: &mut i32) {
    if hx.abs_diff(*tx) <= 1 && hy.abs_diff(*ty) <= 1 {
        return;
    }

    if hx < *tx {
        *tx -= 1;
    }

    if *tx < hx {
        *tx += 1;
    }

    if hy < *ty {
        *ty -= 1;
    }

    if *ty < hy {
        *ty += 1;
    }
}

fn do_9g() -> String {
    let lines = read_lines("./res/9");

    let mut xs = [0; 10];
    let mut ys = [0; 10];

    let mut places = HashSet::new();
    places.insert((0, 0));

    for line in lines {
        let (dir, count_str) = line.split_once(' ').unwrap();
        let count: i32 = count_str.parse().unwrap();
        for _ in 0..count {
            match dir {
                "L" => xs[0] -= 1,
                "R" => xs[0] += 1,
                "D" => ys[0] -= 1,
                "U" => ys[0] += 1,
                _ => unreachable!(),
            }

            for i in 0..9 {
                trace_knot(xs[i], ys[i], &mut xs[i + 1], &mut ys[i + 1]);
            }

            places.insert((xs[9], ys[9]));
        }
    }

    places.len().to_string()
}

fn do_10s() -> String {
    let mut lines = read_lines("./res/10");

    let mut x = 1;
    let mut cycle = 1;
    let mut signal_strength_sum = 0;

    let mut instruction_cyles_remaining = 0;
    let mut instruction_effect_on_x = 0;

    loop {
        if instruction_cyles_remaining == 0 {
            x += instruction_effect_on_x;

            match lines.next() {
                Some(instruction) => {
                    if instruction == "noop" {
                        instruction_cyles_remaining = 1;
                        instruction_effect_on_x = 0;
                    } else {
                        instruction_cyles_remaining = 2;
                        instruction_effect_on_x =
                            instruction.split_once(' ').unwrap().1.parse().unwrap()
                    }
                }
                None => return signal_strength_sum.to_string(),
            }
        }

        if (cycle - 20) % 40 == 0 {
            signal_strength_sum += x * cycle
        }

        cycle += 1;
        instruction_cyles_remaining -= 1;
    }
}

fn do_10g() -> String {
    let mut lines = read_lines("./res/10");

    let mut x: i32 = 1;
    let mut cycle = 1;
    let mut crt = '\n'.to_string();

    let mut instruction_cyles_remaining = 0;
    let mut instruction_effect_on_x = 0;

    loop {
        if instruction_cyles_remaining == 0 {
            x += instruction_effect_on_x;

            match lines.next() {
                Some(instruction) => {
                    if instruction == "noop" {
                        instruction_cyles_remaining = 1;
                        instruction_effect_on_x = 0;
                    } else {
                        instruction_cyles_remaining = 2;
                        instruction_effect_on_x =
                            instruction.split_once(' ').unwrap().1.parse().unwrap()
                    }
                }
                None => return crt,
            }
        }

        crt.push(if x.abs_diff((cycle - 1) % 40) <= 1 {
            '#'
        } else {
            '.'
        });

        if cycle % 40 == 0 {
            crt.push('\n');
        }

        cycle += 1;
        instruction_cyles_remaining -= 1;
    }
}

fn do_11s() -> String {
    // let lines = read_lines("./res/11");

    let mut items = [
        VecDeque::from([54, 98, 50, 94, 69, 62, 53, 85]),
        VecDeque::from([71, 55, 82]),
        VecDeque::from([77, 73, 86, 72, 87]),
        VecDeque::from([97, 91]),
        VecDeque::from([78, 97, 51, 85, 66, 63, 62]),
        VecDeque::from([88]),
        VecDeque::from([87, 57, 63, 86, 87, 53]),
        VecDeque::from([73, 59, 82, 65]),
    ];

    fn apply_op(old: i64, monkey_id: usize) -> i64 {
        match monkey_id {
            0 => old * 13,
            1 => old + 2,
            2 => old + 8,
            3 => old + 1,
            4 => old * 17,
            5 => old + 3,
            6 => old * old,
            7 => old + 6,
            _ => unreachable!(),
        }
    }

    let test_div = [3, 13, 19, 17, 5, 7, 11, 2];
    let trues = [2, 7, 4, 6, 6, 1, 5, 4];
    let falses = [1, 2, 7, 5, 3, 0, 0, 3];

    let mut inspecitions = [0; 8];

    for _ in 0..20 {
        for monkey_id in 0..8 {
            while !items[monkey_id].is_empty() {
                inspecitions[monkey_id] += 1;

                let mut item = items[monkey_id].pop_front().unwrap();
                item = apply_op(item, monkey_id);
                item /= 3;
                items[if item % test_div[monkey_id] == 0 {
                    trues[monkey_id]
                } else {
                    falses[monkey_id]
                }]
                .push_back(item);
            }
        }
    }

    inspecitions.sort();

    (inspecitions[6] * inspecitions[7]).to_string()
}

fn do_11g() -> String {
    // let lines = read_lines("./res/11");

    let mut items = [
        VecDeque::from([54, 98, 50, 94, 69, 62, 53, 85]),
        VecDeque::from([71, 55, 82]),
        VecDeque::from([77, 73, 86, 72, 87]),
        VecDeque::from([97, 91]),
        VecDeque::from([78, 97, 51, 85, 66, 63, 62]),
        VecDeque::from([88]),
        VecDeque::from([87, 57, 63, 86, 87, 53]),
        VecDeque::from([73, 59, 82, 65]),
    ];

    fn apply_op(old: i64, monkey_id: usize) -> i64 {
        match monkey_id {
            0 => old * 13,
            1 => old + 2,
            2 => old + 8,
            3 => old + 1,
            4 => old * 17,
            5 => old + 3,
            6 => old * old,
            7 => old + 6,
            _ => unreachable!(),
        }
    }

    let test_div = [3, 13, 19, 17, 5, 7, 11, 2];
    let trues = [2, 7, 4, 6, 6, 1, 5, 4];
    let falses = [1, 2, 7, 5, 3, 0, 0, 3];

    let mut inspecitions: [i64; 8] = [0; 8];

    for _ in 0..10000 {
        for monkey_id in 0..8 {
            while !items[monkey_id].is_empty() {
                inspecitions[monkey_id] += 1;

                let mut item = items[monkey_id].pop_front().unwrap();
                item = apply_op(item, monkey_id);
                item %= 9699690;
                items[if item % test_div[monkey_id] == 0 {
                    trues[monkey_id]
                } else {
                    falses[monkey_id]
                }]
                .push_back(item);
            }
        }
    }

    inspecitions.sort();

    (inspecitions[6] * inspecitions[7]).to_string()
}

#[derive(PartialEq, Eq)]
struct Event {
    dist: i32,
    a: usize,
    b: usize,
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .dist
            .cmp(&self.dist)
            .then_with(|| self.a.cmp(&other.a))
            .then_with(|| self.b.cmp(&other.b))
    }
}

fn do_12s() -> String {
    let lines: Vec<String> = read_lines("./res/12").collect();

    let heights: Vec<Vec<i32>> = lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|ch| {
                    char_value(match ch {
                        'S' => 'a',
                        'E' => 'z',
                        ch => ch,
                    })
                })
                .collect()
        })
        .collect();

    let m = heights.len();
    let n = heights.first().unwrap().len();

    let start_a = lines.iter().position(|line| line.contains('S')).unwrap();
    let start_b = lines[start_a].chars().position(|ch| ch == 'S').unwrap();

    let end_a = lines.iter().position(|line| line.contains('E')).unwrap();
    let end_b = lines[end_a].chars().position(|ch| ch == 'E').unwrap();

    let mut events = BinaryHeap::new();

    events.push(Event {
        dist: 0,
        a: start_a,
        b: start_b,
    });

    let mut visited = HashSet::<(usize, usize)>::new();

    loop {
        let event = events.pop().unwrap();

        if event.a == end_a && event.b == end_b {
            return event.dist.to_string();
        }

        if !visited.insert((event.a, event.b)) {
            continue;
        };

        if 0 < event.a && heights[event.a - 1][event.b] <= heights[event.a][event.b] + 1 {
            events.push(Event {
                dist: event.dist + 1,
                a: event.a - 1,
                b: event.b,
            });
        }

        if event.a < m - 1 && heights[event.a + 1][event.b] <= heights[event.a][event.b] + 1 {
            events.push(Event {
                dist: event.dist + 1,
                a: event.a + 1,
                b: event.b,
            });
        }

        if 0 < event.b && heights[event.a][event.b - 1] <= heights[event.a][event.b] + 1 {
            events.push(Event {
                dist: event.dist + 1,
                a: event.a,
                b: event.b - 1,
            });
        }

        if event.b < n - 1 && heights[event.a][event.b + 1] <= heights[event.a][event.b] + 1 {
            events.push(Event {
                dist: event.dist + 1,
                a: event.a,
                b: event.b + 1,
            });
        }
    }
}

fn do_12g() -> String {
    let lines: Vec<String> = read_lines("./res/12").collect();

    let heights: Vec<Vec<i32>> = lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|ch| {
                    char_value(match ch {
                        'S' => 'a',
                        'E' => 'z',
                        ch => ch,
                    })
                })
                .collect()
        })
        .collect();

    let m = heights.len();
    let n = heights.first().unwrap().len();

    let end_a = lines.iter().position(|line| line.contains('E')).unwrap();
    let end_b = lines[end_a].chars().position(|ch| ch == 'E').unwrap();

    let mut events = BinaryHeap::new();

    events.push(Event {
        dist: 0,
        a: end_a,
        b: end_b,
    });

    let mut visited = HashSet::<(usize, usize)>::new();

    loop {
        let event = events.pop().unwrap();

        if heights[event.a][event.b] == 1 {
            return event.dist.to_string();
        }

        if !visited.insert((event.a, event.b)) {
            continue;
        };

        if 0 < event.a && heights[event.a][event.b] <= heights[event.a - 1][event.b] + 1 {
            events.push(Event {
                dist: event.dist + 1,
                a: event.a - 1,
                b: event.b,
            });
        }

        if event.a < m - 1 && heights[event.a][event.b] <= heights[event.a + 1][event.b] + 1 {
            events.push(Event {
                dist: event.dist + 1,
                a: event.a + 1,
                b: event.b,
            });
        }

        if 0 < event.b && heights[event.a][event.b] <= heights[event.a][event.b - 1] + 1 {
            events.push(Event {
                dist: event.dist + 1,
                a: event.a,
                b: event.b - 1,
            });
        }

        if event.b < n - 1 && heights[event.a][event.b] <= heights[event.a][event.b + 1] + 1 {
            events.push(Event {
                dist: event.dist + 1,
                a: event.a,
                b: event.b + 1,
            });
        }
    }
}

fn compare_signals(lefto: &str, righto: &str) -> Ordering {
    let mut left = lefto.chars().peekable();
    let mut right = righto.chars().peekable();

    loop {
        let mut l = left.next().unwrap();
        let mut r = right
            .next()
            .unwrap_or_else(|| panic!("{} {}", lefto, righto));

        if l == ',' {
            l = left.next().unwrap();
        }

        if r == ',' {
            r = right.next().unwrap();
        }

        if l == r
            && (l != '1' || ((*left.peek().unwrap() == '0') == (*right.peek().unwrap() == '0')))
        {
            continue;
        }

        if l == ']' {
            return Ordering::Less;
        }

        if r == ']' {
            return Ordering::Greater;
        }

        if l == '[' {
            let mut d = 1;
            loop {
                l = left.next().unwrap();

                if l != '[' {
                    break;
                }
                d += 1;
            }
            if l == ']' {
                return Ordering::Less;
            }

            let ld = if *left.peek().unwrap() == '0' {
                left.next();
                10
            } else {
                l.to_digit(10).unwrap_or_else(|| panic!("{}", l))
            };
            let rd = if *right.peek().unwrap() == '0' {
                right.next();
                10
            } else {
                r.to_digit(10).unwrap()
            };

            match ld.cmp(&rd) {
                std::cmp::Ordering::Less => return Ordering::Less,
                std::cmp::Ordering::Equal => {}
                std::cmp::Ordering::Greater => return Ordering::Greater,
            }

            for _ in 0..d {
                if left.next().unwrap() != ']' {
                    return Ordering::Greater;
                }
            }
        }

        if r == '[' {
            let mut d = 1;
            loop {
                r = right.next().unwrap();
                if r != '[' {
                    break;
                }
                d += 1;
            }
            if r == ']' {
                return Ordering::Greater;
            }

            let ld = if *left.peek().unwrap() == '0' {
                left.next();
                10
            } else {
                l.to_digit(10).unwrap()
            };
            let rd = if *right.peek().unwrap() == '0' {
                right.next();
                10
            } else {
                r.to_digit(10).unwrap_or_else(|| panic!("{}", r))
            };

            match ld.cmp(&rd) {
                std::cmp::Ordering::Less => {
                    return Ordering::Less;
                }
                std::cmp::Ordering::Equal => {}
                std::cmp::Ordering::Greater => return Ordering::Greater,
            }

            for _ in 0..d {
                if right.next().unwrap() != ']' {
                    return Ordering::Less;
                }
            }
        }

        let ld = if *left.peek().unwrap() == '0' {
            left.next();
            10
        } else {
            l.to_digit(10).unwrap()
        };
        let rd = if *right.peek().unwrap() == '0' {
            right.next();
            10
        } else {
            r.to_digit(10).unwrap_or_else(|| panic!("{}", r))
        };

        match ld.cmp(&rd) {
            std::cmp::Ordering::Less => {
                return Ordering::Less;
            }
            std::cmp::Ordering::Equal => {}
            std::cmp::Ordering::Greater => return Ordering::Greater,
        }
    }
}

fn do_13s() -> String {
    let mut lines = read_lines("./res/13");

    let mut result = 0;

    let mut index = 0;

    loop {
        index += 1;

        let left_tmp = lines.next().unwrap();
        let right_tmp = lines.next().unwrap();

        if compare_signals(&left_tmp, &right_tmp) == Ordering::Less {
            result += index;
        }

        if lines.next() == None {
            return result.to_string();
        }
    }
}

fn do_13g() -> String {
    let mut lines: Vec<String> = read_lines("./res/13").filter(|st| !st.is_empty()).collect();
    lines.push("[[2]]".to_owned());
    lines.push("[[6]]".to_owned());

    lines.sort_by(|l, r| compare_signals(l, r));

    ((lines.iter().position(|s| s == "[[2]]").unwrap() + 1)
        * (lines.iter().position(|s| s == "[[6]]").unwrap() + 1))
        .to_string()
}

fn do_14s() -> String {
    let lines = read_lines("./res/14");

    let mut blocked: HashSet<(i32, i32)> = HashSet::new();

    for line in lines {
        let coords: Vec<(i32, i32)> = line
            .split(" -> ")
            .map(|st| {
                let (x, y) = st.split_once(',').unwrap();
                (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap())
            })
            .collect();

        for window in coords.windows(2) {
            let (x1, y1) = window[0];
            let (x2, y2) = window[1];

            if x1 != x2 {
                for x in x1.min(x2)..(x1.max(x2) + 1) {
                    blocked.insert((x, y1));
                }
            } else {
                for y in y1.min(y2)..(y1.max(y2) + 1) {
                    blocked.insert((x1, y));
                }
            }
        }
    }

    let abyss_y = blocked.iter().max_by_key(|c| c.1).unwrap().1;

    let mut result = 0;

    'sand: loop {
        let (mut x, mut y) = (500, 0);
        while y < abyss_y {
            if !blocked.contains(&(x, y + 1)) {
                y += 1;
            } else if !blocked.contains(&(x - 1, y + 1)) {
                x -= 1;
                y += 1
            } else if !blocked.contains(&(x + 1, y + 1)) {
                x += 1;
                y += 1
            } else {
                blocked.insert((x, y));
                result += 1;
                continue 'sand;
            }
        }

        return result.to_string();
    }
}

fn do_14g() -> String {
    let lines = read_lines("./res/14");

    let mut blocked: HashSet<(i32, i32)> = HashSet::new();

    for line in lines {
        let coords: Vec<(i32, i32)> = line
            .split(" -> ")
            .map(|st| {
                let (x, y) = st.split_once(',').unwrap();
                (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap())
            })
            .collect();

        for window in coords.windows(2) {
            let (x1, y1) = window[0];
            let (x2, y2) = window[1];

            if x1 != x2 {
                for x in x1.min(x2)..(x1.max(x2) + 1) {
                    blocked.insert((x, y1));
                }
            } else {
                for y in y1.min(y2)..(y1.max(y2) + 1) {
                    blocked.insert((x1, y));
                }
            }
        }
    }

    let abyss_y = blocked.iter().max_by_key(|c| c.1).unwrap().1 + 2;

    let mut result = 0;

    let mut path = vec![(500, 0)];

    'sand: loop {
        loop {
            let (mut x, mut y) = path.last().unwrap();

            if !blocked.contains(&(x, y + 1)) && y + 1 < abyss_y {
                y += 1;
                path.push((x, y))
            } else if !blocked.contains(&(x - 1, y + 1)) && y + 1 < abyss_y {
                x -= 1;
                y += 1;
                path.push((x, y))
            } else if !blocked.contains(&(x + 1, y + 1)) && y + 1 < abyss_y {
                x += 1;
                y += 1;
                path.push((x, y))
            } else {
                blocked.insert((x, y));
                result += 1;
                path.pop();
                if path.is_empty() {
                    return result.to_string();
                }
                continue 'sand;
            }
        }
    }
}

fn do_15s() -> String {
    let lines: Vec<String> = read_lines("./res/15").collect();

    let mut excluded = HashSet::new();

    for line in lines.clone() {
        let mut split = line.split(|ch| ch == '=' || ch == ',' || ch == ':');

        let sx = split.nth(1).unwrap().parse::<i32>().unwrap();
        let sy = split.nth(1).unwrap().parse::<i32>().unwrap();
        let bx = split.nth(1).unwrap().parse::<i32>().unwrap();
        let by = split.nth(1).unwrap().parse::<i32>().unwrap();

        let d = (sx.abs_diff(bx) + sy.abs_diff(by)) as i32;

        let l = sy.abs_diff(2000000) as i32;

        for x in (sx - d + l)..(sx + d - l + 1) {
            excluded.insert(x);
        }
    }

    for line in lines {
        let mut split = line.split(|ch| ch == '=' || ch == ',' || ch == ':');

        let _ = split.nth(1).unwrap().parse::<i32>().unwrap();
        let _ = split.nth(1).unwrap().parse::<i32>().unwrap();
        let bx = split.nth(1).unwrap().parse::<i32>().unwrap();
        let by = split.nth(1).unwrap().parse::<i32>().unwrap();

        if by == 2000000 {
            excluded.remove(&bx);
        }
    }

    excluded.len().to_string()
}

fn do_15g() -> String {
    let lines = read_lines("./res/15");

    let data: Vec<_> = lines
        .map(|line| {
            let mut split = line.split(|ch| ch == '=' || ch == ',' || ch == ':');

            let sx = split.nth(1).unwrap().parse::<i32>().unwrap();
            let sy = split.nth(1).unwrap().parse::<i32>().unwrap();
            let bx = split.nth(1).unwrap().parse::<i32>().unwrap();
            let by = split.nth(1).unwrap().parse::<i32>().unwrap();

            let d = (sx.abs_diff(bx) + sy.abs_diff(by)) as i32;

            (sx, sy, d)
        })
        .collect();

    for y in 0..4000001 {
        let mut excluded = Vec::new();
        excluded.reserve(data.len());

        for (sx, sy, d) in &data {
            let l = sy.abs_diff(y) as i32;

            if l <= *d {
                let min = sx - d + l;
                let max = sx + d - l + 1;
                excluded.push((min, max));
            }
        }

        let mut x = 0;

        'outer: loop {
            for (min, max) in &excluded {
                if *min <= x && x < *max {
                    x = *max;
                    if (4000001) <= x {
                        break 'outer;
                    } else {
                        continue 'outer;
                    }
                }
            }
            return ((4000000 * x as i64) + y as i64).to_string();
        }
    }

    panic!();
}

fn do_16s() -> String {
    let lines = read_lines("./res/16");

    let mut flows = Vec::new();
    let mut untranslated_targets = Vec::new();

    let mut translations: HashMap<String, usize> = HashMap::new();

    for line in lines {
        let mut iter = line.split(|ch| ch == '=' || ch == ';' || ch == ' ' || ch == ',');

        let room = iter.nth(1).unwrap();

        let flow = iter.nth(3).unwrap().parse::<i32>().unwrap();
        iter.nth(4);
        let targets: Vec<String> = iter
            .filter(|st| !st.is_empty())
            .map(|st| st.to_owned())
            .collect();

        translations.insert(room.to_owned(), flows.len());
        flows.push(flow);
        untranslated_targets.push(targets);
    }

    let start_id = translations["AA"];

    let targets: Vec<Vec<usize>> = untranslated_targets
        .iter()
        .map(|tgts| tgts.iter().map(|tg| translations[tg]).collect())
        .collect();

    fn score(
        remaining_minutes: i32,
        current_flow: i32,
        opened_valves_bits: u64,
        position: usize,
        flows: &Vec<i32>,
        targets: &Vec<Vec<usize>>,
        came_from: usize,
    ) -> i32 {
        if remaining_minutes == 0 {
            return 0;
        }

        let mut result = -1;
        if (opened_valves_bits & (1 << position)) == 0 && 1 <= flows[position] {
            result = result.max(
                current_flow
                    + score(
                        remaining_minutes - 1,
                        current_flow + flows[position],
                        opened_valves_bits | (1 << position),
                        position,
                        flows,
                        targets,
                        position,
                    ),
            );
        }
        for tgs in &targets[position] {
            if *tgs == came_from {
                continue;
            }
            result = result.max(
                current_flow
                    + score(
                        remaining_minutes - 1,
                        current_flow,
                        opened_valves_bits,
                        *tgs,
                        flows,
                        targets,
                        position,
                    ),
            )
        }

        result
    }

    score(30, 0, 0, start_id, &flows, &targets, start_id).to_string()
}

fn do_16g() -> String {
    let lines = read_lines("./res/16");

    let mut flows = Vec::new();
    let mut untranslated_targets = Vec::new();

    let mut translations: HashMap<String, usize> = HashMap::new();

    for line in lines {
        let mut iter = line.split(|ch| ch == '=' || ch == ';' || ch == ' ' || ch == ',');

        let room = iter.nth(1).unwrap();

        let flow = iter.nth(3).unwrap().parse::<i32>().unwrap();
        iter.nth(4);
        let targets: Vec<String> = iter
            .filter(|st| !st.is_empty())
            .map(|st| st.to_owned())
            .collect();

        translations.insert(room.to_owned(), flows.len());
        flows.push(flow);
        untranslated_targets.push(targets);
    }

    let mut valve_id_to_id = Vec::new();

    for (i, fl) in flows.iter().enumerate() {
        if 1 <= *fl {
            valve_id_to_id.push(i);
        }
    }

    let start_id = translations["AA"];

    let targets: Vec<Vec<usize>> = untranslated_targets
        .iter()
        .map(|tgts| tgts.iter().map(|tg| translations[tg]).collect())
        .collect();

    // let number_valves = flows.iter().filter(|flow| 1 <= **flow).count();

    fn visited(targets: &Vec<Vec<usize>>, mut already_there: u64, position: usize) -> u64 {
        already_there |= 1 << position;
        for t in &targets[position] {
            if already_there & (1 << t) != 0 {
                continue;
            }

            already_there |= visited(targets, already_there, *t);
        }

        already_there
    }

    let already_there = visited(&targets, 0, start_id);

    for i in 0..flows.len() {
        if already_there & (1 << i) != 0 {
            println!("found {}", i);
        }
    }

    println!("in total {}", already_there.count_ones());

    #[allow(clippy::too_many_arguments)]
    fn score(
        remaining_minutes: i32,
        current_flow: i32,
        opened_valves_bits: u64,
        position: usize,
        mut visited_since_valve: u64,
        flows: &Vec<i32>,
        targets: &Vec<Vec<usize>>,
        allowed_valves_bit: u64,
    ) -> i32 {
        visited_since_valve |= 1 << position;

        if opened_valves_bits == allowed_valves_bit {
            return remaining_minutes * current_flow;
        }

        if remaining_minutes == 0 {
            return 0;
        }

        let mut result = -1;
        let bit = 1 << position;
        if (allowed_valves_bit & bit) != 0 && (opened_valves_bits & bit) == 0 {
            result = result.max(
                current_flow
                    + score(
                        remaining_minutes - 1,
                        current_flow + flows[position],
                        opened_valves_bits | bit,
                        position,
                        0,
                        flows,
                        targets,
                        allowed_valves_bit,
                    ),
            );
        }

        for tgs in &targets[position] {
            if visited_since_valve & (1 << *tgs) != 0 {
                continue;
            }
            result = result.max(
                current_flow
                    + score(
                        remaining_minutes - 1,
                        current_flow,
                        opened_valves_bits,
                        *tgs,
                        visited_since_valve,
                        flows,
                        targets,
                        allowed_valves_bit,
                    ),
            )
        }

        result
    }

    let maxx = 2_u32.pow(valve_id_to_id.len() as u32 + 1);

    let par_iter = (0..maxx).into_par_iter().map(|i| {
        println!("lets do {} out of {}", i, maxx);

        let mut bits_p = 0;
        let mut bits_e = 0;
        for (j, item) in valve_id_to_id.iter().enumerate() {
            if i & (1 << j) == 0 {
                bits_p |= 1 << item;
            } else {
                bits_e |= 1 << item;
            }
        }

        score(26, 0, 0, start_id, 0, &flows, &targets, bits_p)
            + score(26, 0, 0, start_id, 0, &flows, &targets, bits_e)
    });

    let result = par_iter.max().unwrap();

    result.to_string()
}
fn do_17s() -> String {
    let lines_vec = read_lines("./res/17")
        .next()
        .unwrap()
        .chars()
        .collect::<Vec<char>>();

    let mut line = lines_vec.iter().cycle();

    let shapes_arr = [
        (vec![(3, 2), (3, 3), (3, 4), (3, 5)], 3),
        (vec![(3, 3), (4, 2), (4, 3), (4, 4), (5, 3)], 5),
        (vec![(3, 2), (3, 3), (3, 4), (4, 4), (5, 4)], 5),
        (vec![(3, 2), (4, 2), (5, 2), (6, 2)], 6),
        (vec![(3, 2), (4, 2), (4, 3), (3, 3)], 4),
    ];

    let mut shapes = shapes_arr.iter().cycle();

    const HEIGHT: usize = (13 * 2022) / 5 + 1 + 3 + 4;
    const WIDTH: usize = 7;

    let mut area = [[false; WIDTH]; HEIGHT];

    let mut height_tower = 0;

    'outer: for _ in 0..2022 {
        let (mut shape, mut heighest_x) = shapes.next().unwrap().clone();

        for (x, _) in &mut shape {
            *x += height_tower;
        }

        heighest_x += height_tower;

        loop {
            let dy: i32 = match line.next().unwrap() {
                '<' => -1,
                '>' => 1,
                _ => unreachable!(),
            };

            if shape.iter().all(|(x, y)| {
                0 <= y + dy && y + dy < WIDTH as i32 && !area[*x as usize][(y + dy) as usize]
            }) {
                for (_, y) in &mut shape {
                    *y += dy;
                }
            }

            if shape
                .iter()
                .all(|(x, y)| 1 <= *x && !area[(x - 1) as usize][*y as usize])
            {
                for (x, _) in &mut shape {
                    *x -= 1;
                }
                heighest_x -= 1;
            } else {
                for (x, y) in &shape {
                    area[*x as usize][*y as usize] = true;
                }

                height_tower = height_tower.max(heighest_x + 1);

                continue 'outer;
            }
        }
    }

    height_tower.to_string()
}

fn do_17g() -> String {
    let lines_vec = read_lines("./res/17")
        .next()
        .unwrap()
        .chars()
        .collect::<Vec<char>>();

    let cycle_length = lines_vec.len();

    let mut line = lines_vec.iter().cycle();

    let shapes_arr = [
        (vec![(3, 2), (3, 3), (3, 4), (3, 5)], 3),
        (vec![(3, 3), (4, 2), (4, 3), (4, 4), (5, 3)], 5),
        (vec![(3, 2), (3, 3), (3, 4), (4, 4), (5, 4)], 5),
        (vec![(3, 2), (4, 2), (5, 2), (6, 2)], 6),
        (vec![(3, 2), (4, 2), (4, 3), (3, 3)], 4),
    ];

    let shapes_len = shapes_arr.len();

    let mut shapes = shapes_arr.iter().cycle();

    const HEIGHT: usize = 200000;
    const WIDTH: usize = 7;

    let mut area = [[false; WIDTH]; HEIGHT];

    let mut height_tower: u64 = 0;

    let mut hidden_height = 0;

    let mut start = Instant::now();

    let mut shape_id = 0;
    let mut cycle_id = 0;

    let mut last_height = height_tower;
    let mut last_i = 0;

    let mut i: u64 = 0;

    'outer: loop {
        if i == 1000000000000_u64 {
            break;
        }
        if i % 10000000 == 0 {
            let duration = start.elapsed();
            println!("{:2}% [{:.2?}]", i as f64 / 10000000000_f64, duration);
            start = Instant::now();
        }

        let (mut shape, mut heighest_x) = shapes.next().unwrap().clone();

        shape_id += 1;
        shape_id %= shapes_len;

        if shape_id == 0 && cycle_id == 1 {
            if height_tower - last_height == 2737 && i - last_i == 1760 {
                println!("Let's go!!!!!!! [i = {}]", i);
                let skip = (1000000000000_u64 - i - 1) / 1760;
                println!("Skipping {} cycles.", skip);
                i += skip * 1760;
                println!("New i is {}.", i);
                height_tower += skip * 2737;
                hidden_height += skip * 2737;
            }

            last_height = height_tower;
            last_i = i;
        }

        for (x, _) in &mut shape {
            *x += height_tower - hidden_height;
        }

        heighest_x += height_tower - hidden_height;

        loop {
            let dy: i32 = match line.next().unwrap() {
                '<' => -1,
                '>' => 1,
                _ => unreachable!(),
            };

            cycle_id += 1;
            cycle_id %= cycle_length;

            if shape.iter().all(|(x, y)| {
                0 <= y + dy && y + dy < WIDTH as i32 && !area[*x as usize][(y + dy) as usize]
            }) {
                for (_, y) in &mut shape {
                    *y += dy;
                }
            }

            if shape
                .iter()
                .all(|(x, y)| 1 <= *x && !area[(x - 1) as usize][*y as usize])
            {
                for (x, _) in &mut shape {
                    *x -= 1;
                }
                heighest_x -= 1;
            } else {
                for (x, y) in &shape {
                    area[*x as usize][*y as usize] = true;
                }

                height_tower = height_tower.max(heighest_x + 1 + hidden_height);

                if (HEIGHT as u64 + hidden_height) - height_tower < 10 {
                    hidden_height += HEIGHT as u64 / 2;
                    for x in 0..(HEIGHT / 2) {
                        area[x] = area[x + HEIGHT / 2];
                    }
                    for row in area.iter_mut().skip(HEIGHT / 2) {
                        row.fill(false);
                    }
                }

                i += 1;
                continue 'outer;
            }
        }
    }

    height_tower.to_string()
}

fn do_18s() -> String {
    // let lines = read_lines("./res/18");

    "TODO".to_owned()
}

fn do_18g() -> String {
    // let lines = read_lines("./res/18");

    "TODO".to_owned()
}

fn do_19s() -> String {
    // let lines = read_lines("./res/19");

    "TODO".to_owned()
}

fn do_19g() -> String {
    // let lines = read_lines("./res/19");

    "TODO".to_owned()
}

fn do_20s() -> String {
    // let lines = read_lines("./res/20");

    "TODO".to_owned()
}

fn do_20g() -> String {
    // let lines = read_lines("./res/20");

    "TODO".to_owned()
}

fn do_21s() -> String {
    // let lines = read_lines("./res/21");

    "TODO".to_owned()
}

fn do_21g() -> String {
    // let lines = read_lines("./res/21");

    "TODO".to_owned()
}

fn do_22s() -> String {
    // let lines = read_lines("./res/22");

    "TODO".to_owned()
}

fn do_22g() -> String {
    // let lines = read_lines("./res/22");

    "TODO".to_owned()
}

fn do_23s() -> String {
    // let lines = read_lines("./res/23");

    "TODO".to_owned()
}

fn do_23g() -> String {
    // let lines = read_lines("./res/23");

    "TODO".to_owned()
}

fn do_24s() -> String {
    // let lines = read_lines("./res/24");

    "TODO".to_owned()
}

fn do_24g() -> String {
    // let lines = read_lines("./res/24");

    "TODO".to_owned()
}

fn do_5g() -> String {
    let mut lines = read_lines("./res/5");

    let mut config = vec![
        vec!['B', 'S', 'V', 'Z', 'G', 'P', 'W'],
        vec!['J', 'V', 'B', 'C', 'Z', 'F'],
        vec!['V', 'L', 'M', 'H', 'N', 'Z', 'D', 'C'],
        vec!['L', 'D', 'M', 'Z', 'P', 'F', 'J', 'B'],
        vec!['V', 'F', 'C', 'G', 'J', 'B', 'Q', 'H'],
        vec!['G', 'F', 'Q', 'T', 'S', 'L', 'B'],
        vec!['L', 'G', 'C', 'Z', 'V'],
        vec!['N', 'L', 'G'],
        vec!['J', 'F', 'H', 'C'],
    ];

    while !lines.next().unwrap().is_empty() {}

    for line in lines {
        let splitted: Vec<_> = line.split(' ').collect();

        let number = splitted[1].parse::<usize>().unwrap();
        let from = splitted[3].parse::<usize>().unwrap() - 1;
        let to = splitted[5].parse::<usize>().unwrap() - 1;

        let mut temp = Vec::new();

        for _ in 0..number {
            let popped = config[from].pop().unwrap();
            temp.push(popped);
        }

        for _ in 0..number {
            let popped = temp.pop().unwrap();
            config[to].push(popped);
        }
    }

    config.iter().map(|stack| stack.last().unwrap()).collect()
}

fn do_5s() -> String {
    let mut lines = read_lines("./res/5");

    let mut config = vec![
        vec!['B', 'S', 'V', 'Z', 'G', 'P', 'W'],
        vec!['J', 'V', 'B', 'C', 'Z', 'F'],
        vec!['V', 'L', 'M', 'H', 'N', 'Z', 'D', 'C'],
        vec!['L', 'D', 'M', 'Z', 'P', 'F', 'J', 'B'],
        vec!['V', 'F', 'C', 'G', 'J', 'B', 'Q', 'H'],
        vec!['G', 'F', 'Q', 'T', 'S', 'L', 'B'],
        vec!['L', 'G', 'C', 'Z', 'V'],
        vec!['N', 'L', 'G'],
        vec!['J', 'F', 'H', 'C'],
    ];

    while !lines.next().unwrap().is_empty() {}

    for line in lines {
        let splitted: Vec<_> = line.split(' ').collect();

        let number = splitted[1].parse::<usize>().unwrap();
        let from = splitted[3].parse::<usize>().unwrap() - 1;
        let to = splitted[5].parse::<usize>().unwrap() - 1;

        for _ in 0..number {
            let popped = config[from].pop().unwrap();
            config[to].push(popped);
        }
    }

    config.iter().map(|stack| stack.last().unwrap()).collect()
}

fn char_value(ch: char) -> i32 {
    if ch.is_lowercase() {
        return (ch as i32) - ('a' as i32) + 1;
    }
    (ch as i32) - ('A' as i32) + char_value('z') + 1
}

fn do_4s() -> String {
    let lines = read_lines("./res/4");

    let mut sum = 0;

    for line in lines {
        let (l, r) = line.split_once(',').unwrap();

        let (a_str, b_str) = l.split_once('-').unwrap();
        let (c_str, d_str) = r.split_once('-').unwrap();

        let a = a_str.parse::<u8>().unwrap();
        let b = b_str.parse::<u8>().unwrap();
        let c = c_str.parse::<u8>().unwrap();
        let d = d_str.parse::<u8>().unwrap();

        if (a <= c && d <= b) || (c <= a && b <= d) {
            sum += 1;
        }
    }

    sum.to_string()
}

fn do_4g() -> String {
    let lines = read_lines("./res/4");

    let mut sum = 0;

    for line in lines {
        let (l, r) = line.split_once(',').unwrap();

        let (a_str, b_str) = l.split_once('-').unwrap();
        let (c_str, d_str) = r.split_once('-').unwrap();

        let a = a_str.parse::<u8>().unwrap();
        let b = b_str.parse::<u8>().unwrap();
        let c = c_str.parse::<u8>().unwrap();
        let d = d_str.parse::<u8>().unwrap();

        if (a <= c && c <= b) || (a <= d && d <= b) || (c <= a && b <= d) {
            sum += 1;
        }
    }

    sum.to_string()
}

fn do_3s() -> String {
    let lines = read_lines("./res/3");

    let mut sum = 0;

    for line in lines {
        let (l, r) = line.split_at(line.len() / 2);

        for ch in l.chars() {
            if r.contains(ch) {
                sum += char_value(ch);
                break;
            }
        }
    }

    sum.to_string()
}

fn do_3g() -> String {
    let mut lines = read_lines("./res/3");

    let mut sum = 0;

    loop {
        let a_wrapped = lines.next();

        if a_wrapped.is_none() {
            return sum.to_string();
        }

        let a = a_wrapped.unwrap();
        let b = lines.next().unwrap();
        let c = lines.next().unwrap();

        for ch in a.chars() {
            if b.contains(ch) && c.contains(ch) {
                sum += char_value(ch);
                break;
            }
        }
    }
}

fn do_1s() -> String {
    let lines = read_lines("./res/1");

    let mut sums = vec![0];

    for line in lines {
        if line.is_empty() {
            sums.push(0);
        } else {
            *sums.last_mut().unwrap() += line.parse::<i32>().unwrap()
        }
    }

    (*sums.iter().max().unwrap()).to_string()
}

fn do_1g() -> String {
    let lines = read_lines("./res/1");

    let mut sums = vec![0];

    for line in lines {
        if line.is_empty() {
            sums.push(0);
        } else {
            *sums.last_mut().unwrap() += line.parse::<i32>().unwrap()
        }
    }

    sums.sort();

    (sums[sums.len() - 1] + sums[sums.len() - 2] + sums[sums.len() - 3]).to_string()
}

fn do_2s() -> String {
    let lines = read_lines("./res/2");

    let mut points = 0;

    for line in lines {
        if line == "A X" {
            points += 4;
        } else if line == "A Y" {
            points += 8;
        } else if line == "A Z" {
            points += 3;
        } else if line == "B X" {
            points += 1;
        } else if line == "B Y" {
            points += 5;
        } else if line == "B Z" {
            points += 9;
        } else if line == "C X" {
            points += 7;
        } else if line == "C Y" {
            points += 2;
        } else if line == "C Z" {
            points += 6;
        } else {
            panic!()
        }
    }

    points.to_string()
}

fn do_2g() -> String {
    let lines = read_lines("./res/2");

    let mut points = 0;

    for line in lines {
        if line == "A X" {
            points += 3;
        } else if line == "A Y" {
            points += 4;
        } else if line == "A Z" {
            points += 8;
        } else if line == "B X" {
            points += 1;
        } else if line == "B Y" {
            points += 5;
        } else if line == "B Z" {
            points += 9;
        } else if line == "C X" {
            points += 2;
        } else if line == "C Y" {
            points += 6;
        } else if line == "C Z" {
            points += 7;
        } else {
            panic!()
        }
    }

    points.to_string()
}

fn read_lines<P>(filename: P) -> impl Iterator<Item = String>
where
    P: AsRef<std::path::Path>,
{
    let file = std::fs::File::open(filename).unwrap();
    std::io::BufRead::lines(std::io::BufReader::new(file)).map(|line| line.unwrap())
}
