use anyhow::Result;
use hypertext::{html_elements, rsx_static};
use proc_macro::TokenStream;
use quote::quote;
use std::path::Path;
use std::{fs, ops::Deref};
use syn::{parse_macro_input, LitStr};

#[proc_macro]
pub fn build_website(input: TokenStream) -> TokenStream {
    let input_str = parse_macro_input!(input as LitStr);
    let output_path = input_str.value();

    match build_website_private(output_path.as_str()) {
        Ok(path) => {
            println!("Website build! Successfully written to {:?}", path);
            quote!().into()
        }
        Err(err) => {
            let error_msg = err.to_string();
            quote!(compile_error!(#error_msg)).into()
        }
    }
}

fn build_website_private(output_path: &str) -> Result<String> {
    let output = Path::new(output_path);
    if output.exists() {
        fs::remove_dir_all(output)?;
    }
    fs::create_dir_all(output)?;
    fs::write(output.join("index.html"), index())?;

    Ok(output.to_string_lossy().deref().to_owned())
}

fn index() -> &'static str {
    rsx_static!(
        <html>
            <head>
                <title>Test Website</title>
            </head>
            <body>
                <h1>How does this even work</h1>
                <p>This is so cursed what the heck</p>
            </body>
        </html>
    )
    .into_inner()
}
