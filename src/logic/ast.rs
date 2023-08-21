use std::cmp::Ordering;

use super::tokenizer::Token;

#[derive(Debug, Clone)]
pub enum ASTNode {
    Program(Box<Vec<ASTNode>>),
    Number(i32),
    String(String),
    Identifier(String),
    VariableDeclaration(String, Box<ASTNode>),
    VariableAssignment(String, Box<ASTNode>),
    FunctionCall(String),
    FunctionDeclaration(String, Vec<ASTNode>),
    If(Box<ASTNode>, Vec<ASTNode>),
    While(Box<ASTNode>, Vec<ASTNode>),
    Print(Box<ASTNode>),
    Boolean(bool),
    Binary(Box<ASTNode>, Token, Box<ASTNode>),
    Unary(Token, Box<ASTNode>),
}

struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    fn parse(&mut self) -> Result<ASTNode, String> {
        let mut statements = Vec::new();

        while self.current_token() != Token::EndOfFile {
            let statement = self.parse_statement()?;
            statements.push(statement);
        }

        Ok(ASTNode::Program(Box::new(statements)))
    }

    // parsing helpers
    fn parse_statement(&mut self) -> Result<ASTNode, String> {
        match self.current_token() {
            Token::V => self.parse_variable_declaration(),
            Token::W => self.parse_while(),
            Token::I => self.parse_if(),
            Token::F => self.parse_function_declaration(),
            Token::Print => self.parse_print(),
            Token::Identifier(_) => self.parse_assignment(),
            _ => Err(format!(
                "Expected statement but got {:?}",
                self.current_token()
            )),
        }
    }

    fn parse_variable_declaration(&mut self) -> Result<ASTNode, String> {
        self.consume(Token::V)?;

        let identifier = self.consume_identifier()?;

        self.consume(Token::Equals)?;

        let expression = self.parse_expression()?;

        Ok(ASTNode::VariableDeclaration(
            identifier,
            Box::new(expression),
        ))
    }

    fn parse_while(&mut self) -> Result<ASTNode, String> {
        self.consume(Token::W)?;

        let condition = self.parse_expression()?;

        self.consume(Token::LeftBrace)?;

        let mut statements = Vec::new();

        while self.current_token() != Token::RightBrace {
            let statement = self.parse_statement()?;
            statements.push(statement);
        }

        self.consume(Token::RightBrace)?;

        Ok(ASTNode::While(Box::new(condition), statements))
    }

    fn parse_if(&mut self) -> Result<ASTNode, String> {
        self.consume(Token::I)?;

        let condition = self.parse_expression()?;

        self.consume(Token::LeftBrace)?;

        let mut statements = Vec::new();

        while self.current_token() != Token::RightBrace {
            let statement = self.parse_statement()?;
            statements.push(statement);
        }

        self.consume(Token::RightBrace)?;

        Ok(ASTNode::If(Box::new(condition), statements))
    }

    fn parse_function_declaration(&mut self) -> Result<ASTNode, String> {
        self.consume(Token::F)?;

        let identifier = self.consume_identifier()?;

        self.consume(Token::LeftBrace)?;

        let mut statements = Vec::new();

        while self.current_token() != Token::RightBrace {
            let statement = self.parse_statement()?;
            statements.push(statement);
        }

        self.consume(Token::RightBrace)?;

        Ok(ASTNode::FunctionDeclaration(identifier, statements))
    }

    fn parse_print(&mut self) -> Result<ASTNode, String> {
        self.consume(Token::Print)?;

        let expression = self.parse_expression()?;

        Ok(ASTNode::Print(Box::new(expression)))
    }

    fn parse_assignment(&mut self) -> Result<ASTNode, String> {
        let identifier = self.consume_identifier()?;

        if self.current_token() == Token::Equals {
            self.consume(Token::Equals)?;

            let expression = self.parse_expression()?;

            Ok(ASTNode::VariableAssignment(
                identifier,
                Box::new(expression),
            ))
        } else if self.current_token() == Token::Bang {
            self.consume(Token::Bang)?;

            Ok(ASTNode::FunctionCall(identifier))
        } else {
            Err(format!(
                "Expected equals or bang but got {:?}",
                self.current_token()
            ))
        }
    }

    fn parse_expression(&mut self) -> Result<ASTNode, String> {
        let mut left_node = self.parse_simple_expression()?;
        while self.current_token() == Token::And || self.current_token() == Token::Or {
            let operator = self.current_token();
            self.consume(operator.clone())?;

            let right_node = self.parse_simple_expression()?;

            left_node = ASTNode::Binary(Box::new(left_node), operator, Box::new(right_node));
        }
        Ok(left_node)
    }

    fn parse_simple_expression(&mut self) -> Result<ASTNode, String> {
        let mut left_node = self.parse_term()?;
        while self.current_token() == Token::Plus
            || self.current_token() == Token::Minus
            || self.current_token() == Token::Asterisk
            || self.current_token() == Token::Slash
            || self.current_token() == Token::Percent
            || self.current_token() == Token::Pow
            || self.current_token() == Token::LessThan
            || self.current_token() == Token::GreaterThan
            || self.current_token() == Token::LessThanEquals
            || self.current_token() == Token::GreaterThanEquals
            || self.current_token() == Token::EqualEqual
            || self.current_token() == Token::NotEqual
            || self.current_token() == Token::Equals
        {
            let operator = self.current_token();
            self.consume(operator.clone())?;

            let right_node = self.parse_term()?;

            left_node = ASTNode::Binary(Box::new(left_node), operator, Box::new(right_node));
        }
        Ok(left_node)
    }

    fn parse_term(&mut self) -> Result<ASTNode, String> {
        let mut left_node = self.parse_factor()?;

        while self.current_token() == Token::Equals || self.current_token() == Token::NotEqual {
            let operator = self.current_token();
            self.consume(operator.clone())?;

            let right_node = self.parse_factor()?;

            left_node = ASTNode::Binary(Box::new(left_node), operator, Box::new(right_node));
        }

        Ok(left_node)
    }

    fn parse_factor(&mut self) -> Result<ASTNode, String> {
        match self.current_token() {
            Token::Number(value) => {
                self.next_token()?;
                Ok(ASTNode::Number(value))
            }
            Token::String(value) => {
                self.next_token()?;
                Ok(ASTNode::String(value))
            }
            Token::Boolean(value) => {
                self.next_token()?;
                Ok(ASTNode::Boolean(value))
            }
            Token::Identifier(value) => {
                self.next_token()?;
                Ok(ASTNode::Identifier(value))
            }
            Token::LeftParen => {
                self.consume(Token::LeftParen)?;

                let expression = self.parse_expression()?;

                self.consume(Token::RightParen)?;

                Ok(expression)
            }
            Token::Minus => {
                self.consume(Token::Minus)?;

                let expression = self.parse_expression()?;

                Ok(ASTNode::Unary(Token::Minus, Box::new(expression)))
            }
            Token::Bang => {
                self.consume(Token::Bang)?;

                let expression = self.parse_expression()?;

                Ok(ASTNode::Unary(Token::Bang, Box::new(expression)))
            }
            _ => Err(format!(
                "Expected number, string, boolean, identifier, or left paren but got {:?}",
                self.current_token()
            )),
        }
    }

    // token helpers

    fn current_token(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn consume(&mut self, token: Token) -> Result<Token, String> {
        if self.current_token() == token {
            self.current += 1;
            Ok(token)
        } else {
            Err(format!(
                "Expected {:?} but got {:?}",
                token,
                self.current_token()
            ))
        }
    }

    fn consume_identifier(&mut self) -> Result<String, String> {
        if let Token::Identifier(identifier) = self.current_token() {
            self.next_token()?;
            Ok(identifier)
        } else {
            Err(format!(
                "Expected identifier but got {:?}",
                self.current_token()
            ))
        }
    }

    fn next_token(&mut self) -> Result<(), String> {
        if self.current < self.tokens.len() - 1 {
            self.current += 1;
            Ok(())
        } else {
            Err("No more tokens".to_string())
        }
    }
}

impl Eq for ASTNode {}

impl PartialOrd for ASTNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (ASTNode::Number(a), ASTNode::Number(b)) => a.partial_cmp(b),
            (ASTNode::String(a), ASTNode::String(b)) => a.partial_cmp(b),
            _ => None,
        }
    }
}

impl Ord for ASTNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialEq for ASTNode {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ASTNode::Number(a), ASTNode::Number(b)) => a == b,
            (ASTNode::String(a), ASTNode::String(b)) => a == b,
            (ASTNode::Identifier(a), ASTNode::Identifier(b)) => a == b,
            _ => false,
        }
    }
}

pub fn parse(tokens: Vec<Token>) -> Result<ASTNode, String> {
    let mut parser = Parser::new(tokens);
    parser.parse()
}
