fn main() {
    // Compile and link the resources for Windows targets
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default() == "windows" {
        embed_resource::compile("resources.rc", embed_resource::NONE);
    }
}
