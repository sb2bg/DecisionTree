use strum_macros::IntoStaticStr;

#[derive(Copy, Clone, IntoStaticStr, PartialEq)]
pub enum TokenType {
    String,
    Number,
    Ident,
    Comma,
    EntryNode,
    BodyNode,
    EndNode,
    Open,
    Close,
}