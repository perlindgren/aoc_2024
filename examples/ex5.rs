use regex::Regex;
fn main() {
    // let hay = "mul(1,3) mul(2,12)";
    // let hay = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    let hay = include_str!("../ex5_input.txt");
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    let it = re.captures_iter(hay);

    let mut acc = 0;
    for caps in it {
        // println!("reg {:?}", caps);
        let (_, [a, b]) = caps.extract();
        let a: i32 = a.parse().unwrap();
        let b: i32 = b.parse().unwrap();
        println!("a {} b {}", a, b);
        acc += a * b;
    }

    println!("acc {}", acc);
}
