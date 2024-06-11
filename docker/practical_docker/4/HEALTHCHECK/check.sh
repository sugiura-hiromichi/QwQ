#!/bin/bash
URL=http://localhost/test.html
stat='curl -w %{http_code} -f -s -o /dev/null $URL'
if ["$stat"=="200" -a "$?"=="0"]; then
	exit 0
else
	exit 1
fi
