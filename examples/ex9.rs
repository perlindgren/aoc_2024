use regex::Regex;
use std::collections::{HashMap, HashSet};
fn main() {
    let re = Regex::new(r"(.+)\|(.+)").unwrap();
    let rules = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13";
    let a = rules.split_whitespace();
    let mut hm_inc: HashMap<i32, HashSet<i32>> = HashMap::new();
    let mut hm_dec: HashMap<i32, HashSet<i32>> = HashMap::new();
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

    let updates = "75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
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

    for up in upd {
        println!("up {:?}", up);
        let mut oks = true;
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
        }
        let mid = up[up.len() / 2];
        // println!("oks {}, mid {}", oks, mid);
        total += if oks { mid } else { 0 };
    }
    println!("total {}", total)
}
