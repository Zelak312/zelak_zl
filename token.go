//go:generate stringer -type=TokenType
package main

import "fmt"

type TokenType int

const (
	tUknown TokenType = iota
	tNumber
	tOpPlus
	tOpMinus
	tOpMultiply
	tOpDivide
	tOpModulo
	tOpPower

	tString
	tChar
	tTemplateString
	tIdentifier

	tEqual
	tComma
	tDot
	tColon
	tSemicolon
	tLeftParen
	tRightParen
	tLeftBrace
	tRightBrace
	tLeftBracket
	tRightBracket
	tExclamationMark
	tQuestionMark
	tGt
	tLt
	tGtEq
	tLtEq
	tEqEq
	tNotEq
	tBinAnd
	tBinOr

	tIf
	tElse
	tWhile
	tFor
	tFn
	tReturn

	tEOF
)

type Token struct {
	_type TokenType
	value any
}

func newToken(t TokenType, v any) Token {
	return Token{_type: t, value: v}
}

func (t Token) GoString() string {
	switch v := t.value.(type) {
	case string:
		return fmt.Sprintf("main.Token{_type: %v, value: %q}", t._type, v)
	case rune:
		return fmt.Sprintf("main.Token{_type: %v, value: %q}", t._type, string(v))
	default:
		return fmt.Sprintf("main.Token{_type: %v, value: %v}", t._type, t.value)
	}
}
