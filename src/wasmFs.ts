import {BaseFileSystem, directoryname, filename} from "./fsContext.ts";

export default class WasmFs implements BaseFileSystem {
    private readonly rootDirectoryHandle: FileSystemDirectoryHandle;
    private readonly dirHandleCache: Map<string, FileSystemDirectoryHandle>;

    constructor(root: FileSystemDirectoryHandle) {
        this.rootDirectoryHandle = root;
        this.dirHandleCache = new Map([["/", this.rootDirectoryHandle]]);
    }

    static getParentPath(path: string): string {
        console.debug(path, path.indexOf("/"), path.lastIndexOf("/"));
        if (path.indexOf("/") === path.lastIndexOf("/")) {
            // There is only 1 /, the root directory.
            return "/";
        }
        return directoryname(path);
    }

    async getDirectoryHandle(path: string): Promise<FileSystemDirectoryHandle> {
        if (this.dirHandleCache.has(path)) {
            return this.dirHandleCache.get(path)!;
        }
        const parentPath = WasmFs.getParentPath(path);
        console.debug(parentPath);
        const parentHandle = await this.getDirectoryHandle(parentPath);
        const folderName = filename(path);
        const handle = await parentHandle.getDirectoryHandle(folderName);
        this.dirHandleCache.set(path, handle);
        return handle;
    }

    async getFileHandle(path: string): Promise<FileSystemFileHandle> {
        const parentHandle = await this.getDirectoryHandle(WasmFs.getParentPath(path));
        const basename = filename(path);
        return await parentHandle.getFileHandle(basename);
    }


    async copyFile(from: string, to: string): Promise<bigint> {
        const src = await this.getFileHandle(from);
        const dstParent = await this.getDirectoryHandle(WasmFs.getParentPath(to));
        const dstFilename = filename(to);
        console.debug(`Copying ${from} to ${to} (parent: ${dstParent.name}, filename: ${dstFilename})`);
        const dst = await dstParent.getFileHandle(dstFilename, {create: true});
        const writable = await dst.createWritable();
        await writable.write(await src.getFile());
        await writable.close();
        return BigInt(0);
    }

    async createDir(path: string): Promise<void> {
        const parentHandle = await this.getDirectoryHandle(WasmFs.getParentPath(path));
        const folderName = filename(path);
        console.debug(`Creating directory ${folderName} in ${parentHandle.name}`);
        await parentHandle.getDirectoryHandle(folderName, {create: true});
    }
    async createDirWithParents(path: string): Promise<void> {
        const parts = path.split("/");
        let current = this.rootDirectoryHandle;
        console.debug(`Creating parts: ${JSON.stringify(parts)}`);
        for (const part of parts) {
            current = await current.getDirectoryHandle(part, {create: true});
        }
    }
    async createFile(path: string): Promise<void> {
        const parentHandle = await this.getDirectoryHandle(WasmFs.getParentPath(path));
        const fileName = filename(path);
        console.debug(`Creating file ${fileName} in ${parentHandle.name}`);
        await parentHandle.getFileHandle(fileName, {create: true});
    }
    async readDir(path: string): Promise<[string, boolean][]> {
        console.debug(path);
        const dirHandle = await this.getDirectoryHandle(path);
        console.debug(dirHandle);
        const entries: FileSystemHandle[] = [];
        console.debug(`Reading directory ${path}`, dirHandle);
        for await (const entry of dirHandle.values()) {
            entries.push(entry);
        }
        console.debug(entries);
        return entries.map((entry) => [entry.name, entry.kind === "directory"]);
    }
    async readToString(path: string): Promise<string> {
        const handle = await this.getFileHandle(path);
        console.debug(`Reading file ${path}`);
        const file = await handle.getFile();
        return await file.text();
    }
    async removeDir(path: string): Promise<void> {
        const parentHandle = await this.getDirectoryHandle(WasmFs.getParentPath(path));
        const folderName = filename(path);
        console.debug(`Removing directory ${folderName} from ${parentHandle.name}`);
        await parentHandle.removeEntry(folderName);
        this.dirHandleCache.delete(path);
    }
    async removeDirRecursive(path: string): Promise<void> {
        const dirHandle = await this.getDirectoryHandle(path);
        const promises: Promise<unknown>[] = [];
        console.debug(`Removing directory ${dirHandle.name} recursively`);
        for await (const value of dirHandle.values()) {
            promises.push(value.kind === "directory" ? this.removeDirRecursive(value.name) : this.removeFile(value.name));
        }
        await Promise.all(promises);
        await this.removeDir(path);
    }
    async removeFile(path: string): Promise<void> {
        const parentHandle = await this.getDirectoryHandle(WasmFs.getParentPath(path));
        const fileName = filename(path);
        console.debug(`Removing file ${fileName} from ${parentHandle.name}`);
        await parentHandle.removeEntry(fileName);
    }
    async renameFile(from: string, to: string): Promise<void> {
        await this.copyFile(from, to);
        await this.removeFile(from);
    }

    async writeFile(path:string, contents:string): Promise<bigint> {
        const parentHandle = await this.getDirectoryHandle(WasmFs.getParentPath(path));
        const fileName = filename(path);
        console.debug(`Writing to file ${fileName} in ${parentHandle.name}`);
        const fileHandle = await parentHandle.getFileHandle(fileName, {create: true});
        const writable = await fileHandle.createWritable();
        await writable.write(contents);
        await writable.close();
        return BigInt(contents.length);

    }
}

export async function initEmptyFs(): Promise<WasmFs> {
    const root = await window.navigator.storage.getDirectory();
    return new WasmFs(root);
}
