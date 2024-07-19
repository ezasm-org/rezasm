import {useContext, useEffect, useMemo, useReducer, useState} from "react";
import {AbstractFsFile, FsActionsContext, FsContext, FsDir} from "../fsContext.ts";

export function FileSidebar(props: {file: AbstractFsFile, clickable?: boolean}) {
    return <span className={(props.clickable ?? true) ? "hover:bg-gray-200 cursor-pointer" : undefined}>{props.file.name}</span>;
}

export function FolderSidebar(props: {folder: FsDir, hash: number}) {
    console.debug(`Rerendering folder sidebar for ${props.folder.path()}, hash: ${props.hash}`);
    const locked = props.folder.parent === null; // Root directory cannot be collapsed
    const [expanded, setExpanded] = useState(locked); // Set for development
    return <div className="folder-sidebar-item">
        <span className={!locked ? "hover:bg-gray-200 cursor-pointer" : undefined} onClick={!locked ? (() => setExpanded(!expanded)) : undefined}>{!locked && (expanded ? "▼" : "▶")}<FileSidebar file={props.folder} clickable={false} /></span>
        {expanded && <div className="folder-sidebar-children flex space-x-2 flex-col"><span />{/* We need the empty span because of how the margin spacing works. */}{Object.values(props.folder.children).map((child) => {
            return child.isDir ? <FolderSidebar folder={child} hash={child.modifiedHash} key={child.name} /> : <FileSidebar file={child} key={child.name} />;
        })}</div>}
    </div>;
}

export default function FilesystemSidebar() {
    const fs = useContext(FsContext);
    const actions = useContext(FsActionsContext);
    const [, setCounter] = useReducer((x) => x + 1, 0);
    useEffect(() => {
        const val = setInterval(() => setCounter(), 100);
        return () => clearInterval(val);
    });
    const rootSidebar = useMemo(() => fs.root ? <FolderSidebar folder={fs.root} hash={fs.root.modifiedHash}/> : "No filesystem loaded, create a file or open a directory.", [fs.root, fs.root?.modifiedHash]);
    return <div className="filesystem-sidebar border mx-2 px-2 flex flex-col">
        {rootSidebar}
        <div className="flex flex-col mt-6 gap-1.5">
            <button className="bg-blue-600 p-1 text-white">Open Folder</button>
            <button className="bg-blue-600 p-1 text-white"
                onClick={() => actions.showCreateDirModal(fs.root!, setCounter)}>Create Directory
            </button>
            <button className="bg-blue-600 p-1 text-white"
                onClick={() => actions.showCreateFileModal(fs.root!, setCounter)}>Create File
            </button>
            <button className="bg-blue-600 p-1 text-white"
                onClick={() => actions.showCreateDirModal(fs.root!, setCounter)}>Save Project
            </button>
        </div>
    </div>;
}
