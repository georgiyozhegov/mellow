#[macro_export]
macro_rules! numeric {
    () => {
        '0'..='9'
    };
}

#[macro_export]
macro_rules! alphabetic {
    () => {
        'a'..='z' | 'A'..='Z'
    };
}

#[macro_export]
macro_rules! skip {
    () => {
        ' ' | '\t' | '\n'
    };
}

#[macro_export]
macro_rules! single {
    () => {
        '+' | '-' | '*' | '/' | '>' | '<' | '?' | '(' | ')' | '=' | '!'
    };
}

#[macro_export]
macro_rules! quote {
    () => {
        '"'
    };
}
