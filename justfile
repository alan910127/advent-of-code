create $day: 
    #!/usr/bin/env bash
    set -euo pipefail
    name=day-$(echo "00$day" | tail -c 3)
    cargo generate --path ./template --name $name

work $day $part:
    #!/usr/bin/env bash
    set -euo pipefail
    name=day-$(echo "00$day" | tail -c 3)
    cargo watch -w $name -x "check -p $name" -s "just test $day $part" -s "just lint $day"

test $day $part:    
    #!/usr/bin/env bash
    set -euo pipefail
    name=day-$(echo "00$day" | tail -c 3)
    part="part$part"
    cargo test -p $name $part
    
lint $day:   
    #!/usr/bin/env bash
    set -euo pipefail
    name=day-$(echo "00$day" | tail -c 3)
    cargo clippy -p day-{{replace_regex("00" + day, '\d*(\d{2})$', '$1')}}
    
run $day $part:
    #!/usr/bin/env bash
    set -euo pipefail
    name=day-$(echo "00$day" | tail -c 3)
    part="part$part"
    cargo run -p $name --bin $part
