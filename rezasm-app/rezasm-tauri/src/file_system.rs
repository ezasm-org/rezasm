use std::fs;

extern crate tauri;

/// Creates a Tauri command from a function that returns () OR an error
macro_rules! void_or_error_command {
    ($fn_name:ident, $wrapped_fn:expr, $( $arg_name:ident : $arg_type:ty ),*) => {
        #[tauri::command]
        pub fn $fn_name($( $arg_name : $arg_type),*) -> Result<(), String> {
            $wrapped_fn($($arg_name), *).map_err(|err| err.to_string())?;
            Ok(())
        }
    };
}

/// Creates a Tauri command from a function that returns the wrapped function's result OR an error
macro_rules! return_or_error_command {
    ($fn_name:ident, $wrapped_fn:expr, $return_type:ty, $( $arg_name:ident : $arg_type:ty ),*) => {
        #[tauri::command]
        pub fn $fn_name($( $arg_name : $arg_type),*) -> Result<$return_type, String> {
            $wrapped_fn($($arg_name), *).map_err(|err| err.to_string())
        }
    };
}

return_or_error_command!(tauri_copy_file, fs::copy, u64, from: &str, to: &str);
return_or_error_command!(tauri_read_to_string, fs::read_to_string, String, path: &str);

void_or_error_command!(tauri_create_dir, fs::create_dir, path: &str);
void_or_error_command!(tauri_create_dir_with_parents, fs::create_dir_all, path: &str);
void_or_error_command!(tauri_create_file, fs::File::create, path: &str);
void_or_error_command!(tauri_remove_file, fs::remove_file, path: &str);
void_or_error_command!(tauri_rename, fs::rename, from: &str, to: &str);
void_or_error_command!(tauri_write_file, fs::write, path: &str, contents: &str);

// Can only delete empty directory
void_or_error_command!(tauri_remove_dir, fs::remove_dir, path: &str);

// Deletes all contents of a (potentially) non-empty directory
void_or_error_command!(tauri_remove_dir_recursive, fs::remove_dir_all, path: &str);

#[tauri::command]
pub fn tauri_read_dir(path: &str) -> Result<Vec<(String, bool)>, String> {
    Ok(fs::read_dir(path)
        .map_err(|err| err.to_string())?
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .filter_map(|path| Some((path.to_str()?.to_string(), path.is_dir())))
        .collect())
}