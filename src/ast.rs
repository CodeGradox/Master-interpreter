use real::Real;

pub enum Statement {
    /// A `break` statement.
    Break,
    /// A `continue` statement.
    Continue,
    /// A 'return' expression with an optional value to be returned.
    Return(Option<Expr>),
}

// Here we merged Test into Expr for convenience.
pub struct Expr {
    pub kind: ExprKind,
}

pub enum ExprKind {
    /// An assignment (ex `a = foo()`)
    Assign(Expr, Expr),
    /// An assigmnet with an operator (ex `a += 1`)
    AssignOP(BinOp, Expr, Expr),
    /// A binary expression (ex `1 + 2`, `a * b`)
    Binary(BinOp, Expr, Expr),
    /// A unary expression (ex `!a`, `-5`)
    Unary(UnOp, Expr),
    /// An atom (ex `1`, `"Hello"`, `false`, `a`)
    Atom(Atom),
    /// A list (ex. `[a, b, c, d]`)
    List(Vec<Expr>),
    /// A function call.
    ///
    /// The first field is the function name,
    /// the second filed is the list of arguments.
    /// (ex. `foo("hamburger", 32)`)
    Call(Expr, Vec<Expr>),
    /// A method call.
    ///
    /// The first field is the name of the object,
    /// the second field is the lsit of arguments.
    /// (ex. `Temp.get()`)
    MethodCall(Expr, Vec<Expr>),
    /// An indexing operation to a list (ex. `a[9]`)
    Index(Expr, Expr),
    /// The `?` operator (ex. `foo?`)
    Try(Expr),
}

/// An atomic expression
pub enum AtomExpr(Atom, Option<Vec<Postfix>>)

/// An atom, i.e. a literal, name or a grouped expression.
pub enum Atom {
    Name(Identifier),
    Literal(Literal),
    Group(Expr),
}

pub struct Identifier {
    id: u32,
}

pub enum Literal {
    Int(i32),
    Real(Real),
    Str(String),
    Bool(bool),
}

/// The binary operators.
pub enum BinOp {
    /// The `+` operator (addition)
    Add,
    /// The `-` operator (subtraction)
    Sub,
    /// The `*` operator (multiplication)
    Mul,
    /// The `/` operator (division)
    Div,
    /// The `==` operator (equality)
    Eq,
    /// The `<` operator (less than)
    Lt,
    /// The `<=` operator (less than or equal to)
    Le,
    /// The `!=` operator (not equal to)
    Ne,
    /// The `>=` operator (greater than or equal to)
    Ge,
    /// The `>` operator (greater than)
    Gt,
    /// The `&` operator (logical and)
    And,
    /// The `|` operator (logical not)
    Or,
}

impl BinOp {
    // it's called lazy because it may not evaluate all expressions
    // as can short-circut.
    pub fn is_lazy(&self) -> bool {
        use self::BinOp::*;
        match *self {
            And | Or => true,
            _ => false,
        }
    }

    pub fn is_comparison(&self) -> bool {
        use self::BinOp::*;
        match *self {
            Eq | Lt | Le | Ne | Ge | Gt => true,
            _ => false,
        }
    }
}

/// Unary operator
pub enum UnOp {
    /// The `!` operator for logical inversion
    Not,
    /// The `-` operator for negation
    Neg,
}