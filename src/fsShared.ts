export enum FsType {
    Tauri = 1,
    WasmLocal = 2 // This is WASM but it does not directly write to the filesystem.
}

export interface FileSystem {

    /**
     * Copies a file in the target filesystem
     *
     * @param props a record with the `from` and `to` paths, represented by `string`s.
     * @returns a promise for the number of bytes copied.
     *
     * @example ```typescript
     * let copiedBytes: bigint = await fs.copy({from: "path/to/file", to: "new/path/to/file"});
     * ```
     */
    copyFile(props: {from: string, to: string}): Promise<bigint>;

    /**
     * Creates a new directory in the target filesystem.
     *
     * This error will not create any missing parent directories while creating the directory.
     *
     * @param props a record with the `path` entry that refers to the target path.
     * @returns an empty promise.
     *
     * @example ```typescript
     * const parent = "some/existing/path";
     * const newDirectory = "new_directory_name"
     * await fs.createDir({path: `${parent}/${newDirectory}`});
     * ```
     */
    createDir(props: {path: string}): Promise<void>;

    /**
     * Creates a new directories and all required parents in the target filesystem.
     *
     * @param props a record with the `path` entry that refers to the target path.
     * @returns an empty promise.
     *
     * @example ```typescript
     * await fs.createDirWithParents({path: "path/to/new/dir"});
     * ```
     */
    createDirWithParents(props: {path: string}): Promise<void>;

    /**
     * Creates a new file in the target filesystem.
     *
     * @param props a record with the `path` entry that refers to the target path.
     * @returns an empty promise.
     *
     * @example ```typescript
     * await fs.createFile({path: "path/to/new/dir"});
     * ```
     */
    createFile(props: {path: string}): Promise<void>;

    /**
     * Reads a directory in the target filesystem.
     *
     * @param props a record with the `path` entry that refers to the target path.
     * @returns a promise containing an array of tuples that contain the relative file name followed
     *      by a boolean that is true iff the file is a directory.
     *
     * @example ```typescript
     * let files: string[] = await fs.readDir({path: "path/to/new/dir"});
     * ```
     */
    readDir(props: {path: string}): Promise<[string, boolean][]>;

    /**
     * Reads a whole file in the target filesystem.
     *
     * @param props a record with the `path` entry that refers to the target path.
     * @returns a promise for a string that contains the data from the whole file.
     *
     * @example ```typescript
     * let fileContents: string = await fs.readToString({path: "path/to/new/dir"});
     * ```
     */
    readToString(props: {path: string}): Promise<string>;

    /**
     * Removes an empty directory from the target filesystem.
     *
     * @param props a record with the `path` entry that refers to the target path.
     * @returns an empty promise.
     *
     * @example ```typescript
     * await fs.removeDir({path: "path/to/empty/dir"});
     * ```
     */
    removeDir(props: {path: string}): Promise<void>;

    /**
     * Removes a directory and all the files it contains in the target filesystem.
     *
     * @param props a record with the `path` entry that refers to the target path.
     * @returns an empty promise.
     *
     * @example ```typescript
     * await fs.removeDirRecursive({path: "path/to/target/dir"});
     * ```
     */
    removeDirRecursive(props: {path: string}): Promise<void>;

    /**
     * Removes a file in the target filesystem.
     *
     * @param props a record with the `path` entry that refers to the target path.
     * @returns an empty promise.
     *
     * @example ```typescript
     * await fs.removeFile({path: "path/to/target/file"});
     * ```
     */
    removeFile(props: {path: string}): Promise<void>;

    /**
     * Removes a file in the target filesystem.
     *
     * @param props a record with the `from` and `to` paths, represented by `string`s.
     * @returns an empty promise.
     *
     * @example ```typescript
     * await fs.rename({from: "old/path", to: "new/path"});
     * ```
     */
    rename(props: {from: string, to: string}): Promise<void>;

    /**
     * Writes a string to a file.
     *
     * @param props a record with the `path` file path and `contents` which holds the contents of the new file
     * @returns an empty promise.
     *
     * @example ```typescript
     * await fs.rename({path: "some/path", contents: "this line will be the only contents of the file"});
     * ```
     */
    writeFile(props: {path: string, contents: string}): Promise<void>;

    /**
     * Identifies the type of FileSystem.
     */
    readonly type: FsType;
}
