use rand::Rng;
use std::{thread, time::Duration};

const WORLD_WIDTH: usize = 20;
const WORLD_HEIGHT: usize = 10;

type World = [[u32; WORLD_WIDTH]; WORLD_HEIGHT];

fn init_world() -> World {
    let mut world: World = [[0; WORLD_WIDTH]; WORLD_HEIGHT];

    let mut rng = rand::thread_rng();

    for row in 0..WORLD_HEIGHT {
        for col in 0..WORLD_WIDTH {
            world[row][col] = rng.gen_range(0..2)
        }
    }

    world
}

fn count_around_living(world: &World, row: usize, col: usize) -> u32 {
    let mut total_living = 0;

    for row_diff in 0..3 {
        for col_diff in 0..3 {
            let r: i32 = row as i32 + row_diff - 1;
            let c: i32 = col as i32 + col_diff - 1;
            if row_diff == 1 && col_diff == 1 {
                continue;
            }
            if r >= 0 && c >= 0 && r < WORLD_HEIGHT as i32 && c < WORLD_WIDTH as i32 {
                total_living += world[r as usize][c as usize];
            };
        }
    }

    total_living
}

fn copy_world(world: &World) -> World {
    let mut dest_world: World = [[0; WORLD_WIDTH]; WORLD_HEIGHT];

    for row in 0..WORLD_HEIGHT {
        for col in 0..WORLD_WIDTH {
            dest_world[row][col] = world[row][col]
        }
    }

    dest_world
}

fn advance_world(world: &mut World) {
    let world_copy = copy_world(world);

    for row in 0..WORLD_HEIGHT {
        for col in 0..WORLD_WIDTH {
            let target = world_copy[row][col];
            let living_number = count_around_living(&world_copy, row, col);
            if target > 0 {
                world[row][col] = if 2 <= living_number && living_number <= 3 {
                    1
                } else {
                    0
                };
            } else {
                world[row][col] = if living_number == 3 { 1 } else { 0 };
            }
        }
    }
}

fn display_world(world: &World, generation: u32) {
    println!("generation: {}", generation);
    for row in 0..WORLD_HEIGHT {
        for col in 0..WORLD_WIDTH {
            print!("{} ", if world[row][col] > 0 { "■" } else { "□" });
        }
        println!("")
    }
}

fn clear() {
    print!("\x1b[{}A", WORLD_HEIGHT + 1);
    print!("\x1b[0J");
}

fn main() {
    let mut generation = 0;
    let mut world = init_world();

    for _ in 0..100 {
        display_world(&world, generation);
        advance_world(&mut world);
        thread::sleep(Duration::from_secs(1));
        clear();
        generation += 1;
    }
}
