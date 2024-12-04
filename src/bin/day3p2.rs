use nom::branch::alt;
use nom::bytes::complete::{tag, take};
use nom::character::complete::{char, i32};
use nom::combinator::map;
use nom::multi::{many0, many_till};
use nom::sequence::{delimited, preceded, separated_pair};
use nom::IResult;

fn mul(s: &str) -> IResult<&str, (i32, i32)> {
    delimited(tag("mul("), mul_args, char(')'))(s)
}

fn mul_args(s: &str) -> IResult<&str, (i32, i32)> {
    separated_pair(i32, char(','), i32)(s)
}

fn remove_garbage<'a, T>(
    parser: impl FnMut(&'a str) -> IResult<&'a str, T>,
) -> impl FnMut(&'a str) -> IResult<&'a str, T> {
    map(many_till(take(1usize), parser), |(_, output)| output)
}

fn dont(s: &str) -> IResult<&str, (i32, i32)> {
    map(
        preceded(tag("don't()"), remove_garbage(tag("do()"))),
        |_| (0, 0),
    )(s)
}

fn parse_file(s: &str) -> IResult<&str, Vec<(i32, i32)>> {
    many0(remove_garbage(alt((dont, mul))))(s)
}

fn main() {
    println!(
        "{}",
        parse_file(include_str!("../input/day3.txt"))
            .unwrap()
            .1
            .iter()
            .fold(0, |acc, (x, y)| acc + x * y)
    );
}
