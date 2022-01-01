class Parser:
    def assert_end(self):
        if self.pos >= len(self.text):
            raise Exception("Reached the end")

    def eat_spaces(self):
        while self.pos < len(self.text) and self.text[self.pos] in " \f\v\r\t\n":
            self.pos += 1
    
    def char(self, chars):
        self.assert_end()
        current = self.text[self.pos]
        for ch in chars:
            if len(ch) == 1 and ch == current:
                self.pos += 1
                return current
            elif ch[0] <= current and ch[2] >= current:
                self.pos += 1
                return current

        raise Exception("Couldn't find char " + chars[0])

    def maybe_char(self, chars):
        try:
            return self.char(chars)
        except:
            return None

    def keyword(self, keywords):
        self.assert_end()
        self.eat_spaces()
        for keyword in keywords:
            low = self.pos
            high = low + len(keyword)
            current = self.text[low:high]
            if current == keyword:
                self.pos = high
                self.eat_spaces()
                return current
        
        raise Exception("Couldn't find keyword")

    def maybe_keyword(self, keywords):
        try:
            return self.keyword(keywords)
        except:
            return None
