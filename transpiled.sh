#!/bin/bash
function add {
	local n=$1
	local n2=$2
	return $(($n + $n2))
}
add 4 6
added=$?
echo $added
