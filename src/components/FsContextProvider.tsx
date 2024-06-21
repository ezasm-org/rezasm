import {FsContext, FsDir} from "../fsContext.ts";
import {PropsWithChildren, useCallback, useState} from "react";

export default function FsContextProvider(props: PropsWithChildren) {
    const [root, setRoot] = useState<FsDir | undefined>(undefined);
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
            const next = current.children.get(path_part);
            if (!next || (num !== paths.length - 1 && !next.isDir)) {
                return null;
            }
            if (num === paths.length - 1 || !next.isDir) {
                return next ?? null;
            }
            current = next;
        }
        return current;

    }, [root])
    return <FsContext.Provider value={{
        root: root,
        getItem,
    }} children={props.children} />;
}
