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
            for x_dir in [-1, 0, 1] {
                for y_dir in [-1, 0, 1] {
                    matches += check(&sv, x, y, x_dir, y_dir);
                }
            }
        }
    }

    println!("matches {}", matches);
}

fn get(data: &Vec<&str>, x: usize, y: usize) -> Option<char> {
    (*data.get(y)?).chars().nth(x)
}

fn check(data: &Vec<&str>, mut x: usize, mut y: usize, x_inc: i32, y_inc: i32) -> i32 {
    let x_org = x;
    let y_org = y;
    for c in "XMAS".chars() {
        // println!("x {} y {} c {}", x, y, c);
        if let Some(g) = get(data, x, y) {
            //  println!("g {}", g);
            if g != c {
                return 0;
            } else {
                x = (x as i32 + x_inc) as usize;
                y = (y as i32 + y_inc) as usize;
            }
        } else {
            return 0;
        }
    }
    println!("x {}, y {}, x_inc {}, y_inc {}", x_org, y_org, x_inc, y_inc);
    1
}
