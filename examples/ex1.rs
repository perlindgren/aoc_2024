fn main() {
    let s = "
    3   4
    4   3
    2   5
    1   3
    3   9
    3   3";
    let s = include_str!("../ex1_input.txt");
    let s_split = s.split_whitespace();
    let mut l1 = vec![];
    let mut l2 = vec![];

    let mut left = true;
    for v in s_split {
        let q: i32 = v.parse().unwrap();
        println!("{:?}", q);
        if left {
            l1.push(q);
        } else {
            l2.push(q);
        }
        left = !left;
    }

    l1.sort();
    l2.sort();

    let r = l1
        .iter()
        .zip(l2.clone())
        .fold(0, |acc, (e1, e2)| acc + (e1 - e2).abs());
    println!("{:?} {:?}, {:?}", l1.len(), l2.len(), r);
    // println!("{:?}", s_split);
}
