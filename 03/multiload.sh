#!/bin/bash

MULTICPU=0
PROGNAME=$0

boo() {
	exec >&2
	echo "おたんこなす!"
	exit 1
}

while getopts "m" OPT ; do
	case $OPT in
		m)
			MULTICPU=1
			;;
		\?)
		    boo
		    ;;
	esac
done

shift $((OPTIND - 1))
CONCURRENCY=$1
SCRIPT=$2
if [ $MULTICPU -eq 0 ]; then
   taskset -p -c 0 $$ > /dev/null
fi
for ((i=0;i<CONCURRENCY;i++)) do
	time "$SCRIPT" &
done

for ((i=0;i<CONCURRENCY;i++)) do
	wait
done
