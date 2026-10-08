fn main() {
    #[cfg(target_arch = "wasm32")]
    deb_credit_rs::mount();
}
