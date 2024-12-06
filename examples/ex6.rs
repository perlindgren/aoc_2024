use regex::Regex;
fn main() {
    // let hay = "mul(1,2) do() do mul(1,2) don't() mul(1,2) don't";
    // let hay = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    let hay = include_str!("../ex5_input.txt");
    let re = Regex::new(r"(?<n>don't\(\))|(?<d>do\(\))|mul\((?<a>[0-9]{1,3}),(?<b>[0-9]{1,3})\)")
        .unwrap();
    let it = re.captures_iter(hay);

    let mut acc = 0;
    let mut state = true;
    for caps in it {
        if caps.name("d").is_some() {
            println!("do");
            state = true;
        };

        if caps.name("n").is_some() {
            println!("don't");
            state = false;
        }
        if let Some(a) = caps.name("a") {
            if let Some(b) = caps.name("b") {
                let a: i32 = a.as_str().parse().unwrap();
                let b: i32 = b.as_str().parse().unwrap();
                println!("mul {}, {}", a, b);
                if state {
                    // println!("acc {}", a, b);
                    acc += a * b;
                }
            }
        }
    }

    println!("acc {}", acc);
}
