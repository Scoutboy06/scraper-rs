#![allow(unused)]

use reqwest::header::DATE;

pub struct Attribute {
    name: String,
    value: String,
}

struct Doctype {
    name: Option<String>,
    public_id: Option<String>,
    system_id: Option<String>,
    force_quirks: bool,
}
struct StartTag {
    tag_name: String,
    self_closing: bool,
    attributes: Vec<Attribute>,
}
struct EndTag {
    tag_name: String,
}

enum Tag {
    StartTag(StartTag),
    EndTag(EndTag),
}

pub enum Token {
    Doctype(Doctype),
    StartTag(StartTag),
    EndTag(EndTag),
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
    ScriptDataDoubleEscaped,
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
    /*
    HTML Standard:
    https://html.spec.whatwg.org/multipage/parsing.html#tokenization

    Actions:
    - Emit token => tokens.push()
    - Switch state => current_state = State::
    - Switch return state => return_state = State::
    - Reconsume => i -= 1
    - Temporary buffer => temporary_buffer
        - Add => temporary_buffer.push()
        - Clear => temporary_buffer.clear()
    - Current tag token => current_tag
        - New => current_tag = Tag::StartTag() | Tag::EndTag()
        - Push tag name => current_tag.tag_name.push()

    - Appropriate end tag token
        - `start_tags` is a vector containing all created start tags
        - End tag is appropriate if it matches the last emitted start tag name
        - When a start tag is to be emitted, it is cloned into `tokens`.
    */

    let chars = html.chars().collect::<Vec<char>>();

    let mut tokens: Vec<Token> = Vec::new();
    let mut current_state = State::Data;
    let mut return_state = State::Data;
    let mut temporary_buffer = String::new();
    let mut current_tag: Option<Tag> = None;
    let mut current_attribute: Option<&Attribute> = None;
    let mut open_start_tags: Vec<&StartTag> = Vec::new();

    let mut ch: char;
    let mut eof = false; // End of file
    let mut i = 0;
    while i <= chars.len() {
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
                _ => tokens.push(Token::Character(ch)),
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
                _ => tokens.push(Token::Character(ch)),
            },

            State::Rawtext => match ch {
                '<' => current_state = State::RawtextLessThanSign,
                _ if eof => tokens.push(Token::EndOfFile),
                _ => tokens.push(Token::Character(ch)),
            },

            State::ScriptData => match ch {
                '<' => current_state = State::ScriptDataLessThanSign,
                _ if eof => tokens.push(Token::EndOfFile),
                _ => tokens.push(Token::Character(ch)),
            },

            State::Plaintext => match ch {
                _ if eof => tokens.push(Token::EndOfFile),
                _ => tokens.push(Token::Character(ch)),
            },

            State::TagOpen => match ch {
                '!' => current_state = State::MarkupDeclarationOpen,
                '/' => current_state = State::EndTagOpen,
                _ if ch.is_ascii_alphabetic() => {
                    current_tag = Some(Tag::StartTag(StartTag {
                        tag_name: String::new(),
                        self_closing: false,
                        attributes: Vec::new(),
                    }));
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
                _ => {
                    tokens.push(Token::Character('<'));
                    i -= 1;
                    current_state = State::Data;
                }
            },

            State::EndTagOpen => match ch {
                _ if ch.is_ascii_alphabetic() => {
                    current_tag = Some(Tag::EndTag(EndTag {
                        tag_name: String::new(),
                    }));
                    i -= 1;
                    current_state = State::TagName;
                }
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
                _ if ch.is_ascii_uppercase() => match &mut current_tag {
                    Some(Tag::StartTag(tag)) => tag.tag_name.push(ch.to_ascii_lowercase()),
                    Some(Tag::EndTag(tag)) => tag.tag_name.push(ch.to_ascii_lowercase()),
                    None => unreachable!(),
                },
                _ if eof => tokens.push(Token::EndOfFile),
                _ => match &mut current_tag {
                    Some(Tag::StartTag(tag)) => tag.tag_name.push(ch),
                    Some(Tag::EndTag(tag)) => tag.tag_name.push(ch),
                    None => unreachable!(),
                },
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
                _ if ch.is_ascii_alphabetic() => {
                    current_tag = Some(Tag::EndTag(EndTag {
                        tag_name: String::new(),
                    }));
                    i -= 1;
                    current_state = State::Rcdata;
                }
                _ => {
                    tokens.push(Token::Character('<'));
                    tokens.push(Token::Character('/'));
                    i -= 1;
                    current_state = State::Rcdata;
                }
            },

            State::RcdataEndTagName => {
                let is_appropriate = is_appropriate_end_tag(&current_tag, &open_start_tags);

                match ch {
                    // Tab | Line feed (LF) | Form feed (FF) | Space
                    '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}' if is_appropriate => {
                        current_state = State::BeforeAttributeName;
                    }
                    '/' if is_appropriate => current_state = State::SelfClosingStartTag,
                    '>' if is_appropriate => {
                        current_state = State::Data;
                        tokens.push(Token::Character(ch));
                    }
                    _ if ch.is_ascii_uppercase() => {
                        match &mut current_tag {
                            Some(Tag::StartTag(tag)) => tag.tag_name.push(ch.to_ascii_lowercase()),
                            Some(Tag::EndTag(tag)) => tag.tag_name.push(ch.to_ascii_lowercase()),
                            None => unreachable!(),
                        }

                        temporary_buffer.push(ch);
                    }
                    _ if ch.is_ascii_lowercase() => {
                        match &mut current_tag {
                            Some(Tag::StartTag(tag)) => tag.tag_name.push(ch),
                            Some(Tag::EndTag(tag)) => tag.tag_name.push(ch),
                            None => unreachable!(),
                        }

                        temporary_buffer.push(ch);
                    }
                    _ => {
                        tokens.push(Token::Character('<'));
                        tokens.push(Token::Character('/'));

                        for buffer_char in temporary_buffer.chars() {
                            tokens.push(Token::Character(buffer_char));
                        }

                        i -= 1;
                        current_state = State::Rcdata;
                    }
                }
            }

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
                    current_tag = Some(Tag::EndTag(EndTag {
                        tag_name: String::new(),
                    }));
                    i -= 1;
                    current_state = State::RawtextEndTagName;
                }
                _ => {
                    tokens.push(Token::Character('<'));
                    tokens.push(Token::Character('/'));
                    i -= 1;
                    current_state = State::Rawtext;
                }
            },

            State::RawtextEndTagName => {
                let is_appropriate = is_appropriate_end_tag(&current_tag, &open_start_tags);

                match ch {
                    // Tab | Line feed (LF) | Form feed (FF) | Space
                    '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}' if is_appropriate => {
                        current_state = State::BeforeAttributeName;
                    }
                    '/' if is_appropriate => current_state = State::SelfClosingStartTag,
                    '>' if is_appropriate => {
                        current_state = State::Data;
                        tokens.push(Token::Character(ch));
                    }
                    _ if ch.is_ascii_uppercase() => {
                        match &mut current_tag {
                            Some(Tag::StartTag(tag)) => tag.tag_name.push(ch.to_ascii_lowercase()),
                            Some(Tag::EndTag(tag)) => tag.tag_name.push(ch.to_ascii_lowercase()),
                            None => unreachable!(),
                        }

                        temporary_buffer.push(ch);
                    }
                    _ if ch.is_ascii_lowercase() => {
                        match &mut current_tag {
                            Some(Tag::StartTag(tag)) => tag.tag_name.push(ch),
                            Some(Tag::EndTag(tag)) => tag.tag_name.push(ch),
                            None => unreachable!(),
                        }

                        temporary_buffer.push(ch);
                    }
                    _ => {
                        tokens.push(Token::Character('<'));
                        tokens.push(Token::Character('/'));

                        for c in temporary_buffer.chars() {
                            tokens.push(Token::Character(c));
                        }

                        i -= 1;
                        current_state = State::Rawtext;
                    }
                }
            }

            State::ScriptDataLessThanSign => match ch {
                '/' => {
                    temporary_buffer.clear();
                    current_state = State::ScriptDataEndTagOpen;
                }
                '!' => {
                    current_state = State::ScriptDataEscapeStart;
                    tokens.push(Token::Character('<'));
                    tokens.push(Token::Character('!'));
                }
                _ => {
                    tokens.push(Token::Character('<'));
                    i -= 1;
                    current_state = State::ScriptData;
                }
            },

            State::ScriptDataEndTagOpen => match ch {
                _ if ch.is_ascii_alphabetic() => {
                    current_tag = Some(Tag::EndTag(EndTag {
                        tag_name: String::new(),
                    }));

                    i -= 1;
                    current_state = State::ScriptDataEndTagName;
                }
                _ => {
                    tokens.push(Token::Character('<'));
                    tokens.push(Token::Character('/'));
                    i -= 1;
                    current_state = State::ScriptData;
                }
            },

            State::ScriptDataEndTagName => {
                let is_appropriate = is_appropriate_end_tag(&current_tag, &open_start_tags);

                match ch {
                    // Tab | Line feed (LF) | Form feed (FF) | Space
                    '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}' if is_appropriate => {
                        current_state = State::BeforeAttributeName;
                    }
                    '/' if is_appropriate => current_state = State::SelfClosingStartTag,
                    '>' if is_appropriate => {
                        current_state = State::Data;
                        tokens.push(Token::Character(ch));
                    }
                    _ if ch.is_ascii_uppercase() => {
                        match &mut current_tag {
                            Some(Tag::StartTag(tag)) => tag.tag_name.push(ch.to_ascii_lowercase()),
                            Some(Tag::EndTag(tag)) => tag.tag_name.push(ch.to_ascii_lowercase()),
                            None => unreachable!(),
                        }

                        temporary_buffer.push(ch);
                    }
                    _ if ch.is_ascii_lowercase() => {
                        match &mut current_tag {
                            Some(Tag::StartTag(tag)) => tag.tag_name.push(ch),
                            Some(Tag::EndTag(tag)) => tag.tag_name.push(ch),
                            None => unreachable!(),
                        }

                        temporary_buffer.push(ch);
                    }
                    _ => {
                        tokens.push(Token::Character('<'));
                        tokens.push(Token::Character('/'));

                        for c in temporary_buffer.chars() {
                            tokens.push(Token::Character(c));
                        }

                        i -= 1;
                        current_state = State::Rawtext;
                    }
                }
            }

            State::ScriptDataEscapeStart => match ch {
                '-' => {
                    current_state = State::ScriptDataEscapeStartDash;
                    tokens.push(Token::Character('-'));
                }
                _ => {
                    i -= 1;
                    current_state = State::ScriptData;
                }
            },

            State::ScriptDataEscapeStartDash => match ch {
                '-' => {
                    current_state = State::ScriptDataEscapedDashDash;
                    tokens.push(Token::Character('-'));
                }
                _ => {
                    i -= 1;
                    current_state = State::ScriptData;
                }
            },

            State::ScriptDataEscaped => match ch {
                '-' => {
                    current_state = State::ScriptDataEscapedDash;
                    tokens.push(Token::Character('-'));
                }
                '<' => current_state = State::ScriptDataEscapedLessThanSign,
                _ if eof => tokens.push(Token::EndOfFile),
                _ => tokens.push(Token::Character(ch)),
            },

            State::ScriptDataEscapedDash => match ch {
                '-' => {
                    current_state = State::ScriptDataEscapedDashDash;
                    tokens.push(Token::Character('-'));
                }
                '<' => current_state = State::ScriptDataEscapedLessThanSign,
                _ if eof => tokens.push(Token::EndOfFile),
                _ => {
                    current_state = State::ScriptDataEscaped;
                    tokens.push(Token::Character(ch));
                }
            },

            State::ScriptDataEscapedDashDash => match ch {
                '-' => tokens.push(Token::Character('-')),
                '<' => current_state = State::ScriptDataEscapedLessThanSign,
                '>' => {
                    current_state = State::ScriptData;
                    tokens.push(Token::Character('>'));
                }
                _ if eof => tokens.push(Token::EndOfFile),
                _ => {
                    current_state = State::ScriptDataEscaped;
                    tokens.push(Token::Character(ch));
                }
            },

            State::ScriptDataEscapedLessThanSign => match ch {
                '/' => {
                    temporary_buffer.clear();
                    current_state = State::ScriptDataEscapedEndTagOpen;
                }
                _ if ch.is_ascii_alphabetic() => {
                    temporary_buffer.clear();
                    tokens.push(Token::Character('<'));
                    i -= 1;
                    current_state = State::ScriptDataDoubleEscapeStart;
                }
                _ => {
                    tokens.push(Token::Character('<'));
                    i -= 1;
                    current_state = State::ScriptDataEscaped;
                }
            },

            State::ScriptDataEscapedEndTagOpen => match ch {
                _ if ch.is_ascii_alphabetic() => {
                    current_tag = Some(Tag::EndTag(EndTag {
                        tag_name: String::new(),
                    }));

                    i -= 1;
                    current_state = State::ScriptDataEscapedEndTagName;
                }
                _ => {
                    tokens.push(Token::Character('<'));
                    tokens.push(Token::Character('/'));
                    i -= 1;
                    current_state = State::ScriptDataEscaped;
                }
            },

            State::ScriptDataEscapedEndTagName => {
                let is_appropriate = is_appropriate_end_tag(&current_tag, &open_start_tags);

                match ch {
                    // Tab | Line feed (LF) | Form feed (FF) | Space
                    '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}' if is_appropriate => {
                        current_state = State::BeforeAttributeName;
                    }
                    '/' if is_appropriate => current_state = State::SelfClosingStartTag,
                    '>' if is_appropriate => {
                        current_state = State::Data;
                        tokens.push(Token::Character(ch));
                    }
                    _ if ch.is_ascii_uppercase() => {
                        match &mut current_tag {
                            Some(Tag::StartTag(tag)) => tag.tag_name.push(ch.to_ascii_lowercase()),
                            Some(Tag::EndTag(tag)) => tag.tag_name.push(ch.to_ascii_lowercase()),
                            None => unreachable!(),
                        };

                        temporary_buffer.push(ch);
                    }

                    _ if ch.is_ascii_lowercase() => {
                        match &mut current_tag {
                            Some(Tag::StartTag(tag)) => tag.tag_name.push(ch),
                            Some(Tag::EndTag(tag)) => tag.tag_name.push(ch),
                            None => unreachable!(),
                        };

                        temporary_buffer.push(ch);
                    }

                    _ => {
                        tokens.push(Token::Character('<'));
                        tokens.push(Token::Character('/'));

                        for buff_char in temporary_buffer.chars() {
                            tokens.push(Token::Character(buff_char));
                        }

                        i -= 1;
                        current_state = State::ScriptData;
                    }
                }
            }

            State::ScriptDataEscapeStart => match ch {
                '-' => {
                    current_state = State::ScriptDataEscapeStartDash;
                    tokens.push(Token::Character('-'));
                }
                _ => {
                    i -= 1;
                    current_state = State::ScriptData;
                }
            },

            State::ScriptDataEscapeStartDash => match ch {
                '-' => {
                    current_state = State::ScriptDataEscapedDashDash;
                    tokens.push(Token::Character('-'));
                }
                _ => {
                    i -= 1;
                    current_state = State::ScriptData;
                }
            },

            State::ScriptDataEscaped => match ch {
                '-' => {
                    current_state = State::ScriptDataEscapedDash;
                    tokens.push(Token::Character('-'));
                }
                '<' => current_state = State::ScriptDataEscapedLessThanSign,
                _ if eof => tokens.push(Token::EndOfFile),
                _ => tokens.push(Token::Character(ch)),
            },

            State::ScriptDataEscapedDash => match ch {
                '-' => {
                    current_state = State::ScriptDataEscapedDashDash;
                    tokens.push(Token::Character('-'));
                }
                '<' => current_state = State::ScriptDataEscapedLessThanSign,
                _ if eof => tokens.push(Token::EndOfFile),
                _ => {
                    current_state = State::ScriptDataEscaped;
                    tokens.push(Token::Character(ch));
                }
            },

            State::ScriptDataEscapedDashDash => match ch {
                '-' => tokens.push(Token::Character('-')),
                '<' => current_state = State::ScriptDataEscapedLessThanSign,
                '>' => {
                    current_state = State::ScriptData;
                    tokens.push(Token::Character('>'));
                }
                _ if eof => tokens.push(Token::EndOfFile),
                _ => {
                    current_state = State::ScriptDataEscaped;
                    tokens.push(Token::Character(ch));
                }
            },

            State::ScriptDataEscapedLessThanSign => match ch {
                '/' => {
                    temporary_buffer.clear();
                    current_state = State::ScriptDataEscapedEndTagOpen;
                }
                _ if ch.is_ascii_alphabetic() => {
                    temporary_buffer.clear();
                    tokens.push(Token::Character('<'));
                    i -= 1;
                    current_state = State::ScriptDataDoubleEscapeStart;
                }
                _ => {
                    tokens.push(Token::Character('<'));
                    i -= 1;
                    current_state = State::ScriptDataEscaped;
                }
            },

            State::ScriptDataEscapedEndTagOpen => match ch {
                _ if ch.is_ascii_alphabetic() => {
                    current_tag = Some(Tag::EndTag(EndTag {
                        tag_name: String::new(),
                    }));

                    i -= 1;
                    current_state = State::ScriptDataEscapedEndTagName;
                }
                _ => {
                    tokens.push(Token::Character('<'));
                    tokens.push(Token::Character('/'));
                    i -= 1;
                    current_state = State::ScriptDataEscaped;
                }
            },

            State::ScriptDataEscapedEndTagName => {
                let is_appropriate = is_appropriate_end_tag(&current_tag, &open_start_tags);

                match ch {
                    // Tab | Line feed (LF) | Form feed (FF) | Space
                    '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}' if is_appropriate => {
                        current_state = State::BeforeAttributeName;
                    }
                    '/' if is_appropriate => current_state = State::SelfClosingStartTag,
                    '>' if is_appropriate => {
                        current_state = State::Data;
                        tokens.push(Token::Character(ch));
                    }
                    _ if ch.is_ascii_uppercase() => {
                        match &mut current_tag {
                            Some(Tag::StartTag(tag)) => tag.tag_name.push(ch.to_ascii_lowercase()),
                            Some(Tag::EndTag(tag)) => tag.tag_name.push(ch.to_ascii_lowercase()),
                            None => unreachable!(),
                        };

                        temporary_buffer.push(ch);
                    }

                    _ if ch.is_ascii_lowercase() => {
                        match &mut current_tag {
                            Some(Tag::StartTag(tag)) => tag.tag_name.push(ch),
                            Some(Tag::EndTag(tag)) => tag.tag_name.push(ch),
                            None => unreachable!(),
                        };

                        temporary_buffer.push(ch);
                    }

                    _ => {
                        tokens.push(Token::Character('<'));
                        tokens.push(Token::Character('/'));

                        for buff_char in temporary_buffer.chars() {
                            tokens.push(Token::Character(buff_char));
                        }

                        i -= 1;
                        current_state = State::ScriptData;
                    }
                }
            }

            State::ScriptDataDoubleEscapeStart => match ch {
                '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}' | '/' | '>' => {
                    if temporary_buffer == "script".to_string() {
                        current_state = State::ScriptDataDoubleEscaped;
                    } else {
                        current_state = State::ScriptDataEscaped;
                    }

                    tokens.push(Token::Character(ch));
                }
                _ if ch.is_ascii_uppercase() => {
                    match &mut current_tag {
                        Some(Tag::StartTag(tag)) => tag.tag_name.push(ch.to_ascii_lowercase()),
                        Some(Tag::EndTag(tag)) => tag.tag_name.push(ch.to_ascii_lowercase()),
                        None => unreachable!(),
                    };

                    tokens.push(Token::Character(ch));
                }

                _ if ch.is_ascii_lowercase() => {
                    match &mut current_tag {
                        Some(Tag::StartTag(tag)) => tag.tag_name.push(ch),
                        Some(Tag::EndTag(tag)) => tag.tag_name.push(ch),
                        None => unreachable!(),
                    };

                    tokens.push(Token::Character(ch));
                }

                _ => {
                    i -= 1;
                    current_state = State::ScriptDataEscaped;
                }
            },

            State::ScriptDataDoubleEscaped => match ch {
                '-' => {
                    current_state = State::ScriptDataDoubleEscapedDash;
                    tokens.push(Token::Character('-'));
                }
                '<' => {
                    current_state = State::ScriptDataDoubleEscapedLessThanSign;
                    tokens.push(Token::Character('<'));
                }
                _ if eof => tokens.push(Token::EndOfFile),
                _ => tokens.push(Token::Character(ch)),
            },

            State::ScriptDataDoubleEscapedDash => match ch {
                '-' => {
                    current_state = State::ScriptDataDoubleEscapedDashDash;
                    tokens.push(Token::Character('-'));
                }
                '<' => {
                    current_state = State::ScriptDataDoubleEscapedLessThanSign;
                    tokens.push(Token::Character('<'));
                }
                _ if eof => tokens.push(Token::EndOfFile),
                _ => {
                    current_state = State::ScriptDataDoubleEscaped;
                    tokens.push(Token::Character(ch));
                }
            },

            State::ScriptDataDoubleEscapedDashDash => match ch {
                '-' => tokens.push(Token::Character('-')),
                '<' => {
                    current_state = State::ScriptDataDoubleEscapedLessThanSign;
                    tokens.push(Token::Character('<'));
                }
                '>' => {
                    current_state = State::ScriptData;
                    tokens.push(Token::Character('>'));
                }
                _ if eof => tokens.push(Token::EndOfFile),
                _ => {
                    current_state = State::ScriptDataDoubleEscaped;
                    tokens.push(Token::Character(ch));
                }
            },

            State::ScriptDataDoubleEscapedLessThanSign => match ch {
                '/' => {
                    temporary_buffer.clear();
                    current_state = State::ScriptDataDoubleEscapeEnd;
                    tokens.push(Token::Character('/'));
                }
                _ => {
                    i -= 1;
                    current_state = State::ScriptDataDoubleEscaped;
                }
            },

            State::ScriptDataDoubleEscapeEnd => match ch {
                '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}' | '/' | '>' => {
                    if temporary_buffer == "script".to_string() {
                        current_state = State::ScriptDataEscaped;
                    } else {
                        current_state = State::ScriptDataDoubleEscaped;
                    }

                    tokens.push(Token::Character(ch));
                }

                _ if ch.is_ascii_uppercase() => {
                    temporary_buffer.push(ch.to_ascii_lowercase());
                    tokens.push(Token::Character(ch));
                }

                _ if ch.is_ascii_lowercase() => {
                    temporary_buffer.push(ch);
                    tokens.push(Token::Character(ch));
                }

                _ => {
                    i -= 1;
                    current_state = State::ScriptDataDoubleEscaped;
                }
            },

            State::BeforeAttributeName => match ch {
                // Tab | Line feed (LF) | Form feed (FF) | Space
                '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}' => {}
                '/' | '>' | _ if eof => {
                    i -= 1;
                    current_state = State::AfterAttributeName;
                }
                '=' => match &mut current_tag {
                    Some(Tag::StartTag(tag)) => {
                        tag.attributes.push(Attribute {
                            name: String::from(ch),
                            value: String::new(),
                        });

                        current_state = State::AttributeName;
                    }

                    _ => unreachable!(),
                },

                _ => match &mut current_tag {
                    Some(Tag::StartTag(tag)) => {
                        tag.attributes.push(Attribute {
                            name: String::new(),
                            value: String::new(),
                        });
                        i -= 1;
                        current_state = State::AttributeName;
                    }
                    _ => unreachable!(),
                },
            },

            State::AttributeName => match ch {
                '\u{0009}' | '\u{000A}' | '\u{000C}' | '\u{0020}' | '/' | '>' | _ if eof => {
                    i -= 1;
                    current_state = State::AfterAttributeName;
                }
                '=' => current_state = State::BeforeAttributeValue,
                _ if ch.is_ascii_uppercase() => match current_tag {
                    Some(Tag::)
                }
            },
        }

        i += 1;
    }

    tokens
}

fn is_appropriate_end_tag(current_tag: &Option<Tag>, open_start_tags: &Vec<&StartTag>) -> bool {
    return match current_tag {
        Some(Tag::EndTag(end_tag)) => {
            if let Some(last_start_tag) = open_start_tags.last() {
                return end_tag.tag_name == last_start_tag.tag_name;
            } else {
                return false;
            }
        }
        _ => false,
    };
}
