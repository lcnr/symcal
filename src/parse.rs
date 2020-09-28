use crate::{BinOp, Node, UnOp};

fn parse_num(src: &str) -> Result<(&str, u128), String> {
    // get num str
    let s = src
        .split(|c: char| !(c.is_alphanumeric() || c == '_'))
        .next()
        .unwrap();

    // strip underscores without allocating a new String unless necessary.
    let s2;
    let mut num_str = if s.chars().any(|c| c == '_') {
        s2 = s.chars().filter(|&c| c != '_').collect::<String>();
        &s2
    } else {
        s
    };

    let base = if num_str.starts_with('0') && num_str.len() > 1 {
        match num_str.as_bytes()[1] {
            b'x' => 16,
            b'o' => 8,
            b'b' => 2,
            _ => 10,
        }
    } else {
        10
    };

    if base != 10 {
        num_str = &num_str[2..];
    }

    match u128::from_str_radix(num_str, base) {
        Ok(v) => Ok((&src[s.len()..], v)),
        Err(_err) => Err(format!("unable to parse `{}` as an integer", s)),
    }
}

fn parse_ident(src: &str) -> Result<(&str, Node), String> {
    let s = src
        .split(|c: char| !(c.is_alphanumeric() || c == '_'))
        .next()
        .unwrap();
    Ok((&src[s.len()..], match s {
        "true" => Node::BoolValue(true),
        "false" => Node::BoolValue(false),
        _ => Node::Constant(s.into()),
    }))
}

pub fn parse(mut s: &str) -> Result<Node, String> {
    let mut stack = Vec::new();
    loop {
        s = s.trim();

        if s.is_empty() {
            return if stack.len() > 1 {
                Err(format!(
                    "too many remaining arguments, expected 1 found {}",
                    s.len()
                ))
            } else if let Some(v) = stack.pop() {
                Ok(v)
            } else {
                Err(format!("missing return value, potentially an empty string"))
            };
        } else if let Some(rest) = s.strip_prefix("-") {
            s = rest;
            let node = stack
                .pop()
                .ok_or_else(|| format!("missing argument for '-'"))?;
            stack.push(Node::UnaryOp(UnOp::Neg, box node));
        } else if let Some(rest) = s.strip_prefix("+") {
            s = rest;
            let r = stack
                .pop()
                .ok_or_else(|| format!("missing first argument for '+'"))?;
            let l = stack
                .pop()
                .ok_or_else(|| format!("missing second argument for '+'"))?;
            stack.push(Node::BinOp(BinOp::Add, box l, box r));
        } else if let Some(rest) = s.strip_prefix("*") {
            s = rest;
            let r = stack
                .pop()
                .ok_or_else(|| format!("missing first argument for '*'"))?;
            let l = stack
                .pop()
                .ok_or_else(|| format!("missing second argument for '*'"))?;
            stack.push(Node::BinOp(BinOp::Mul, box l, box r));
        } else if let Some(rest) = s.strip_prefix("/") {
            s = rest;
            let r = stack
                .pop()
                .ok_or_else(|| format!("missing first argument for '/'"))?;
            let l = stack
                .pop()
                .ok_or_else(|| format!("missing second argument for '/'"))?;
            stack.push(Node::BinOp(BinOp::Add, box l, box r));
        } else if let Some(rest) = s.strip_prefix("=") {
            s = rest;
            let r = stack
                .pop()
                .ok_or_else(|| format!("missing first argument for '='"))?;
            let l = stack
                .pop()
                .ok_or_else(|| format!("missing second argument for '='"))?;
            stack.push(Node::BinOp(BinOp::Eq, box l, box r));
        } else if s.chars().next().unwrap().is_digit(10) {
            let (s_, v) = parse_num(s)?;
            s = s_;
            stack.push(Node::IntValue(v));
        } else if s.chars().next().map_or(false, |c| c.is_alphabetic() || c == '_') {
            let (s_, v) = parse_ident(s)?;
            s = s_;
            stack.push(v);
        } else {
            return Err(format!("unexpected symbol: '{}'", s.chars().next().unwrap()));
        }
    }
}
