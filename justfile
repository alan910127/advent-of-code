create day: 
    cargo generate --path ./template --name {{day}}

work day part:
    cargo watch -w {{day}} -x "check -p {{day}}" -s "just test {{day}} {{part}}" -s "just lint {{day}}"

test day part:
    cargo test -p {{day}} {{part}}
    
lint day:
    cargo clippy -p {{day}}
    
run day part:
    cargo run -p {{day}} --bin {{part}}