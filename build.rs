fn main() {
    println!("cargo:rerun-if-changed=assets/noven.png");
    println!("cargo:rerun-if-changed=assets/noven.ico");

    #[cfg(windows)]
    {
        let mut resource = winresource::WindowsResource::new();
        resource.set_icon("assets/noven.ico");
        resource.set("FileDescription", "Noven Markdown Note Editor");
        resource.set("ProductName", "Noven");
        resource
            .compile()
            .expect("failed to embed Windows application icon");
    }
}
