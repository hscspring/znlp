
#[derive(Hash, Eq, PartialEq, Debug)]
struct TokenItem {
  word: String,
  pos: String
}

struct TextRank {
  tokens: TokenItem
}
