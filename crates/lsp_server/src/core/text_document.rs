use std::borrow::Cow;

use lsp_types::{Position, Range, TextDocumentContentChangeEvent};
use ropey::Rope;

#[allow(unused)]
#[derive(Clone)]
pub struct TextDocument {
    version: i32,
    uri: String,
    language_id: String,
    content: Rope,
}

impl TextDocument {
    pub fn new(version: i32, uri: String, language_id: String, content: &str) -> Self {
        Self {
            version,
            uri,
            language_id,
            content: content.into(),
        }
    }

    pub fn get_text(&self) -> String {
        self.content.to_string()
    }

    pub fn apply_change(&mut self, change: &TextDocumentContentChangeEvent) {
        let change_text = change.text.as_str();
        let text_bytes = change_text.as_bytes();
        let text_end_byte_index = text_bytes.len();

        let range = match change.range {
            Some(range) => range,
            None => {
                let start = byte_index_to_position(&self.content, 0);
                let end = byte_index_to_position(&self.content, text_end_byte_index);
                Range { start, end }
            }
        };

        let start_index = position_to_byte_index(&self.content, range.start);
        let end_index = position_to_byte_index(&self.content, range.end);

        self.content.remove(start_index..end_index);
        self.content.insert(start_index, change_text);
    }

    pub fn get_text_region(&self, start: Position, end: Position) -> Cow<'_, str> {
        let start = position_to_byte_index(&self.content, start);
        let end = position_to_byte_index(&self.content, end);
        self.content.get_byte_slice(start..end).unwrap().into()
    }
}

pub fn position_to_byte_index(rope: &Rope, position: Position) -> usize {
    let row_index = position.line as usize;
    let column_index = position.character as usize;

    rope.line_to_char(row_index) + rope.utf16_cu_to_char(column_index)
}

pub fn byte_index_to_position(rope: &Rope, byte_index: usize) -> Position {
    let line_index = rope.byte_to_line(byte_index);

    let line_utf16_cu_index = {
        let char_index = rope.line_to_char(line_index);
        rope.char_to_utf16_cu(char_index)
    };

    let character_utf16_cu_index = {
        let char_index = rope.byte_to_char(byte_index);
        rope.char_to_utf16_cu(char_index)
    };

    let character = character_utf16_cu_index - line_utf16_cu_index;

    Position::new(line_index as u32, character as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_insertion() {
        let mut text_document = TextDocument::new(
            0,
            "file:///test.rs".to_string(),
            "rust".to_string(),
            "this is\na\npiece of text",
        );

        let change = TextDocumentContentChangeEvent {
            range: Some(Range {
                start: Position {
                    line: 1,
                    character: 1,
                },
                end: Position {
                    line: 1,
                    character: 1,
                },
            }),
            range_length: None,
            text: " nice".to_string(),
        };

        text_document.apply_change(&change);
        assert_eq!(text_document.get_text(), "this is\na nice\npiece of text");
    }

    #[test]
    fn test_apply_modification() {
        let mut text_document = TextDocument::new(
            0,
            "file:///test.rs".to_string(),
            "rust".to_string(),
            "this is\na\npiece of text",
        );

        let change = TextDocumentContentChangeEvent {
            range: Some(Range {
                start: Position {
                    line: 2,
                    character: 9,
                },
                end: Position {
                    line: 2,
                    character: 13,
                },
            }),
            range_length: None,
            text: "cake".to_string(),
        };

        text_document.apply_change(&change);
        assert_eq!(text_document.get_text(), "this is\na\npiece of cake");
    }

    #[test]
    fn test_position_to_byte_index() {
        let text = "line 1\nline 2\nline 3";
        let rope = Rope::from(text);

        let byte_index = position_to_byte_index(
            &rope,
            Position {
                line: 1,
                character: 5,
            },
        );

        assert_eq!(byte_index, 12);
        assert_eq!(text.as_bytes()[byte_index], b'2');
    }
}
