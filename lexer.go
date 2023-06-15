package main

import (
	"bufio"
	"io"
	"strconv"
	"strings"
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
	'=': tEqual,
	',': tComma,
	'.': tDot,
	':': tColon,
	';': tSemicolon,
	'(': tLeftParen,
	')': tRightParen,
	'{': tLeftBrace,
	'}': tRightBrace,
	'[': tLeftBracket,
	']': tRightBracket,
	'!': tExclamationMark,
	'?': tQuestionMark,
	'>': tGt,
	'<': tLt,
	'&': tBinAnd,
	'|': tBinOr,
}

var operatorsExtended = map[string]TokenType{
	"...": tTripleDot,
	"==":  tEqEq,
	"!=":  tNotEq,
	">=":  tGtEq,
	"<=":  tLtEq,
}

var keywords = map[string]TokenType{
	"if":       tIf,
	"else":     tElse,
	"while":    tWhile,
	"for":      tFor,
	"in":       tIn,
	"loop":     tLoop,
	"break":    tBreak,
	"continue": tContinue,
	"fn":       tFn,
	"return":   tReturn,
	"true":     tTrue,
	"false":    tFalse,
}

var stringType = map[rune]TokenType{
	'"':  tString,
	'\'': tChar,
	'`':  tTemplateString,
}

func lex(s *bufio.Reader) Token {
	c, _, err := s.ReadRune()
	if unicode.IsSpace(c) {
		return lex(s)
	}

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

		extended, err := lexOperatorExtended(s, c)
		if err != io.EOF {
			utils.Panic(err)
		}

		if _type, ok := operatorsExtended[extended]; ok {
			t = newToken(_type, extended)
		}
	} else if _type, ok := stringType[c]; ok {
		str, err := lexString(s, c)
		utils.Panic(err)
		t = newToken(_type, str)
	} else if unicode.IsLetter(c) || c == '_' {
		s, err := lexIdentifier(s, c)
		utils.Panic(err)

		if _type, ok := keywords[s]; ok {
			t = newToken(_type, s)
		} else {
			t = newToken(tIdentifier, s)
		}
	} else {
		t.value = c
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

func lexOperatorExtended(s *bufio.Reader, c rune) (string, error) {
	str := string(c)
	for n, err := peekRune(s); !unicode.IsSpace(n); n, err = peekRune(s) {
		if err != nil {
			return str, err
		}

		str += string(n)
		found := false
		for k := range operatorsExtended {
			if strings.HasPrefix(k, str) {
				s.ReadRune()
				found = true
			}
		}

		if !found {
			break
		}
	}

	return str, nil
}

func lexString(s *bufio.Reader, c rune) (string, error) {
	var str string
	for n, err := peekRune(s); n != c; n, err = peekRune(s) {
		if err != nil {
			return str, err
		}

		if n == '\\' {
			s.ReadRune()
			n, err = peekRune(s)
			if err != nil {
				return str, err
			}

			switch n {
			case 'n':
				str += "\n"
			case 't':
				str += "\t"
			case 'r':
				str += "\r"
			case '\\':
				str += "\\"
			default:
				str += string(n)
			}
		} else {
			str += string(n)
			s.ReadRune()
		}
	}

	s.ReadRune()
	return str, nil
}

func lexIdentifier(s *bufio.Reader, c rune) (string, error) {
	str := string(c)
	for n, err := peekRune(s); unicode.IsLetter(n) || unicode.IsDigit(n) || n == '_'; n, err = peekRune(s) {
		if err != nil {
			return str, err
		}

		str += string(n)
		s.ReadRune()
	}

	return str, nil
}
