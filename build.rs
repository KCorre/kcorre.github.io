use std::{fs::File, io::{Read, Write}};

// Example custom build script.
fn main() {
    let mut pres_md = File::open("static/markdown/presentation.md").unwrap();
    let mut pres_html = File::create("static/html/presentation.html").unwrap();
    convert_markdown_to_html(pres_md, pres_html);
    
}

fn convert_markdown_to_html(mut src: File, mut dst: File) {
    let mut content = String::new();
    let md = src.read_to_string(&mut content).unwrap();
    let html = markdown::to_html(&content);
    dst.write_all(html.as_bytes());
}