use std::io;
use std::io::Read;
use std::collections::*;

fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input);
    let res = _day12(input.trim());
    println!("{:?}", res);
}

fn _day12(input : &str) -> usize {
    let parsed = input.lines().map(|l| {
        let mut iter = l.chars();
        let dir = iter.next().unwrap();
        let n = iter.as_str().parse::<i32>().unwrap();
        (dir, n)
    });
    let (mut wx, mut wy) = (10, 1);
    let (mut x, mut y) = (0, 0);
    let rotate = |deg, wx : i32, wy : i32| -> (i32, i32) {
        match deg {
                90 => (wy, -wx),
                180 => (wx * -1, wy * -1),
                270 => (-wy,wx),
            _ => panic!()
        }
    };
    for (dir, i) in parsed
    {
        match dir {
            'S' => wy -= i,
            'W' => wx -= i,
            'E' => wx += i,
            'N' => wy += i,
            'R' => { 
                let (nx, ny) = rotate(i, wx, wy); 
                wx = nx;
                wy = ny;
            },
            'L' => {
                let (nx, ny) = rotate(360-i, wx, wy); 
                wx = nx;
                wy = ny;
            }
            'F' => { x += wx * i; y += wy * i; }
            _ => panic!()
        }
        println!("{} {} | {} {}",x, y, wx, wy);
    }
    (x.abs()+y.abs()) as usize
}

fn _day12_0(input : &str) -> usize {
    let parsed = input.lines().map(|l| {
        let mut iter = l.chars();
        let dir = iter.next().unwrap();
        let n = iter.as_str().parse::<i32>().unwrap();
        (dir, n)
    });
    let dirs = ['S', 'W', 'N', 'E'];
    let mut dir_iter = dirs.iter().cycle();
    let mut front = 'E';
    let (mut x, mut y) = (0, 0);
    for (dir, i) in parsed
    {
        let d = if dir == 'F' { front } else { dir };
        match d {
            'S' => y -= i,
            'W' => x -= i,
            'E' => x += i,
            'N' => y += i,
            'R' => for _ in 0..(i/90) {front = *dir_iter.next().unwrap()},
            'L' => for _ in 0..((360-i)/90) {front = *dir_iter.next().unwrap()},
            // 'F' => {},
            _ => panic!()
        }
        println!("F {}, {} {}", front, x, y);
    }
    (x.abs()+y.abs()) as usize
}

fn _day11(input : &str) -> usize {
    fn adjacent(grid : &[Vec<char>], x : i32, y : i32) -> (i32, i32) {
        let getter = |x : i32, y : i32, dx : i32, dy : i32| -> char {
            let mut x = x + dx;
            let mut y = y + dy;
            loop {
                if x < 0 || y < 0 {
                    return '.';
                }
                match grid.get(y as usize).map(|v| v.get(x as usize)).unwrap_or(None) {
                    None => return '.',
                    Some('.') => {
                        x += dx;
                        y += dy;
                    },
                    Some(c) => return *c,
                }
            }
        };
        let dirs = [(-1, -1), 
                    (-1, 1),
                    (-1, 0),
                    (1, -1), 
                    (1, 1), 
                    (1, 0), 
                    (0, 1), 
                    (0, -1)];
        let (mut empty, mut occ) = (0, 0);

        for (dx, dy) in dirs.iter() {
            let c = getter(x, y, *dx, *dy);
            if c == 'L' {
                empty += 1;
            } else if c == '#' {
                occ += 1;
            }
        }
        (empty, occ)
    }

    let mut grid : Vec<Vec<char>> = input.lines().map(|l| {
        l.chars().collect::<Vec<char>>()
    }).collect();
    let mut next = grid.clone();
    let mut changed = false;
    let mut i = -1;
    loop {
        for y in 0..grid.len() {
            for x in 0..grid[0].len() {
                let (_, occ) = adjacent(&grid, x as i32, y as i32);
                match grid[y][x] {
                    'L' if occ == 0 => next[y][x] = '#',
                    '#' if occ >= 5 => next[y][x] = 'L',
                    _ => {}
                }
                changed |= grid[y][x] != next[y][x];
            }
        }
        let pr = next.iter().map(|l| l.iter().collect::<String>()).collect::<Vec<_>>();

        i -= 1;
        if i == 0 {
            return 0;
        }
        if !changed  {
            return grid.iter().map(|l| l.iter().filter(|&c| *c == '#').count()).sum()
        } else {
            changed = false;
            grid = next;
            next = grid.clone();
        }
    }
}

fn _day11_0(input : &str) -> usize {
    fn adjacent(grid : &[Vec<char>], x : i32, y : i32) -> (i32, i32) {
        let (mut empty, mut occ) = (0, 0);
        let xmin = (x-1).max(0);
        let xmax = (x+1).min(grid[0].len() as i32 -1);
        let ymin = (y-1).max(0);
        let ymax = (y+1).min(grid.len() as i32 -1);
        for i in xmin..=xmax {
        for j in ymin..=ymax {
            if i == x && y == j {
                continue;
            }
            let c = grid[j as usize][i as usize];
            if c == 'L' {
                empty += 1;
            } else if c == '#' {
                occ += 1;
            }
        }
        }
        (empty, occ)
    }

    let mut grid : Vec<Vec<char>> = input.lines().map(|l| {
        l.chars().collect::<Vec<char>>()
    }).collect();
    let mut next = grid.clone();
    let mut changed = false;
    let mut i = -1;
    loop {
        for y in 0..grid.len() {
            for x in 0..grid[0].len() {
                let (_, occ) = adjacent(&grid, x as i32, y as i32);
                match grid[y][x] {
                    'L' if occ == 0 => next[y][x] = '#',
                    '#' if occ >= 4 => next[y][x] = 'L',
                    _ => {}
                }
                changed |= grid[y][x] != next[y][x];
            }
        }
        let pr = next.iter().map(|l| l.iter().collect::<String>()).collect::<Vec<_>>();
        println!("{}", pr.join("\n"));
        println!("            ");
        i -= 1;
        if i == 0 {
            return 0;
        }
        if !changed  {
            return grid.iter().map(|l| l.iter().filter(|&c| *c == '#').count()).sum()
        } else {
            changed = false;
            grid = next;
            next = grid.clone();
        }
    }
}

fn _day10(input : &str) -> usize {
    let mut adapters : Vec<usize> = (0..1).into_iter().chain(input.lines().map(|l| l.trim().parse::<usize>().unwrap())).collect();
    adapters.sort();
    adapters.push(adapters.last().unwrap() + 3);
    let removable = adapters.windows(3).map(|slice| {
        if slice[2] - slice[0] <=3 {
            1 
        } else {
            0
        }
    }).collect::<Vec<_>>();
    let mut v = Vec::new();
    let mut i = 0;
    for &n in &removable {
        if n == 1 {
            i += 1;
        } else if i != 0 {
            v.push(i);
            i = 0;
        }
    }
    v.iter().map(|n| {
        match n {
            1 => 2,
            2 => 4,
            3 => 7,
            _ => panic!()
        }
    }).product()
}

fn _day10_0(input : &str) -> usize {
    let mut adapters : Vec<usize> = (0..1).into_iter().chain(input.lines().map(|l| l.parse::<usize>().unwrap())).collect();
    adapters.sort();
    adapters.push(adapters.last().unwrap() + 3);
    let iter1 = adapters.windows(2).map(|slice| slice[1] - slice[0]);
    let iter2 = iter1.clone();
    iter1.filter(|n| *n==3).count() * iter2.filter(|n| *n == 1).count()
}

fn _day9(input : &str) -> usize {
    let nums = input.lines().map(|l| l.parse::<usize>().unwrap()).collect::<Vec<_>>();
    let running_sums = nums.iter().skip(1).fold(vec!(nums[0]), |mut acc, e| {
        acc.push(acc.iter().last().unwrap() + e);
        acc
    });
    
    let target = 23278925;
    for i in 0..nums.len() {
        for j in i..nums.len() {
            if running_sums[j] - running_sums[i] == target {
                return nums[i..j].iter().max().unwrap() + nums[i..j].iter().min().unwrap();
            }
        }
    }

    panic!()
}

fn _day9_0(input : &str) -> usize {
    let nums = input.lines().map(|l| l.parse::<usize>().unwrap()).collect::<Vec<_>>();

    fn check(slice : &[usize], n : usize) -> bool {
        for (i, a) in slice.iter().enumerate() {
            for b in slice[i..].iter() {
                if a + b == n {
                    return true;
                }
            }
        }
        false
    }
    let preamble_len = 25;
    for i in preamble_len..nums.len() {
        let n = nums[i];
        if !check(&nums[(i-preamble_len)..i], n) {
            return n;
        }
    }

    panic!()
}

enum _PResult {
    Loop(i32),
    End(i32)
}

fn execute_program(program : &[(&str, i32)]) -> _PResult {
    let mut visited = HashSet::new();
    let lines = program;
    let mut acc = 0;
    let mut pc : usize = 1;
    loop {
        if visited.contains(&pc) {
            return _PResult::Loop(acc);
        } else if pc == program.len() + 1 {
            return _PResult::End(acc)
        }
        let (cmd, arg) = lines[pc-1 as usize];
        //println!("{} {}", cmd, arg);
        visited.insert(pc);
        match cmd {
            "jmp" => {
                pc = (pc as i32 + arg) as usize;
                continue;
            },
            "acc" => acc += arg,
            "nop" => {},
            _ => unimplemented!()
        }
        pc += 1;
    }
}

fn _day8(input : &str) -> i32 {
    let lines = input.lines().map(|l| {
        let mut inst_iter = l.split_whitespace();
        let cmd = inst_iter.next().unwrap();
        let arg = inst_iter.next().unwrap().parse::<i32>().unwrap();
        (cmd, arg)
    }).collect::<Vec<_>>();
    let mut program = lines.clone();
    for (i, (cmd, _)) in lines.iter().enumerate().filter(|(_, (c, _))| *c != "acc") {
        program[i].0 = match *cmd {
            "nop" => "jmp",
            "jmp" => "nop",
            _ => panic!()
        };
        if let _PResult::End(n) = execute_program(&program) {
            return n;
        }
        program = lines.clone();
    }
    unreachable!()
}

fn _day8_0(input : &str) -> i32 {
    let mut visited = HashSet::new();
    let lines = input.lines().collect::<Vec<_>>();
    let mut acc = 0;
    let mut pc : usize = 0;
    loop {
        if visited.contains(&pc) {
            break;
        }
        let mut inst_iter = lines[pc as usize].split_whitespace();
        let cmd = inst_iter.next().unwrap();
        let arg = inst_iter.next().unwrap().parse::<i32>().unwrap();
        println!("{} {} {}", pc+1, cmd, arg);
        match cmd {
            "jmp" if pc == lines.len()-1 => {}, //jmp to nop
            "jmp" => {
                pc = (pc as i32 + arg) as usize;
                continue;
            },
            "acc" => acc += arg,
            "nop" => {},
            _ => unimplemented!()
        }
        visited.insert(pc);
        pc += 1;
    }
    acc
}

fn _day7(input : &str) -> usize {
    let mapping : HashMap<String, Vec<(String, usize)>> = input.lines().map(|l| {
        let key = l.split_whitespace().take(2).collect::<String>();
        let mut ptr = l.split_whitespace().skip(4);
        let mut adjacent : Vec<(String, usize)> = Vec::new();
        loop {
            match ptr.next() {
                Some("no") | None => break,
                Some(other) => { 
                    let mut k = String::from(ptr.next().unwrap());
                    k.push_str(ptr.next().unwrap());
                    let v = other.parse::<usize>().unwrap();
                    adjacent.push((k, v));

                    let _ = ptr.next();
                }
            }
        }
        (key, adjacent)
    }).collect();

    fn traverse(mapping : &HashMap<String, Vec<(String, usize)>>, key : &str) -> usize {
        let s : usize = mapping.get(key).unwrap().iter().map(|(ref k, ref v)| {
            v * traverse(&mapping, k)
        }).sum();
        s + 1
    }

    traverse(&mapping, "shinygold") - 1
}

fn _day7_0(input : &str) -> usize {
    let mapping : HashMap<String, Vec<(String, usize)>> = input.lines().map(|l| {
        let key = l.split_whitespace().take(2).collect::<String>();
        let mut ptr = l.split_whitespace().skip(4);
        let mut adjacent : Vec<(String, usize)> = Vec::new();
        loop {
            match ptr.next() {
                Some("no") | None => break,
                Some(other) => { 
                    let mut k = String::from(ptr.next().unwrap());
                    k.push_str(ptr.next().unwrap());
                    let v = other.parse::<usize>().unwrap();
                    adjacent.push((k, v));

                    let _ = ptr.next();
                }
            }
        }
        (key, adjacent)
    }).collect();


    fn traverse(mapping : &HashMap<String, Vec<(String, usize)>>, key : &str) -> bool {
        let mut res = false;
        for (k, _) in mapping.get(key).unwrap().iter() {
            match k.as_ref() {
                "shinygold" => return true,
                other => res |= traverse(mapping, other),
            }
        }
        res
    }

    mapping.iter().filter(|(k,_)| {
        traverse(&mapping, k)
    }).count()
}

fn _day6(input : &str) -> usize {
    input.split("\n\n").map(|s| {
        let num_people = s.split("\n").count();
        let mut set : HashMap<char, usize> = HashMap::new();
        for c in s.chars().filter(|c| c.is_alphabetic()) {
            set.insert(c, *set.get(&c).unwrap_or(&0) + 1);
        }
        set.values()
            .filter(|v| **v == num_people).count()
    }).sum()
}

fn _day6_0(input : &str) -> usize {
    input.split("\n\n").map(|s| {
        s.chars().filter(|c| c.is_alphabetic()).collect::<HashSet<_>>().len()
    }).sum()
}

fn _day5(input : &str) -> usize {
    let seats = input.lines().map(|line| {
        let (mut min, mut max) = (0, 127);
        for c in line.chars().take(7) {
            match c {
                'B' => min += (max - min + 1) / 2,
                'F' => max -= (max - min) / 2,
                _ => panic!(),
            }
        }
        let row = min;

        let (mut min, mut max) = (0, 7);
        for c in line.chars().skip(7).take(3) {
            match c {
                'R' => min += (max - min + 1) / 2,
                'L' => max -= (max - min) / 2,
                _ => panic!(),
            }
        }
        let col = min;
        row * 8 + col
    }).collect::<BTreeSet<_>>();
    let mut prev = 0;
    for s in seats {
        if s - prev == 2 {
            return s- 1;
        }
        prev = s;
    }
    0
}

fn _day5_0(input : &str) -> usize {
    input.lines().map(|line| {
        let (mut min, mut max) = (0, 127);
        for c in line.chars().take(7) {
            match c {
                'B' => min += (max - min + 1) / 2,
                'F' => max -= (max - min) / 2,
                _ => unimplemented!(),
            }
        }
        let row = min;

        let (mut min, mut max) = (0, 7);
        for c in line.chars().skip(7).take(3) {
            match c {
                'R' => min += (max - min + 1) / 2,
                'L' => max -= (max - min) / 2,
                _ => unimplemented!(),
            }
        }
        let col = min;
        row * 8 + col
    }).max().unwrap()
}

fn _day4(input : &str) -> usize {
    let must_have = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    input.split("\n\n").filter_map(|chunk| {
        let keys : HashMap<_, _> = chunk.split_whitespace().map(|token| { 
            let mut kv = token.split(":").take(2);
            (kv.next().unwrap(), kv.next().unwrap())
        }).collect();
        if must_have.iter().all(|s| keys.contains_key(s)) {
            Some(keys)
        } else {
            None
        }
    }).filter(|hash_map| {
        let eyes : HashSet<_> = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth" ].into_iter().collect();
        let bools = vec![
            hash_map.get("byr").unwrap().parse::<usize>().map(|v| v >= 1920 && v <= 2002).unwrap_or(false),
            hash_map.get("iyr").unwrap().parse::<usize>().map(|v| v >= 2010 && v <= 2020).unwrap_or(false),
            hash_map.get("eyr").unwrap().parse::<usize>().map(|v| v >= 2020 && v <= 2030).unwrap_or(false),
            {
                let height = hash_map.get("hgt").unwrap();
                let num = height.chars().take_while(|c| c.is_digit(10)).collect::<String>().parse::<usize>().unwrap();
                let measure : String = height.chars().skip_while(|u| u.is_digit(10)).collect();
                match measure.as_ref() {
                    "in" => num >= 59 && num <= 76,
                    "cm" => num >= 150 && num <= 193,
                    _ => false
                }
            },
            {
                let s = hash_map.get("hcl").unwrap();
                s.chars().take(1).next().unwrap() == '#' && s.chars().skip(1).all(|v| v.is_digit(16))
            },
            eyes.contains(hash_map.get("ecl").unwrap()),
            { 
               let pid = hash_map.get("pid").unwrap();
               pid.len() == 9 && pid.chars().all(|c| c.is_digit(10))
            }
        ];
        bools.into_iter().all(|b| b)
    }).count()
}

fn _day4_0(input : &str) -> usize {
    let must_have = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    input.split("\n\n").filter(|chunk| {
        let keys : HashSet<_> = chunk.split_whitespace().map(|token| token.split(":").next().unwrap()).collect();
        must_have.iter().all(|s| keys.contains(s))
    }).count()
}

fn _day3(input : &str) -> usize {
    let lines : Vec<&str> = input.lines().collect();
    fn single_run(lines: &Vec<&str>, dx : usize, dy : usize) -> usize {
        let mut x = 0;
        let mut y = 0;
        let delta_x = dx;
        let delta_y = dy;
        let mut trees = 0;
        while y < lines.len() {
            let line = lines[y];
            let cnt = line.len();
            if line.chars().nth(x % cnt).unwrap() == '#'  {
                trees += 1;
            }
            x += delta_x;
            y += delta_y;
        }
        trees
    }
    vec![(1,1), (3,1), (5,1), (7,1), (1,2)].into_iter()
        .map(|(dx, dy)| single_run(&lines, dx, dy))
        .product()
}

fn _day3_0(input : &str) {
    let lines : Vec<&str> = input.lines().collect();
    let mut x = 0;
    let mut y = 0;
    let delta_x = 3;
    let delta_y = 1;
    let mut trees = 0;
    while y < lines.len() {
        let line = lines[y];
        let cnt = line.len();
        if line.chars().nth(x % cnt).unwrap() == '#'  {
            trees += 1;
        }
        x += delta_x;
        y += delta_y;
    }
    println!("{}", trees);
}

fn _day2(input : String) {
    let mut valids = 0;
    for line in input.trim().lines() {
        let tokens : Vec<&str> = line.split(" ").collect();
        let range : Vec<usize> = tokens[0].split("-").map(|x| x.parse::<usize>().unwrap()).collect();
        let letter = tokens[1].chars().next().unwrap();
        let letter_1 : bool = tokens[2].chars().nth(range[0] - 1).unwrap() == letter;
        let letter_2 : bool = tokens[2].chars().nth(range[1] - 1).unwrap() == letter;
        if letter_1 ^ letter_2 {
            valids += 1;
        }
    }
    println!("{}", valids);
}
fn _day2_0(input : String) {
    let mut valids = 0;
    for line in input.trim().lines() {
        let tokens : Vec<&str> = line.split(" ").collect();
        let range : Vec<i32> = tokens[0].split("-").map(|x| x.parse::<i32>().unwrap()).collect();
        let letter = tokens[1].chars().next().unwrap();
        let num_letter : i32 = tokens[2].chars().map(|c| if c == letter { 1 } else { 0 }).sum();
        if num_letter >= range[0] && num_letter <= range[1] {
            valids += 1;
        }
    }
    println!("{}", valids);
}

fn _day1(input : String) {
    let lines : Vec<i32> = input.trim().lines().map(|x| x.parse::<i32>().unwrap()).collect();
    for i in 0..lines.len() {
    for j in 0..lines.len() {
    for k in 0..lines.len() {
        if i == j ||  i == k || j == k {
            continue;
        }
        if lines[i] + lines[j] + lines[k] == 2020 {
            println!("{}", lines[i] * lines[j] * lines[k])
        }
    }
    }
    }

}
fn _day1_0(input : String) {
    let mut set : HashSet<i32> = HashSet::new();
    for s in input.lines().map(|x| x.parse::<i32>().unwrap()) {
        if set.contains(&s) {
            println!("{}", s * (2020 - s));
            break;
        }
        set.insert(2020 - s);
    }
}
