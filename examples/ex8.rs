fn main() {
    //     let s = "..X...
    // .SAMX.
    // .A..A.
    // XMAS.S
    // .X....";
    //     let s = "MMMSXXMASM
    // MSAMXMSMSA
    // AMXSXMAAMM
    // MSAMASMSMX
    // XMASAMXAMM
    // XXAMMXXAMA
    // SMSMSASXSS
    // SAXAMASAAA
    // MAMMMXMMMM
    // MXMXAXMASX";

    // .M.S
    //  .A.
    // .M.S.
    //
    let x_mas: [[[char; 3]; 3]; 4] = [
        //
        [['M', '.', 'S'], ['.', 'A', '.'], ['M', '.', 'S']],
        //
        [['S', '.', 'S'], ['.', 'A', '.'], ['M', '.', 'M']],
        //
        [['M', '.', 'M'], ['.', 'A', '.'], ['S', '.', 'S']],
        //
        [['S', '.', 'M'], ['.', 'A', '.'], ['S', '.', 'M']],
    ];

    let s = include_str!("../ex7_input.txt");
    let sv: Vec<&str> = s.split_whitespace().collect();

    let y_max = sv.len();
    let x_max = sv.get(0).unwrap().len();

    println!("x_max {}, y_max {}", x_max, y_max);

    for r in &sv {
        println!("{}", r);
    }

    let mut matches = 0;
    for x in 0..x_max {
        for y in 0..y_max {
            matches += check(&sv, x, y, &x_mas);
            // check(&sv, 0, 0, &x_mas);
        }
    }

    println!("matches {}", matches);
}

fn get(data: &Vec<&str>, x: usize, y: usize) -> Option<char> {
    (*data.get(y)?).chars().nth(x)
}

fn check(data: &Vec<&str>, x: usize, y: usize, x_mas: &[[[char; 3]; 3]; 4]) -> i32 {
    for group in x_mas {
        println!("group {:?}", group);
        let mut acc = 0;
        for (x_, line) in group.iter().enumerate() {
            for (y_, row) in line.iter().enumerate() {
                if let Some(g) = get(data, x + x_, y + y_) {
                    println!("x {} y {} g {} r {}", x + x_, y + y_, g, row);
                    if g == *row {
                        acc += 1;
                    }
                } else {
                    // return 0;
                }
            }
        }
        if acc == 5 {
            return 1;
        }
    }
    return 0;
}
