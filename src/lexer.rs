#[derive(Debug, PartialEq)]
pub enum Literal {
    Int,
    Str,
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    Identifier,
    Keyword,
    Literal(Literal),
    Operator,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    token_type: TokenType,
    token_data: String,
}

#[derive(Debug)]
pub struct Tokenizer {
    input: Vec<String>,
    tokens: Vec<Token>,
}

impl Tokenizer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.split(" ").map(String::from).collect(),
            tokens: vec![],
        }
    }

    pub fn tokenize(&mut self) {
        let mut in_string = false;
        let mut string_buffer = String::new();

        for word in self.input.clone() {
            if in_string {
                string_buffer.push_str(" ");
                if let Some(pos) = word.find('"') {
                    string_buffer.push_str(&word[..pos]);
                    self.tokens.push(Token {
                        token_type: TokenType::Literal(Literal::Str),
                        token_data: string_buffer.clone(),
                    });

                    string_buffer.clear();
                    in_string = false;

                    let rest = &word[pos + 1..];
                    if !rest.is_empty() {
                        self.process_word(rest);
                    }
                } else {
                    string_buffer.push_str(&word);
                }
                continue;
            }

            if let Some(pos) = word.find('"') {
                if pos > 0 {
                    self.process_word(&word[..pos]);
                }

                in_string = true;
                string_buffer.push_str(&word[pos + 1..]);
            } else {
                self.process_word(&word);
            }
        }
    }

    fn process_word(&mut self, word: &str) {
        let words = word.split_whitespace();
        for word in words {
            let t_type = if Self::is_literal_int(word) {
                TokenType::Literal(Literal::Int)
            } else if Self::is_keyword(word) {
                TokenType::Keyword
            } else if Self::is_identifier(word) {
                TokenType::Identifier
            } else if Self::is_operator(word) {
                TokenType::Operator
            } else {
                println!("Unknown token: {}", word);
                return;
            };

            self.tokens.push(Token {
                token_type: t_type,
                token_data: word.to_string(),
            });
        }
    }

    fn is_literal_int(input: &str) -> bool {
        let is_integer = input.chars().all(|c| c.is_ascii_digit() || c == '-');
        is_integer
    }

    fn is_keyword(input: &str) -> bool {
        let keyword_list = ["main", "in", "out", "int", "str"];
        keyword_list.contains(&input)
    }

    fn is_identifier(input: &str) -> bool {
        if !input.chars().next().unwrap().is_ascii_lowercase() {
            println!(
                "{}",
                "Identifiers should start with a lowercase letter.".to_string()
            );
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
    use crate::lexer::*;
    use crate::Tokenizer;

    #[test]
    fn test_new() {
        let input = "main := \"Hello World\"\n";
        println!("input: {:?}", input);
        println!("Error:");
        let mut tokenizer = Tokenizer::new(input);
        tokenizer.tokenize();
        println!("Tokens:\n{:?}\n", tokenizer.tokens);

        let expected_tokens = vec![
            Token {
                token_type: TokenType::Keyword,
                token_data: "main".to_string(),
            },
            Token {
                token_type: TokenType::Operator,
                token_data: ":=".to_string(),
            },
            Token {
                token_type: TokenType::Literal(Literal::Str),
                token_data: "Hello World".to_string(),
            },
        ];

        assert_eq!(tokenizer.tokens, expected_tokens);
    }
}
