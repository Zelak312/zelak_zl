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
	_ = x[tTrue-12]
	_ = x[tFalse-13]
	_ = x[tEqual-14]
	_ = x[tComma-15]
	_ = x[tDot-16]
	_ = x[tColon-17]
	_ = x[tTripleDot-18]
	_ = x[tSemicolon-19]
	_ = x[tLeftParen-20]
	_ = x[tRightParen-21]
	_ = x[tLeftBrace-22]
	_ = x[tRightBrace-23]
	_ = x[tLeftBracket-24]
	_ = x[tRightBracket-25]
	_ = x[tExclamationMark-26]
	_ = x[tQuestionMark-27]
	_ = x[tGt-28]
	_ = x[tLt-29]
	_ = x[tGtEq-30]
	_ = x[tLtEq-31]
	_ = x[tEqEq-32]
	_ = x[tNotEq-33]
	_ = x[tBinAnd-34]
	_ = x[tBinOr-35]
	_ = x[tAnd-36]
	_ = x[tOr-37]
	_ = x[tIf-38]
	_ = x[tElse-39]
	_ = x[tWhile-40]
	_ = x[tFor-41]
	_ = x[tIn-42]
	_ = x[tLoop-43]
	_ = x[tBreak-44]
	_ = x[tContinue-45]
	_ = x[tFn-46]
	_ = x[tReturn-47]
	_ = x[tEOF-48]
}

const _TokenType_name = "tUknowntNumbertOpPlustOpMinustOpMultiplytOpDividetOpModulotOpPowertStringtChartTemplateStringtIdentifiertTruetFalsetEqualtCommatDottColontTripleDottSemicolontLeftParentRightParentLeftBracetRightBracetLeftBrackettRightBrackettExclamationMarktQuestionMarktGttLttGtEqtLtEqtEqEqtNotEqtBinAndtBinOrtAndtOrtIftElsetWhiletFortIntLooptBreaktContinuetFntReturntEOF"

var _TokenType_index = [...]uint16{0, 7, 14, 21, 29, 40, 49, 58, 66, 73, 78, 93, 104, 109, 115, 121, 127, 131, 137, 147, 157, 167, 178, 188, 199, 211, 224, 240, 253, 256, 259, 264, 269, 274, 280, 287, 293, 297, 300, 303, 308, 314, 318, 321, 326, 332, 341, 344, 351, 355}

func (i TokenType) String() string {
	if i < 0 || i >= TokenType(len(_TokenType_index)-1) {
		return "TokenType(" + strconv.FormatInt(int64(i), 10) + ")"
	}
	return _TokenType_name[_TokenType_index[i]:_TokenType_index[i+1]]
}
