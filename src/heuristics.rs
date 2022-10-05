// Эвристика по числу номеров, стоязих е на своих местах.
pub fn misses(map: &Vec<usize>, size: usize) -> usize {
    map.iter()
        .enumerate()
        .filter(|(i, num)| **num != 0 && *i != **num - 1)
        .count()
}

// Манхэттенское расстояние
pub fn manhattan(map: &Vec<usize>, size: usize) -> usize {
    let mut dist = 0;
    for (i, &num) in map.iter().enumerate() {
        if num == 0 {
            continue;
        }
        let target_row = (num - 1) / size;
        let target_col = (num - 1) % size;
        let row = i / size;
        let col = i % size;

        if target_row >= row {
            dist += target_row - row;
        } else {
            dist += row - target_row;
        }
        if target_col >= col {
            dist += target_col - col;
        } else {
            dist += col - target_col;
        }
    }

    dist
}

// Число инверсий
pub fn inversions(map: &Vec<usize>) -> usize {
    let mut count = 0;
    for i in 0..(map.len() - 1) {
        if map[i] == 0 {
            continue;
        }
        for j in (i + 1)..map.len() {
            if map[j] != 0 && map[i] > map[j] {
                count += 1;
            }
        }
    }
    count
}