// pub type Tokens = impl Iterator<Item = u8>;

pub fn match_(c: char, tokens: &mut Peekable<impl Iterator<Item = char>>) -> Result<(), String> {
    if tokens.next().unwrap_or('$') != c {
        return Err(format!("Expected {c}"));
    }
    Ok(())
}

fn parse_s_(
    prods: &mut Vec<u8>,
    tokens: &mut Peekable<impl Iterator<Item = char>>,
) -> Result<(), String> {
    /* check lookahead(S' -> S$) = {a,b,$} */
    let next = tokens.peek().copied().unwrap_or('$');
    if next == 'a' || next == 'b' || next == '$' {
        prods.push(0);
        parse_s(prods, tokens)?;
        match_('$', tokens)?;
        Ok(())
    } else {
        Err(format!("unacceptable input in rule S' -> S$: {next}"))
    }
}
fn parse_s(
    prods: &mut Vec<u8>,
    tokens: &mut Peekable<impl Iterator<Item = char>>,
) -> Result<(), String> {
    /* check lookahead(S -> AB) = {a,b,$} */
    let next = tokens.peek().copied().unwrap_or('$');
    if next == 'a' || next == 'b' || next == '$' {
        prods.push(1);
        parse_a(prods, tokens)?;
        parse_b(prods, tokens)?;
        Ok(()) // (* follow production *)
    } else {
        Err(format!("unacceptable input in rule S -> AB: {next}"))
    }
}
fn parse_a(
    prods: &mut Vec<u8>,
    tokens: &mut Peekable<impl Iterator<Item = char>>,
) -> Result<(), String> {
    /* branch on lookahead(A -> aAb) = {a} */
    let next = tokens.peek().copied().unwrap_or('$');
    if next == 'a' {
        prods.push(2);
        match_('a', tokens)?;
        parse_a(prods, tokens)?;
        match_('b', tokens)?;
        Ok(())
    // (* branch on lookahead(A -> ) = {b, $} *)
    } else if next == 'b' || next == '$' {
        prods.push(3);
        Ok(())
    } else {
        Err(format!("unacceptable input in rule A -> aAb | : {next}"))
    }
}
fn parse_b(
    prods: &mut Vec<u8>,
    tokens: &mut Peekable<impl Iterator<Item = char>>,
) -> Result<(), String> {
    /* branch on lookahead(B -> bB) = {b} */
    let next = tokens.peek().copied().unwrap_or('$');
    if next == 'b' {
        prods.push(4);
        match_('b', tokens)?;
        parse_b(prods, tokens)?;
        Ok(())
    // (* branch on lookahead(B -> ) = {$} *)
    } else if next == '$' {
        prods.push(5);
        Ok(())
    } else {
        Err(format!("unacceptable input in rule B -> bB | : {next}"))
    }
}

use std::{io::stdin, iter::Peekable};

fn parse(s: &str) -> Result<Vec<u8>, String> {
    let mut prods = Vec::new();
    parse_s_(&mut prods, &mut s.trim().chars().peekable())?;
    Ok(prods)
}

fn main() {
    loop {
        let mut s = String::new();
        stdin().read_line(&mut s).unwrap();
        match parse(&s) {
            Ok(v) => println!("{:?}", v),
            Err(s) => eprintln!("{}", s),
        }
    }
}
