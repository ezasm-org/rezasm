import {BaseFileSystem, type ContextFileSystem, directoryname, filename, FsDir, joinPath} from "./fsContext.ts";
import {FsType} from "./fsShared.ts";
import {ProjectDataEntry, ProjectDataStore} from "./projectData.ts";

export default class WasmFs implements BaseFileSystem {
    readonly type = FsType.WasmLocal;

    private readonly rootDirectoryHandle: FileSystemDirectoryHandle;
    private readonly dirHandleCache: Map<string, FileSystemDirectoryHandle>;

    constructor(root: FileSystemDirectoryHandle) {
        this.rootDirectoryHandle = root;
        this.dirHandleCache = new Map([["/", this.rootDirectoryHandle]]);
    }

    static getParentPath(path: string): string {
        // console.debug(path, path.indexOf("/"), path.lastIndexOf("/"));
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
        // console.debug(parentPath);
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


    async copyFile({from, to}: {from: string, to: string}): Promise<bigint> {
        const src = await this.getFileHandle(from);
        const dstParent = await this.getDirectoryHandle(WasmFs.getParentPath(to));
        const dstFilename = filename(to);
        // console.debug(`Copying ${from} to ${to} (parent: ${dstParent.name}, filename: ${dstFilename})`);
        const dst = await dstParent.getFileHandle(dstFilename, {create: true});
        const writable = await dst.createWritable();
        await writable.write(await src.getFile());
        await writable.close();
        return BigInt(0);
    }

    async createDir({path}: {path: string}): Promise<void> {
        const parentHandle = await this.getDirectoryHandle(WasmFs.getParentPath(path));
        const folderName = filename(path);
        // console.debug(`Creating directory ${folderName} in ${parentHandle.name}`);
        await parentHandle.getDirectoryHandle(folderName, {create: true});
    }
    async createDirWithParents({path}: {path: string}): Promise<void> {
        const parts = path.split("/");
        let current = this.rootDirectoryHandle;
        // console.debug(`Creating parts: ${JSON.stringify(parts)}`);
        for (const part of parts) {
            current = await current.getDirectoryHandle(part, {create: true});
        }
    }
    async createFile({path}: {path: string}): Promise<void> {
        const parentHandle = await this.getDirectoryHandle(WasmFs.getParentPath(path));
        const fileName = filename(path);
        // console.debug(`Creating file ${fileName} in ${parentHandle.name}`);
        await parentHandle.getFileHandle(fileName, {create: true});
    }
    async readDir({path}: {path: string}): Promise<[string, boolean][]> {
        // console.debug(path);
        const dirHandle = await this.getDirectoryHandle(path);
        // console.debug(dirHandle);
        const entries: FileSystemHandle[] = [];
        // console.debug(`Reading directory ${path}`, dirHandle);
        for await (const entry of dirHandle.values()) {
            entries.push(entry);
        }
        // console.debug(entries);
        return entries.map((entry) => [entry.name, entry.kind === "directory"]);
    }
    async readToString({path}: {path: string}): Promise<string> {
        const handle = await this.getFileHandle(path);
        // console.debug(`Reading file ${path}`);
        const file = await handle.getFile();
        return await file.text();
    }
    async removeDir({path}: {path: string}): Promise<void> {
        const parentHandle = await this.getDirectoryHandle(WasmFs.getParentPath(path));
        const folderName = filename(path);
        // console.debug(`Removing directory ${folderName} from ${parentHandle.name}`);
        await parentHandle.removeEntry(folderName);
        this.dirHandleCache.delete(path);
    }
    async removeDirRecursive({path}: {path: string}): Promise<void> {
        const dirHandle = await this.getDirectoryHandle(path);
        const promises: Promise<unknown>[] = [];
        // console.debug(`Removing directory ${dirHandle.name} recursively`);
        for await (const value of dirHandle.values()) {
            const truePath = joinPath(path, value.name);
            promises.push(value.kind === "directory" ? this.removeDirRecursive({path: truePath}) : this.removeFile({path: truePath}));
        }
        await Promise.all(promises);
        await this.removeDir({path});
    }
    async removeFile({path}: {path: string}): Promise<void> {
        const parentHandle = await this.getDirectoryHandle(WasmFs.getParentPath(path));
        const fileName = filename(path);
        // console.debug(`Removing file ${fileName} from ${parentHandle.name}`);
        await parentHandle.removeEntry(fileName);
    }
    async rename({from, to}: {from: string, to: string}): Promise<void> {
        await this.copyFile({from, to});
        await this.removeFile({path: from});
    }

    async writeFile({path, contents}: {path: string, contents: string}): Promise<void> {
        const parentHandle = await this.getDirectoryHandle(WasmFs.getParentPath(path));
        const fileName = filename(path);
        // console.debug(`Writing to file ${fileName} in ${parentHandle.name}`);
        const fileHandle = await parentHandle.getFileHandle(fileName, {create: true});
        const writable = await fileHandle.createWritable();
        await writable.write(contents);
        await writable.close();

    }
}

export async function initEmptyFs(): Promise<WasmFs> {
    const root = await window.navigator.storage.getDirectory();
    return new WasmFs(root);
}

/*

SAVE/OPEN PROJECT FUNCTIONALITY

*/


function promisifyRequest<T>(request: IDBRequest<T>): Promise<T> {
    return new Promise((resolve, reject) => {
        request.onsuccess = () => resolve(request.result);
        request.onerror = () => reject(request.error);
    });
}

function promisifyTransaction(transaction: IDBTransaction): Promise<void> {
    return new Promise((resolve, reject) => {
        transaction.oncomplete = () => resolve();
        transaction.onerror = () => reject(transaction.error);
    });
}

interface IndexedDbProjectEntry extends ProjectDataEntry {
    name: string;
}

export interface IndexedDbProjectItem {
    name: string;
    isDir: boolean;
    contents: string | null;
    children: IndexedDbProjectItem[] | null;
}


export class WasmProjectDataStore extends ProjectDataStore {
    private indexedDb: IDBDatabase | null = null;
    private readonly ops: ContextFileSystem;
    private readonly basefs: WasmFs;
    private static readonly highestVersion = 1;
    
    constructor(ops: ContextFileSystem, basefs: WasmFs) {
        super();
        this.ops = ops;
        this.basefs = basefs;
    }
    /**
     * Serialize a filesystem directory to an object suitable for IndexedDb insertion.
     * @param input The directory to serialize
     * @param ops Operations to interact with the filesystem
     * @param nameOverride Used to override the name of the top-level directory, should only be used for the project root.
     */
    private async serializeFsDirToIndexedDb(input: FsDir, nameOverride?: string): Promise<IndexedDbProjectItem> {
        const children = await Promise.all(Object.values(input.children).map(async (child) => {
            if (child instanceof FsDir) {
                return this.serializeFsDirToIndexedDb(child);
            } else {
                return {
                    name: child.name,
                    isDir: false,
                    contents: await this.ops.readToString(child),
                    children: null
                };
            }
        }));
        return {
            name: nameOverride ?? input.name,
            isDir: true,
            contents: null,
            children
        };
    }

    /**
     * Deserialize an IndexedDb object to a filesystem directory.
     * @param input The object to deserialize
     * @param basefs The filesystem to deserialize to
     * @param parentDirName The name of the parent directory (do not touch this when calling externally)
     */
    private async deserializeIndexedDbToWasmFs(input: IndexedDbProjectItem, parentDirName = "/") {
        if (!input.children) {
            throw new Error("Invalid project data");
        }
        await Promise.all(input.children.map(async (child) => {
            const path = joinPath(parentDirName, child.name);
            if (child.isDir) {
                await this.basefs.createDir({path});
                await this.deserializeIndexedDbToWasmFs(child, path);
            } else {
                await this.basefs.writeFile({path, contents: child.contents!});
            }
        }));
    }

    private async migrate(event: IDBVersionChangeEvent): Promise<void> {
        // eslint-disable-next-line @typescript-eslint/no-explicit-any -- This is a hack to get the result of the request because the DOM API typings are lacking here.
        const db: IDBDatabase = (event.target! as any).result as IDBDatabase;
        const currentVersion = event.oldVersion;
        // Make sure all objectStore variables are created before the first promisifyTransaction, otherwise it *will* error,
        if (currentVersion <= 0) {
            // Initial DB structure
            const objectStore = db.createObjectStore("projects", {keyPath: "name"});
            objectStore.createIndex("lastSaved", "lastSaved", { unique: false });
            const objectStore2 = db.createObjectStore("projectData", {keyPath: "name"});
            await promisifyTransaction(objectStore.transaction);
            await promisifyTransaction(objectStore2.transaction);
            // currentVersion = 1;
        }

    }
    async initDataStore(): Promise<void> {
        const request = indexedDB.open("projectData", WasmProjectDataStore.highestVersion);
        request.onupgradeneeded = (event) => this.migrate(event);
        this.indexedDb = await promisifyRequest(request);
        const fetchTransaction = this.indexedDb.transaction("projects", "readonly");
        const objectStore = fetchTransaction.objectStore("projects");
        const data: IndexedDbProjectEntry[] = await promisifyRequest(objectStore.getAll());
        data.forEach(item => this.savedProjects[item.name] = item);
    }

    async saveProject(item: FsDir, projectName: string): Promise<void> {
        if (this.indexedDb === null) {
            throw new Error("IndexedDB not initialized");
        }
        const transaction = this.indexedDb.transaction("projects", "readwrite");
        const projectsObjectStore = transaction.objectStore("projects");
        const lastModifiedTime = Date.now();
        this.savedProjects[projectName] = {lastModified: lastModifiedTime, rootPath: "/"};
        const entry: IndexedDbProjectEntry = {
            lastModified: lastModifiedTime,
            rootPath: "/",
            name: projectName
        };
        projectsObjectStore.put(entry);
        await promisifyTransaction(transaction);
        const projectData = await this.serializeFsDirToIndexedDb(item, projectName);
        const transaction2 = this.indexedDb.transaction("projectData", "readwrite");
        const projectDataObjectStore = transaction2.objectStore("projectData");
        projectDataObjectStore.put(projectData);
        await promisifyTransaction(transaction2);
    }

    async closeProject(): Promise<void> {
        const existingData = await this.basefs.readDir({path: "/"});
        await Promise.all(existingData.map(([name, isDir]) => isDir ? this.basefs.removeDirRecursive({path: "/" + name}) : this.basefs.removeFile({path: "/" +name})));
    }

    async getProject(projectName: string): Promise<FsDir | null> {
        if (this.indexedDb === null) {
            throw new Error("IndexedDB not initialized");
        }
        const transaction = this.indexedDb.transaction("projectData", "readonly");
        const objectStore = transaction.objectStore("projectData");
        const data: IndexedDbProjectItem | null = (await promisifyRequest(objectStore.get(projectName))) ?? null;
        if (data === null) {
            return null;
        } else {
            await this.deserializeIndexedDbToWasmFs(data);
            return new FsDir("/", null);
        }
    }
}
