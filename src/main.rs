use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
enum Token {
    Source,
    Manifest,
    Reveal(String),
    Define(String, f64), // لتعريف متغير مثل العملة
    Unknown(String),
}

struct AxonEngine {
    tokens: Vec<Token>,
}

impl AxonEngine {
    fn new(input: &str) -> Self {
        let mut tokens = Vec::new();
        let words: Vec<&str> = input.split_whitespace().collect();
        let mut i = 0;

        while i < words.len() {
            match words[i] {
                "Source" => tokens.push(Token::Source),
                "Manifest" => tokens.push(Token::Manifest),
                "Define" if i + 2 < words.len() => {
                    let name = words[i + 1].to_string();
                    let val: f64 = words[i + 2].parse().unwrap_or(0.0);
                    tokens.push(Token::Define(name, val));
                    i += 2;
                }
                "Reveal" if i + 1 < words.len() => {
                    let mut msg = words[i+1].to_string();
                    if msg.starts_with('"') { msg = msg.trim_matches('"').to_string(); }
                    tokens.push(Token::Reveal(msg));
                    i += 1;
                }
                _ => tokens.push(Token::Unknown(words[i].to_string())),
            }
            i += 1;
        }

        AxonEngine { tokens }
    }

    fn run(&self) {
        let mut memory: HashMap<String, f64> = HashMap::new();
        let has_source = self.tokens.contains(&Token::Source);
        let has_manifest = self.tokens.contains(&Token::Manifest);

        println!("--- AXON Phase 3: Financial Logic ---");
        if has_source && has_manifest {
            println!("⚖️ Equilibrium: Established.");
            for token in &self.tokens {
                match token {
                    Token::Define(name, val) => {
                        memory.insert(name.clone(), *val);
                        println!("💰 Asset Defined: {} = {}", name, val);
                    }
                    Token::Reveal(msg) => {
                        // محاولة البحث في الذاكرة أولاً
                        if let Some(val) = memory.get(msg) {
                            println!("📢 {} Balance: {}", msg, val);
                        } else {
                            println!("📢 Notification: {}", msg);
                        }
                    }
                    _ => {}
                }
            }
        } else {
            println!("❌ Error: Unbalanced Logic detected.");
        }
    }
}

fn main() {
    // كود AXON يحاكي تعريف عملات
    let code = r#"
        Source
            Define YER 5000
            Define SAR 100
            Reveal YER
            Reveal SAR
            Reveal "Sovereign_System_Active"
        Manifest
    "#;

    let engine = AxonEngine::new(code);
    engine.run();
}
