fn main() {
    println!("cargo:rerun-if-changed=pdfree.rc");
    println!("cargo:rerun-if-changed=../../assets/pdfree.ico");
    embed_resource::compile_for("pdfree.rc", ["pdfree"], embed_resource::NONE)
        .manifest_required()
        .expect("embed PDFree executable icon");

    slint_build::compile_with_config(
        "ui/app.slint",
        slint_build::CompilerConfiguration::new().with_style("fluent".into()),
    )
    .expect("compile Slint UI");
}
