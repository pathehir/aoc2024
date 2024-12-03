fn main() {
    let input = include_str!("day2.txt")
        .split('\n')
        .filter(|f| !f.is_empty())
        .collect::<Vec<_>>();

    let mut safe = 0;
    'outer: for line in input {
        let mut which = None;
        for win in line
            .split(' ')
            .map(|n| n.parse().unwrap())
            .collect::<Vec<i32>>()
            .windows(2)
        {
            let cmp = win[0].cmp(&win[1]);
            let diff = win[0].abs_diff(win[1]);
            if let Some(w) = which {
                if w != cmp || !(1..=3).contains(&diff) {
                    continue 'outer;
                }
            } else if cmp.is_ne() && (1..=3).contains(&diff) {
                which = Some(cmp);
            } else {
                continue 'outer;
            }
        }
        safe += 1;
    }

    println!("{}", safe);
}
