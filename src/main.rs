#![allow(unused_variables)]
#![allow(dead_code)]
use std::io;
use std::io::Read;
use std::collections::*;

fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input);
    let res = _day222(input.trim());
    println!("{:?}", res);
}

fn _day222(input : &str) -> usize { 
    let mut players_iter = input.split("\n\n").map(|p| {
        p.lines().filter_map(|s| s.parse::<usize>().ok()).collect::<VecDeque<_>>()
    });

    let mut p1 = players_iter.next().unwrap();
    let mut p2 = players_iter.next().unwrap();
    println!("{:?}", p1);
    println!("{:?}", p2);

    let mut prev_configs = HashSet::new();
    fn game(p1 : &mut VecDeque<usize>, p2 : &mut VecDeque<usize>, gamenum : usize, prev_configsmap : &mut HashSet<String>) -> usize {

        let mut round = 1;
        while p1.len() > 0 && p2.len() > 0 {
            {
                let i2 = p2.iter().flat_map(|i| vec![(*i as u8) as char, ','].into_iter());
                let s1 = p1.iter().flat_map(|i| vec![(*i as u8) as char, ','].into_iter()).chain(std::iter::once(';')).chain(i2).collect::<String>();
                if prev_configsmap.contains(&s1) {
                    return 1;
                } else {
                    prev_configsmap.insert(s1);
                }
            }
            
            let (c1, c2) = (p1.pop_front().unwrap(), p2.pop_front().unwrap());
            if c1 <= p1.len() && c2 <= p2.len() {
                let p1c = p1.clone();
                let p2c = p2.clone();
                let mut newset = HashSet::new();
                let mut new_deck1 = p1c.iter().cloned().take(c1).collect::<VecDeque<_>>();
                let mut new_deck2 = p2c.iter().cloned().take(c2).collect::<VecDeque<_>>();
                match game(&mut new_deck1, &mut new_deck2, gamenum+1, &mut newset) {
                    1 => {
                        p1.push_back(c1);
                        p1.push_back(c2);
                    },
                    2 => {
                        p2.push_back(c2);
                        p2.push_back(c1);
                    },
                    _ => panic!()
                };
            } else if c1 > c2 {
                p1.push_back(c1);
                p1.push_back(c2);
            } else {
                p2.push_back(c2);
                p2.push_back(c1);
            }
            round += 1;
        }
        if p1.len() > 0 {
            1
        } else {
            2
        }
    }
    let winner = match game(&mut p1, &mut p2, 1, &mut prev_configs) {
        1 => &p1,
        2 => &p2,
        _ => panic!()
    };
    println!("{:?}", p1);
    println!("{:?}", p2);
    winner.iter().rev().enumerate().map(|(i, n)| (i+1) * n).sum()
}

fn _day25(input : &str) -> usize { 
    let v = input.lines().filter_map(|l| l.parse::<usize>().ok()).collect::<Vec<_>>();
    let card = 5764801;
    let door = 17807724;
    let card = v[0];
    let door = v[1];

    let rem = 20201227;
    let mut value = card;
    let mut loop_size = 0;
    let subject = 7;
    while value != 1 {
        while value % subject != 0 {
            value += rem;
        }
        assert!(value % subject == 0);
        value /= subject;
        loop_size += 1;
    }

    let mut value = 1;
    let subject = door;
    for i in 0..loop_size {
        value *= subject;
        value %= rem;
    }

    value
}

fn _day24(input : &str) -> usize { 
    let line_iter = input.lines().map(|l| {
        let mut chars = l.chars().peekable();
        let mut res_vec = Vec::new();
        while let Some(c) = chars.peek() {
            let n = match c {
                'n' | 's' => 2,
                'e' | 'w' => 1,
                _ => panic!(),
            };
            res_vec.push(chars.by_ref().take(n).collect::<String>());
        }
        res_vec
    });

    fn get_coord(s : &str) -> (i32, i32) {
        match s {
                "w"  => (-2, 0),
                "e"  => (2, 0),
                "se"  => (1, -1),
                "sw"  => (-1, -1),
                "ne"  => (1, 1),
                "nw"  => (-1 ,1),
                _ => panic!()
        }
    }

    fn get_neighbours(coord : (i32, i32)) -> Vec<(i32,i32)> {
        let (x,y) = coord;
        [(-2, 0), (2, 0), (1, -1), (-1, -1), (1, 1), (-1 ,1)].iter().map(|(a,b)| (x+a, y+b)).collect::<Vec<_>>()
    }

    let mut set = HashSet::new();
    for line in line_iter {
        let coord = line.iter().fold((0,0), |(x,y), s| {
            let (dx, dy) = get_coord(s);
            (x+dx, y+dy)
        });
        if set.contains(&coord) {
            set.remove(&coord);
        } else {
            set.insert(coord);
        }
    }
    let mut visited = HashSet::new();
    let mut next_set = HashSet::new();
    for _ in 0..100 {
        for &coord in set.iter() {
            let neighbours = get_neighbours(coord);
            let black_neighbours = neighbours.iter().filter(|c| set.contains(c)).count();
            match black_neighbours {
                1 | 2 => { let _ = next_set.insert(coord); },
                _ => {}
            }
            visited.insert(coord);
            for n in neighbours {
                if visited.contains(&n) || set.contains(&n) {
                    continue;
                }
                let neighbours = get_neighbours(n);
                let black_neighbours = neighbours.iter().filter(|c| set.contains(c)).count();
                match black_neighbours {
                    2 => { let _ = next_set.insert(n); },
                    _ => {}
                }
                visited.insert(n);
            }
        }
        visited.clear();
        set = next_set;
        next_set = HashSet::new();
    }

    set.len()
}

const MAX : usize = 1_000_000;
fn _day232(input : &str) -> usize { 
    let parsed = input.chars().map(|c| c.to_digit(10).unwrap() as usize).collect::<VecDeque<_>>();

    let mut arr = vec![0; MAX + 1];
    arr[MAX] = parsed[0];
    arr[parsed[8]] = 10;
    for i in 10..MAX {
        arr[i] = i+1;
    }
    for i in 1..parsed.len() {
        let prev = parsed[i-1];
        arr[prev] = parsed[i];
    }
    let mut current = parsed[0];
    let mut destination;
    let mut pick_up = [0; 3];

    for i in 0..10_000_000 {
        // print(&arr, current);
        let mut next = arr[current];
        for i in 0..3 {
            pick_up[i] = next;
            next = arr[next];
        }
        destination = current;
        loop  {
            destination -= 1;
            if destination == 0 {
                destination = MAX;
            }
            if pick_up.iter().all(|&p| p != destination) {
                break;
            }
        }
        let current_next = arr[pick_up[2]];
        let destination_next = pick_up[0];
        let pick_up2_next = arr[destination];

        // println!("current {:?} ", current);
        // println!("pick_up {:?} ", pick_up);
        // println!("destination {} ", destination);
        // dbg!(current_next);
        // dbg!(destination_next);
        // dbg!(pick_up2_next);

        arr[current] = current_next;
        arr[destination] = destination_next;
        arr[pick_up[2]] = pick_up2_next;

        // print(&arr, current);
        current = current_next;
    }

    let mut c = arr[1];
    let mut res = 1;
    for _ in 0..2 {
        print!("{} ", c);
        res *= c;
        c = arr[c];
    }
    println!("");

    println!("");
    res
}

fn _day23(input : &str) -> usize { 
    let mut parsed = input.chars().map(|c| c.to_digit(10).unwrap()).collect::<VecDeque<_>>();
    let mut max = parsed.iter().cloned().max().unwrap();
    parsed.push_back(MAX as u32);
    let mut pick_up = [0; 3];
    let mut destination;
    let mut percentage = 1;

    fn pop_front(parsed : &mut VecDeque<u32>, current_max : &mut u32) -> u32 {
        if *parsed.front().unwrap() == MAX as u32 {
            *current_max += 1;
            *current_max
        } else {
            parsed.pop_front().unwrap()
        }
    }

    let print = true;
    for i in 1..=10 { //_000_000 {
        if i %      1_000*percentage == 0{
            println!("{}", percentage);
            percentage += 1;
        }
        if print {
            println!("-- move {} --", i);
            println!("cups: {:?}", parsed);
        }
        let current = parsed.pop_front().unwrap();
        parsed.push_back(current);
        pick_up[0] = pop_front(&mut parsed, &mut max);
        pick_up[1] = pop_front(&mut parsed, &mut max);
        pick_up[2] = pop_front(&mut parsed, &mut max);
        let next = pop_front(&mut parsed, &mut max);
        parsed.push_front(next);
        destination = if current == 1 { MAX as u32 } else { current-1 } ;
        while pick_up.iter().any(|&p| p == destination) {
            destination -= 1;
            if destination == 0 {
                destination = MAX as u32;
            }
        }
        if print {
        println!("pick up: {:?}", pick_up);
        println!("destination: {}", destination);
        println!("");
        }
        while let Some(i) = parsed.pop_front() {
            if i == destination {
                parsed.push_back(i);
                parsed.push_back(pick_up[0]);
                parsed.push_back(pick_up[1]);
                parsed.push_back(pick_up[2]);
                break;
            } else {
                parsed.push_back(i);
            }
        }

        while let Some(i) = parsed.pop_back() {
            if i != next {
                parsed.push_front(i);
            } else {
                parsed.push_front(i);
                break;
            }
        }
    }
    println!("");
    0
}

fn _day23_0(input : &str) -> usize { 
    let mut parsed = input.chars().map(|c| c.to_digit(10).unwrap()).collect::<VecDeque<_>>();
    let max = parsed.iter().cloned().max().unwrap();
    let mut pick_up = [0; 3];
    let mut destination;
    for i in 1..=100 {
        println!("-- move {} --", i);
        println!("cups: {:?}", parsed);
        let current = parsed.pop_front().unwrap();
        parsed.push_back(current);
        pick_up[0] = parsed.pop_front().unwrap();
        pick_up[1] = parsed.pop_front().unwrap();
        pick_up[2] = parsed.pop_front().unwrap();
        let next = *parsed.front().unwrap();
        destination = if current == 1 { max } else { current-1 } ;
        while pick_up.iter().any(|&p| p == destination) {
            destination -= 1;
            if destination == 0 {
                destination = max;
            }
        }
        println!("pick up: {:?}", pick_up);
        println!("destination: {}", destination);
        println!("");
        while let Some(i) = parsed.pop_front() {
            if i == destination {
                parsed.push_back(i);
                parsed.push_back(pick_up[0]);
                parsed.push_back(pick_up[1]);
                parsed.push_back(pick_up[2]);
                break;
            } else {
                parsed.push_back(i);
            }
        }

        while let Some(i) = parsed.pop_front() {
            if i != next {
                parsed.push_back(i);
            } else {
                parsed.push_front(i);
                break;
            }
        }
    }
    while let Some(i) = parsed.pop_front() {
        if i != 1 {
            parsed.push_back(i);
        } else {
            break;
        }
    }
    for i in parsed {
        print!("{}", i);
    }
    println!("");
    0
}

fn _day22(input : &str) -> usize { 
    let mut players_iter = input.split("\n\n").map(|p| {
        p.lines().filter_map(|s| s.parse::<usize>().ok()).collect::<VecDeque<_>>()
    });

    let mut p1 = players_iter.next().unwrap();
    let mut p2 = players_iter.next().unwrap();
    println!("{:?}", p1);
    println!("{:?}", p2);

    let mut prev_configs = HashSet::new();
    fn game(p1 : &mut VecDeque<usize>, p2 : &mut VecDeque<usize>, print : bool, prev_configsmap : &mut HashSet<String>) -> usize {

        while p1.len() > 0 && p2.len() > 0 {
            {
                let i2 = p2.iter().flat_map(|i| vec![(*i as u8) as char, ','].into_iter());
                let s1 = p1.iter().flat_map(|i| vec![(*i as u8) as char, ','].into_iter()).chain(std::iter::once(';')).chain(i2).collect::<String>();
                if prev_configsmap.contains(&s1) {
                    println!("Already discovered {:?}", prev_configsmap.len());
                    return 1;
                } else {
                    prev_configsmap.insert(s1);
                }
            }
            
            let (c1, c2) = (p1.pop_front().unwrap(), p2.pop_front().unwrap());
            if c1 <= p1.len() && c2 <= p2.len() {
                println!("RECURSE {}", p1.len() + p2.len());
                let mut p1c = p1.clone();
                let mut p2c = p2.clone();
                match game(&mut p1c, &mut p2c, false, prev_configsmap) {
                    1 => {
                        p1.push_back(c1);
                        p1.push_back(c2);
                    },
                    2 => {
                        p2.push_back(c2);
                        p2.push_back(c1);
                    },
                    _ => panic!()
                };
            } else if c1 > c2 {
                p1.push_back(c1);
                p1.push_back(c2);
            } else {
                p2.push_back(c2);
                p2.push_back(c1);
            }
        }
        if p1.len() > 0 {
            1
        } else {
            2
        }
    }
    let winner = match game(&mut p1, &mut p2, true, &mut prev_configs) {
        1 => &p1,
        2 => &p2,
        _ => panic!()
    };
    println!("{:?}", p1);
    println!("{:?}", p2);
    winner.iter().rev().enumerate().map(|(i, n)| (i+1) * n).sum()
}

fn _day22_0(input : &str) -> usize { 
    let mut players_iter = input.split("\n\n").map(|p| {
        p.lines().filter_map(|s| s.parse::<usize>().ok()).collect::<VecDeque<_>>()
    });

    let mut p1 = players_iter.next().unwrap();
    let mut p2 = players_iter.next().unwrap();

    while p1.len() > 0 && p2.len() > 0 {
        let (c1, c2) = (p1.pop_front().unwrap(), p2.pop_front().unwrap());
        if c1 > c2 {
            p1.push_back(c1);
            p1.push_back(c2);
        } else {
            p2.push_back(c2);
            p2.push_back(c1);
        }
    }
    let winner = if p1.len() > 0 {
        p1
    } else {
        p2
    };
    winner.iter().rev().enumerate().map(|(i, n)| (i+1) * n).sum()
}

fn _day21(input : &str) -> usize { 
    let pairs = input.lines().map(|l| {
        let mut it = l.split(" (contains ");
        let ingridients = it.next().unwrap().split_whitespace().collect::<HashSet<_>>();
        let allergens = { 
            let s = it.next().unwrap();
            let s = &s[0..s.len()-1];
            s.split(", ").collect::<HashSet<_>>()
        };
        (ingridients, allergens)
    }).collect::<Vec<_>>();

    let mut map : HashMap<&str, HashSet<&str>> = HashMap::new();
    for (ingv, algs) in pairs.iter() {
        for a in algs.iter() {
            if let Some(cur_set) = map.get_mut(a) {
                cur_set.retain(|key| ingv.contains(key));
            } else {
                map.insert(a, ingv.clone());
            }
        }
    }

    let mut res = Vec::new();
    println!("{:?}", map);
    while let Some(t) = map.iter().find_map(|(k, v)| if v.len() == 0 { Some((*k, *v.iter().next().unwrap())) } else { None }) {
        res.push(t);
        for set in map.values_mut() {
            set.remove(t.1);
        }
    } 
    res.sort_by(|a, b| a.0.cmp(b.0));
    for s in res.iter().map(|(i, a)| a) {
        print!("{},", s);
    }
    0
}

fn _day21_0(input : &str) -> usize { 
    let pairs = input.lines().map(|l| {
        let mut it = l.split(" (contains ");
        let ingridients = it.next().unwrap().split_whitespace().collect::<HashSet<_>>();
        let allergens = { 
            let s = it.next().unwrap();
            let s = &s[0..s.len()-1];
            s.split(", ").collect::<HashSet<_>>()
        };
        (ingridients, allergens)
    }).collect::<Vec<_>>();

    let mut map : HashMap<&str, HashSet<&str>> = HashMap::new();
    for (algs, ingv) in pairs.iter() {
        for i in ingv.iter() {
            if let Some(cur_set) = map.get_mut(i) {
                cur_set.retain(|key| algs.contains(key));
            } else {
                map.insert(i, algs.clone());
            }
        }
    }

    println!("{:?}", map);
    let possible = map.values().into_iter().flatten().collect::<HashSet<_>>();
    println!("{:?}", possible);
    pairs.iter().map(|(v, _)| v).flatten().filter(|s| !possible.contains(s)).inspect(|v| println!("{}", v)).count()
}


#[derive(Debug)]
enum Edge {
    Left = 0,
    LeftRev = 1,
    Right = 2,
    RightRev = 3,
    Top = 4,
    TopRev = 5,
    Bottom = 6,
    BottomRev = 7
}
impl Edge {
    fn from(i : usize) -> Self {
        match i {
            0 => Self::Left,
            1 => Self::LeftRev,
            2 => Self::Right,
            3 => Self::RightRev,
            4 => Self::Top,
            5 => Self::TopRev,
            6 => Self::Bottom,
            7 => Self::BottomRev,
            _ => panic!(),
        }
    }
}

fn flip_y(grid : &[Vec<u8>]) -> Vec<Vec<u8>> {
    let mut res = grid.iter().map(|r| r.iter().map(|ri| *ri).collect::<Vec<_>>()).collect::<Vec<_>>();
    let len = grid.len();
    for y in 0..len {
        for x in 0..len {
            res[len-1-y][x] = grid[y][x];
        }
    }
    res
}

fn flip_x(grid : &[Vec<u8>]) -> Vec<Vec<u8>> {
    let mut res = grid.iter().map(|r| r.iter().map(|ri| *ri).collect::<Vec<_>>()).collect::<Vec<_>>();
    let len = grid.len();
    for y in 0..len {
        for x in 0..len {
            res[y][len-1-x] = grid[y][x];
        }
    }
    res
}

struct Tile(Vec<Vec<u8>>);

impl Tile {

    fn all_possible_edges(&self) -> Vec<Row> {
        vec![ 
          self.get_column(0), 
          self.get_column(9),
          self.get_row(0),
          self.get_row(9),
        ].into_iter()
         .flat_map(|v| v.into_iter())
         .collect::<Vec<_>>()
    }

    fn clone(&self) -> Self {
        Self(self.0.clone())
    }

    fn flip_y(&self) -> Self {
        Self::from(flip_y(&self.0))
    }

    fn rotate(&self) -> Self {
        let mut res = self.0.clone();
        for i in 0..self.0.len() {
            for j in 0..self.0.len() {
                res[i][j] =self.0[j][i];
            }
        }
        Self(res)
    }

    fn flip_x(&self) -> Self {
        Self::from(flip_x(&self.0))
    }

    fn inside(&self) -> Self {
        let mut res = Vec::new();
        let n = self.0.len() - 1;
        for j in 1..n {
            let mut r = Vec::new();
            for i in 1..n {
                r.push(self.0[j][i]);
            }
            if r.len() != self.0.len() - 2 {
                panic!()
            }
            res.push(r);
        }
        if res.len() != self.0.len() - 2 {
            panic!()
        }
        Self(res)
    }

    fn from(v : Vec<Vec<u8>>) -> Self {
        let mut res = v.clone();
        for y in 0..v.len() {
            for x in 0..v.len() {
                res[y][x] = v[y][x];
            }
        }
        Tile(res)
    }

    fn pattern_search(&self, pattern : &[Row]) -> usize {
        let grid = &self.0;
        grid.windows(pattern.len()).enumerate().map(|(_i, rows)| {
            let mut monsters = 0;
            'outer : for i in 0..(grid.len() - pattern[0].len()) {
                for j in 0..pattern.len() {
                    let v = pattern[j].as_slice();
                    let subrow = &rows[j][i..i+pattern[0].len()];
                    if !v.iter().enumerate()
                        .filter(|(_, v)| *v == &1)
                        .all(|(i,_)| subrow[i] == 1) 
                        {
                            continue 'outer;
                        }
                }
                monsters += 1;
            }
            monsters
        }).sum()
    }


    fn get_column(&self, i : usize) -> Vec<Row> {
        let mut edges = Vec::new();
        let mut a = self.0[0].clone();
        for row in 0..10 {
            a[row] = self.0[row][i];
        }
        let b = clone_rev(&a);
        edges.push(a);
        edges.push(b);
        edges
    }


    fn get_row(&self, i : usize) -> Vec<Row> {
        let mut edges = Vec::new();
        let a = self.0[i].clone();
        let b = clone_rev(&a);
        edges.push(a);
        edges.push(b);
        edges
    }
}

fn clone_rev(arr : &Row) -> Row {
    let mut b = arr.clone();
    for i in 0..10 {
        b[i] = arr[9-i];
    }
    b
}

type Row = Vec<u8>;

fn _day20(input : &str) -> usize { 
    let tiles = input.split("\n\n")
        .map(|p| {
            let mut iter = p.trim().lines();
            let key = iter.next().unwrap().split_whitespace().skip(1).take(1).next().unwrap()[0..4].parse::<usize>().unwrap();
            let mut tile = vec![vec![0u8; 10]; 10];
            for (i,j) in iter.enumerate().flat_map(|(y, l)| { 
                l.chars().enumerate().filter(|(_, c)| c == &'#').map(move |(x,_)| (x, y)) 
            }) {
                tile[j][i] = 1;
            }
            (key, Tile(tile))
        }).collect::<HashMap<_,_>>();

    fn any_match(a : &[Row], b : &[Row]) -> Option<Edge> {
        for i in 0..a.len() {
            for j in 0..b.len() {
                if a[i] == b[j] {
                    return Some(Edge::from(i));
                }
            }
        }
        None
    }

    fn any_match2(a : &[Row], b : &[Row]) -> Option<(Edge, Edge)> {
        for i in 0..a.len() {
            for j in 0..b.len() {
                if a[i] == b[j] {
                    return Some((Edge::from(i), Edge::from(j)));
                }
            }
        }
        None
    }
    let edge_map = tiles.iter().map(|(k, v)| (*k, v.all_possible_edges())).collect::<HashMap<_,_>>();

    let _dim = if edge_map.len() < 12 { 3 } else { 12 };

    let (tl_key, tl_ns, neighbours) = {
        let mut r = None;
        let mut neighbours : HashMap<usize, Vec<usize>> = HashMap::new();
        for (key, e) in edge_map.iter() {
            let keyclone = *key;
            let ns = edge_map.iter().filter_map(|(k, edges)| {
                if *k == keyclone {
                    return None;
                }
                let mtch = any_match(e, edges);
                if let Some(edg) = mtch {
                    if !neighbours.contains_key(&keyclone) {
                        neighbours.insert(*key, Vec::new());
                    }
                    neighbours.get_mut(&key).unwrap().push(*k);
                    Some((key, edg))
                } else {
                    None
                }
            }).collect::<Vec<_>>();
            if ns.len() == 2 {
                println!("MTCH:  {}: {:?}", key, ns);
                r = Some((key, ns));
            }
        }
        println!("{:?}", r);
        let (tla, tlb) = r.unwrap();
        (tla, tlb, neighbours)
    };

    let mut visited = HashSet::new();
    visited.insert(tl_key);
    let mut tile = tiles.get(tl_key).unwrap().clone();
    for (_, edg)  in tl_ns {
        match edg {
            Edge::Top => tile = tile.flip_y(),
            Edge::Left => tile = tile.flip_x(),
            _ => {}
        }
    }
    let mut proper_flip = vec![vec![(*tl_key, tile)]];
    for i in 1.._dim {
        let v = &mut proper_flip[0];
        let (pkey, ptile) = &v[i-1];
        let pedges = ptile.all_possible_edges();
        'outer: for k in neighbours.get(pkey).unwrap() {
            let mut tile = tiles.get(k).unwrap().clone();
            if visited.contains(k) {
                continue;
            }
            loop {
                let edges = tile.all_possible_edges();
                let m = any_match2(&pedges, &edges).unwrap();
                match m {
                    (Edge::Bottom, _) => continue 'outer,
                    (Edge::Right, Edge::Left) => {}
                    (Edge::Right, Edge::Right) => tile = tile.flip_x(),
                    (Edge::Right, Edge::RightRev) => {
                        tile = tile.flip_y();
                        tile = tile.flip_x();
                    }
                    (Edge::Right, Edge::LeftRev) => tile = tile.flip_y(),
                    (Edge::Right, _) => {
                        tile = tile.rotate();
                        continue;
                    },
                    _ => panic!()
                };
                break;
            }
            // let edges = tile.all_possible_edges();
            // let m = any_match2(&pedges, &edges).unwrap();
            // println!("HUH ASSERT {:?} {}", m, k);
            v.push((*k, tile));
            visited.insert(k);
        }
    }
    for i in 1.._dim {
        proper_flip.push(Vec::new());
        // let current_row = &mut proper_flip[i];
        for x in 0.._dim {
            let (pkey, ptile) = {
                let (a, b) = &proper_flip[i-1][x];
                (a, b.clone())
            };
            let pedges = ptile.all_possible_edges();
            for k in neighbours.get(pkey).unwrap() {
                let mut tile = tiles.get(k).unwrap().clone();
                if visited.contains(k) {
                    continue;
                }
                loop {
                    let edges = tile.all_possible_edges();
                    let m = any_match2(&pedges, &edges).unwrap();
                    match m {
                        (Edge::Bottom, Edge::Top) => {},
                        (Edge::Bottom, Edge::TopRev) => tile = tile.flip_x(),
                        (Edge::Bottom, Edge::Bottom) => tile = tile.flip_y(),
                        (Edge::Bottom, Edge::BottomRev) => {
                            tile = tile.flip_x();
                            tile = tile.flip_y();
                        },
                        (Edge::Bottom, _) => {
                            tile = tile.rotate();
                            continue;
                        },
                        _ => panic!()
                    };
                    break;
                }
                // let edges = tile.all_possible_edges();
                // let m = any_match2(&pedges, &edges).unwrap();
                // println!("HUH ASSERT {:?} {}", m, k);
                proper_flip[i].push((*k, tile));
                visited.insert(k);
            }
        }
    }

    let cutted = proper_flip.iter().map(|v| v.iter().map(|(_, t)| t.inside()).collect::<Vec<_>>()).collect::<Vec<_>>();
    let mut combined = Vec::new();
    let tilesize = cutted[0][0].0.len();

    for j in 0.._dim*tilesize {
        let mut row = Vec::new();
        for i in 0.._dim*tilesize {
            let tile = &cutted[j/tilesize][i/tilesize];
            let v = tile.0[j%tilesize][i%tilesize];
            row.push(v);
            print!("{}", if v == 1 { '#' } else { '.' });
        }
        println!("");
        combined.push(row);
    }

    let pattern = vec![
        vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0,],
        vec![1,0,0,0,0,1,1,0,0,0,0,1,1,0,0,0,0,1,1,1,],
        vec![0,1,0,0,1,0,0,1,0,0,1,0,0,1,0,0,1,0,0,0,]
    ];

    let mut tile = Tile(combined);
    let mut s;
    let mut i = 0;
    let mut b = false;
    loop {
        s = tile.pattern_search(&pattern);
        println!("{}", i);
        if s != 0 {
            let mut sum = 0;
            for j in 0.._dim*tilesize {
                for i in 0.._dim*tilesize {
                    let tile = &tile;
                    let v = tile.0[j][i];
                    sum += v as usize;
                    print!("{}", if v == 1 { '#' } else { '.' });
                }
                println!("");
            }
            s = sum - s*pattern.iter().map(|p| p.iter().sum::<u8>() as usize).sum::<usize>();
            break;
        }
        match i {
            0 => tile = tile.flip_y(),
            1 => tile = tile.flip_x(),
            2 => tile = tile.flip_y(),
            3 => {
                if b {
                    panic!()
                }
                tile = tile.rotate();
                i = -1;
                b = true;
            }
            _ => panic!(),
        }
        for j in 0.._dim*tilesize {
            for i in 0.._dim*tilesize {
                let tile = &tile;
                let v = tile.0[j][i];
                print!("{}", if v == 1 { '#' } else { '.' });
            }
            println!("");
        }
        i += 1;
    }
    // for i in 1.._dim {
    //     for j in 1.._dim {
    //         let (_, cur) = &proper_flip[i][j];
    //         let (_, up) = &proper_flip[i-1][j];
    //         let (_, left) = &proper_flip[i][j-1];
    //         let edges = cur.all_possible_edges();
    //         let edges_up = up.all_possible_edges();
    //         let edges_left = left.all_possible_edges();
    //         let m = any_match2(&edges, &edges_up).unwrap();
    //         println!("HUH ASSERT up {:?} ", m,);
    //         let m = any_match2(&edges, &edges_left).unwrap();
    //         println!("HUH ASSERT left {:?} ", m,);
    //     }
    // }
    s
} 

// fn _day20_0(input : &str) -> usize { 
//     let tiles = input.split("\n\n")
//         .map(|p| {
//             let mut iter = p.trim().lines();
//             let key = iter.next().unwrap().split_whitespace().skip(1).take(1).next().unwrap()[0..4].parse::<usize>().unwrap();
//             let mut tile = [[0u8; 10]; 10];
//             for (i,j) in iter.enumerate().flat_map(|(y, l)| { 
//                 l.chars().enumerate().filter(|(_, c)| c == &'#').map(move |(x,_)| (x, y)) 
//             }) {
//                 tile[j][i] = 1;
//             }
//             (key, Tile(tile))
//         }).collect::<HashMap<_,_>>();
// 
//     fn any_match(a : &[Row], b : &[Row]) -> Option<(usize,usize)> {
//         for i in 0..a.len() {
//             for j in 0..b.len() {
//                 if a[i] == b[j] {
//                     return Some((i, j));
//                 }
//             }
//         }
//         None
//     }
//     let edge_map = tiles.iter().map(|(k, v)| (k, v.all_possible_edges())).collect::<HashMap<_,_>>();
// 
//     let mut vres = 1;
//     for (key, e) in edge_map.iter() {
//         let ns = edge_map.iter().filter_map(|(k, edges)| {
//             if k != key && any_match(e, edges).is_some() {
//                 Some(k)
//             } else {
//                 None
//             }
//         }).collect::<Vec<_>>();
//         if ns.len() == 2 {
//             vres *= *key;
//         }
// 
//         println!("{}: {:?}", key, ns);
//     }
// 
//     vres
// } 

fn _day19(input : &str) -> usize { 
    #[derive(Debug)]
    enum Match {
        Char(char),
        Ref(usize),
        Conc(Vec<usize>)
    }
    type RuleMap = HashMap<usize, Vec<Match>>;

    let mut ps = input.split("\n\n");
    let rule_map = ps.next().unwrap().trim().lines().map(|l| {
        let mut kr = l.split(": ");
        let key = kr.next().unwrap().parse::<usize>().unwrap();
        let rule_str = match key {
            // 8 => "42 | 42 8",
            // 11 => "42 31 | 42 11 31",
            // 0 => "42 42 31",
            _ => kr.next().unwrap(),
        };
        let rules = rule_str.split(" | ").map(|st| {
            let s = st.as_bytes();
            if s[0] == b'"' {
                Match::Char(s[1] as char)
            } else {
                println!("{}", st);
                let nums = st.split(" ").map(|s| s.parse::<usize>().unwrap()).collect::<Vec<_>>();
                if nums.len() > 1 {
                    Match::Conc(nums)
                } else {
                    Match::Ref(nums[0])
                }
            }
        }).collect::<Vec<_>>();
        (key, rules)
    }).collect::<HashMap<_,_>>();


    fn _match_single<'a>(rule_map : &RuleMap, substr : &'a[u8], rule : &Match) -> Option<&'a[u8]>{
        if 0 == substr.len() {
            return None;
        }
        match rule {
            Match::Char(c) => if substr[0] as char == *c { Some(&substr[1..]) } else { None },
            Match::Ref(a) => { 
                let r1 = rule_map.get(&a).unwrap();
                _match(rule_map, substr, r1)
            }
            Match::Conc(vec) => {
                let mut res = None;
                let mut s = substr;
                for i in vec.iter() {
                    // if i == &8 || i == &11 {
                    //     println!("{}: {}", i, std::str::from_utf8(s).unwrap())
                    // }
                    let rule = rule_map.get(&i).unwrap();
                    res = _match(rule_map, s, rule);
                    if let Some(ns) = res {
                        s = ns;
                    } else {
                        return None
                    }
                }
                res
            }
        }
    }

    let r42 = rule_map.get(&42).unwrap();
    let r31 = rule_map.get(&31).unwrap();

    fn _match<'a>(rule_map : &RuleMap, substr : &'a [u8], rules : &[Match]) -> Option<&'a[u8]> {
        rules.iter().map(|r| {
            _match_single(rule_map, substr, r)
        })
        .find(|r| r.is_some()).unwrap_or(None)
    }
    ps.next().unwrap().lines().enumerate().filter(|(_, l)| {
        let mut n42 = 0;
        let mut n31 = 0;
        let mut s = l.as_bytes();

        while s.len() > 0 {
            if let Some(res) = _match(&rule_map, &s, &r42) {
                n42 += 1;
                s = res;
            } else {
                break;
            }
        }
        while s.len() > 0 {
            if let Some(res) = _match(&rule_map, &s, &r31) {
                s = res;
                n31 += 1;
            } else {
                return false;
            }
        }
        println!("{:?} {}: {}", l,  n42, n31);
        n42 > n31 && n31 > 0
    }).count()
}

fn _day19_0(input : &str) -> usize { 
    #[derive(Debug)]
    enum Match {
        Char(char),
        Ref(usize),
        Conc(usize, usize)
    }
    type RuleMap = HashMap<usize, Vec<Match>>;

    let mut ps = input.split("\n\n");
    let rule_map = ps.next().unwrap().trim().lines().map(|l| {
        let mut kr = l.split(": ");
        let key = kr.next().unwrap().parse::<usize>().unwrap();
        let rules = kr.next().unwrap().split(" | ").map(|st| {
            let s = st.as_bytes();
            if s[0] == b'"' {
                Match::Char(s[1] as char)
            } else {
                let nums = st.split(" ").map(|s| s.parse::<usize>().unwrap()).collect::<Vec<_>>();
                if nums.len() > 1 {
                    Match::Conc(nums[0], nums[1])
                } else {
                    Match::Ref(nums[0])
                }
            }
        }).collect::<Vec<_>>();
        (key, rules)
    }).collect::<HashMap<_,_>>();

    let zero = rule_map.get(&0).unwrap();

    fn _match_single<'a>(rule_map : &RuleMap, substr : &'a[u8], rule : &Match) -> Option<&'a[u8]>{
        if 0 == substr.len() {
            return None;
        }
        match rule {
            Match::Char(c) => if substr[0] as char == *c { Some(&substr[1..]) } else { None },
            Match::Ref(a) => { 
                let r1 = rule_map.get(&a).unwrap();
                _match(rule_map, substr, r1)
            }
            Match::Conc(a,b) => {
                let r1 = rule_map.get(&a).unwrap();
                if let Some(i) =  _match(rule_map, substr, r1) {
                    let r2 = rule_map.get(&b).unwrap();
                    if let Some(i) =  _match(rule_map, i, r2) {
                        return Some(i)
                    }
                } 
                None
            }
        }
    }
    fn _match<'a>(rule_map : &RuleMap, substr : &'a [u8], rules : &[Match]) -> Option<&'a[u8]> {
        rules.iter().map(|r| {
            _match_single(rule_map, substr, r)
        })
        .find(|r| r.is_some()).unwrap_or(None)
    }
    ps.next().unwrap().lines().enumerate().filter(|(i, l)| {
        let res = _match(&rule_map, &l.as_bytes(), &zero);
        let res = res.map(|s| s.len() == 0).unwrap_or(false);
        println!("{:?} {}: {}", l,  i, res);
        res
    }).count()
}

fn _day18(input : &str) -> usize { 
    fn parse(slice : &[u8], i : &mut usize) -> usize {
        let mut stack = Vec::new();
        let mut op = None;
        
        loop {
            if *i >= slice.len() {
                break;
            }
            let c = slice[*i] as char;
            *i+=1;
            let mut pushed = false;
            match  c {
                '(' => {
                    pushed = true;
                    stack.push(parse(&slice, i));
                },
                ')' => {
                    return stack.iter().product();
                }
                '+' => op = Some('+'),
                '*' => op = Some('*'),
                d if ('0'..='9').contains(&d) => {
                    pushed = true;
                    stack.push(d.to_digit(10).unwrap() as usize)
                },
                _ => {},
            }
            if pushed && stack.len() >= 2 && op == Some('+') {
                let res = match op.unwrap() {
                    '*' => stack.pop().unwrap() * stack.pop().unwrap(),
                    '+' => stack.pop().unwrap() + stack.pop().unwrap(),
                    _ => panic!()
                };
                op = None;
                stack.push(res);
            };
        }
        stack.iter().product()
    }

    input.lines().map(|l| {
        let mut i = 0;
        let res = parse(l.as_bytes(), &mut i);
        res
    }).sum::<usize>()
}

fn _day18_0(input : &str) -> usize { 
    fn parse(slice : &[u8], i : &mut usize) -> usize {
        let mut stack = Vec::new();
        let mut op = None;
        
        loop {
            if *i >= slice.len() {
                break;
            }
            let c = slice[*i] as char;
            *i+=1;
            match  c {
                '(' => {
                    stack.push(parse(&slice, i));
                },
                ')' => {
                    return stack[0];
                }
                '+' => op = Some('+'),
                '*' => op = Some('*'),
                d if ('0'..='9').contains(&d) => stack.push(d.to_digit(10).unwrap() as usize),
                _ => {},
            }
            if stack.len() == 2 {
                let res = match op.unwrap() {
                    '*' => stack.pop().unwrap() * stack.pop().unwrap(),
                    '+' => stack.pop().unwrap() + stack.pop().unwrap(),
                    _ => panic!()
                };
                stack.push(res);
            };
        }
        stack[0]
    }

    input.lines().map(|l| {
        let mut i = 0;
        // println!("{}",  l.as_bytes().len());
        let res = parse(l.as_bytes(), &mut i);
        println!("");
        res
    }).sum::<usize>()
}

fn _day172(input : &str) -> usize { 
    type Grid = HashMap<i64, HashMap<i64, HashMap<i64, HashSet<i64>>>>;
    type Coord = (i64, i64, i64, i64);
    let mut grid : Grid = HashMap::new();

    fn insert(grid : &mut Grid, t : Coord) {
        let (x, y, z, w) = t;
        if let Some(sy) = grid.get_mut(&x) {
            if let Some(sz) = sy.get_mut(&y) {
                if let Some(sw) = sz.get_mut(&z) {
                    sw.insert(w);
                } else {
                    let mut sw = HashSet::new();
                    sw.insert(w);
                    sz.insert(z, sw);
                }
            } else {
                let mut sw = HashSet::new();
                sw.insert(w);
                let mut sz = HashMap::new();
                sz.insert(z, sw);
                sy.insert(y, sz);
            }
        } else {
            let mut sw = HashSet::new();
            sw.insert(w);
            let mut sz = HashMap::new();
            sz.insert(z, sw);
            let mut sy = HashMap::new();
            sy.insert(y, sz);
            grid.insert(x, sy);
        }
    }

    fn remove(grid : &mut Grid, t : Coord) {
        let (x, y, z, w ) = t;
        if let Some(sy) = grid.get_mut(&x) {
            if let Some(sz) = sy.get_mut(&y) {
                if let Some(sw) = sz.get_mut(&z) {
                    sw.remove(&w);
                }
            }
        }
    }

    fn get(grid : &Grid, t : Coord) -> bool {
        let (x, y, z, w) = t;
        if let Some(sy) = grid.get(&x) {
            if let Some(sz) = sy.get(&y) {
                if let Some(sw) = sz.get(&z) {
                    return sw.contains(&w);
                }
            }
        }
        false
    }

    fn neighbours(grid : &Grid, t : Coord) -> usize {
        let (x,y,z, w) = t;
        let mut sum = 0;
        for i in x-1..=x+1 {
            for j in y-1..=y+1 {
                for k in z-1..=z+1 {
                    for l in w-1..=w+1 {
                        if i == x && j == y && k == z && l == w{
                            continue;
                        }
                        if let Some(sy) = grid.get(&i) {
                            if let Some(sz) = sy.get(&j) {
                                if let Some(sw) = sz.get(&k) {
                                    if sw.contains(&l) {
                                        sum += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        sum
    }

    for (ix, iy) in input.lines().enumerate().flat_map(|(i, l)| {
        l.chars().enumerate().filter_map(move |(j,c)| if c == '#' { Some((j as i64, i as i64)) } else { None } )
    }) {
        insert(&mut grid, (ix, iy, 0, 0));
    }
    let mut next = grid.clone();

    for _ in 0..6 {
        for (x, yaxis) in grid.iter() {
            for (y, zaxis) in yaxis.iter() {
                for (z, waxis) in zaxis.iter() {
                    for w in waxis.iter() {
                        for i in x-1..=x+1 {
                            for j in y-1..=y+1 {
                                for k in z-1..=z+1 {
                                    for l in w-1..=w+1 {
                                        match (get(&grid, (i,j,k, l)), neighbours(&grid, (i,j,k, l))) {
                                            (true, a) if !(a == 2 || a == 3) => remove(&mut next, (i,j,k, l)),
                                            (false, 3) => insert(&mut next, (i,j,k, l)),
                                            _ => {}
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        grid = next;
        next = grid.clone();
    }

    let _sum = grid.values().fold(0, |acc, map| {
        acc + map.values().fold(0, |acc, map| acc + map.values().fold(0, |acc, set| acc + set.len()))
    });
    _sum
}

fn _day17(input : &str) -> usize { 
    let mut grid : HashMap<i64, HashMap<i64, HashSet<i64>>> = HashMap::new();

    fn insert(grid : &mut HashMap<i64, HashMap<i64, HashSet<i64>>>, t : (i64, i64, i64)) {
        let (x, y, z) = t;
        if let Some(sy) = grid.get_mut(&x) {
            if let Some(sz) = sy.get_mut(&y) {
                sz.insert(z);
            } else {
                let mut sz = HashSet::new();
                sz.insert(z);
                sy.insert(y, sz);
            }
        } else {
            let mut sz = HashSet::new();
            sz.insert(z);
            let mut sy = HashMap::new();
            sy.insert(y, sz);
            grid.insert(x, sy);
        }
    }

    fn remove(grid : &mut HashMap<i64, HashMap<i64, HashSet<i64>>>, t : (i64, i64, i64)) {
        let (x, y, z) = t;
        if let Some(sy) = grid.get_mut(&x) {
            if let Some(sz) = sy.get_mut(&y) {
                sz.remove(&z);
            }
        }
    }

    fn get(grid : &HashMap<i64, HashMap<i64, HashSet<i64>>>, t : (i64, i64, i64)) -> bool {
        let (x, y, z) = t;
        if let Some(sy) = grid.get(&x) {
            if let Some(sz) = sy.get(&y) {
                return sz.contains(&z);
            }
        }
        false
    }

    fn neighbours(grid : &HashMap<i64, HashMap<i64, HashSet<i64>>>, t : (i64, i64, i64)) -> usize {
        let (x,y,z) = t;
        let mut sum = 0;
        for i in x-1..=x+1 {
            for j in y-1..=y+1 {
                for k in z-1..=z+1 {
                    if i == x && j == y && k == z {
                        continue;
                    }
                    if let Some(sy) = grid.get(&i) {
                        if let Some(sz) = sy.get(&j) {
                            if sz.contains(&k) {
                                sum += 1;
                            }
                        }
                    }
                }
            }
        }
        sum
    }

    for (ix, iy) in input.lines().enumerate().flat_map(|(i, l)| {
        l.chars().enumerate().filter_map(move |(j,c)| if c == '#' { Some((j as i64, i as i64)) } else { None } )
    }) {
        insert(&mut grid, (ix, iy, 0));
    }
    let mut next = grid.clone();

    for i in 0..6 {
        println!("RUN {}", i);
        let mut v = Vec::new();
        for (x, yaxis) in grid.iter() {
            for (y, zaxis) in yaxis.iter() {
                for z in zaxis.iter() {
                    v.push((x, y, z));
                }
            }
        }
        v.sort_by(|a, b| a.2.cmp(b.2));
        for (x, y, z) in v {
            println!("{} {} {}", x, y, z);
        }
        println!("---------");
        for (x, yaxis) in grid.iter() {
            for (y, zaxis) in yaxis.iter() {
                for z in zaxis.iter() {
                    for i in x-1..=x+1 {
                        for j in y-1..=y+1 {
                            for k in z-1..=z+1 {
                                match (get(&grid, (i,j,k)), neighbours(&grid, (i,j,k))) {
                                    (true, a) if !(a == 2 || a == 3) => remove(&mut next, (i,j,k)),
                                    (false, 3) => insert(&mut next, (i,j,k)),
                                    _ => {}
                                }
                            }
                        }
                    }
                }
            }
        }
        grid = next;
        next = grid.clone();
        let _sum = grid.values().fold(0, |acc, map| {
            acc + map.values().fold(0, |acc, set| acc + set.len())
        });
        println!("{}", _sum);
    }

    grid.values().fold(0, |acc, map| {
        acc + map.values().fold(0, |acc, set| acc + set.len())
    })
}

fn _day16(input : &str) -> usize { 
    let mut paragraphs = input.split("\n\n");
    
    let ranges_str = paragraphs.next().unwrap();
    let old_ranges = ranges_str
        .trim().lines().flat_map(|l| {
            l.split_whitespace()
             .filter(|s| { 
                 s.chars().any(|c| c.is_digit(10)) 
             })
             .map(|s| {
                 let mut it = s.split("-") .map(|num_string| num_string.parse::<usize>().unwrap());
                 (it.next().unwrap())..=(it.next().unwrap())
             })
        }).collect::<Vec<_>>();
    let ranges = ranges_str
        .trim().lines().map(|l| {
            let mut it = l.split(":");
            let key = it.next().unwrap();
            let ranges = it.next().unwrap()
             .split_whitespace()
             .filter(|s| { 
                 s.chars().any(|c| c.is_digit(10)) 
             })
             .map(|s| {
                 let mut it = s.split("-") .map(|num_string| num_string.parse::<usize>().unwrap());
                 (it.next().unwrap())..=(it.next().unwrap())
             }).collect::<Vec<_>>();
             (key, ranges)
        }).collect::<HashMap<_,_>>();
    
    let my_ticket = paragraphs.next().unwrap().trim().lines().nth(1).unwrap().split(",").map(|s| s.parse::<usize>().unwrap()).collect::<Vec<_>>();
    let mut maps = Vec::new();
    for _ in 0..my_ticket.len() {
        maps.push(ranges.clone());
    }

    let valid_tickets = paragraphs.next().unwrap().trim().lines().skip(1).filter_map(|l| {
        let iter = l.split(",").map(|s| s.parse::<usize>().unwrap());
        if iter.clone().all(|n : usize| old_ranges.iter().any(|r| r.contains(&n))) {
            Some(iter.collect::<Vec<_>>())
        } else {
            None 
        }
    }).collect::<Vec<_>>();

    println!("{:?}", valid_tickets);

    for i in 0..my_ticket.len() {
        let set = &mut maps[i];
        for ticket in valid_tickets.iter() {
            let n = ticket[i];
            let impossible_keys = set.iter().filter_map(|(k, ranges)| {
                if ranges.iter().any(|r| r.contains(&n)) {
                    None
                } else {
                    Some(*k)
                }
            }).collect::<Vec<_>>();
            println!(" {} {} : {:?} ", i, n, impossible_keys);
            for k in impossible_keys {
                set.remove(k);
            }
        }
    }

    let mut res = Vec::new();
    loop {
        let opt = maps.iter().enumerate().find_map(|(i, map)| if map.len() == 1 { Some((i, *map.iter().next().unwrap().0)) } else { None } );
        if let Some((i, key)) = opt {
            res.push((i, key));
            for map in maps.iter_mut() {
                map.remove(key);
            };
        } else {
            return res.iter().filter_map(|(i, k)| if &k.as_bytes()[0..2] == b"de" {Some(my_ticket[*i])} else {None}).product();
        }
    }
}

fn _day16_0(input : &str) -> usize { 
    let mut paragraphs = input.split("\n\n");
    
    let ranges = paragraphs
        .next()
        .unwrap()
        .trim().lines().flat_map(|l| {
            l.split_whitespace()
             .filter(|s| { 
                 s.chars().any(|c| c.is_digit(10)) 
             })
             .map(|s| {
                 let mut it = s.split("-") .map(|num_string| num_string.parse::<usize>().unwrap());
                 (it.next().unwrap())..(it.next().unwrap())
             })
        }).collect::<Vec<_>>();
    
    let _my_ticket = paragraphs.next();

    paragraphs.next().unwrap().trim().lines().skip(1).map(|l| {
        l.split(",").map(|s| s.parse::<usize>().unwrap()).filter(|n : &usize| !ranges.iter().any(|r| r.contains(n))).sum::<usize>()
    }).sum()
}

fn _day15(input : &str) -> usize {
    let mut prev = 0;
    let mut map = input.split(",").enumerate().map(|(i, s)| {
        let n = s.parse::<usize>().unwrap();
        prev = n;
        (n, i+1)
    }).collect::<HashMap<_,_>>();
    let _ = map.remove(&prev);
    let iterations = 30000000;
    for i in map.len()+2..=iterations {
        let next = if let Some(n) = map.get(&prev) {
            (i-1) - n
        } else {
            0
        };
        map.insert(prev, i-1);
        prev = next;
        if i == iterations {
            return prev;
        }
    }
    panic!()
}

fn _day14(input : &str) -> usize {
    fn parse_mask(line : &str) -> Vec<(usize, char)> {
        line
        .split_whitespace()
        .skip(2)
        .next().unwrap()
        .chars()
        .rev()
        .enumerate()
        .collect::<Vec<_>>()
    }
    fn parse_write(l : &str) -> (usize, usize)  {
        let mut it = l.split_whitespace();
        let address = it.next().unwrap().chars().skip(4).take_while(|c| c.is_digit(10)).collect::<String>().parse::<usize>().unwrap();
        let val = it.skip(1).next().unwrap().parse::<usize>().unwrap();
        (address,val)
    }

    fn combinations(res : &mut Vec<usize>, mask : &Vec<usize>, n : usize) {
        let mut temp = Vec::new();
        for i in res.iter() {
            temp.push(i + (1 << mask[n]));
        }
        res.extend(temp);
        if n + 1 < mask.len() {
            combinations(res, mask, n+1);
        }
    }

    let mut memory  = HashMap::new();
    let mut mask = Vec::new();
    for l in input.lines()
    {
        if l.as_bytes()[1] == b'a' {
            mask = parse_mask(l);
        } else {
            let (mut masked, r) = parse_write(l);
            for i in mask.iter().filter_map(|&(i,c)| if c == 'X' { Some(i) } else { None }) {
                masked = masked & !(1 << i);
            }
            for i in mask.iter().filter_map(|&(i,c)| if c == '1' { Some(i) } else { None }) {
                masked |= 1 << i;
            }
            let xes = mask.iter().filter_map(|&(i,c)| if c == 'X' { Some(i) } else { None }).collect::<Vec<_>>();
            let mut addresses = vec![masked];
            combinations(&mut addresses, &xes, 0);
            for a in addresses { 
                memory.insert(a, r);
            }
        }
    }
    memory.values().sum()
}

fn _day14_0(input : &str) -> usize {
    fn parse_mask(line : &str) -> Vec<(usize, usize)> {
        line
        .split_whitespace()
        .skip(2)
        .next().unwrap()
        .chars()
        .rev()
        .enumerate()
        .filter(|(_,c)| *c != 'X')
        .map(|(i, c)| (i, c.to_digit(10).unwrap() as usize))
        .collect::<Vec<_>>()
    }
    fn parse_write(l : &str, overwrites : &Vec<(usize, usize)>) -> (usize, usize)  {
        let mut it = l.split_whitespace();
        let address = it.next().unwrap().chars().skip(4).take_while(|c| c.is_digit(10)).collect::<String>().parse::<usize>().unwrap();
        let val = it.skip(1).next().unwrap().parse::<usize>().unwrap();
        // (address, val)
        let mut r = val;
        for &(i, o) in overwrites {
            if o == 0 {
                r = r & !(1 << i);
            } else {
                r |= 1 << i;
            }
        }
        (address,r)
    }
    let mut memory  = HashMap::new();
    let mut mask = Vec::new();
    for l in input.lines()
    {
        if l.as_bytes()[1] as char == 'a' {
            mask = parse_mask(l);
        } else {
            let (a, r) = parse_write(l, &mask);
            memory.insert(a, r);
        }
    }
    memory.values().sum()
}


fn _steps_needed(n : usize, p_a : usize, p_b : usize) -> usize {
    let step_size = p_a % p_b;
    let current_step = n % p_b;
    if current_step == 0 {
        return 0;
    }
    let _steps_needed = if current_step == 0 { 0} else {step_size * p_b % current_step};
    let _need2 = (p_b-current_step)/step_size;
    let mut brute = current_step;
    let mut brute_need = 0;
    while brute % p_b != 0 {
        brute += step_size;
        brute_need += 1;
    }
    // println!("N {}, Pa {}, Pb {}", n, p_a, p_b);
    // println!("ss {}, cur {}, ned {} ned2 {}", step_size, current_step, steps_needed, need2);
    // println!("");
    // println!("");
    brute_need
}

fn _day13(input : &str) -> usize {
    let ids = input.lines().next().unwrap().split(",").enumerate().filter(|(_, id)| *id != "x").map(|(n, i)| (n as usize,i.parse::<usize>().unwrap())).collect::<Vec<_>>();
    println!("\t{:?}", ids);
    // let (_, step) = ids[0];
    let (s, step) = ids.iter().max_by(|(_, a), (_,b)| a.cmp(b)).unwrap();
    let mut current_timestamp = step-s ;//step - s;
    let mut cnt = 1;
    loop {
    // for _ in 0..100 {
        let prev = current_timestamp.clone();
        for f in ids.iter().filter(|(_, id)| id != step).map(|(i, id)| _steps_needed(prev + i, *step, *id)) {
            current_timestamp += f * step;
        }
        if prev == current_timestamp {
            return current_timestamp;
        }
        // current_timestamp += f * step;
        //                         100_000_000_000_000
        //                          16_500_000_033_654
        //                         100_000_000_000_000
        if current_timestamp > cnt * 100_000_000_000 {
            println!("{}", cnt);
            cnt += 1;
        }
    }
}
fn _day13_2(input : &str) -> usize {
    let ids = input.lines().next().unwrap().split(",").enumerate().filter(|(_, id)| *id != "x").map(|(n, i)| (n as usize,i.parse::<usize>().unwrap())).collect::<Vec<_>>();
    println!("\t{:?}", ids);
    // let (_, step) = ids[0];
    let (s, step) = ids.iter().max_by(|(_, a), (_,b)| a.cmp(b)).unwrap();
    let mut current_timestamp = step-s ;//step - s;
    let mut cnt = 1;
    loop {
    // for _ in 0..100 {
        let f = ids.iter().filter(|(_, id)| id != step).map(|(i, id)| _steps_needed(current_timestamp + i, *step, *id)).max().unwrap();
        if f == 0 {
            return current_timestamp;
        }
        current_timestamp += f * step;
        //                         100_000_000_000_000
        //                          16_500_000_033_654
        //                         100_000_000_000_000
        if current_timestamp > cnt *   100_000_000_000 {
            let b4 = current_timestamp;
            println!("{} + f{}*{} = {}", b4, f,step, current_timestamp);
            cnt += 1;
        }
    }
}

fn _day13_0(input : &str) -> usize {
    let mut iter = input.lines();
    let n = iter.next().unwrap().parse::<usize>().unwrap();

    let calc = |a| a - (n % a);

    let ids = iter.next().unwrap().split(",").filter(|id| *id != "x").map(|i| i.parse::<usize>().unwrap()).collect::<Vec<_>>();
    let min = ids.into_iter().inspect(|i| {
        let m = n % i;
        let r = i - m;
        println!("{}: {} {}", i, m, r);
    }).min_by(|a, b| (a - (n % a)).cmp(&(b - (n % b)))).unwrap();
    min * calc(min)
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
        // let pr = next.iter().map(|l| l.iter().collect::<String>()).collect::<Vec<_>>();

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

fn _execute_program(program : &[(&str, i32)]) -> _PResult {
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
        if let _PResult::End(n) = _execute_program(&program) {
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
