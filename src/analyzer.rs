use crate::expression::Expression;

pub struct Analyzer {
    expressions: Vec<Expression>
}

impl Analyzer {
    pub fn new(expressions: Vec<Expression>) -> Self {
        Self {
            expressions
        }
    }

    pub fn analyze_ast(&mut self) {
        for expr in self.expressions.iter().into_iter() {
            // self.analyze_variable_declaration(expr)
        }
    }

    fn analyze_variable_declaration(&mut self, expr: Expression) {
            
    }
}