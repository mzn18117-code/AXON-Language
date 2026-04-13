use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
enum Token {
    Source,
    Manifest,
    Reveal(String),
    Define(String, f64),
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
            // إضافة دعم الكلمات العربية والإنجليزية معاً
            match words[i] {
                "Source" | "مصدر" => tokens.push(Token::Source),
                "Manifest" | "تجلي" => tokens.push(Token::Manifest),
                "Define" | "تعريف" if i + 2 < words.len() => {
                    let name = words[i + 1].to_string();
                    let val: f64 = words[i + 2].parse().unwrap_or(0.0);
                    tokens.push(Token::Define(name, val));
                    i += 2;
                }
                "Reveal" | "إظهار" if i + 1 < words.len() => {
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

        println!("--- AXON Phase 4: Sovereign Arabic Logic ---");
        if has_source && has_manifest {
            println!("⚖️ الميزان: مستقر ومنضبط");
            for token in &self.tokens {
                match token {
                    Token::Define(name, val) => {
                        memory.insert(name.clone(), *val);
                        println!("💰 تم تعريف الأصل: {} بقيمة {}", name, val);
                    }
                    Token::Reveal(msg) => {
                        if let Some(val) = memory.get(msg) {
                            println!("📢 رصيد {}: {}", msg, val);
                        } else {
                            println!("📢 تنبيه: {}", msg);
                        }
                    }
                    _ => {}
                }
            }
        } else {
            println!("❌ خطأ: المنطق غير متزن. يجب أن يبدأ بـ 'مصدر' وينتهي بـ 'تجلي'.");
        }
    }
}

fn main() {
    // الآن نكتب الكود بالعربية تماماً!
    let code = r#"
        مصدر
            تعريف الريال_اليمني 750000
            تعريف الريال_السعودي 2500
            إظهار الريال_اليمني
            إظهار الريال_السعودي
            إظهار "تم_تفعيل_السيادة_الرقمية"
        تجلي
    "#;

    let engine = AxonEngine::new(code);
    engine.run();
}
