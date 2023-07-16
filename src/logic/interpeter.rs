use std::collections::HashMap;

use super::{ast::ASTNode, tokenizer::Token};

struct Interpreter {
    variables: HashMap<String, ASTNode>,
}

impl Interpreter {
    fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    fn interpret(&mut self, ast: ASTNode) {
        match ast {
            ASTNode::Program(statements) => {
                for statement in statements.iter() {
                    self.interpret(statement.clone());
                }
            }
            ASTNode::VariableDeclaration(name, value) => {
                self.variables.insert(name, *value);
            }
            ASTNode::VariableAssignment(name, value) => {
                let evaluated_value = self.evaluate_expression(*value);
                if let Some(variable) = self.variables.get_mut(&name) {
                    // eval value
                    *variable = evaluated_value;
                } else {
                    panic!("Variable {} not found", name);
                }
            }
            ASTNode::Identifier(name) => {
                if let Some(value) = self.variables.get(&name) {
                    self.interpret(value.clone());
                } else {
                    panic!("Variable {} not found", name);
                }
            }
            ASTNode::FunctionCall(name) => {
                if let Some(value) = self.variables.get(&name) {
                    let program = match value.clone() {
                        ASTNode::FunctionDeclaration(_, body) => ASTNode::Program(Box::new(body)),
                        _ => panic!("Unexpected ASTNode: {:?}", value),
                    };
                    self.interpret(program);
                } else {
                    panic!("Function {} not found", name);
                }
            }
            ASTNode::FunctionDeclaration(name, body) => {
                self.variables
                    .insert(name.clone(), ASTNode::FunctionDeclaration(name, body));
            }
            ASTNode::Print(expression) => {
                let evaluated_expression = self.evaluate_expression(*expression);
                println!("{:?}", self.stringify_value(evaluated_expression));
            }
            ASTNode::If(expression, statements) => {
                let evaluated_expression = self.evaluate_expression(*expression);

                if self.stringify_value(evaluated_expression) == "true" {
                    for statement in statements.iter() {
                        self.interpret(statement.clone());
                    }
                }
            }
            ASTNode::While(expression, statements) => {
                // check if expression is true, if so, interpret statements and re-evaluate expression
                let mut evaluated_expression = self.evaluate_expression(*expression.clone());

                while self.stringify_value(evaluated_expression) == "true" {
                    for statement in statements.iter() {
                        self.interpret(statement.clone());
                    }
                    evaluated_expression = self.evaluate_expression(*expression.clone());
                }
            }
            _ => panic!("Unexpected ASTNode: {:?}", ast),
        }
    }

    fn stringify_value(&mut self, ast: ASTNode) -> String {
        match ast {
            ASTNode::Number(value) => value.to_string(),
            ASTNode::String(value) => value,
            ASTNode::Boolean(value) => value.to_string(),
            _ => panic!("Unexpected AST node: {:?}", ast),
        }
    }

    fn evaluate_expression(&mut self, ast: ASTNode) -> ASTNode {
        match ast {
            ASTNode::Binary(left, operator, right) => {
                let left_val = self.evaluate_expression(*left);
                let right_val = self.evaluate_expression(*right);
                self.evaluate_binary_operation(left_val, operator, right_val)
            }
            ASTNode::Unary(operator, right) => {
                let right_val = self.evaluate_expression(*right);
                self.evaluate_unary_operation(operator, right_val)
            }
            ASTNode::Boolean(value) => ASTNode::Boolean(value),
            ASTNode::String(value) => ASTNode::String(value),
            ASTNode::Number(value) => ASTNode::Number(value),
            ASTNode::Identifier(name) => {
                if let Some(value) = self.variables.get(&name) {
                    self.evaluate_expression(value.clone())
                } else {
                    panic!("Variable {} not found", name);
                }
            }
            ASTNode::FunctionCall(name) => {
                if let Some(value) = self.variables.get(&name) {
                    self.evaluate_expression(value.clone())
                } else {
                    panic!("Function {} not found", name);
                }
            }
            _ => panic!("Unexpected ASTNode: {:?}", ast),
        }
    }

    fn evaluate_binary_operation(
        &mut self,
        left: ASTNode,
        operator: Token,
        right: ASTNode,
    ) -> ASTNode {
        match operator {
            Token::Plus => self.evaluate_addition(left, right),
            Token::Minus => self.evaluate_subtraction(left, right),
            Token::Asterisk => self.evaluate_multiplication(left, right),
            Token::Slash => self.evaluate_division(left, right),
            Token::Percent => self.evaluate_modulo(left, right),
            Token::Equals => self.evaluate_equal(left, right),
            Token::EqualEqual => self.evaluate_equal_equal(left, right),
            Token::Bang => self.evaluate_not_equal(left, right),
            Token::NotEqual => self.evaluate_not_equal(left, right),
            Token::LessThan => self.evaluate_less_than(left, right),
            Token::LessThanEquals => self.evaluate_less_than_equals(left, right),
            Token::GreaterThan => self.evaluate_greater_than(left, right),
            Token::GreaterThanEquals => self.evaluate_greater_than_equals(left, right),
            Token::And => self.evaluate_and(left, right),
            Token::Or => self.evaluate_or(left, right),
            _ => panic!("Unexpected operator: {:?}", operator),
        }
    }

    fn evaluate_unary_operation(&mut self, operator: Token, right: ASTNode) -> ASTNode {
        match operator {
            Token::Minus => self.evaluate_negation(right),
            Token::Bang => self.evaluate_not(right),
            _ => panic!("Unexpected operator: {:?}", operator),
        }
    }

    // binary operations

    fn evaluate_addition(&mut self, left: ASTNode, right: ASTNode) -> ASTNode {
        match (left.clone(), right.clone()) {
            (ASTNode::Number(left), ASTNode::Number(right)) => ASTNode::Number(left + right),
            (ASTNode::String(left), ASTNode::String(right)) => {
                ASTNode::String(format!("{}{}", left, right))
            }
            _ => panic!("Unexpected operands: {:?} + {:?}", left, right),
        }
    }

    fn evaluate_subtraction(&mut self, left: ASTNode, right: ASTNode) -> ASTNode {
        match (left.clone(), right.clone()) {
            (ASTNode::Number(left), ASTNode::Number(right)) => ASTNode::Number(left - right),
            _ => panic!("Unexpected operands: {:?} - {:?}", left, right),
        }
    }

    fn evaluate_multiplication(&mut self, left: ASTNode, right: ASTNode) -> ASTNode {
        match (left.clone(), right.clone()) {
            (ASTNode::Number(left), ASTNode::Number(right)) => ASTNode::Number(left * right),
            _ => panic!("Unexpected operands: {:?} * {:?}", left, right),
        }
    }

    fn evaluate_division(&mut self, left: ASTNode, right: ASTNode) -> ASTNode {
        match (left.clone(), right.clone()) {
            (ASTNode::Number(left), ASTNode::Number(right)) => ASTNode::Number(left / right),
            _ => panic!("Unexpected operands: {:?} / {:?}", left, right),
        }
    }

    fn evaluate_modulo(&mut self, left: ASTNode, right: ASTNode) -> ASTNode {
        match (left.clone(), right.clone()) {
            (ASTNode::Number(left), ASTNode::Number(right)) => ASTNode::Number(left % right),
            _ => panic!("Unexpected operands: {:?} % {:?}", left, right),
        }
    }

    fn evaluate_equal(&mut self, left: ASTNode, right: ASTNode) -> ASTNode {
        match (left.clone(), right.clone()) {
            (ASTNode::Identifier(left), ASTNode::Identifier(right)) => {
                ASTNode::Boolean(left == right)
            }
            (ASTNode::Number(left), ASTNode::Number(right)) => ASTNode::Boolean(left == right),
            (ASTNode::String(left), ASTNode::String(right)) => ASTNode::Boolean(left == right),
            (ASTNode::Boolean(left), ASTNode::Boolean(right)) => ASTNode::Boolean(left == right),
            _ => panic!("Unexpected operands: {:?} == {:?}", left, right),
        }
    }

    fn evaluate_equal_equal(&mut self, left: ASTNode, right: ASTNode) -> ASTNode {
        match (left.clone(), right.clone()) {
            (ASTNode::Identifier(left), ASTNode::Identifier(right)) => {
                ASTNode::Boolean(left == right)
            }
            (ASTNode::Number(left), ASTNode::Number(right)) => ASTNode::Boolean(left == right),
            (ASTNode::String(left), ASTNode::String(right)) => ASTNode::Boolean(left == right),
            (ASTNode::Boolean(left), ASTNode::Boolean(right)) => ASTNode::Boolean(left == right),
            _ => panic!("Unexpected operands: {:?} == {:?}", left, right),
        }
    }

    fn evaluate_not_equal(&mut self, left: ASTNode, right: ASTNode) -> ASTNode {
        match (left.clone(), right.clone()) {
            (ASTNode::Identifier(left), ASTNode::Identifier(right)) => {
                ASTNode::Boolean(left != right)
            }
            (ASTNode::Number(left), ASTNode::Number(right)) => ASTNode::Boolean(left != right),
            (ASTNode::String(left), ASTNode::String(right)) => ASTNode::Boolean(left != right),
            (ASTNode::Boolean(left), ASTNode::Boolean(right)) => ASTNode::Boolean(left != right),
            _ => panic!("Unexpected operands: {:?} != {:?}", left, right),
        }
    }

    fn evaluate_less_than(&mut self, left: ASTNode, right: ASTNode) -> ASTNode {
        match (left.clone(), right.clone()) {
            (ASTNode::Number(left), ASTNode::Number(right)) => ASTNode::Boolean(left < right),
            _ => panic!("Unexpected operands: {:?} < {:?}", left, right),
        }
    }

    fn evaluate_less_than_equals(&mut self, left: ASTNode, right: ASTNode) -> ASTNode {
        match (left.clone(), right.clone()) {
            (ASTNode::Number(left), ASTNode::Number(right)) => ASTNode::Boolean(left <= right),
            _ => panic!("Unexpected operands: {:?} <= {:?}", left, right),
        }
    }

    fn evaluate_greater_than(&mut self, left: ASTNode, right: ASTNode) -> ASTNode {
        match (left.clone(), right.clone()) {
            (ASTNode::Number(left), ASTNode::Number(right)) => ASTNode::Boolean(left > right),
            _ => panic!("Unexpected operands: {:?} > {:?}", left, right),
        }
    }

    fn evaluate_greater_than_equals(&mut self, left: ASTNode, right: ASTNode) -> ASTNode {
        match (left.clone(), right.clone()) {
            (ASTNode::Number(left), ASTNode::Number(right)) => ASTNode::Boolean(left >= right),
            _ => panic!("Unexpected operands: {:?} >= {:?}", left, right),
        }
    }

    fn evaluate_and(&mut self, left: ASTNode, right: ASTNode) -> ASTNode {
        match (left.clone(), right.clone()) {
            (ASTNode::Boolean(left), ASTNode::Boolean(right)) => ASTNode::Boolean(left && right),
            _ => panic!("Unexpected operands: {:?} && {:?}", left, right),
        }
    }

    fn evaluate_or(&mut self, left: ASTNode, right: ASTNode) -> ASTNode {
        match (left.clone(), right.clone()) {
            (ASTNode::Boolean(left), ASTNode::Boolean(right)) => ASTNode::Boolean(left || right),
            _ => panic!("Unexpected operands: {:?} || {:?}", left, right),
        }
    }

    // unary operations

    fn evaluate_negation(&mut self, right: ASTNode) -> ASTNode {
        match right {
            ASTNode::Number(right) => ASTNode::Number(-right),
            _ => panic!("Unexpected operand: -{:?}", right),
        }
    }

    fn evaluate_not(&mut self, right: ASTNode) -> ASTNode {
        match right {
            ASTNode::Boolean(right) => ASTNode::Boolean(!right),
            _ => panic!("Unexpected operand: !{:?}", right),
        }
    }
}

pub fn interpret(ast: ASTNode) {
    let mut interpreter = Interpreter::new();
    interpreter.interpret(ast);
}
