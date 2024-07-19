import {PropsWithChildren, useMemo, useState} from "react";
import { FsActions, FsActionsContext, FsDir} from "../fsContext.ts";
import {CreateFileModal} from "./FilesystemModals.tsx";

export default function FsActionsProvider(props: PropsWithChildren) {
    const [createFileModalDir, setCreateFileModalDir] = useState<FsDir | null>(null);
    const [createDirModalDir, setCreateDirModalDir] = useState<FsDir | null>(null);
    const [createFileModalOnSuccessHandler, setCreateFileModalOnSuccessHandler] = useState<((filename: string) => unknown) | null>(null);
    const [createDirModalOnSuccessHandler, setCreateDirModalOnSuccessHandler] = useState<((filename: string) => unknown) | null>(null);
    const [showOpenProjectModal, setShowOpenProjectModal] = useState(false);
    const [saveProjectModalRoot, setSaveProjectModalRoot] = useState<FsDir | null>(null);
    const actions: FsActions = useMemo(() => ({
        showCreateFileModal: (folder: FsDir, onSuccess: (filename: string) => unknown) => {
            setCreateFileModalDir(folder);
            setCreateFileModalOnSuccessHandler(() => onSuccess); // passing a callback to setState calls the callback to set the state.
        },
        showCreateDirModal: (folder: FsDir, onSuccess: (filename: string) => unknown) => {
            setCreateDirModalDir(folder);
            setCreateDirModalOnSuccessHandler(() => onSuccess); // passing a callback to setState calls the callback to set the state.
        },
        showOpenProjectModal: () => {
            setShowOpenProjectModal(true);
        },
        showSaveProjectModal: (root: FsDir) => {
            setSaveProjectModalRoot(root);
        }
    }), []);
    return <FsActionsContext.Provider value={actions}>
        {(createFileModalDir !== null && createFileModalOnSuccessHandler !== null) && <CreateFileModal folder={createFileModalDir} onSuccess={createFileModalOnSuccessHandler} closeModal={() => setCreateFileModalDir(null)} creatingDirectory={false} setAlternateDirectory={setCreateFileModalDir} />}
        {(createDirModalDir !== null && createDirModalOnSuccessHandler !== null) && <CreateFileModal folder={createDirModalDir} onSuccess={createDirModalOnSuccessHandler} closeModal={() => setCreateDirModalDir(null)} creatingDirectory={true} setAlternateDirectory={setCreateDirModalDir} />}
        {props.children}</FsActionsContext.Provider>;
}
