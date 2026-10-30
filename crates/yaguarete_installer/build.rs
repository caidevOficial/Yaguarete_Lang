fn main() {
    println!("cargo:rerun-if-changed=../../yaguarete_icon.ico");
    #[cfg(windows)]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("../../yaguarete_icon.ico");
        res.set("ProductName", "Instalador YAGUARETÉ");
        res.set("FileDescription", "Instalador Gráfico Todo-en-Uno del Lenguaje YAGUARETÉ");
        res.set("LegalCopyright", "Copyright (c) 2026 Facu Falcone");
        res.set("FileVersion", "0.1.1.0");
        res.set("ProductVersion", "0.1.1.0");
        if let Err(e) = res.compile() {
            eprintln!("Warning: could not compile Windows resource: {}", e);
        }
    }
}
