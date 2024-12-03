fn main() {
    let (first, second) = include_str!("day1.txt")
        .split('\n')
        .filter(|n| !n.is_empty())
        .map(|n| {
            let split = n
                .split(' ')
                .filter(|s| !s.is_empty())
                .collect::<Vec<&str>>();

            (
                split[0].parse::<i32>().unwrap(),
                split[1].parse::<i32>().unwrap(),
            )
        })
        .collect::<(Vec<_>, Vec<_>)>();

    let mut total = 0;
    for num in first {
        let mut t = 0;
        for other in second.clone() {
            if num == other {
                t += 1;
            }
        }

        total += num * t;
    }

    println!("{}", total);
}
