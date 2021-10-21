# Builds the Windows binary
#
# Run in source root directory, the one with the `src` directory in it.
#
rustup target add x86_64-pc-windows-gnu
cargo build --target=x86_64-pc-windows-gnu --release

VERSION=$(awk -F ' = ' '$1 ~ /version/ { gsub(/[\"]/, "", $2); printf("%s",$2) }' Cargo.toml)

cd target/x86_64-pc-windows-gnu/release
zip "gtk-icon-debloat-v$VERSION-x86_64-pc-windows-gnu.zip" gtk-icon-debloat.exe