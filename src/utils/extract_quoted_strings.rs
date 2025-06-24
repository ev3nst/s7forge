pub fn extract_quoted_strings(content: &str) -> Vec<String> {
    let mut quoted_strings = Vec::new();
    let mut chars = content.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '"' {
            let mut quoted_content = String::new();

            while let Some(inner_ch) = chars.next() {
                if inner_ch == '"' {
                    quoted_strings.push(quoted_content);
                    break;
                } else if inner_ch == '\\' {
                    if let Some(escaped_ch) = chars.next() {
                        match escaped_ch {
                            '"' => quoted_content.push('"'),
                            '\\' => quoted_content.push('\\'),
                            'n' => quoted_content.push('\n'),
                            't' => quoted_content.push('\t'),
                            'r' => quoted_content.push('\r'),
                            _ => {
                                quoted_content.push('\\');
                                quoted_content.push(escaped_ch);
                            }
                        }
                    } else {
                        quoted_content.push('\\');
                    }
                } else {
                    quoted_content.push(inner_ch);
                }
            }
        }
    }

    quoted_strings
}
