diesel migration run --config-file .\backend\diesel.toml
trunk build -d dist .\frontend\index.html
cargo run --manifest-path .\backend\Cargo.toml