#!/bin/sh

cp -a ./frameworks/non-keyed/xilem_web ./frameworks/non-keyed/xilem_web-$1

find ./frameworks/non-keyed/xilem_web-$1 \( -type d -name .git -prune \) -o -type f -not \( -name "Cargo.toml" -o -name "main.rs" \) -print0 | xargs -0 sed -i "s/xilem_web/xilem_web-$1/g"

sed -z -i "s/xilem_web/xilem_web $1/2" "./frameworks/non-keyed/xilem_web-$1/src/main.rs"
sed -z -i "s/xilem_web/xilem_web-$1/1" "./frameworks/non-keyed/xilem_web-$1/Cargo.toml"
