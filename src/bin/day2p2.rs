fn main() {
    let input = include_str!("../input/day2.txt")
        .split('\n')
        .filter(|f| !f.is_empty())
        .collect::<Vec<_>>();

    let mut safe = 0;
    for line in input {
        let line = line
            .split(' ')
            .filter(|f| !f.is_empty())
            .map(|p| p.parse().unwrap())
            .collect();

        if brute_force(line) {
            safe += 1;
        }
    }

    println!("{}", safe);
}

fn brute_force(input: Vec<i32>) -> bool {
    let mut force_input = vec![input.clone()];

    for (i, _) in input.iter().enumerate() {
        let mut uhhh = input.clone();
        uhhh.remove(i);
        force_input.push(uhhh);
    }

    for i in force_input {
        if is_safe(i) {
            return true;
        }
    }

    false
}

fn is_safe(input: Vec<i32>) -> bool {
    let mut which = None;
    for win in input.windows(2) {
        let cmp = win[0].cmp(&win[1]);
        let diff = win[0].abs_diff(win[1]);
        if let Some(w) = which {
            if w != cmp || !(1..=3).contains(&diff) {
                return false;
            }
        } else if cmp.is_ne() && (1..=3).contains(&diff) {
            which = Some(cmp);
        } else {
            return false;
        }
    }

    true
}
