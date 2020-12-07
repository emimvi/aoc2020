use std::io;
use std::io::Read;
use std::collections::*;

fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input);
    let res = _day7(input.trim());
    println!("{}", res);
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
