extern crate nom;
use nom::{
    IResult,
    bytes::complete::{tag, take_until, take_till1, take_while},
    character::complete::{multispace0, space0},
    sequence::{delimited, tuple, terminated, separated_pair, pair, preceded},
    branch::alt,
    multi::{many_till, many0}
};
use crate::structs::{Node, Root, Rule, AtRule, Declaration, Comment, CommentRaws};

fn take_while_rev<F>(cond: F) -> impl Fn(&str) -> IResult<&str, &str> where F: Fn(char) -> bool {
    move |input| {
        match input.rfind(|c| !cond(c)) {
            Some(i) => Ok((&input[..=i], &input[i+1..])),
            None => Ok((input, ""))
        }
    }
}

pub fn comment(input: &str) -> IResult<&str, Node> {
    let (input, (left, text)) = delimited(tag("/*"), pair(take_while(|c: char| c.is_whitespace()), take_until("*/")), tag("*/"))(input)?;
    let (text, right) = take_while_rev(|c| c.is_whitespace())(text)?;
    Ok((input, Node::Comment(Comment { text, raws: CommentRaws { before: "", left, right } })))
}

pub fn prop(input: &str) -> IResult<&str, &str> {
    let (input, prop) = take_till1(|c| c == ':' || c == '{' || c == ';' || c == '\n')(input)?;
    Ok((input, prop))
}

pub fn value(input: &str) -> IResult<&str, &str> {
    let (input, _) = space0(input)?;
    let (input, prop) = take_till1(|c| c == '{' || c == ';' || c == '\n')(input)?;
    Ok((input, prop))
}

pub fn declaration(input: &str) -> IResult<&str, Node> {
    let (input, (prop, value)) = terminated(separated_pair(prop, tag(":"), value), tag(";"))(input)?;
    Ok((input, Node::Declaration(Declaration { prop, value })))
}

pub fn rule(input: &str) -> IResult<&str, Node> {
    let (input, (selector, nodes)) = pair(take_until("{"), preceded(tag("{"), nodes))(input)?;
    Ok((input, Node::Rule(Rule { selector, nodes })))
}

pub fn at_rule_name(input: &str) -> IResult<&str, &str> {
    let (input, prop) = take_till1(|c: char| c.is_whitespace())(input)?;
    let (input, _) = space0(input)?;
    Ok((input, prop))
}

pub fn at_rule_params(input: &str) -> IResult<&str, &str> {
    let (input, prop) = take_till1(|c| c == ';' || c == '{' || c == '\n')(input)?;
    Ok((input, prop))
}

pub fn at_rule_empty(input: &str) -> IResult<&str, Vec<Node>> {
    let (input, _) = tag(";")(input)?;
    Ok((input, vec![]))
}

pub fn at_rule_nodes(input: &str) -> IResult<&str, Vec<Node>> {
    let (input, nodes) = alt((at_rule_empty, preceded(tag("{"), nodes)))(input)?;
    Ok((input, nodes))
}

pub fn at_rule(input: &str) -> IResult<&str, Node> {
    let (input, (name, params, nodes)) = preceded(tag("@"), tuple((at_rule_name, at_rule_params, at_rule_nodes)))(input)?;
    Ok((input, Node::AtRule(AtRule { name, params, nodes })))
}

pub fn node(input: &str) -> IResult<&str, (Node, &str)> {
    let (input, node) = alt((comment, at_rule, declaration, rule))(input)?;
    let (input, after) = multispace0(input)?;
    Ok((input, (node, after)))
}

pub fn nodes_terminator(input: &str) -> IResult<&str, &str> {
    let (input, bracket) = tag("}")(input)?;
    Ok((input, bracket))
}

fn set_before<'a>(before: &'a str, nodes: &mut Vec<(Node<'a>, &'a str)>) -> &'a str {
    let mut before = before;
    for (node, after) in nodes.iter_mut() {
        match node {
            &mut Node::Comment(ref mut comment) => comment.raws.before = before,
            _ => ()
        }
        before = after;
    }
    before
}

pub fn nodes(input: &str) -> IResult<&str, Vec<Node>> {
    let (input, before) = multispace0(input)?;
    let (input, (mut nodes, _)) = many_till(node, nodes_terminator)(input)?;
    let _ = set_before(before, &mut nodes);
    let nodes = nodes.into_iter().map(|(node, _)| node).collect();
    Ok((input, nodes))
}

pub fn root(input: &str) -> IResult<&str, Root> {
    let (input, before) = multispace0(input)?;
    let (input, mut nodes) = many0(node)(input)?;
    let _ = set_before(before, &mut nodes);
    let nodes = nodes.into_iter().map(|(node, _)| node).collect();
    Ok((input, Root { nodes }))
}

#[test]
fn parse_comment() {
    assert_eq!(
        comment("/* some comment */"),
        Ok(("", Node::Comment(Comment { text: "some comment", raws: CommentRaws { before: "", left: " ", right: " " } })))
    );
}

#[test]
fn parse_declaration() {
    assert_eq!(declaration("prop: value;"), Ok(("", Node::Declaration(Declaration { prop: "prop", value: "value" }))));
}
