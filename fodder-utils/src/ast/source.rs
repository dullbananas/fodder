use super::{
    infix,
    Located,
    Name,
    Position,
    Tuple,
};


pub type Expression = Located<InnerExpression>;

pub enum InnerExpression {
    Access {
        record: Box<Expression>,
        field_name: Located<Name>,
    },
    Accessor {
        field_name: Name,
    },
    Call {
        function: Box<Expression>,
        parameters: Vec<Expression>,
    },
    Case {
        expression: Box<Expression>,
        branches: Vec<(Pattern, Expression)>,
    },
    Character {
        value: char,
    },
    Float {
        value: f64,
    },
    If {
        branches: Vec<(Expression, Expression)>,
        fallback: Box<Expression>,
    },
    Infix {
        until_last: Vec<(Expression, Located<Name>)>,
        last: Box<Expression>,
    },
    Integer {
        value: i32,
    },
    Lambda {
        arguments: Vec<Pattern>,
        body: Box<Expression>,
    },
    Let {
        definitions: Vec<Located<LetDefinition>>,
        body: Box<Expression>,
    },
    List {
        items: Vec<Expression>,
    },
    Negate {
        expression: Box<Expression>,
    },
    Operator {
        name: Name,
    },
    Update {
        record_name: Located<Name>,
        fields: Vec<(Located<Name>, Expression)>,
    },
    Record {
        fields: Vec<(Located<Name>, Expression)>,
    },
    String {
        value: String,
    },
    Tuple {
        tuple: Box<Tuple<Expression>>,
    },
    Variable {
        kind: VariableKind,
        path: Option<Name>,
        name: Name,
    },
}


pub enum VariableKind {
    Lowercase,
    Uppercase,
}


enum LetDefinition {
    Define {
        type_annotation: Option<Type>,
        name: Located<Name>,
        arguments: Vec<Pattern>,
        body: Expression,
    },
    Destruct {
        pattern: Pattern,
        expression: Expression,
    },
}


pub type Pattern = Located<InnerPattern>;

pub enum InnerPattern {
    Alias {
        pattern: Box<Pattern>,
        name: Located<Name>,
    },
    Anything,
    Character {
        value: char,
    },
    Cons {
        head: Box<Pattern>,
        tail: Box<Pattern>,
    },
    Constructor {
        start: Position,
        end: Position,
        path: Option<Name>,
        name: Name,
        arguments: Vec<Pattern>,
    },
    Integer {
        value: i32,
    },
    List {
        items: Vec<Pattern>,
    },
    Record {
        field_names: Vec<Name>,
    },
    String {
        value: String,
    },
    Tuple {
        tuple: Box<Tuple<InnerPattern>>,
    },
    Variable {
        name: Name,
    },
}


type Type = Located<InnerType>;

enum InnerType {
    Lambda {
        argument: Box<Type>,
        output: Box<Type>,
    },
    Named {
        start: Position,
        end: Position,
        path: Option<Name>,
        name: Name,
        parameters: Vec<Type>, 
    },
    Record {
        extending: Option<Located<Name>>,
        fields: Vec<(Located<Name>, Type)>,
    },
    Tuple {
        tuple: Box<Tuple<Type>>,
    },
    Variable {
        name: Name,
    },
}


pub struct Module {
    name: Option<Located<Name>>,
    effects: Effects,
    exposing: Located<Exposing>,
    imports: Vec<Import>,
    operators: Vec<Located<InfixDefinition>>,
    aliases: Vec<Located<Alias>>,
    unions: Vec<Located<Union>>,
    values: Vec<Located<Value>>,
}


pub struct Import {
    name: Located<Name>,
    alias: Option<Name>,
    exposing: Exposing,
}

pub struct Value {
    name: Located<Name>,
    type_annotation: Option<Type>,
    arguments: Vec<Pattern>,
    body: Expression,
}

pub struct Union {
    name: Located<Name>,
    arguments: Vec<Located<Name>>,
    constructors: Vec<(Located<Name>, Vec<Type>)>,
}

pub struct Alias {
    name: Located<Name>,
    arguments: Vec<Located<Name>>,
    body: Type,
}

pub struct InfixDefinition {
    symbol: Name,
    associativity: infix::Associativity,
    precedence: infix::Precedence,
    function_name: Name,
}

pub struct Port {
    name: Located<Name>,
    type_annotation: Type,
}


pub enum Effects {
    None,
    Ports {
        ports: Vec<Port>,
    },
    Manager {
        start: Position,
        end: Position,
        manager: EffectManager,
    },
}

pub enum EffectManager {
    Command {
        command:  Located<Name>,
    },
    Subscription {
        subscription: Located<Name>,
    },
    Both {
        command:  Located<Name>,
        subscription: Located<Name>,
    }
}


pub enum Exposing {
    All,
    Listed {
        items: Vec<Exposed>,
    },
}

pub enum Exposed {
    Lowercase {
        name: Located<Name>,
    },
    Uppercase {
        name: Located<Name>,
        privacy: Privacy,
    },
    Operator {
        start: Position,
        end: Position,
        name: Name,
    },
}

pub enum Privacy {
    Public {
        start: Position,
        end: Position,
    },
    Private,
}
