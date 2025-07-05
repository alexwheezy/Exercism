use regex_lite::Regex;

#[derive(Clone, Copy)]
enum BinOp {
    Plus,
    Minus,
    Multiply,
    Divide,
    Power,
    Unknown,
}

impl From<&str> for BinOp {
    fn from(value: &str) -> Self {
        match value {
            "plus" => BinOp::Plus,
            "minus" => BinOp::Minus,
            "multiplied by" => BinOp::Multiply,
            "divided by" => BinOp::Divide,
            "raised to the" => BinOp::Power,
            _ => BinOp::Unknown,
        }
    }
}

#[derive(Clone, Copy)]
enum Ast {
    Operand(i32),
    Operator(BinOp),
}

struct Expr {
    left_op: i32,
    right_op: i32,
    operation: BinOp,
}

impl Expr {
    fn new(stack: &[Ast]) -> Option<Self> {
        let (left_op, operation, right_op) = match (stack[0], stack[1], stack[2]) {
            (Ast::Operand(left), Ast::Operator(op), Ast::Operand(right_op)) => (left, op, right_op),
            _ => return None,
        };

        Some(Self {
            left_op,
            right_op,
            operation,
        })
    }

    fn evaluate(&self) -> Option<i32> {
        let result = match self.operation {
            BinOp::Plus => self.left_op + self.right_op,
            BinOp::Minus => self.left_op - self.right_op,
            BinOp::Multiply => self.left_op * self.right_op,
            BinOp::Divide => self.left_op / self.right_op,
            BinOp::Power => self.left_op.pow(self.right_op as u32),
            _ => return None,
        };
        Some(result)
    }
}

pub fn answer(command: &str) -> Option<i32> {
    if command.is_empty() {
        return None;
    }

    const MAX_STACK_SIZE: usize = 3;

    let mut stack = Vec::with_capacity(MAX_STACK_SIZE);
    let re = Regex::new(r"(-?\d+)|([^-\d?]+)").unwrap();

    for token in re
        .find_iter(command)
        .skip(1)
        .map(|m| m.as_str().trim())
        .filter(|s| !s.is_empty())
        .filter(|&s| !s.contains("power"))
    {
        let parse = match token.parse::<i32>() {
            Ok(value) => Ast::Operand(value),
            _ => Ast::Operator(token.into()),
        };
        stack.push(parse);

        if stack.len() == MAX_STACK_SIZE {
            let value = Expr::new(&stack)?.evaluate()?;
            stack.clear();
            stack.push(Ast::Operand(value));
        }
    }

    if stack.len() != 1 {
        return None;
    }

    let Ast::Operand(value) = stack[0] else {
        return None;
    };

    Some(value)
}
