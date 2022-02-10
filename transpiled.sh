#!/bin/bash
function add {
	local num1=$1
	function other {
		function shit {
			function nice {
				echo "oof"
			}
		}
	}
	echo $num1
	if [ $num1 -lt 50 ]
	then
		add $(($num1 + 1))
		added=$?
		return $added
	fi
	return $num1
}
add 1
result=$?
echo $result
