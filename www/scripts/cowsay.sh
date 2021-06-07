#!/bin/sh

if ! command -v cowsay > /dev/null; then
    export PATH="~pbui/pub/pkgsrc/bin:$PATH"
fi

echo "HTTP/1.0 200 OK"
echo "Content-type: text/html"
echo

MESSAGE=$(echo $QUERY_STRING | sed -En 's|.*message=([^&]*).*|\1|p' | sed 's/+/ /g')
TEMPLATE=$(echo $QUERY_STRING | sed -En 's|.*template=([^&]*).*|\1|p' | sed 's/+/ /g')

if [ -z "$TEMPLATE" ]; then
    TEMPLATE=default
fi

cat <<EOF
<h1>Cowsay</h1>
<hr>
<form>
<input type="text" name="message" value="$MESSAGE">
<select name="template">
EOF
for template in $(cowsay -l | awk 'NR > 1'); do
    if [ "$template" = "$TEMPLATE" ]; then
	echo "<option selected>$template</option>"
    else
	echo "<option>$template</option>"
    fi
done

cat <<EOF
</select>
<input type="submit">
</form>
<hr>

EOF

#echo 1>&2 $QUERY_STRING
#echo 1>&2 $MESSAGE
#echo 1>&2 $TEMPLATE

if [ -n "$MESSAGE" ]; then
    cat <<EOF
<pre>
$(cowsay -f "$TEMPLATE" "$MESSAGE")
</pre>
EOF

fi
