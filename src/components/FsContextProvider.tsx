import {
    BaseFileSystem,
    ContextFileSystem,
    directoryname,
    DummyFsOps,
    filename,
    FsContext,
    FsDir,
    FsFile,
    FsItem,
    joinPath,
    parts
} from "../fsContext.ts";
import {FsType} from "../fsShared.ts";
import {PropsWithChildren, useCallback, useEffect, useMemo, useState} from "react";
import WasmFs, {initEmptyFs, WasmProjectDataStore} from "../wasmFs.ts";
import {ProjectDataStore, TauriProjectDataStore} from "../projectData.ts";

export default function FsContextProvider(props: PropsWithChildren) {
    const [root, setRoot] = useState<FsDir | undefined>(undefined);
    const [fsProvider, setFsProvider] = useState<BaseFileSystem | undefined>(undefined);
    const [projectDataStore, setProjectDataStore] = useState<ProjectDataStore | undefined>(undefined);
    const getItem = useCallback((path: string) => {
        if (!root || !path) {
            return null;
        }
        if (path === "/") {
            return root;
        }
        const paths = path.split("/");
        if (paths[0] === root.name || paths[0] === "") {
            paths.shift();
        }
        console.log(paths);
        let current: FsDir = root;
        for (let num = 0; num < paths.length; num++) {
            console.log(current);
            const path_part = paths[num];
            const next = current.getChild(path_part);
            console.log(next, !next, num !== paths.length, !next!.isDir);
            if (!next || (num !== paths.length - 1 && !next.isDir)) {
                return null;
            }
            if (num === paths.length - 1 || !next.isDir) {
                return next ?? null;
            }
            current = next;
        }
        console.log("Current: %o", current);
        return current;

    }, [root]);

    const FsOps: ContextFileSystem = useMemo(() => {
        if (!fsProvider) {
            return DummyFsOps;
        }
        const copyFile: ContextFileSystem["copyFile"] = async (from: FsFile, toParent: FsDir, toName?: string) => {
            const fromPath = from.path();
            const toFileName = toName ?? from.name;
            await fsProvider!.copyFile({from: fromPath, to: joinPath(toParent, toFileName)});
            const toFile = new FsFile(toFileName, toParent);
            toParent.addChild(toFile);
            return toFile;
        };

        const createFile: ContextFileSystem["createFile"] = async (parent: FsDir, path: string) => {
            const targetPath = joinPath(parent, path);
            await fsProvider!.createFile({path: targetPath});
            const fileName = filename(targetPath);
            const newFile = new FsFile(fileName, parent);
            parent.addChild(newFile);
            return newFile;
        };

        const createDir: ContextFileSystem["createDir"] = async (parent: FsDir, path: string) => {
            const targetPath = joinPath(parent, path);
            await fsProvider!.createDir({path: targetPath});
            const dirName = filename(targetPath);
            const newDir = new FsDir(dirName, parent);
            parent.addChild(newDir);
            return newDir;
        };

        const createDirWithParents: ContextFileSystem["createDirWithParents"] = async (parent: FsDir, path: string) => {
            const pieces = parts(path);
            let current = parent;
            for (let i = 0; i < pieces.length; i++) {
                const piece = pieces[i];
                if (!current.getChild(piece)) {
                    const part = await createDir(current, piece);
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
            // console.debug("Starting: ");
            // console.debug(parent);
            const items = await fsProvider!.readDir({path: parent.path()});
            // console.debug(items);
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
            await Promise.all(dirs.map(readDir));
            return map;
        };

        const readToString: ContextFileSystem["readToString"] = async (file: FsFile) => {
            return fsProvider!.readToString({path: file.path()});
        };

        const removeFile: ContextFileSystem["removeFile"] = async (file: FsFile) => {
            await fsProvider!.removeFile({path: file.path()});
            file.parent.removeChild(file.name);
        };

        const removeDir: ContextFileSystem["removeDir"] = async (dir: FsDir) => {
            if (dir.parent === null) {
                throw new Error("Cannot remove root directory.");
            }
            await fsProvider!.removeDir({path: dir.path()});
            dir.parent.removeChild(dir.name);
        };

        const removeDirRecursive: ContextFileSystem["removeDirRecursive"] = async (dir: FsDir) => {
            if (dir.parent === null) {
                throw new Error("Cannot remove root directory.");
            }
            await fsProvider!.removeDirRecursive({path: dir.path()});
            dir.parent.removeChild(dir.name);
        };

        const rename: ContextFileSystem["rename"] = async (file: FsFile, newPath: string) => {
            const newName = filename(newPath);
            const newPathParent = getItem(directoryname(newPath));
            if (!newPathParent) {
                throw new Error(`Parent directory of ${newPath} does not exist.`);
            }
            await fsProvider!.rename({from: file.path(), to: newPath});
            file.parent.removeChild(file.name);
            file.name = newName;
            file.parent.addChild(file);
            return file;
        };

        const writeFile: ContextFileSystem["writeFile"] = async (file: FsFile, contents: string) => {
            await fsProvider!.writeFile({path: file.path(), contents});
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
            rename,
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
        // console.debug(root, FsOps.init);
        if (root && FsOps.init &&
            (!(window.__TAURI__ && root.name === "/")) // Don't load root when on Tauri, this will brick the program since we recursively read all data.
        ) {
            // console.debug(root);
            FsOps.readDir(root);
        }
    }, [root, FsOps]);

    useEffect(() => {
        if (!fsProvider) {
            return;
        }
        switch (fsProvider.type) {
        case FsType.Tauri: {
            setProjectDataStore(new TauriProjectDataStore());
            break;
        }
        case FsType.WasmLocal: {
            if (!(fsProvider instanceof WasmFs)) {
                throw new Error("WasmLocal filesystem must be an instance of WasmFs");
            }
            setProjectDataStore(new WasmProjectDataStore(FsOps, fsProvider));
            break;
        }
        default: {
            throw new Error(`Unknown filesystem type: ${fsProvider.type}`);
        }
        }
    }, [FsOps, fsProvider]);

    useEffect(() => {
        if (projectDataStore) {
            projectDataStore.initDataStore();
        }
    }, [projectDataStore]);

    return <FsContext.Provider value={{
        root,
        getItem,
        ops: FsOps,
        projectHandler: projectDataStore!,
        type: fsProvider?.type,
        setRoot,
        setBaseFS: setFsProvider
    }} children={props.children}/>;
}
