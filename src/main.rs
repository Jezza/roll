
mod lang;

//use logos::Logos;
//
//#[derive(Logos, Debug, PartialEq)]
//enum TokenKind {
//    // Logos requires that we define two default variants,
//    // one for end of input source,
//    #[end]
//    End,
//
//    // ...and one for errors. Those can be named anything
//    // you wish as long as the attributes are there.
//    #[error]
//    Error,
//
//    // Tokens can be literal strings, of any length.
//    #[token = "fast"]
//    Fast,
//
//    #[token = "."]
//    Period,
//
//    // Or regular expressions.
//    #[regex = "[a-zA-Z]+"]
//    Text,
//}




enum Roll {
    Die {
        amount: u64,
        faces: u32,
    },
    Constant(i64)
}


fn main() {
    println!("Hello, world!");

    // roll (2d5 + 2d(123)) + 8

//    1d(2 + 2)
//    1d2 - 2
//    1d2 * 2
//    1d2 / 2

    use lang::parser::Parser;
    use lang::lexer::TokenKind;

    let mut parser = Parser::new(r#"
        1d2 + 1d(2d2)
    "#);

    let t = parser.take_kind();

    

    loop {
        if t == TokenKind::End {
            break;
        }
        println!("{:?}", t);
    }

}


fn parse_rolls() {

}



