use proc_macro::TokenStream;
use quote::{TokenStreamExt, quote};
use std::collections::HashSet;
use std::path::Path;
use std::{env, fs};
use syn::{ItemFn, ItemStruct, parse_macro_input};

mod function;
mod info;
use function::{find_workspace_dir, get_workspace_pkg_name};

#[proc_macro_attribute]
pub fn auto_collect_struct_spec(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    let struct_name = input.ident.to_string();

    let crate_name = get_workspace_pkg_name();
    let cmd_full_name = format!("{}::{}", crate_name.replace("-", "_"), struct_name);

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let workspace_root = find_workspace_dir(Path::new(&manifest_dir));
    let structs_dir = workspace_root
        .join("target")
        .join("tauri_auto_register_for_specta")
        .join("structs");
    fs::create_dir_all(&structs_dir).ok();
    let file_path = structs_dir.join(format!("{}.txt", crate_name));

    let mut lines = Vec::new();
    if file_path.exists() {
        if let Ok(content) = fs::read_to_string(&file_path) {
            lines = content
                .lines()
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

#[proc_macro_attribute]
pub fn auto_collect_command_spec(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let fn_name = input.sig.ident.to_string();

    let crate_name = get_workspace_pkg_name();
    let cmd_full_name = format!("{}::{}", crate_name.replace("-", "_"), fn_name);

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let workspace_root = find_workspace_dir(Path::new(&manifest_dir));
    let commands_dir = workspace_root
        .join("target")
        .join("tauri_auto_register_for_specta")
        .join("commands");
    fs::create_dir_all(&commands_dir).ok();
    let file_path = commands_dir.join(format!("{}.txt", crate_name));

    let mut lines = Vec::new();
    if file_path.exists() {
        if let Ok(content) = fs::read_to_string(&file_path) {
            lines = content
                .lines()
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
    let struct_dir = workspace_root
        .join("target")
        .join("tauri_auto_register_for_specta")
        .join("structs");

    let mut commands = HashSet::new();

    if let Ok(entries) = fs::read_dir(struct_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("txt") {
                if let Ok(content) = fs::read_to_string(&path) {
                    for line in content.lines() {
                        let line = line.trim();
                        if line.is_empty() {
                            continue;
                        }

                        // 直接返回完整类型路径
                        commands.insert(line.to_string());
                    }
                }
            }
        }
    }

    commands
}

fn collect_total_commands(_calling_crate: String) -> HashSet<String> {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let workspace_root = find_workspace_dir(Path::new(&manifest_dir));
    let struct_dir = workspace_root
        .join("target")
        .join("tauri_auto_register_for_specta")
        .join("commands");

    let mut commands = HashSet::new();

    if let Ok(entries) = fs::read_dir(struct_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("txt") {
                if let Ok(content) = fs::read_to_string(&path) {
                    for line in content.lines() {
                        let line = line.trim();
                        if line.is_empty() {
                            continue;
                        }

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
    let total_struct_types = collect_total_struct_types(calling_crate.clone());
    let total_commands = collect_total_commands(calling_crate);

    let input_ts = proc_macro2::TokenStream::from(input);
    let mut total_structs_chain = quote! {};
    let mut total_commands_chain = quote! {};

    // ✅ 修复：把字符串转成合法的 Rust 类型
    for ty_str in total_struct_types {
        let ty: syn::Type = syn::parse_str(&ty_str).unwrap();
        total_structs_chain.append_all(quote! { .typ::<#ty>() });
    }

    for cmd_str in total_commands.clone() {
        let cmd: syn::Path = syn::parse_str(&cmd_str).unwrap();
        total_commands_chain.append_all(quote! { #cmd, });
    }

    let commans_chain = if !total_commands.is_empty() {
        quote! { .commands(collect_commands!(#total_commands_chain)) }
    } else {
        quote! {}
    };

    let expanded = quote! {
        #input_ts #total_structs_chain #commans_chain
    };

    TokenStream::from(expanded)
}
