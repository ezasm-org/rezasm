import {useContext, useMemo, useState} from "react";
import {AbstractFsFile, FsContext, FsDir} from "../fsContext.ts";

export function FileSidebar(props: {file: AbstractFsFile}) {
    return <span className="hover:bg-gray-50 cursor-pointer">{props.file.name}</span>;
}

export function FolderSidebar(props: {folder: FsDir}) {
    const [expanded, setExpanded] = useState(true); // Set for development
    const children = useMemo(() => {
        return props.folder.children.map((child) => {
            return child.isDir ? <FolderSidebar folder={child} /> : <FileSidebar file={child} />;
        });
    }, [props.folder.children]);
    return <div className="folder-sidebar-item">
        <span onClick={() => setExpanded(!expanded)}>{expanded ? "V" : ">"}<FileSidebar file={props.folder} /></span>
        {expanded && <div className="folder-sidebar-children flex space-x-2">{children}</div>}
    </div>
}

export default function FilesystemSidebar() {
    const fs = useContext(FsContext);
    return <div className="filesystem-sidebar border mx-2 px-2">
        {fs.root ? <FolderSidebar folder={fs.root}/> : "No filesystem loaded, create a file or open a directory."}
        <button>Create Test File</button>
        <button>Open Folder</button>
    </div>;
}
