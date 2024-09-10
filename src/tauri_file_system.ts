import {get_rust_function} from "./rust_functions";
import {FsType, FileSystem} from "./fsShared.ts";

// TODO: comment in the exception that is thrown when an error is encountered in the functions


/**
 * File system interaction
 *
 * Depending on the build target (WASM/Tauri), this object may either modify the local
 * browser based or system filesystem.
 */
const fs = {
    copyFile: get_rust_function("copy", ["from", "to"]),
    createDir: get_rust_function("create_dir", ["path"]),
    createDirWithParents: get_rust_function("create_dir_with_parents", ["path"]),
    createFile: get_rust_function("create_file", ["path"]),
    readDir: get_rust_function("read_dir", ["path"]),
    readToString: get_rust_function("read_to_string", ["path"]),
    removeDir: get_rust_function("remove_dir", ["path"]),
    removeDirRecursive: get_rust_function("remove_dir_recursive", ["path"]),
    removeFile: get_rust_function("remove_file", ["path"]),
    rename: get_rust_function("rename", ["from", "to"]),
    writeFile: get_rust_function("write_file", ["path", "contents"]),
    type: FsType.Tauri,
} as FileSystem;

export default fs;
