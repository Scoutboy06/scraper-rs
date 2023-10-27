pub struct Token<'a> {
    _type: &'a str,
    value: &'a str,
}

pub struct Node<'a> {
    children: &'a Vec<Node<'a>>,
    first_child: &'a Option<Node<'a>>,
    last_child: &'a Option<Node<'a>>,
    previous_sibling: &'a Option<Node<'a>>,
    next_sibling: &'a Option<Node<'a>>,
    parent_node: &'a Node<'a>,
    node_name: &'a str,
}

impl<'a> Node<'a> {
    // fn contains(&self) -> bool {}
    // fn get_attribute(&self, attribute: String) -> Option<String> {}
    // fn get_attribute_list(&self) -> Vec<String> {}
}

pub struct Document {}

pub fn parse_html(html: &String) -> Document {
    // let tag_regex = Regex::new(r"<[a-zA-Z][a-zA-Z0-9-]");
    let chars = html.chars();

    // for (i, ch) in html.chars().enumerate() {
    //     let token: Option<Token> = match ch {
    //         ' ' | '\t' | '\n' => None,
    //         '<' => Some(Token {
    //             _type: "LESS_THAN",
    //             value: "<",
    //         }),
    //         _ => None,
    //     };
    // }

    Document {}
}
