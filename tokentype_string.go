// Code generated by "stringer -type=TokenType"; DO NOT EDIT.

package main

import "strconv"

func _() {
	// An "invalid array index" compiler error signifies that the constant values have changed.
	// Re-run the stringer command to generate them again.
	var x [1]struct{}
	_ = x[tUknown-0]
	_ = x[tNumber-1]
	_ = x[tOpPlus-2]
	_ = x[tOpMinus-3]
	_ = x[tOpMultiply-4]
	_ = x[tOpDivide-5]
	_ = x[tOpModulo-6]
	_ = x[tOpPower-7]
	_ = x[tString-8]
	_ = x[tChar-9]
	_ = x[tTemplateString-10]
	_ = x[tIdentifier-11]
	_ = x[tEqual-12]
	_ = x[tComma-13]
	_ = x[tDot-14]
	_ = x[tColon-15]
	_ = x[tSemicolon-16]
	_ = x[tLeftParen-17]
	_ = x[tRightParen-18]
	_ = x[tLeftBrace-19]
	_ = x[tRightBrace-20]
	_ = x[tLeftBracket-21]
	_ = x[tRightBracket-22]
	_ = x[tExclamationMark-23]
	_ = x[tQuestionMark-24]
	_ = x[tGt-25]
	_ = x[tLt-26]
	_ = x[tGtEq-27]
	_ = x[tLtEq-28]
	_ = x[tEqEq-29]
	_ = x[tNotEq-30]
	_ = x[tBinAnd-31]
	_ = x[tBinOr-32]
	_ = x[tIf-33]
	_ = x[tElse-34]
	_ = x[tWhile-35]
	_ = x[tFor-36]
	_ = x[tFn-37]
	_ = x[tReturn-38]
	_ = x[tEOF-39]
}

const _TokenType_name = "tUknowntNumbertOpPlustOpMinustOpMultiplytOpDividetOpModulotOpPowertStringtChartTemplateStringtIdentifiertEqualtCommatDottColontSemicolontLeftParentRightParentLeftBracetRightBracetLeftBrackettRightBrackettExclamationMarktQuestionMarktGttLttGtEqtLtEqtEqEqtNotEqtBinAndtBinOrtIftElsetWhiletFortFntReturntEOF"

var _TokenType_index = [...]uint16{0, 7, 14, 21, 29, 40, 49, 58, 66, 73, 78, 93, 104, 110, 116, 120, 126, 136, 146, 157, 167, 178, 190, 203, 219, 232, 235, 238, 243, 248, 253, 259, 266, 272, 275, 280, 286, 290, 293, 300, 304}

func (i TokenType) String() string {
	if i < 0 || i >= TokenType(len(_TokenType_index)-1) {
		return "TokenType(" + strconv.FormatInt(int64(i), 10) + ")"
	}
	return _TokenType_name[_TokenType_index[i]:_TokenType_index[i+1]]
}
