use std::{fs, path::Path};

fn main() {
    println!("cargo::rerun-if-changed=fonts/icons.toml");
    iced_lucide::build("fonts/icons.toml").expect("build Lucide icons");

    let path = Path::new("src/icons.rs");
    let source = fs::read_to_string(path).expect("read generated Lucide icons");
    let mut generated = source.clone();

    if !generated.contains(".line_height(1.0)") {
        generated = generated.replace(
            r#"text(codepoint).font(Font::new("lucide"))"#,
            r#"text(codepoint).font(Font::new("lucide")).line_height(1.0)"#,
        );
    }

    if source != generated {
        fs::write(path, generated).expect("set the generated Lucide icon line height");
    }
}
