fn main() {

    // Tell Cargo to rerun this build script if any .slint files change
    println!("cargo:rerun-if-changed=../ui/index.slint");
    // Compile the .slint files into Rust code
    slint_build::compile("../ui/index.slint").unwrap();

    // Tell Cargo to rerun this build script if any .slint files change
    // println!("cargo:rerun-if-changed=ui/main_window.slint");
    // Compile the .slint files into Rust code
    // slint_build::compile("ui/main_window.slint").unwrap();

    //println!("cargo:rerun-if-changed=ui/table_component.slint");

    //slint_build::compile("ui/table_component.slint").unwrap();
}
