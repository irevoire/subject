use std::{
    env,
    fs::File,
    io::{prelude::BufRead, BufReader},
    num::ParseIntError,
    str::FromStr,
};

pub struct LanternFish {
    timer: usize,
}

impl LanternFish {
    pub fn new(timer: usize) -> Self {
        Self { timer }
    }

    pub fn is_ready(&self) -> bool {
        self.timer == 0
    }

    pub fn give_birth(&mut self) -> Self {
        self.timer = 8;
        Self::new(6)
    }

    pub fn decrement(&mut self) {
        self.timer -= 1;
    }
}

impl FromStr for LanternFish {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse().map(Self::new)
    }
}

fn step(lanternfishes: &mut Vec<LanternFish>) {
    let mut new_fishes = Vec::new();

    for lanternfish in lanternfishes.iter_mut() {
        if lanternfish.is_ready() {
            new_fishes.push(lanternfish.give_birth());
        } else {
            lanternfish.decrement();
        }
    }

    lanternfishes.append(&mut new_fishes);
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let f = File::open(&args[1]).unwrap();
    let f = BufReader::new(f);

    let lines: Vec<String> = f.lines().collect::<Result<_, _>>().unwrap();
    let split: Vec<&str> = lines[0].split(',').collect();
    let mut lanternfishes: Vec<LanternFish> = split
        .iter()
        .map(|x| x.parse::<LanternFish>().unwrap())
        .collect();

    for i in 0..200 {
        println!("Computed generation {i}");
        step(&mut lanternfishes);
    }

    println!("{:?}", lanternfishes.len());
}
