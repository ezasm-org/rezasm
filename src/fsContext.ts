import { createContext } from "react";

export abstract class AbstractFsFile {
    public name: string;
    public isDir: boolean;
    public parent: FsDir | null; // null for root

    constructor(name: string, isDir: boolean, parent: FsDir | null) {
        this.name = name;
        this.isDir = isDir;
        this.parent = parent;
    }

    path(): string {
        return this.parent ? this.parent.path() + "/" + this.name : this.name;
    }
}

export class FsFile extends AbstractFsFile{
    public isDir: false = false;
    public parent: FsDir;

    constructor(name: string, parent: FsDir) {
        super(name, false, parent);
        this.parent = parent;
    }
}

export class FsDir extends AbstractFsFile {
    public isDir: true = true;
    public children: Map<string, FsItem> = new Map();

    constructor(name: string, parent: FsDir | null) {
        super(name, true, parent);
    }

    refresh() {
        throw new Error("Method not implemented.");
    }
}

export type FsItem = FsFile | FsDir;

export interface FsContext {
    root: FsDir | undefined;
    getItem(path: string): FsItem | null;
}

export const FsContext = createContext<FsContext >({
    root: undefined,
    getItem: () => null
});
