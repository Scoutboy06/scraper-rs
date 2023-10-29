#![allow(unused)]

pub struct Attribute {
    name: String,
    value: String,
}

pub enum Token {
    Doctype {
        name: Option<String>,
        public_id: Option<String>,
        system_id: Option<String>,
        force_quirks: bool,
    },
    StartTag {
        tag_name: String,
        self_closing: bool,
        attributes: Vec<Attribute>,
    },
    EndTag {
        tag_name: String,
        self_closing: bool,
        attributes: Vec<Attribute>,
    },
    Character(char),
    Comment(String),
    EndOfFile,
}

pub enum InsertionMode {
    Initial,
    BeforeHtml,
    BeforeHead,
    InHead,
    InHeadNoscript,
    AfterHead,
    InBody,
    Text,
    InTable,
    InTableText,
    InCaption,
    InColumnGroup,
    InTableBody,
    InRow,
    InCell,
    InSelect,
    InSelectInTable,
    InTemplate,
    AfterBody,
    InFrameset,
    AfterFrameset,
    AfterAfterBody,
    AfterAfterFrameset,
}

pub enum State {
    Data,
    Rcdata,
    Rawtext,
    ScriptData,
    Plaintext,
    TagOpen,
    EndTagOpen,
    TagName,
    RcdataLessThanSign,
    RcdataEndTagOpen,
    RcdataEndTagName,
    RawtextLessThanSign,
    RawtextEndTagOpen,
    RawtextEndTagName,
    ScriptDataLessThanSign,
    ScriptDataEndTagOpen,
    ScriptDataEndTagName,
    ScriptDataEscapeStart,
    ScriptDataEscapeStartDash,
    ScriptDataEscaped,
    ScriptDataEscapedDash,
    ScriptDataEscapedDashDash,
    ScriptDataEscapedLessThanSign,
    ScriptDataEscapedEndTagOpen,
    ScriptDataEscapedEndTagName,
    ScriptDataDoubleEscapeStart,
    ScriptDataDoubleEscapede,
    ScriptDataDoubleEscapedDash,
    ScriptDataDoubleEscapedDashDash,
    ScriptDataDoubleEscapedLessThanSign,
    ScriptDataDoubleEscapeEnd,
    BeforeAttributeName,
    AttributeName,
    AfterAttributeName,
    BeforeAttributeValue,
    AttributeValueDoubleQuoted,
    AttributeValueSingleQuoted,
    AttributeValueUnquoted,
    AfterAttributeValueQuoted,
    SelfClosingStartTag,
    BogusComment,
    MarkupDeclarationOpen,
    CommentStart,
    CommentStartDash,
    Comment,
    CommentLessThanSign,
    CommentLessThanSignBang,
    CommentLessthanSignBangDash,
    CommentLessThanSignBangDashDash,
    CommentEndDash,
    CommentEnd,
    CommentEndBang,
    Doctype,
    BeforeDoctypeName,
    DoctypeName,
    AfterDoctypeName,
    AfterDoctypePublicKeyword,
    BeforeDoctypePublicIdentifier,
    DoctypePublicIdentifierDoubleQuoted,
    DoctypePublicIdentifierSingleQuoted,
    AfterDoctypePublicIdentifier,
    BetweenDoctypePublicAndSystemIdentifiers,
    AfterDoctypeSystemKeyword,
    BeforeDoctypeSystemIdentifier,
    DoctypeSystemIdentifierDoubleQuoted,
    DoctypeSystemIdentifierSingleQuoted,
    AfterDoctypeSystemIdentifier,
    BogusDoctype,
    CDataSection,
    CDataSectionBracket,
    CDataSectionEnd,
    CharacterReference,
    NamedCharacterReference,
    AmbiguousAmpersand,
    NumericCharacterReference,
    HexadecimalCharacterReferenceStart,
    DecimalCharacterReferenceStart,
    HexadecimalCharacterReference,
    DecimalCharacterReference,
    NumericCharacterReferenceEnd,
}

pub fn tokenize(html: &String) -> Vec<Token> {
    let chars = html.chars().collect::<Vec<char>>();
    let mut tokens: Vec<Token> = Vec::new();

    let mut current_state = State::Data;
    let mut return_state = State::Data;
    let mut parser_pause = false;
    let mut tokenized_chars = String::new(); // TODO: Implement this
    let mut current_tag_token_name: &mut String; // TODO: Implement this
    let mut temporary_buffer = String::new();

    let mut ch: char;
    let mut eof = false; // End of file
    let mut i = 0;
    while i <= chars.len() {
        if parser_pause {
            continue;
        }

        if i >= chars.len() {
            ch = '\0';
            eof = true;
        } else {
            ch = chars[i];
        }

        match current_state {
            State::Data => match ch {
                '&' => {
                    return_state = State::Data;
                    current_state = State::CharacterReference;
                }
                '<' => current_state = State::TagOpen,
                _ if eof => tokens.push(Token::EndOfFile),
                _ => tokenized_chars.push(ch),
            },

            State::Rcdata => match ch {
                '&' => {
                    return_state = State::Rcdata;
                    current_state = State::CharacterReference;
                }
                '<' => {
                    current_state = State::RcdataLessThanSign;
                }
                _ if eof => tokens.push(Token::EndOfFile),
                _ => tokenized_chars.push(ch),
            },

            State::Rawtext => match ch {
                '<' => current_state = State::RawtextLessThanSign,
                _ if eof => tokens.push(Token::EndOfFile),
                _ => tokenized_chars.push(ch),
            },

            State::ScriptData => match ch {
                '<' => current_state = State::ScriptDataLessThanSign,
                _ if eof => tokens.push(Token::EndOfFile),
                _ => tokenized_chars.push(ch),
            },

            State::Plaintext => match ch {
                _ if eof => tokens.push(Token::EndOfFile),
                _ => tokenized_chars.push(ch),
            },

            State::TagOpen => match ch {
                '!' => current_state = State::MarkupDeclarationOpen,
                '/' => current_state = State::EndTagOpen,
                _ if ch.is_ascii_alphabetic() => {
                    tokens.push(Token::StartTag {
                        tag_name: String::new(),
                        self_closing: false,
                        attributes: Vec::new(),
                    });

                    i -= 1;
                    current_state = State::TagName;
                }
                '?' => {
                    current_state = State::BogusComment;
                    i -= 1;
                }
                _ if eof => {
                    tokens.push(Token::Character('<'));
                    tokens.push(Token::EndOfFile);
                }
                _ => {}
            },

            State::EndTagOpen => match ch {
                _ if ch.is_ascii_alphabetic() => tokens.push(Token::EndTag {
                    tag_name: String::new(),
                    self_closing: false,
                    attributes: Vec::new(),
                }),
                '>' => current_state = State::Data,
                _ if eof => {
                    tokens.push(Token::Character('<'));
                    tokens.push(Token::Character('/'));
                    tokens.push(Token::EndOfFile);
                }
                _ => {
                    tokens.push(Token::Comment(String::new()));
                    i -= 1;
                    current_state = State::BogusComment;
                }
            },

            State::TagName => match ch {
                // Tab | Line feed (LF) | Form feed (FF) | Space
                '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}' => {
                    current_state = State::BeforeAttributeName
                }
                '/' => current_state = State::SelfClosingStartTag,
                '>' => current_state = State::Data,
                _ if ch.is_ascii_uppercase() => {
                    current_tag_token_name.push(ch.to_ascii_lowercase());
                }
                _ if eof => tokens.push(Token::EndOfFile),
                _ => current_tag_token_name.push(ch),
            },

            State::RcdataLessThanSign => match ch {
                '/' => {
                    temporary_buffer = String::new();
                    current_state = State::RcdataEndTagOpen;
                }
                _ => {
                    tokens.push(Token::Character('<'));
                    i -= 1;
                    current_state = State::Rcdata;
                }
            },

            State::RcdataEndTagOpen => match ch {
                _ if ch.is_ascii_alphabetic() => tokens.push(Token::EndTag {
                    tag_name: String::new(),
                    self_closing: false,
                    attributes: Vec::new(),
                }),
                _ => {
                    tokens.push(Token::Character('<'));
                    tokens.push(Token::Character('/'));
                    i -= 1;
                    current_state = State::Rcdata;
                }
            },

            State::RcdataEndTagName => match ch {
                // Tab | Line feed (LF) | Form feed (FF) | Space
                '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}' => {
                    todo!();
                }
                '/' => todo!(),
                '>' => todo!(),
                _ if ch.is_ascii_uppercase() => {
                    current_tag_token_name.push(ch.to_ascii_lowercase());
                    temporary_buffer.push(ch);
                }
                _ if ch.is_ascii_lowercase() => {
                    current_tag_token_name.push(ch);
                    temporary_buffer.push(ch);
                }
                _ => {
                    for buffer_char in temporary_buffer.chars() {
                        todo!("Emit </ and a character token for");
                    }
                    i -= 1;
                    current_state = State::Rcdata;
                }
            },

            State::RawtextLessThanSign => match ch {
                '/' => {
                    temporary_buffer.clear();
                    current_state = State::RawtextEndTagOpen;
                }
                _ => {
                    tokens.push(Token::Character('<'));
                    i -= 1;
                    current_state = State::Rawtext;
                }
            },

            State::RawtextEndTagOpen => match ch {
                '/' => {
                    temporary_buffer.clear();
                    current_state = State::RawtextEndTagOpen;
                }
                _ => {}
            },
        }

        i += 1;
    }

    tokens
}

pub fn parse(html: &String) {}

// pub fn parser(tokens: Vec<Token>) {
//     // https://html.spec.whatwg.org/multipage/parsing.html#the-insertion-mode
//     let mut insertion_mode = InsertionMode::Initial;
//     let mut original_insertion_mode: Option<InsertionMode> = None;
//     let mut stack_of_template_insertion_modes: Vec<InsertionMode> = Vec::new();
//     let mut current_template_insertion_mode: Option<InsertionMode> = None;

//     // let mut head_element_pointer = None;
//     // let mut form_element_pointer = None;

//     let mut open_elements: Vec<u8> = Vec::new();

//     let mut last = false;
//     let mut node = open_elements.last();

//     // let mut token: Token;
//     let mut i: usize = 0;
//     while i < tokens.len() {
//         // token = tokens[i];

//         i += 1;
//     }
// }
