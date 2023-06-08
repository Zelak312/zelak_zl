package main

import (
	"bufio"
	"flag"
	"fmt"
	"os"
	"zelak312/zelak_zl/utils"
)

func main() {
	input := flag.String("input", "test.zl", "Input file")
	_ = flag.String("output", "transpiled.sh", "Output file")
	debugLexer := flag.Bool("dlexer", false, "Debug lexer")
	flag.Parse()

	f, err := os.OpenFile(*input, os.O_RDONLY, 0644)
	utils.Panic(err)
	defer f.Close()
	r := bufio.NewReader(f)

	if *debugLexer {
		var t Token
		for t._type != tEOF {
			t = lex(r)
			fmt.Printf("%#v\n", t)
		}

		os.Exit(0)
	}
}
