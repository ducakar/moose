#!/bin/sh

cat > cows.go <<EOF
// Generated. DO NOT EDIT!

package cows

// Cows holds a map of all available name-ASCII cow pairs.
var Cows = map[string]string{
EOF
for i in /usr/share/cows/*.cow; do
   name="`basename $i .cow`"
   echo -ne "\t\"$name\": \`" >> cows.go
   cat $i | sed '
      0,/^$the_cow = <<EOC/ d;
      /^EOC/,$ d;
      s/${*thoughts}*/%[1]c/g;
      s/${*eyes}*/%[2]c%[3]c/g;
      s/${*tongue}*/%[4]c%[5]c/g;
      s/\\\\/\\/g;
      s/\\@/@/g;
      s/`/`+"`"+`/g' >> cows.go
   echo '`,' >> cows.go
done
echo '}' >> cows.go
