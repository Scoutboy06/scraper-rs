mod lexer;
mod scraper;

fn main() {
    // let response = reqwest::blocking::get("http://example.com/").unwrap();
    // let html = response.text().unwrap();

    let html = "<p>Hello World!</p>".to_string();

    let _lex = lexer::html(html);
}
