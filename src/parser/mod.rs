use crate::lexer::Lexer;
use crate::lexer::token::Token;
use crate::parser::node::Node;
use crate::lexer::token_type::TokenType;

pub mod node;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        Self { lexer }
    }

    fn parse(&mut self) -> Vec<&dyn Node> {
        let nodes = Vec::new();

        while let Some(token) = self.lexer.next() {
            if !vec![TokenType::HeadNode, TokenType::BodyNode, TokenType::EndNode].contains(&token.get_type()) {

            }
        }

        nodes
    }
}