use std::time::Instant;

#[derive(Debug)]
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
        self.timer = 6;
        Self::new(8)
    }

    pub fn decrement(&mut self) {
        self.timer -= 1;
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
    let iteration = std::env::args().nth(1).unwrap_or(String::from("160"));
    let iteration = iteration
        .parse::<usize>()
        .expect("Could not parse the number of iteration");

    let mut lanternfishes = vec![
        LanternFish::new(3),
        LanternFish::new(4),
        LanternFish::new(3),
        LanternFish::new(1),
        LanternFish::new(2),
    ];

    let start = Instant::now();

    for _ in 0..iteration {
        step(&mut lanternfishes);
    }

    let elapsed = start.elapsed();

    println!("Computed {iteration} generation in {elapsed:.2?}");
    println!("There is a total of {} lanternfishes.", lanternfishes.len());
}
