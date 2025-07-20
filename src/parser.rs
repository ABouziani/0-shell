#[derive(Debug)]
pub struct ShellParser {
    pub input: String,
    pub position: usize,
}

impl ShellParser {
    pub fn new(input: String) -> Self {
        Self { input, position: 0 }
    }
    
    pub fn parse(&mut self) -> Result<Vec<String>, String> {
        let mut tokens = Vec::new();
        
        while self.position < self.input.len() {
            self.skip_whitespace();
            
            if self.position >= self.input.len() {
                break;
            }
            
            let token = self.parse_token()?;
            if !token.is_empty() {
                tokens.push(token);
            }
        }
        
        Ok(tokens)
    }
    
    pub fn parse_token(&mut self) -> Result<String, String> {
        let mut token = String::new();
        
        while self.position < self.input.len() {
            let ch = self.current_char();
            
            match ch {
                ' ' | '\t' => break,
                '\'' => {
                    self.advance();
                    token.push_str(&self.parse_single_quoted()?);
                }
                '"' => {
                    self.advance();
                    token.push_str(&self.parse_double_quoted()?);
                }
                _ => {
                    token.push(ch);
                    self.advance();
                }
            }
        }
        
        Ok(token)
    }
    
    pub fn parse_single_quoted(&mut self) -> Result<String, String> {
        let mut content = String::new();
        
        while self.position < self.input.len() {
            let ch = self.current_char();
            
            if ch == '\'' {
                self.advance();
                return Ok(content);
            }
            
            content.push(ch);
            self.advance();
        }
        
        Err("quote>".to_string())
    }
    
    pub fn parse_double_quoted(&mut self) -> Result<String, String> {
        let mut content = String::new();
        
        while self.position < self.input.len() {
            let ch = self.current_char();
            
            if ch == '"' {
                self.advance();
                return Ok(content);
            }
            
            if ch == '\\' && self.position + 1 < self.input.len() {
                self.advance();
                let escaped = self.current_char();
                match escaped {
                    '"' | '\\' | '$' | '`' => content.push(escaped),
                    _ => {
                        content.push('\\');
                        content.push(escaped);
                    }
                }
            } else {
                content.push(ch);
            }
            
            self.advance();
        }
        
        Err("dquote>".to_string())
    }
    
    pub fn skip_whitespace(&mut self) {
        while self.position < self.input.len() && 
              self.current_char().is_whitespace() {
            self.advance();
        }
    }
    
    pub fn current_char(&self) -> char {
        self.input.chars().nth(self.position).unwrap_or('\0')
    }
    
    pub fn advance(&mut self) {
        self.position += 1;
    }
}
