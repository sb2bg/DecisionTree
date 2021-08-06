use strum_macros::IntoStaticStr;

#[derive(Copy, Clone, IntoStaticStr, PartialEq)]
pub enum TokenType {
    True,
    False,
    String,
    Number,
    Ident,
    Comma,
    HeadNode,
    BodyNode,
    EndNode,
    Open,
    Close,
}