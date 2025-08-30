use std::io::Read;

pub fn resolve_message_structs(source_directory: &'static str) {
    let mut message_structs = Vec::new();

    for try_entry in walkdir::WalkDir::new(source_directory) {
        if let Ok(entry) = try_entry {
            let path = entry.path();

            if !path.is_file() {
                continue;
            }

            if let Some(ext) = path.extension() {
                if ext != "rs" {
                    continue;
                }
            } else {
                continue;
            }

            let mut file = {
                match std::fs::File::open(path) {
                    Ok(f) => f,
                    Err(err) => {
                        println!("Error while opening file: {err}");
                        continue;
                    }
                }
            };

            let mut file_text = String::new();
            if let Err(err) = file.read_to_string(&mut file_text) {
                println!("Error while reading file: {err}");
                continue;
            }

            let ast = syn::parse_file(&file_text).expect("Error while parsing file");

            for item in ast.items {
                if let syn::Item::Struct(item_struct) = item {

                    // the label here is strickly speaking unnecessary, but it makes it easier to read.
                    'inner: for attribute in &item_struct.attrs {

                        if attribute.meta.path().is_ident("tauri_message") {
                            message_structs.push(item_struct);
                            break 'inner;
                        }
                    }
                }
            }
        }
    }
}
