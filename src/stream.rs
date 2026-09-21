use std::io::{self, BufRead};
use regex::Regex;

pub struct StreamingSqlReader<R> {
    reader: R,
    buffer: String,
    skip_data: bool,
    unsupported_re: Regex,
}

impl<R: BufRead> StreamingSqlReader<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buffer: String::new(),
            skip_data: false,
            unsupported_re: Regex::new(
                r"(?i)\b(ALTER\s+.*?\s+OWNER\s+TO|CREATE\s+SEQUENCE|CREATE\s+AGGREGATE)\b"
            ).unwrap(),
        }
    }

    pub fn next_statement(&mut self) -> io::Result<Option<String>> {
        loop {
            // First, see if there's already a complete statement in the buffer
            if let Some(stmt) = self.extract_statement() {
                // If it's an unsupported statement, drop it and continue loop
                if self.unsupported_re.is_match(&stmt) {
                    continue;
                }
                return Ok(Some(stmt));
            }

            // Otherwise, read a new line
            let mut line = String::new();
            let bytes_read = self.reader.read_line(&mut line)?;
            if bytes_read == 0 {
                // EOF
                if !self.buffer.trim().is_empty() {
                    let result = std::mem::take(&mut self.buffer);
                    if self.unsupported_re.is_match(&result) {
                        return Ok(None);
                    }
                    return Ok(Some(result));
                }
                return Ok(None);
            }

            if self.skip_data {
                if line.trim_end() == "\\." {
                    self.skip_data = false;
                }
                continue;
            }

            // Quick check for COPY ... FROM stdin;
            // It usually appears on its own line in pg_dump
            if line.to_uppercase().starts_with("COPY ") && line.trim_end().to_uppercase().ends_with(" FROM STDIN;") {
                self.skip_data = true;
                // We return the COPY statement so it can be verified/linted if needed
                self.buffer.push_str(&line);
                let result = std::mem::take(&mut self.buffer);
                return Ok(Some(result));
            }

            self.buffer.push_str(&line);
        }
    }

    fn extract_statement(&mut self) -> Option<String> {
        let chars: Vec<char> = self.buffer.chars().collect();
        let mut i = 0;
        let mut statement_end = None;

        // Reset state because we re-scan the buffer each time (it's small, just one statement usually)
        let mut in_single_quote = false;
        let mut in_double_quote = false;
        let mut in_multiline_comment = false;
        let mut in_single_line_comment = false;

        while i < chars.len() {
            let c = chars[i];
            let next_c = if i + 1 < chars.len() { Some(chars[i + 1]) } else { None };

            if in_single_line_comment {
                if c == '\n' {
                    in_single_line_comment = false;
                }
            } else if in_multiline_comment {
                if c == '*' && next_c == Some('/') {
                    in_multiline_comment = false;
                    i += 1;
                }
            } else if in_single_quote {
                if c == '\'' {
                    if next_c == Some('\'') {
                        i += 1;
                    } else {
                        in_single_quote = false;
                    }
                }
            } else if in_double_quote {
                if c == '"' {
                    if next_c == Some('"') {
                        i += 1;
                    } else {
                        in_double_quote = false;
                    }
                }
            } else {
                if c == '-' && next_c == Some('-') {
                    in_single_line_comment = true;
                    i += 1;
                } else if c == '/' && next_c == Some('*') {
                    in_multiline_comment = true;
                    i += 1;
                } else if c == '\'' {
                    in_single_quote = true;
                } else if c == '"' {
                    in_double_quote = true;
                } else if c == ';' {
                    statement_end = Some(i + 1);
                    break;
                }
            }
            i += 1;
        }

        if let Some(end_idx) = statement_end {
            // Find byte index for the end_idx which is a char index
            let byte_idx = self.buffer.char_indices().nth(end_idx).map(|(idx, _)| idx).unwrap_or(self.buffer.len());
            let statement = self.buffer[..byte_idx].to_string();
            self.buffer = self.buffer[byte_idx..].to_string();
            
            // Trim leading whitespace for the remaining buffer so it doesn't pile up
            if self.buffer.trim_start().is_empty() {
                self.buffer.clear();
            }

            Some(statement)
        } else {
            None
        }
    }
}
