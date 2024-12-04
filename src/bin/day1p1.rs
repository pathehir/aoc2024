fn main() {
    let (mut first, mut second) = include_str!("../input/day1.txt")
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

    first.sort();
    second.sort();

    let out = first
        .iter()
        .zip(second)
        .map(|(f, s)| f.abs_diff(s))
        .collect::<Vec<_>>();

    let mut total = 0;
    for diff in out {
        total += diff;
    }

    println!("{}", total);
}
