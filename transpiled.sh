#!/bin/bash
nice=$(expr 10 + 2 \* 2)
other=$(expr $nice \* 5)
printf "%s, %s\n" $nice $other
