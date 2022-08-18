#! /usr/bin/env bash
AUTHOR=$1
ALL=`git ls-files | wc -l`
TOUCHED=`git ls-files | xargs -I {} sh -c "git blame --porcelain {} | (rg -m 1 '^author $AUTHOR' > /dev/null && echo 1)" | wc -l`
echo "$TOUCHED / $ALL files $(( TOUCHED * 100 / ALL ))% are touched by $AUTHOR"