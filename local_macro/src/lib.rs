use proc_macro::TokenStream;
use quote::{TokenStreamExt, quote};
use std::collections::HashSet;
use std::path::Path;
use std::{env, fs};
use syn::{ItemStruct, parse_macro_input};

mod info;
mod function;
use function::{find_workspace_dir, get_workspace_pkg_name};

#[proc_macro_attribute]
pub fn auto_collect_struct_type(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    let struct_name = input.ident.to_string();

    let crate_name = get_workspace_pkg_name();
    let cmd_full_name = format!("{}::{}", crate_name.replace("-", "_"), struct_name);

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let workspace_root = find_workspace_dir(Path::new(&manifest_dir));
    let commands_dir = workspace_root.join("target").join("tauri_auto_register_for_specta");
    fs::create_dir_all(&commands_dir).ok();
    let file_path = commands_dir.join(format!("{}.txt", crate_name));

    let mut lines = Vec::new();
    if file_path.exists() {
        if let Ok(content) = fs::read_to_string(&file_path) {
            lines = content.lines()
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
        }
    }

    if !lines.contains(&cmd_full_name) {
        lines.push(cmd_full_name);
        let _ = fs::write(&file_path, lines.join("\n"));
    }

    quote! { #input }.into()
}

fn collect_total_struct_types(_calling_crate: String) -> HashSet<String> {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let workspace_root = find_workspace_dir(Path::new(&manifest_dir));
    let commands_dir = workspace_root.join("target").join("tauri_auto_register_for_specta");

    let mut commands = HashSet::new();

    if let Ok(entries) = fs::read_dir(commands_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("txt") {
                if let Ok(content) = fs::read_to_string(&path) {
                    for line in content.lines() {
                        let line = line.trim();
                        if line.is_empty() { continue; }

                        // 直接返回完整类型路径
                        commands.insert(line.to_string());
                    }
                }
            }
        }
    }

    commands
}

#[proc_macro]
pub fn auto_add_type_impl(input: TokenStream) -> TokenStream {
    let calling_crate = get_workspace_pkg_name();
    let total_struct_types = collect_total_struct_types(calling_crate);

    let input_ts = proc_macro2::TokenStream::from(input);
    let mut chain = quote! {};

    // ✅ 修复：把字符串转成合法的 Rust 类型
    for ty_str in total_struct_types {
        let ty: syn::Type = syn::parse_str(&ty_str).unwrap();
        chain.append_all(quote! { .typ::<#ty>() });
    }

    let expanded = quote! {
        #input_ts #chain
    };

    TokenStream::from(expanded)
}