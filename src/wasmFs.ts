import {BaseFileSystem, filename} from "./fsContext.ts";

export default class WasmFs implements BaseFileSystem {
    private readonly rootDirectoryHandle: FileSystemDirectoryHandle;
    private readonly dirHandleCache: Map<string, FileSystemDirectoryHandle>;

    constructor(root: FileSystemDirectoryHandle) {
        this.rootDirectoryHandle = root;
        this.dirHandleCache = new Map([["/", this.rootDirectoryHandle]]);
    }

    static getParentPath(path: string): string {
        return path.substring(0, path.lastIndexOf("/"));
    }

    async getDirectoryHandle(path: string): Promise<FileSystemDirectoryHandle> {
        if (this.dirHandleCache.has(path)) {
            return this.dirHandleCache.get(path)!;
        }
        const parentHandle = await this.getDirectoryHandle(WasmFs.getParentPath(path));
        return await parentHandle.getDirectoryHandle(path);
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
        const dst = await dstParent.getFileHandle(dstFilename, {create: true});
        const writable = await dst.createWritable();
        await writable.write(await src.getFile());
        await writable.close();
        return BigInt(0);
    }

    async createDir(path: string): Promise<void> {
        const parentHandle = await this.getDirectoryHandle(WasmFs.getParentPath(path));
        const folderName = filename(path);
        await parentHandle.getDirectoryHandle(folderName, {create: true});
    }
    async createDirWithParents(path: string): Promise<void> {
        const parts = path.split("/");
        let current = this.rootDirectoryHandle;
        for (const part of parts) {
            current = await current.getDirectoryHandle(part, {create: true});
        }
    }
    async createFile(path: string): Promise<void> {
        const parentHandle = await this.getDirectoryHandle(WasmFs.getParentPath(path));
        const fileName = filename(path);
        await parentHandle.getFileHandle(fileName, {create: true});
    }
    async readDir(path: string): Promise<[string, boolean][]> {
        const dirHandle = await this.getDirectoryHandle(path);
        const entries: FileSystemHandle[] = [];
        for await (const entry of dirHandle.values()) {
            entries.push(entry);
        }
        return entries.map((entry) => [entry.name, entry.kind === "directory"]);
    }
    async readToString(path: string): Promise<string> {
        const handle = await this.getFileHandle(path);
        const file = await handle.getFile();
        return await file.text();
    }
    async removeDir(path: string): Promise<void> {
        const parentHandle = await this.getDirectoryHandle(WasmFs.getParentPath(path));
        const folderName = filename(path);
        await parentHandle.removeEntry(folderName);
    }
    async removeDirRecursive(path: string): Promise<void> {
        const dirHandle = await this.getDirectoryHandle(path);
        const promises: Promise<unknown>[] = [];
        for await (const value of dirHandle.values()) {
            promises.push(value.kind === "directory" ? this.removeDirRecursive(value.name) : this.removeFile(value.name));
        }
        await Promise.all(promises);
        await this.removeDir(path);
    }
    async removeFile(path: string): Promise<void> {
        const parentHandle = await this.getDirectoryHandle(WasmFs.getParentPath(path));
        const fileName = filename(path);
        await parentHandle.removeEntry(fileName);
    }
    async renameFile(from: string, to: string): Promise<void> {
        await this.copyFile(from, to);
        await this.removeFile(from);
    }
}

export async function initEmptyFs(): Promise<WasmFs> {
    const root = await window.navigator.storage.getDirectory();
    return new WasmFs(root);
}
