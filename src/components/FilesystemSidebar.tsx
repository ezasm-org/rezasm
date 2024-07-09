import {useContext, useMemo, useState} from "react";
import {AbstractFsFile, FsContext, FsDir} from "../fsContext.ts";

export function FileSidebar(props: {file: AbstractFsFile}) {
    return <span className="hover:bg-gray-50 cursor-pointer">{props.file.name}</span>;
}

export function FolderSidebar(props: {folder: FsDir}) {
    const [expanded, setExpanded] = useState(true); // Set for development
    const children = useMemo(() => {
        return [...props.folder.children.values()].map((child) => {
            return child.isDir ? <FolderSidebar folder={child} /> : <FileSidebar file={child} />;
        });
    }, [props.folder.children]);
    return <div className="folder-sidebar-item">
        <span onClick={() => setExpanded(!expanded)}>{expanded ? "V" : ">"}<FileSidebar file={props.folder} /></span>
        {expanded && <div className="folder-sidebar-children flex space-x-2">{children}</div>}
    </div>;
}

export default function FilesystemSidebar() {
    const fs = useContext(FsContext);
    return <div className="filesystem-sidebar border mx-2 px-2 flex flex-col">
        {fs.root ? <FolderSidebar folder={fs.root}/> : "No filesystem loaded, create a file or open a directory."}
        <div className="flex flex-col mt-6 gap-1.5">
            <button className="bg-blue-600 p-1 text-white">Open Folder</button>
            <button className="bg-blue-600 p-1 text-white">Create File</button>
        </div>
    </div>;
}
