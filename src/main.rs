mod heuristics;

use heuristics::{inversions, manhattan, misses};
use rand::Rng;
use std::collections::HashSet;

fn possible_movements(size: usize, row: usize, col: usize) -> Vec<&'static str> {
    let mut movements: Vec<&str> = Vec::with_capacity(4);
    if row > 0 {
        movements.push("up");
    }
    if row < size - 1 {
        movements.push("down");
    }
    if col > 0 {
        movements.push("left");
    }
    if col < size - 1 {
        movements.push("right");
    }
    movements
}

fn gen_map(size: usize) -> (Vec<usize>, usize) {
    let n = size * size;
    let mut map: Vec<usize> = (0..n).collect();

    let mut rng = rand::thread_rng();
    for i in 0..n {
        let j = rng.gen_range(0..n);
        map.swap(i, j);
    }

    let mut pos = 0;
    for (i, &x) in map.iter().enumerate() {
        if x == 0 {
            pos = i;
            break;
        }
    }

    if (inversions(&map) + size + pos/size + 1) % 2 == 1 {
        if pos >= n - 2 {
            map.swap(0, 1);
        } else {
            map.swap(n - 2, n - 1);
        }
    }
    println!("{}", inversions(&map) + size + pos/size + 1);

    (map, pos)
}

fn main() {
    let size = std::env::args()
        .nth(1)
        .expect("No number given")
        .parse::<usize>()
        .expect("Enter a number!");

    if size <= 1 {
        return;
    }

    let (start, pos) = gen_map(size);
    for row in 0..size {
        for col in 0..size {
            print!("{} ", start[row * size + col]);
        }
        println!("");
    }

    let mut stack: Vec<(Vec<usize>, usize, usize)> = Vec::new();
    let mut used: HashSet<Vec<usize>> = HashSet::new();
    used.insert(start.clone());
    stack.push((start, pos, 0));

    let mut step = 0usize;
    while let Some((board, pos, depth)) = stack.pop() {
        step += 1;
        let h = manhattan(&board, size);
        //let h = misses(&board, size);
        //let h = inversions(&board);
        println!("{} {} {} {:?}", step, depth, h, board);
        if h == 0 {
            return;
        }

        let row = pos / size;
        let col = pos % size;
        let movements = possible_movements(size, row, col);

        let mut try_next: Vec<(Vec<usize>, usize, usize)> = Vec::with_capacity(movements.len());

        for mov in movements.into_iter() {
            let new_pos;
            let mut new_board = board.clone();
            match mov {
                "up" => {
                    new_board.swap(pos, (row - 1) * size + col);
                    new_pos = (row - 1) * size + col;
                }
                "down" => {
                    new_board.swap(pos, (row + 1) * size + col);
                    new_pos = (row + 1) * size + col;
                }
                "left" => {
                    new_board.swap(pos, row * size + (col - 1));
                    new_pos = row * size + (col - 1);
                }
                "right" => {
                    new_board.swap(pos, row * size + (col + 1));
                    new_pos = row * size + (col + 1);
                }
                _ => unreachable!(),
            }
            let new_h = manhattan(&new_board, size);
            //let new_h = misses(&new_board, size);
            //let new_h = inversions(&new_board);

            if !used.contains(&new_board) {
                try_next.push((new_board, new_pos, new_h));
            }
        }

        try_next.sort_by_key(|(_, _, heu)| *heu);
        for (new_board, new_pos, _) in try_next.into_iter().rev() {
            used.insert(new_board.clone());
            stack.push((new_board, new_pos, depth + 1));
        }
    }
}
