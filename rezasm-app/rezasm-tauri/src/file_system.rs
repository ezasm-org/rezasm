use std::fs;

extern crate tauri;

/// Creates a Tauri command from a function that returns () OR an error
macro_rules! void_or_error_command {
    ($fn_name:ident, $wrapped_fn:expr, $( $arg_name:ident : $arg_type:ty ),*) => {
        #[tauri::command]
        fn $fn_name($( $arg_name : $arg_type),*) -> Result<(), String> {
            $wrapped_fn($($arg_name), *).map_err(|err| err.to_string())?;
            Ok(())
        }
    };
}

/// Creates a Tauri command from a function that returns the wrapped function's result OR an error
macro_rules! return_or_error_command {
    ($fn_name:ident, $wrapped_fn:expr, $return_type:ty, $( $arg_name:ident : $arg_type:ty ),*) => {
        #[tauri::command]
        fn $fn_name($( $arg_name : $arg_type),*) -> Result<$return_type, String> {
            $wrapped_fn($($arg_name), *).map_err(|err| err.to_string())
        }
    };
}

return_or_error_command!(tauri_copy, std::fs::copy, u64, from: &str, to: &str);
return_or_error_command!(tauri_read_to_string, std::fs::read_to_string, String, path: &str);

void_or_error_command!(tauri_create_dir, std::fs::create_dir, path: &str);
void_or_error_command!(tauri_create_dir_with_parents, std::fs::create_dir_all, path: &str);
void_or_error_command!(tauri_create_file, std::fs::File::create, path: &str);
void_or_error_command!(tauri_remove_file, std::fs::remove_file, path: &str);
void_or_error_command!(tauri_rename, std::fs::rename, from: &str, to: &str);

// Can only delete empty directory
void_or_error_command!(tauri_remove_dir, std::fs::remove_dir, path: &str);

// Deletes all contents of a (potentially) non-empty directory
void_or_error_command!(tauri_remove_dir_recursive, std::fs::remove_dir_all, path: &str);

#[tauri::command]
fn tauri_read_dir(path: &str) -> Result<Vec<String>, String> {
    Ok(fs::read_dir(path)
        .map_err(|err| err.to_string())?
        .into_iter()
        .filter_map(|entry| entry.ok()?.path().to_str().map(|s| s.to_string()))
        .collect())
}

lazy_static::lazy_static! {
    /// The tauri handler containing all file system methods
    pub static ref HANDLER: Box<dyn Fn(tauri::Invoke) + Send + Sync> =
        Box::new(tauri::generate_handler![
            tauri_copy,
            tauri_create_dir,
            tauri_create_dir_with_parents,
            tauri_create_file,
            tauri_read_dir,
            tauri_read_to_string,
            tauri_remove_dir,
            tauri_remove_dir_recursive,
            tauri_remove_file,
            tauri_rename,
        ]);
}
