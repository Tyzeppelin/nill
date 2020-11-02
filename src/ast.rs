use token;

// Still not sure how I want this to behave.
pub trait Expr {
    fn accept<V: Visitor>(&mut self, expr_vis: &mut V);
}

#[derive(Debug, Clone)]
pub enum Sexpr {
    L(Box<Lexpr>),
    // |
    A(Litteral),
}

#[derive(Debug, Clone)]
pub enum Lexpr {
    SL(Box<Sexpr>, Box<Lexpr>),
    // |
    Eps(Epsilon),
}

#[derive(Debug, Clone)]
pub enum Litteral {
    Value(token::TokenType),
}

#[derive(Debug, Clone)]
pub enum Epsilon {
    Eps,
}


pub struct ExprVisitor{}
pub trait Visitor {}
impl Visitor for ExprVisitor{}


impl ExprVisitor {
    fn visit_lexpr(&mut self, l: &Lexpr){}
    fn visit_litteral(&mut self, lit: &Litteral){}

}

//impl Sexpr{
//    pub fn new_lexpr(lexpr: &Lexpr) -> Self {
//        Self { l: lexpr }
//    }
//
//    pub fn new_lit(lit: Litteral) -> Self {
//        Self { a: lit }
//    }
//}
impl Expr for Sexpr {
    fn accept<V: Visitor>(&mut self, expr_vis: &mut V){

    }
}

impl Expr for Lexpr {
    fn accept<V: Visitor>(&mut self, expr_vis: &mut V) {

    }
}

impl Expr for Litteral {
    fn accept<V: Visitor>(&mut self, expr_vis: &mut V) {
    }
}


impl Expr for Epsilon {
    fn accept<V: Visitor>(&mut self, expr_vis: &mut V) {

    }
}


// Printer

// not yet

