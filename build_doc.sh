cargo doc --no-deps --release

cp -r target/doc/bian_rs/* target/doc/
rm -rf target/doc/bian_rs

mv  target/doc docs
