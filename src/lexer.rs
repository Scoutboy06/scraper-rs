use regex::Regex;

pub struct Token<'a>(&'a str, &'a str);

pub struct LexerGenerator {
    // rules: Vec<>,
}

impl LexerGenerator {
    fn build() -> Self {
        Self {}
    }

    fn add(&mut self, token: &str, regex: &str) -> &mut Self {
        let re = Regex::new(regex).unwrap();

        self
    }

    fn ignore(&self, token: &str, regex: &str) -> Self {
        Self {}
    }
}

pub fn html(html: String) -> LexerGenerator {
    let mut lg = LexerGenerator {};
    lg.add(
        "OPENING_TAG",
        r#"<\s*[a-zA-Z_][a-zA-Z0-9_-]*(?:\s+[a-zA-Z_][a-zA-Z0-9_-]*(?:="\w*")*)*\s*\/?>"#,
    );
    // lg.add("ATTRIBUTE", r"[a-zA-Z_][a-zA-Z0-9_-]*()?");
    lg.add("SELF_CLOSING_TAG", r"/>");
    lg.add("CLOSING_TAG", r">");
    lg.add("TEXT", "");

    lg
}
