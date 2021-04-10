export CARGO_BUILD_RUSTDOCFLAGS="-Z unstable-options --static-root-path /bian_rs/"
rm -rf target/doc
cargo +nightly doc --no-deps --release

cp -r target/doc/bian_rs/* target/doc/
rm -rf target/doc/bian_rs

mv  target/doc docs
