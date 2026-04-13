// ==========================================================
// AXON Sovereign Language - Core Engine v1.1
// Developed by: mzn18117-code & Gemini (The Duo)
// Repository: https://github.com/mzn18117-code/AXON-Language
// ==========================================================

#[derive(Debug, Clone, PartialEq)]
enum AxonToken {
    Source,      // مبدأ الأصل
    Manifest,    // مبدأ التجلي
    Measure,     // قانون القياس
    Reveal,      // الكشف (الإخراج)
    Identifier(String),
    StringLiteral(String),
    OpenBrace,
    CloseBrace,
}

struct BalanceEngine {
    /// الميزان: كل Source يضيف ثقلاً، وكل Manifest يحقق التوازن.
    integrity_score: i32,
}

impl BalanceEngine {
    fn new() -> Self {
        Self { integrity_score: 0 }
    }

    /// فحص الاتزان المنطقي (Equilibrium Check)
    fn check_integrity(&mut self, tokens: &[AxonToken]) -> bool {
        for token in tokens {
            match token {
                AxonToken::Source => self.integrity_score += 1,
                AxonToken::Manifest => self.integrity_score -= 1,
                _ => {}
            }
        }
        // يجب أن يعود الميزان للصفر ليكون الكود "سيادياً" ومتزناً
        self.integrity_score == 0
    }
}

/// المحلل اللغوي (Lexer) لتحويل النص إلى رموز منطقية
struct Lexer {
    input: Vec<char>,
    pos: usize,
}

impl Lexer {
    fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            pos: 0,
        }
    }

    fn tokenize(&mut self) -> Vec<AxonToken> {
        let mut tokens = Vec::new();
        while let Some(token) = self.next_token() {
            tokens.push(token);
        }
        tokens
    }

    fn next_token(&mut self) -> Option<AxonToken> {
        self.skip_whitespace();
        if self.pos >= self.input.len() { return None; }

        let c = self.input[self.pos];
        match c {
            '{' => { self.pos += 1; Some(AxonToken::OpenBrace) }
            '}' => { self.pos += 1; Some(AxonToken::CloseBrace) }
            '"' => self.read_string(),
            _ if c.is_alphabetic() => self.read_identifier(),
            _ => { self.pos += 1; None }
        }
    }

    fn read_identifier(&mut self) -> Option<AxonToken> {
        let mut ident = String::new();
        while self.pos < self.input.len() && self.input[self.pos].is_alphanumeric() {
            ident.push(self.input[self.pos]);
            self.pos += 1;
        }
        match ident.as_str() {
            "Source" => Some(AxonToken::Source),
            "Manifest" => Some(AxonToken::Manifest),
            "Measure" => Some(AxonToken::Measure),
            "Reveal" => Some(AxonToken::Reveal),
            _ => Some(AxonToken::Identifier(ident)),
        }
    }

    fn read_string(&mut self) -> Option<AxonToken> {
        self.pos += 1; // Skip "
        let mut s = String::new();
        while self.pos < self.input.len() && self.input[self.pos] != '"' {
            s.push(self.input[self.pos]);
            self.pos += 1;
        }
        self.pos += 1; // Skip "
        Some(AxonToken::StringLiteral(s))
    }

    fn skip_whitespace(&mut self) {
        while self.pos < self.input.len() && self.input[self.pos].is_whitespace() {
            self.pos += 1;
        }
    }
}

fn main() {
    // كود AXON تجريبي لاختبار الميزان
    let axon_code = "Source MyApp { Reveal \"AXON Logic is Operational\" } Manifest";
    
    println!("--- AXON Compiler v1.1 ---");
    println!("Input Code: {}", axon_code);

    let mut lexer = Lexer::new(axon_code);
    let tokens = lexer.tokenize();

    let mut engine = BalanceEngine::new();
    
    println!("Starting Integrity Check...");
    if engine.check_integrity(&tokens) {
        println!("✅ Success: Equilibrium Reached. The code is Sovereign.");
        println!("Tokens Processed: {:?}", tokens.len());
    } else {
        println!("❌ Failure: Violation of Balance. Deployment Blocked.");
        println!("Logic Offset: {}", engine.integrity_score);
    }
}
