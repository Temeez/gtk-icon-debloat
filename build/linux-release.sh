# Builds the Linux binary
#
# Run in source root directory, the one with the `src` directory in it.
#
rustup target add x86_64-unknown-linux-gnu
cargo build --target=x86_64-unknown-linux-gnu --release

VERSION=$(awk -F ' = ' '$1 ~ /version/ { gsub(/[\"]/, "", $2); printf("%s",$2) }' Cargo.toml)

cd target/x86_64-unknown-linux-gnu/release
tar -czvf "gtk-icon-debloat-v$VERSION-x86_64-unknown-linux-gnu.tar.gz" gtk-icon-debloat