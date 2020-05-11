#[derive(Debug, PartialEq, Serialize)]
pub enum Node<'a> {
    Rule(Rule<'a>),
    AtRule(AtRule<'a>),
    Declaration(Declaration<'a>),
    Comment(Comment<'a>)
}

#[derive(Debug, PartialEq, Serialize)]
pub struct Rule<'a> {
    pub selector: &'a str,
    pub nodes: Vec<Node<'a>>
}

#[derive(Debug, PartialEq, Serialize)]
pub struct AtRule<'a> {
    pub name: &'a str,
    pub params: &'a str,
    pub nodes: Vec<Node<'a>>
}

#[derive(Debug, PartialEq, Serialize)]
pub struct Declaration<'a> {
    pub prop: &'a str,
    pub value: &'a str
}

#[derive(Debug, PartialEq, Serialize, Default)]
pub struct CommentRaws<'a> {
    pub before: &'a str,
    pub left: &'a str,
    pub right: &'a str
}

#[derive(Debug, PartialEq, Serialize)]
pub struct Comment<'a> {
    pub text: &'a str,
    pub raws: CommentRaws<'a>
}

#[derive(Debug, PartialEq, Serialize)]
pub struct Root<'a> {
    pub nodes: Vec<Node<'a>>
}