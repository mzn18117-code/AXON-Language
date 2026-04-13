use std::collections::HashMap;
use std::env;
use std::fs;

/// --- AXON Sovereign Language Core v1.4 ---
/// ⚖️ The principle of Logical Equilibrium & Decision Making.

#[derive(Debug, PartialEq, Clone)]
enum Token {
    Source,
    Manifest,
    Reveal(String),
    Define(String, f64),
    Calculate(String, String),
    Check(String, String, f64),
    Unknown(String),
}

struct AxonEngine {
    tokens: Vec<Token>,
}

impl AxonEngine {
    fn new(input: &str) -> Self {
        let mut tokens = Vec::new();
        let mut words = Vec::new();
        let mut current_word = String::new();
        let mut in_quotes = false;

        // المحلل اللفظي المطور لمعالجة المسافات داخل علامات التنصيص
        for c in input.chars() {
            if c == '"' { in_quotes = !in_quotes; }
            if c.is_whitespace() && !in_quotes {
                if !current_word.is_empty() {
                    words.push(current_word.clone());
                    current_word.clear();
                }
            } else {
                current_word.push(c);
            }
        }
        if !current_word.is_empty() { words.push(current_word); }

        let mut i = 0;
        while i < words.len() {
            match words[i].as_str() {
                "Source" | "مصدر" => tokens.push(Token::Source),
                "Manifest" | "تجلي" => tokens.push(Token::Manifest),
                "Define" | "تعريف" if i + 2 < words.len() => {
                    let name = words[i + 1].clone();
                    let val: f64 = words[i + 2].parse().unwrap_or(0.0);
                    tokens.push(Token::Define(name, val));
                    i += 2;
                }
                "Calculate" | "احسب" if i + 5 < words.len() => {
                    let target = words[i + 1].clone();
                    let expression = format!("{} {} {}", words[i + 3], words[i + 4], words[i + 5]);
                    tokens.push(Token::Calculate(target, expression));
                    i += 5;
                }
                "Check" | "تحقق" if i + 3 < words.len() => {
                    let var_name = words[i + 1].clone();
                    let operator = words[i + 2].clone();
                    let threshold = words[i + 3].parse().unwrap_or(0.0);
                    tokens.push(Token::Check(var_name, operator, threshold));
                    i += 3;
                }
                "Reveal" | "إظهار" if i + 1 < words.len() => {
                    let msg = words[i + 1].trim_matches('"').to_string();
                    tokens.push(Token::Reveal(msg));
                    i += 1;
                }
                _ => tokens.push(Token::Unknown(words[i].clone())),
            }
            i += 1;
        }
        AxonEngine { tokens }
    }

    fn run(&self) {
        let mut memory: HashMap<String, f64> = HashMap::new();
        let mut condition_met = true; 
        
        let has_source = self.tokens.contains(&Token::Source);
        let has_manifest = self.tokens.contains(&Token::Manifest);

        println!("\n--- 🌐 AXON Sovereign Engine v1.4 ---");
        if has_source && has_manifest {
            println!("⚖️ Status: Equilibrium Established");
            for token in &self.tokens {
                match token {
                    Token::Define(name, val) => { memory.insert(name.clone(), *val); }
                    Token::Calculate(target, expr) => {
                        let parts: Vec<&str> = expr.split_whitespace().collect();
                        let op1 = memory.get(parts[0]).cloned().unwrap_or(0.0);
                        let op2 = memory.get(parts[2]).cloned().unwrap_or(0.0);
                        let result = match parts[1] {
                            "+" => op1 + op2, "-" => op1 - op2,
                            "*" => op1 * op2, "/" => if op2 != 0.0 { op1 / op2 } else { 0.0 },
                            _ => 0.0,
                        };
                        memory.insert(target.clone(), result);
                        println!("⚡ [Math] {} = {}", target, result);
                    }
                    Token::Check(var, op, threshold) => {
                        let val = memory.get(var).cloned().unwrap_or(0.0);
                        condition_met = match op.as_str() {
                            ">" => val > *threshold,
                            "<" => val < *threshold,
                            "==" => val == *threshold,
                            _ => false,
                        };
                        let status = if condition_met { "PASSED" } else { "FAILED" };
                        println!("🔍 [Check] {} {} {} : {}", var, op, threshold, status);
                    }
                    Token::Reveal(msg) => {
                        if condition_met {
                            if let Some(val) = memory.get(msg) { println!("📢 [Output] {}: {}", msg, val); }
                            else { println!("📢 [Log] {}", msg); }
                        }
                    }
                    _ => {}
                }
            }
            println!("-------------------------------------------");
        } else {
            println!("❌ Error: Unbalanced Logic | 'Source' & 'Manifest' required.");
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("🚀 AXON Environment Ready. Use: cargo run <file.ax>");
        return;
    }
    let filename = &args[1];
    match fs::read_to_string(filename) {
        Ok(code) => {
            let engine = AxonEngine::new(&code);
            engine.run();
        }
        Err(_) => println!("❌ Error: Could not read file '{}'", filename),
    }
}
