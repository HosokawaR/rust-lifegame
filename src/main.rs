extern crate termsize;
use rand::Rng;
use std::{thread, time::Duration};

type World = Vec<Vec<u32>>;

fn init_world(width: &usize, height: &usize) -> World {
    let mut world: World = Vec::<Vec<u32>>::with_capacity(*height);

    let mut rng = rand::thread_rng();

    for row in 0..*height {
        world.push(Vec::with_capacity(*width));
        for _ in 0..*width {
            world[row].push(rng.gen_range(0..2))
        }
    }

    world
}

fn count_around_living(
    world: &World,
    row: usize,
    col: usize,
    width: &usize,
    height: &usize,
) -> u32 {
    let mut total_living = 0;

    for row_diff in 0..3 {
        for col_diff in 0..3 {
            let r: i32 = row as i32 + row_diff - 1;
            let c: i32 = col as i32 + col_diff - 1;
            if row_diff == 1 && col_diff == 1 {
                continue;
            }
            if r >= 0 && c >= 0 && r < *height as i32 && c < *width as i32 {
                total_living += world[r as usize][c as usize];
            };
        }
    }

    total_living
}

fn copy_world(world: &World, width: &usize, height: &usize) -> World {
    let mut dest_world: World = Vec::<Vec<u32>>::with_capacity(*height);

    for row in 0..*height {
        dest_world.push(Vec::with_capacity(*width));
        for col in 0..*width {
            dest_world[row].push(world[row][col])
        }
    }

    dest_world
}

fn advance_world(world: &mut World, width: &usize, height: &usize) {
    let world_copy = copy_world(world, width, height);

    for row in 0..*height {
        for col in 0..*width {
            let target = world_copy[row][col];
            let living_number = count_around_living(&world_copy, row, col, width, height);
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

fn display_world(world: &World, generation: u32, width: &usize, height: &usize) {
    println!("generation: {}", generation);
    for row in 0..*height {
        for col in 0..*width {
            print!(
                "{} ",
                if world[row][col] > 0 {
                    "\x1b[32m■\x1b[0m"
                } else {
                    "\x1b[30m□\x1b[0m"
                }
            );
        }
        println!("")
    }
}

fn clear(height: &usize) {
    print!("\x1b[{}A", height + 1);
    print!("\x1b[0J");
}

fn main() {
    let termsize::Size { rows, cols } = termsize::get().unwrap();
    let width = cols as usize / 2;
    let height = rows as usize - 2;

    let mut generation = 0;
    let mut world = init_world(&width, &height);

    for _ in 0..10_0000 {
        display_world(&world, generation, &width, &height);
        advance_world(&mut world, &width, &height);
        thread::sleep(Duration::from_millis(500));
        clear(&height);
        generation += 1;
    }
    display_world(&world, generation, &width, &height);
}
