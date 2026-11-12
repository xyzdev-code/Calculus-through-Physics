use std::env;
use std::fs;
enum Environment {
    Math,
    Text,
}
fn parse(text: &String) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();
    let mut has_break_before = true;
    for char in text.chars() {
        if char.is_whitespace() {
            output.push(char.to_string());
            has_break_before = true;
        } else if char.is_control() {
            output.push(char.to_string());
            has_break_before = true;
        } else if has_break_before {
            has_break_before = false;
            output.push(char.to_string());
        } else {
            if let Some(last_item) = output.last_mut() {
                last_item.push(char);
            }
        }
    }
    return output;
}
impl Environment {
    fn ends_math_env(token: &String) -> bool {
        return token.contains("\\end{equation}")
            || token.contains("\\end{equaion*}")
            || token.contains("\\end{align}")
            || token.contains("\\end{align*}")
            || token.contains("\\)");
    }
    fn starts_math_env(token: &String) -> bool {
        return token.contains("\\begin{equation}")
            || token.contains("\\begin{equaion*}")
            || token.contains("\\begin{align}")
            || token.contains("\\begin{align*}")
            || token.contains("\\(");
    }
}
fn main() {
    println!("This is a small linter for LaTEX code. It mainly catches wrong spacing.");
    let mut filenames: Vec<String> = env::args().collect();
    filenames.remove(0);
    for filename in filenames {
        println!("Scanning file {}", filename);
        assert!(
            filename.contains(".tex"),
            "The file {} does not have an .tex extension!",
            filename
        );
        let mut curr_env = Environment::Text;
        println!("Reading file {}", filename);
        let file = fs::read_to_string(&filename).expect("The file does not exist");
        let result = parse(&file);
        let res = result.clone();
        let mut line_number = 1;
        let mut has_break_before = true;
        for (index, token) in &mut result.into_iter().enumerate() {
            match curr_env {
                Environment::Math => {
                    if Environment::ends_math_env(&token) {
                        curr_env = Environment::Text;
                    }
                }
                Environment::Text => {
                    if Environment::starts_math_env(&token) && token.chars().nth(0).unwrap() == '\\'
                    {
                        curr_env = Environment::Math;
                    } else if Environment::starts_math_env(&token) {
                        let index_of_backslash = token.find("\\").unwrap();
                        curr_env = Environment::Math;
                        let mut out: Vec<String> = Vec::new();
                        for idx in 0..res.len()-1 {
                            println!("Index is {}", idx);
                            let tok = res[index - idx].chars().nth(0).unwrap();
                            if tok.is_control() {
                                break;
                            }
                            out.push(res[index - idx].clone());
                        }
                        out.reverse();
                        println!("Out: {}", out.len());
                        let text = out.join("");
                        println!("File {} line {}:", &filename, line_number);
                        println!("{}", text);
                        println!(
                            "{}^ There should be a space here",
                            " ".repeat(index_of_backslash)
                        );
                    }
                    if Environment::ends_math_env(&token) {
                        curr_env = Environment::Text;
                    }
                    if token.chars().nth(0).unwrap().is_control() {
                        line_number += 1;
                        has_break_before = true;
                    } else if token.chars().nth(0).unwrap().is_whitespace() {
                        has_break_before = true
                    }
                    if token.contains("(") && token.chars().nth(0).unwrap() != '(' {
                        let index_of_bracket = token.find("(").unwrap();
                        let mut out: Vec<String> = Vec::new();
                        for idx in 0..res.len()-1 {
                            println!("Index is {} and len is {}", idx, index);
                            let tok = res[index - idx].chars().nth(0).unwrap();
                            if tok.is_control() {
                                break;
                            }
                            out.push(res[index - idx].clone());
                        }
                        out.reverse();
                        println!("Out: {}", out.len());
                        println!("Token: {}", token.len());
                        println!("Index: {}", index_of_bracket);
                        let text = out.join("");
                        println!("File {} line {}:", &filename, line_number);
                        println!("{}", text);
                        println!(
                            "{}^ There should be a space here",
                            " ".repeat(index_of_bracket)
                        );
                    }
                }
            }
        }
    }
}
