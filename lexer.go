package main

import (
	"bufio"
	"io"
	"strconv"
	"unicode"
	"zelak312/zelak_zl/utils"
)

var operators = map[rune]TokenType{
	'+': tOpPlus,
	'-': tOpMinus,
	'*': tOpMultiply,
	'/': tOpDivide,
	'%': tOpModulo,
	'^': tOpPower,
}

func lex(s *bufio.Reader) Token {
	c, _, err := s.ReadRune()
	if err == io.EOF {
		return newToken(tEOF, nil)
	}

	utils.Panic(err)
	var t Token
	if unicode.IsDigit(c) {
		i, err := lexNumber(s, c)
		utils.Panic(err)

		t = newToken(tNumber, i)
	} else if _type, ok := operators[c]; ok {
		t = newToken(_type, c)
	}

	return t
}

func peekRune(s *bufio.Reader) (rune, error) {
	c, _, err := s.ReadRune()
	if err != nil {
		return c, err
	}

	err = s.UnreadRune()
	return c, err
}

func lexNumber(s *bufio.Reader, c rune) (int64, error) {
	i := string(c)
	for n, err := peekRune(s); unicode.IsDigit(n); n, err = peekRune(s) {
		if err != nil {
			return strconv.ParseInt(i, 10, 64)
		}

		i += string(n)
		s.ReadRune()
	}

	return strconv.ParseInt(i, 10, 64)
}
