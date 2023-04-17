fn main() {
    tonic_build::configure()
        .out_dir("src/")
        .compile(&["src/orderbook.proto"], &["src/"])
        .unwrap();
}
