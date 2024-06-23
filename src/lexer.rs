#[derive(Debug)]
enum Literal {
    Int,
    Str,
}

#[derive(Debug)]
enum TokenType {
    Identifier,
    Keyword,
    Literal(Literal),
    Operator,
}

#[derive(Debug)]
struct Token {
    token_type: TokenType,
    token_data: String,
}

#[derive(Debug)]
pub struct Tokenizer {
    input: Vec<String>,
    tokens: Vec<Token>,
    error: Vec<String>,
}

impl Tokenizer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.split(" ").map(String::from).collect(),
            tokens: vec![],
            error: vec![],
        }
    }

    pub fn tokenize(&mut self) {
        let mut in_string = false;
        let mut string_buffer = String::new();

        for word in self.input.clone() {
            if !in_string && word.starts_with('"') {
                in_string = true;
                string_buffer.push_str(word.trim_start_matches('"'));

                if word.ends_with('"') {
                    in_string = false;
                    string_buffer = string_buffer.trim_end_matches('"').to_string();
                    self.tokens.push(Token {
                        token_type: TokenType::Literal(Literal::Str),
                        token_data: string_buffer.clone(),
                    });
                    string_buffer.clear();
                }
                continue;
            }

            if in_string {
                string_buffer.push_str(" ");
                string_buffer.push_str(word.as_str());
                if word.ends_with('"') {
                    in_string = false;
                    string_buffer = string_buffer.trim_end_matches('"').to_string();

                    self.tokens.push(Token {
                        token_type: TokenType::Literal(Literal::Str),
                        token_data: string_buffer.clone(),
                    });
                    string_buffer.clear();
                }
                continue;
            }

            //let strs = word.split_whitespace();

            //for word in strs {
            let t_type = if Self::is_literal(word.as_str()) {
                TokenType::Literal(Literal::Int)
            } else if Self::is_keyword(word.as_str()) {
                TokenType::Keyword
            } else if Self::is_identifier(self, word.as_str()) {
                TokenType::Identifier
            } else if Self::is_operator(word.as_str()) {
                TokenType::Operator
            } else {
                self.error.push(format!("Unknown token: {}", word));
                continue;
            };

            self.tokens.push(Token {
                token_type: t_type,
                token_data: word.to_string(),
            })
            //}
        }
    }

    fn is_literal(input: &str) -> bool {
        let is_integer = input.chars().all(|c| c.is_ascii_digit() || c == '-');
        is_integer
    }

    fn is_keyword(input: &str) -> bool {
        let keyword_list = ["main", "in", "out", "int", "str"];
        keyword_list.contains(&input)
    }

    fn is_identifier(&mut self, input: &str) -> bool {
        if !input.chars().next().unwrap().is_ascii_lowercase() {
            self.error
                .push("Identifiers should start with a lowercase letter.".to_string());
            return false;
        };

        input
            .chars()
            .skip(1)
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
    }

    fn is_operator(input: &str) -> bool {
        let operator_list = [":=", "+", "-"];
        operator_list.contains(&input)
    }
}

#[cfg(test)]
mod tests {
    use crate::Tokenizer;

    #[test]
    fn test_new() {
        let input = "main := \"Hello World\"\n";
        println!("{:?}", input);
        let mut tokenizer = Tokenizer::new(input);
        println!("{:?}", tokenizer);
        tokenizer.tokenize();
        println!("{:?}", tokenizer.tokens);

        assert!(tokenizer.tokens.is_empty());
    }
}
