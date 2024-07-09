/* eslint-disable no-unused-vars */
import { createContext } from "react";

export abstract class AbstractFsFile {
    public name: string;
    public isDir: boolean;
    public parent: FsDir | null; // null for root

    protected constructor(name: string, isDir: boolean, parent: FsDir | null) {
        this.name = name;
        this.isDir = isDir;
        this.parent = parent;
    }

    path(): string {
        return this.parent ? this.parent.path() + "/" + this.name : this.name;
    }
}

export class FsFile extends AbstractFsFile{
    public isDir = false as const;
    public parent: FsDir;

    constructor(name: string, parent: FsDir) {
        super(name, false, parent);
        this.parent = parent;
    }
}

export class FsDir extends AbstractFsFile {
    public isDir = true as const;
    public children: Map<string, FsItem> = new Map();

    constructor(name: string, parent: FsDir | null) {
        super(name, true, parent);
    }

    addChild(child: FsItem) {
        this.children.set(child.name, child);
    }
}

export type FsItem = FsFile | FsDir;

export function parts(path: string): string[];
export function parts(path: FsItem, returnFsItems: false): string[];
export function parts(path: FsItem, returnFsItems: true): FsItem[];
export function parts(path: string | FsItem, returnFsItems: boolean = true): string[] | FsItem[] {
    if (returnFsItems && typeof path !== "string") {
        const partsArr: FsItem[] = [path];
        const parent = path.parent;
        while (parent !== null) {
            partsArr.push(parent);
        }
        return partsArr.reverse();
    } else {
        const pathStr = typeof path === "string" ? path : path.path();
        return pathStr.split("/").filter((part) => part !== "");
    }
}

export function joinPath(first: string | FsDir, ...rest: string[]): string {
    let firstStr: string;
    if (typeof first !== "string") {
        firstStr = first.path();
    } else {
        firstStr = first;
    }
    const firstSegments = firstStr.split("/");
    const parts = [...firstSegments, ...rest];
    const validatedParts: string[] = [];
    for (let i: number = 0; i < parts.length; i++) {
        const part = parts[i];
        if (part === ".") {
            continue;
        } else if (part === "..") {
            validatedParts.pop();
        } else if (part.indexOf("/") !== -1) {
            validatedParts.push(...part.split("/"));
        } else {
            validatedParts.push(part);
        }
    }
    return validatedParts.join("/");
}

export function filename(path: string): string {
    return path.substring(path.lastIndexOf("/") + 1);
}

export function directoryname(path: string): string {
    return path.substring(0, path.lastIndexOf("/"));
}

export interface BaseFileSystem {
    copyFile(from: string, to: string): Promise<bigint>;
    createDir(path: string): Promise<void>;
    createDirWithParents(path: string): Promise<void>;
    createFile(path: string): Promise<void>;

    /**
     * Read a directory and return its contents
     * @param path The path to read
     * @returns A list of [name, isDir] tuples
     */
    readDir(path: string): Promise<[string, boolean][]>;
    readToString(path: string): Promise<string>;
    removeDir(path: string): Promise<void>;
    removeDirRecursive(path: string): Promise<void>;
    removeFile(path: string): Promise<void>;
    renameFile(from: string, to: string): Promise<void>;
}

export interface ContextFileSystem {
    copyFile(from: FsFile, toParent: FsDir, toName?: string): Promise<FsFile>;
    createDir(parent: FsDir, path: string): Promise<FsDir>;
    createDirWithParents(parent: FsDir, path: string): Promise<FsDir>;
    createFile(parent: FsDir, path: string): Promise<FsFile>;
    readDir(parent: FsDir): Promise<Map<string, FsItem>>;
    readToString(path: FsFile): Promise<string>;
    removeDir(path: FsDir): Promise<void>;
    removeDirRecursive(path: FsDir): Promise<void>;
    removeFile(path: FsFile): Promise<void>;
    renameFile(from: FsFile, to: string): Promise<FsFile>;
}


export interface FsContext {
    root: FsDir | undefined;
    getItem(path: string): FsItem | null;
    ops: ContextFileSystem;
}

const notDefined = () => {
    throw new Error("Method not implemented.");
};

export const FsContext = createContext<FsContext>({
    root: undefined,
    getItem: () => null,
    ops: {
        copyFile: notDefined,
        createDir: notDefined,
        createDirWithParents: notDefined,
        createFile: notDefined,
        readDir: notDefined,
        readToString: notDefined,
        removeDir: notDefined,
        removeDirRecursive: notDefined,
        removeFile: notDefined,
        renameFile: notDefined,
    }
});
