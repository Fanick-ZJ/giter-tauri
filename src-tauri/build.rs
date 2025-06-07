use std::fs;
use syn;

fn main() {
    // 生成ts的错误枚举
    generaste_ts_error_enum();
    tauri_build::build()
}


fn generaste_ts_error_enum() {
    let codes = vec![
        (r"crates\giter-utils\src\types\error.rs", "GitUtilsErrorCode"),
        (r"crates\giter-watcher\src\error.rs", "WatcherErrorCode"),
        (r"src\types\error.rs", "CommonErrorCode"),
    ];

    let mut error_code = String::new();
    for code in codes {
        let ts_code = generate_ts_code(code);
        error_code.push_str(&ts_code);
        error_code.push_str("\n");
    }
    error_code.push_str("\n");
    error_code.push_str("export type ErrorCode = typeof CommonErrorCode | typeof GitUtilsErrorCode | typeof WatcherErrorCode");
    fs::write(r"..\src\enum\error.ts", error_code).unwrap();
}

fn generate_ts_code((path, error_name): (&str, &str)) -> String {
    // 相对路径转绝对路径

    let code = fs::read_to_string(path).unwrap();
    let ast = syn::parse_file(&code).unwrap();
    let mut variants = vec![];
    for item in ast.items {
        if let syn::Item::Enum(item_enum) = item {
            if item_enum.ident.to_string().ends_with("ErrorCode") {
                for variant in item_enum.variants {
                    variants.push(variant.ident.to_string());
                }
            }
        }
    }

    let ts_code = format!(
        r#"
export const {} = {{
  {}
}} as const"#, 
        error_name, 
        variants.iter().enumerate()
            .map(|(i, v)| format!("{}: {}", v, i))
            .collect::<Vec<_>>()
            .join(",\n  ")
    );
    return ts_code;
}