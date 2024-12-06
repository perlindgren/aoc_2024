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
        println!("#{} {:?}", total, i);

        let it: Vec<_> = i
            .split_whitespace()
            .map(|e| e.parse::<i32>().unwrap())
            .collect();

        if it.len() < 3 {
            panic!("too short");
        }
        'outer: for i in 0..it.clone().len() {
            let x = it.clone();
            // println!("i {}", i);
            let mut v1: Vec<_> = x.iter().take(i).collect();
            let v2: Vec<_> = x.iter().skip(i + 1).collect();

            // println!("v1 {:?}, v2 {:?}", v1, v2);
            v1.extend(v2);

            // println!("v1 ext {:?}", v1);

            let mut it = v1.iter().peekable();
            let mut increasing = None;
            while let Some(p) = it.next() {
                if let Some(p1) = it.peek() {
                    let diff = **p1 - **p;
                    let abs = diff.abs();

                    // println!("p = {}, {}, diff {}", p, p1, diff);

                    if abs < 1 || abs > 3 {
                        // println!("unsafe range {}", diff);
                        break;
                    }

                    match increasing {
                        None => increasing = Some(diff > 0),
                        Some(v) => {
                            if v != (diff > 0) {
                                // println!("unsafe dir");
                                break;
                            }
                        }
                    }
                } else {
                    safe += 1;
                    println!("#{} safe {}", total, safe);
                    break 'outer;
                }
            }
        }
    }
    println!("safe {}", safe);
    println!("total {}", total);
}
