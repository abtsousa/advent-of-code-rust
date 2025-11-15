advent_of_code::solution!(7);

use std::collections::HashMap;

#[derive(Debug)]
enum Arg {
    Literal(u16),
    Identifier(String)
}

impl From<u16> for Arg {
    fn from(value: u16) -> Self {
        Arg::Literal(value)
    }
}

impl From<&str> for Arg {
    fn from(value: &str) -> Self {
        if let Ok(x) = value.parse::<u16>() {
            return Arg::Literal(x)
        }
        Arg::Identifier(String::from(value))
    }
}

#[derive(Debug)]
enum Op {
    AND(Arg, Arg),
    OR(Arg, Arg),
    LSHIFT(Arg, Arg),
    RSHIFT(Arg, Arg),
    NOT(Arg),
    UNARY(Arg)
}

impl Op {

    fn map_args<F>(&self, f: F) -> Op
    where
        F: Fn(&Arg) -> Arg,
    {
        match self {
            Op::AND(a, b) => Op::AND(f(a), f(b)),
            Op::OR(a, b) => Op::OR(f(a), f(b)),
            Op::LSHIFT(a, b) => Op::LSHIFT(f(a), f(b)),
            Op::RSHIFT(a, b) => Op::RSHIFT(f(a), f(b)),
            Op::NOT(a) => Op::NOT(f(a)),
            Op::UNARY(a) => Op::UNARY(f(a)),
        }
    }

    fn resolve(&self, env: &std::collections::HashMap<String, u16>) -> Op {
        self.map_args(|arg| match arg {
            Arg::Literal(x) => Arg::Literal(*x),
            Arg::Identifier(name) => env.get(name).cloned().map_or(Arg::Identifier(name.clone()), Arg::Literal),
        })
    }

    fn solve(&self) -> Option<u16> {
        match self {
            Op::AND(Arg::Literal(x), Arg::Literal(y)) => Some(*x & *y),
            Op::OR(Arg::Literal(x), Arg::Literal(y)) => Some(*x | *y),
            Op::LSHIFT(Arg::Literal(x), Arg::Literal(y)) => Some(*x << *y),
            Op::RSHIFT(Arg::Literal(x), Arg::Literal(y)) => Some(*x >> *y),
            Op::NOT(Arg::Literal(x)) => Some(!*x),
            Op::UNARY(Arg::Literal(x)) => Some(*x),
            _ => None,
        }
    }
}

fn parse_line(line: &str) -> Result<(Op, Arg), ()> {
    match *line.split_whitespace().collect::<Vec<&str>>().as_slice() {
        [a, "AND", b, "->", out] => Ok((Op::AND(a.into(), b.into()), out.into())),
        [a, "OR", b, "->", out] => Ok((Op::OR(a.into(), b.into()), out.into())),
        [a, "LSHIFT", b, "->", out] => Ok((Op::LSHIFT(a.into(), b.into()), out.into())),
        [a, "RSHIFT", b, "->", out] => Ok((Op::RSHIFT(a.into(), b.into()), out.into())),
        ["NOT", a, "->", out] => Ok((Op::NOT(a.into()), out.into())),
        [a, "->", out] => Ok((Op::UNARY(a.into()), out.into())),
        _ => Err(())
    }
}

pub fn part_one_solver(mut env: HashMap<String, u16>, input: &str) -> HashMap<String,u16> {
    let mut expressions = input.lines().map(|l| parse_line(l).unwrap()).collect::<Vec<(Op, Arg)>>();

    let mut debug_round = 0;

    while !env.contains_key("a") {
        debug_round += 1;
        println!("\n=== ROUND {debug_round} ===");
        println!("env so far: {:?}", env.keys().collect::<Vec<_>>());
        println!("remaining expressions: {}", expressions.len());

        let mut progress = false;

        expressions = expressions.into_iter().filter_map(|(op, arg)| {
            let op_ = op.resolve(&env);

            println!("checking: {:?} -> {:?}", op, arg);
            println!("  resolved as: {:?}", op_);

            if let Some(x) = op_.solve() && let Arg::Identifier(id) = arg {
                if !env.contains_key(id.as_str()) {
                println!("    solved {} = {}", id, x);
                env.insert(id.clone(), x);
                progress = true;
                }
                None
            } else {
                println!("    cannot solve yet");
                Some((op_, arg))
            }
        }).collect();

        if !progress {
            println!("\n!!! NO PROGRESS THIS ROUND — deadlock detected !!!");
            panic!()
        }
    };
    env
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut env = HashMap::new();
    env = part_one_solver(env, input);
    Some(env["a"] as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut env = HashMap::new();
    env = part_one_solver(env, input);
    let signal = env["a"];
    env.clear();
    env.insert("b".to_string(), signal);
    env = part_one_solver(env, input);
    Some(env["a"] as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
