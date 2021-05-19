use crate::graph::Graph;

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct TokenItem<'a> {
    word: &'a str,
    pos: &'a str,
}

#[derive(Debug)]
struct TextRank<'a> {
    tokens: TokenItem<'a>,
}
