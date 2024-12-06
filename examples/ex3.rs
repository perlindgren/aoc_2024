fn main() {
    //     let s = "7 6 4 2 1
    // 1 2 7 8 9
    // 9 7 6 2 1
    // 1 3 2 4 5
    // 8 6 4 4 1
    // 1 3 6 7 9";
    let s = include_str!("../ex3_input.txt");
    let s_split = s.split_terminator("\n");

    let mut safe = 0;
    let mut total = 0;
    for i in s_split {
        total += 1;
        println!("{:?}", i);

        let it: Vec<_> = i
            .split_whitespace()
            .map(|e| e.parse::<i32>().unwrap())
            .collect();

        let mut it = it.iter().peekable();

        if it.len() < 2 {
            panic!("too short");
        }

        let mut increasing = None;
        while let Some(p) = it.next() {
            if let Some(p1) = it.peek() {
                let diff = *p1 - p;
                let abs = diff.abs();

                // println!("p = {}, {}, diff {}", p, p1, diff);

                if abs < 1 || abs > 3 {
                    println!("unsafe range {}", diff);
                    break;
                }

                match increasing {
                    None => increasing = Some(diff > 0),
                    Some(v) => {
                        if v != (diff > 0) {
                            println!("unsafe dir");
                            break;
                        }
                    }
                }
            } else {
                safe += 1;
            }
        }
        println!("safe {}", safe)
    }
    println!("safe {}", safe);
    println!("total {}", total);
}
