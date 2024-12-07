use regex::Regex;
use std::collections::{HashMap, HashSet};
fn main() {
    let re = Regex::new(r"(.+)\|(.+)").unwrap();
    // let rules = "47|53
    // 97|13
    // 97|61
    // 97|47
    // 75|29
    // 61|13
    // 75|53
    // 29|13
    // 97|29
    // 53|29
    // 61|53
    // 97|53
    // 61|29
    // 47|13
    // 75|47
    // 97|75
    // 47|61
    // 75|61
    // 47|29
    // 75|13
    // 53|13";
    let rules = include_str!("../ex9_rules.txt");

    let a = rules.split_whitespace();
    let mut hm_inc: HM = HashMap::new();
    let mut hm_dec: HM = HashMap::new();
    for hay in a {
        println!("hay {}", hay);
        let it = re.captures_iter(hay);
        for caps in it {
            let (_, [l, r]) = caps.extract();
            let l: i32 = l.parse().unwrap();
            let r: i32 = r.parse().unwrap();
            println!("l {} r {}", l, r);
            if let Some(set) = hm_inc.get_mut(&l) {
                set.insert(r);
            } else {
                let mut set = HashSet::new();
                set.insert(r);
                hm_inc.insert(l, set);
            }

            if let Some(set) = hm_dec.get_mut(&r) {
                set.insert(l);
            } else {
                let mut set = HashSet::new();
                set.insert(l);
                hm_dec.insert(r, set);
            }
        }
    }

    println!("hm_inc {:?}", hm_inc);
    println!("hm_dec {:?}", hm_dec);

    //     let updates = "75,47,61,53,29
    // 97,61,53,29,13
    // 75,29,13
    // 75,97,47,61,53
    // 61,13,29
    // 97,13,75,29,47";
    let updates = include_str!("../ex9_updates.txt");
    let u: Vec<_> = updates.split(&"\n").collect();
    println!("u {:?}", u);
    let mut upd = vec![];
    for l in u {
        let up: Vec<i32> = l.split(&",").map(|s| s.parse().unwrap()).collect();
        println!("up {:?}", up);
        upd.push(up);
    }

    println!("upd {:?}", upd);
    let mut total = 0;
    let mut total_fixed = 0;

    for mut up in upd {
        println!("up {:?}", up);
        let mut oks = true;
        let mut fixed = false;
        for i in 0..up.len() - 1 {
            let cur = up[i];
            let tail = &up[i + 1..];
            // println!("cur {:?}, tail {:?}", cur, tail);
            if let Some(set) = hm_inc.get(&cur) {
                let ok = tail.iter().all(|r| set.contains(r));
                oks = oks && ok;
                //  println!("ok right inc {}, i = {}, curr = {}", ok, i, cur);
            }
            if let Some(set) = hm_dec.get(&cur) {
                let ok = !tail.iter().any(|r| set.contains(r));
                oks = oks && ok;
                // println!("ok right dec {}, i = {}, curr = {}", ok, i, cur);
            }
            if !oks {
                println!("-- broken -- {:?}", up);

                fix(&hm_dec, &mut up);
                fixed = true;
            }
        }
        let mid = up[up.len() / 2];
        // println!("oks {}, mid {}", oks, mid);
        total_fixed += if fixed { mid } else { 0 };
        total += if oks { mid } else { 0 };
    }
    println!("total {}", total);
    println!("total_fixed {}", total_fixed);

    // let mut v = vec![75, 97, 47, 61, 53];
    // fix(&hm_dec, &mut v);
    // let mut v = vec![61, 13, 29];
    // fix(&hm_dec, &mut v);
    // let mut v = vec![97, 13, 75, 29, 47];
    // fix(&hm_dec, &mut v);
}

type HM = HashMap<i32, HashSet<i32>>;
fn fix(hm_dec: &HM, up: &mut Vec<i32>) {
    loop {
        let mut cont = false;
        for i in 0..up.len() - 1 {
            let cur = up[i];
            let next = &up[i + 1];
            if let Some(set) = hm_dec.get(&cur) {
                if set.contains(next) {
                    println!("swap i {}, cur {}, next {}", i, cur, next);
                    up[i] = *next;
                    up[i + 1] = cur;
                    cont = true;
                }
            }
        }
        if !cont {
            break;
        }
    }
    println!("{:?}", up);
    //}
}

// hm_inc {
//     53: {13, 29},
//     75: {53, 61, 13, 29, 47},
//     47: {13, 29, 61, 53},
//     97: {53, 75, 47, 29, 13, 61},
//     61: {13, 53, 29},
//     29: {13}
// }
// hm_dec {
//     47: {75, 97},
//     13: {61, 75, 29, 97, 47, 53},
//     61: {97, 47, 75},
//     29: {61, 47, 75, 53, 97},
//     75: {97},
//     53: {61, 97, 47, 75}
// }
//
// 75,97,47,61,53 becomes 97,75,47,61,53.
// 75,97,... -> 97,75,...
//
// 61,13,29 becomes 61,29,13.
// ...,13,29,...-> ...,29,13,...
//
// 97,[13,75],29,47 becomes 97,75,47,29,13
// 97,75,[13,29],47
// 97,75,29,[13,47]
// 97,75,29,47,13
// 97,75,47,29,13
