import {
    BaseFileSystem,
    ContextFileSystem, directoryname, DummyFsOps,
    filename,
    FsContext,
    FsDir,
    FsFile,
    FsItem,
    joinPath,
    parts
} from "../fsContext.ts";
import {PropsWithChildren, useCallback, useEffect, useMemo, useState} from "react";
import {initEmptyFs} from "../wasmFs.ts";

export default function FsContextProvider(props: PropsWithChildren) {
    const [root, setRoot] = useState<FsDir | undefined>(undefined);
    const [fsProvider, setFsProvider] = useState<BaseFileSystem | undefined>(undefined);
    const getItem = useCallback((path: string) => {
        if (!root || !path) {
            return null;
        }
        const paths = path.split("/");
        if (paths[0] === root.name) {
            paths.shift();
        }
        let current: FsDir = root;
        for (let num = 0; num < paths.length; num++) {
            const path_part = paths[num];
            const next = current.getChild(path_part);
            if (!next || (num !== paths.length - 1 && !next.isDir)) {
                return null;
            }
            if (num === paths.length - 1 || !next.isDir) {
                return next ?? null;
            }
            current = next;
        }
        return current;

    }, [root]);

    const FsOps: ContextFileSystem = useMemo(() => {
        if (!fsProvider) {
            return DummyFsOps;
        }
        const copyFile: ContextFileSystem["copyFile"] = async (from: FsFile, toParent: FsDir, toName?: string) => {
            const fromPath = from.path();
            const toFileName = toName ?? from.name;
            await fsProvider!.copyFile(fromPath, joinPath(toParent, toFileName));
            const toFile = new FsFile(toFileName, toParent);
            toParent.addChild(toFile);
            return toFile;
        };
        
        const createFile: ContextFileSystem["createFile"] = async(parent: FsDir, path: string)  => {
            const targetPath = joinPath(parent, path);
            await fsProvider!.createFile(targetPath);
            const fileName = filename(targetPath);
            const newFile = new FsFile(fileName, parent);
            parent.addChild(newFile);
            return newFile;
        };
        
        const createDir: ContextFileSystem["createDir"] = async (parent: FsDir, path: string)=> {
            const targetPath = joinPath(parent, path);
            await fsProvider!.createDir(targetPath);
            const dirName = filename(targetPath);
            const newDir = new FsDir(dirName, parent);
            parent.addChild(newDir);
            return newDir;
        };
        
        const createDirWithParents: ContextFileSystem["createDirWithParents"] =  async (parent: FsDir, path: string) => {
            const pieces = parts(path);
            let current = parent;
            for (let i = 0; i < pieces.length; i++) {
                const piece = pieces[i];
                if (!current.getChild(piece)) {
                    const part = await FsOps.createDir(current, piece);
                    current.addChild(part);
                    current = part;
                } else {
                    const part = current.getChild(piece)!;
                    if (!part.isDir) {
                        throw new Error(`Path ${joinPath(parent, ...pieces.slice(0, i))} already exists as a file.`);
                    }
                    current = part;
                }
            }
            console.assert(current.path() === joinPath(parent, path), `Path ${current.path()} does not match ${joinPath(parent, path)}`);
            return current;
        };
        
        const readDir: ContextFileSystem["readDir"] = async (parent: FsDir): Promise<Map<string, FsItem>> => {
            console.debug("Starting: ");
            console.debug(parent);
            const items = await fsProvider!.readDir(parent.path());
            console.debug(items);
            const map = new Map<string, FsItem>();
            const dirs: FsDir[] = [];
            for (const [fileName, isDir] of items) {
                const name = filename(fileName);
                const newItem = isDir ? new FsDir(name, parent) : new FsFile(name, parent);
                map.set(name, newItem);
                if (newItem instanceof FsDir) {
                    dirs.push(newItem);
                }
            }
            parent.children = map;
            console.debug(dirs);
            await Promise.all(dirs.map(readDir));
            return map;
        };

        const readToString: ContextFileSystem["readToString"] = async (file: FsFile) => {
            return fsProvider!.readToString(file.path());
        };

        const removeFile: ContextFileSystem["removeFile"] = async (file: FsFile) => {
            await fsProvider!.removeFile(file.path());
            file.parent.removeChild(file.name);
        };

        const removeDir: ContextFileSystem["removeDir"] = async (dir: FsDir) => {
            if (dir.parent === null) {
                throw new Error("Cannot remove root directory.");
            }
            await fsProvider!.removeDir(dir.path());
            dir.parent.removeChild(dir.name);
        };

        const removeDirRecursive: ContextFileSystem["removeDirRecursive"] = async (dir: FsDir) => {
            if (dir.parent === null) {
                throw new Error("Cannot remove root directory.");
            }
            await fsProvider!.removeDirRecursive(dir.path());
            dir.parent.removeChild(dir.name);
        };

        const renameFile: ContextFileSystem["renameFile"] = async (file: FsFile, newPath: string) => {
            const newName = filename(newPath);
            const newPathParent = getItem(directoryname(newPath));
            if (!newPathParent) {
                throw new Error(`Parent directory of ${newPath} does not exist.`);
            }
            await fsProvider!.renameFile(file.path(), newPath);
            file.parent.removeChild(file.name);
            file.name = newName;
            file.parent.addChild(file);
            return file;
        };

        const writeFile: ContextFileSystem["writeFile"] = async (file: FsFile, contents: string) => {
            await fsProvider!.writeFile(file.path(), contents);
        };

        return {
            copyFile,
            createFile,
            createDir,
            createDirWithParents,
            readDir,
            readToString,
            removeFile,
            removeDir,
            removeDirRecursive,
            renameFile,
            writeFile,
            init: true
        };
    }, [fsProvider, getItem]);

    useEffect(() => {
        initEmptyFs().then((fs) => setFsProvider(fs));
        setRoot(new FsDir("/", null));
    }, []);

    useEffect(() => {
        // Load dir cache on root change
        console.debug(root, FsOps.init);
        if (root && FsOps.init) {
            console.debug(root);
            FsOps.readDir(root);
        }
    }, [root, FsOps]);

    return <FsContext.Provider value={{
        root: root,
        getItem,
        ops: FsOps
    }} children={props.children} />;
}
