import {Label, Listbox, ListboxButton, ListboxOption, ListboxOptions} from "@headlessui/react";
import {FsContext, FsDir} from "../fsContext.ts";
import React, {useContext, useMemo, useState} from "react";
import {ProjectDataEntry} from "../projectData.ts";

function DirectorySelectorContent(props: { directory: FsDir }) {
    const parentPath = props.directory.parent ? props.directory.parent.path() : null;
    const name = props.directory.name;
    return <span className="ml-3 block truncate font-normal group-data-[selected]:font-semibold">
        {parentPath && <span className="text-gray-400">{parentPath === "/" ? null : parentPath}/</span>}
        {name}
    </span>;
}

function DirectorySelectorOption(props: { directory: FsDir }) {
    return <ListboxOption
        value={props.directory}
        className="group relative cursor-default select-none py-2 pl-3 pr-9 text-gray-900 data-[focus]:bg-indigo-600 data-[focus]:text-white"
    >
        <div className="flex items-center">
            <DirectorySelectorContent directory={props.directory}/>
        </div>
    </ListboxOption>;
}

function buildDirectorySelectorOptions(directory: FsDir): React.ReactNode[] {
    const options: React.ReactNode[] = [<DirectorySelectorOption key={directory.path()} directory={directory}/>];
    for (const child of Object.values(directory.children)) {
        if (child.isDir) {
            options.push(...buildDirectorySelectorOptions(child));
        }
    }
    return options;
}

function ModalPrimaryButton(props: Omit<React.JSX.IntrinsicElements["button"], "type" | "className">) {
    return <button type="button"
        className="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800" {...props} />;
}

function ModalSecondaryButton(props: Omit<React.JSX.IntrinsicElements["button"], "type" | "className">) {
    return <button type="button" {...props}
        className="py-2.5 px-5 ms-3 text-sm font-medium text-gray-900 focus:outline-none bg-white rounded-lg border border-gray-200 hover:bg-gray-100 hover:text-blue-700 focus:z-10 focus:ring-4 focus:ring-gray-100 dark:focus:ring-gray-700 dark:bg-gray-800 dark:text-gray-400 dark:border-gray-600 dark:hover:text-white dark:hover:bg-gray-700"/>;
}

function CloseModalButton(props: React.PropsWithChildren<{ closeModal: () => unknown }>) {
    return <ModalSecondaryButton onClick={props.closeModal}>{props.children}</ModalSecondaryButton>;
}

function ModalButtons(props: React.PropsWithChildren) {
    return <div
        className="flex items-center p-4 md:p-5 border-t border-gray-200 rounded-b dark:border-gray-600">{props.children}</div>;
}

function Modal(props: React.PropsWithChildren<{ heading: React.ReactNode, closeModal: () => unknown }>) {
    return <dialog open={true}
        className="overflow-y-auto overflow-x-hidden fixed flex justify-center items-center w-full md:inset-0 h-[calc(100%-1rem)] max-h-full bg-transparent">
        <div className="relative p-4 w-full max-w-2xl max-h-full">
            <div className="relative bg-white rounded-lg shadow dark:bg-gray-700">
                <div className="flex items-center justify-between p-4 md:p-5 border-b rounded-t dark:border-gray-600">
                    <h3 className="text-xl font-semibold text-gray-900 dark:text-white">
                        {props.heading}
                    </h3>
                    <button type="button"
                        className="text-gray-400 bg-transparent hover:bg-gray-200 hover:text-gray-900 rounded-lg text-sm w-8 h-8 ms-auto inline-flex justify-center items-center dark:hover:bg-gray-600 dark:hover:text-white"
                        onClick={props.closeModal}>
                        <svg className="w-3 h-3" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none"
                            viewBox="0 0 14 14">
                            <path stroke="currentColor" strokeLinecap="round" strokeLinejoin="round" strokeWidth="2"
                                d="m1 1 6 6m0 0 6 6M7 7l6-6M7 7l-6 6"/>
                        </svg>
                        <span className="sr-only">Close modal</span>
                    </button>
                </div>
                <div className="p-4 md:p-5 space-y-4 text-white">
                    {props.children}
                </div>
            </div>
        </div>
    </dialog>;
}

export function CreateFileModal(props: {
    folder: FsDir,
    closeModal: () => unknown,
    onSuccess: (filename: string) => unknown,
    creatingDirectory: boolean,
    setAlternateDirectory: (directory: FsDir) => unknown
}) {
    const [name, setName] = useState("");
    const fs = useContext(FsContext);
    const options = useMemo(() => buildDirectorySelectorOptions(fs.root!), [fs.root]);
    return <Modal heading={`Create ${props.creatingDirectory ? "Folder" : "File"}`} closeModal={props.closeModal}>
        <div className="p-4 md:p-5 space-y-4">
            <label htmlFor="first_name"
                className="block mb-2 text-sm font-medium text-gray-900 dark:text-white">{props.creatingDirectory ? "Folder" : "File"} Name</label>
            <input type="text" id="filename"
                className="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                required={true} minLength={1} onChange={(e) => setName(e.target.value)}
                value={name}/>
            <Listbox value={props.folder} onChange={props.setAlternateDirectory}>
                <Label className="block mb-2 text-sm font-medium text-gray-900 dark:text-white">Create
                    in</Label>
                <div className="relative mt-2">
                    <ListboxButton
                        className="relative w-full cursor-default rounded-md bg-white py-1.5 pl-3 pr-10 text-left text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 focus:outline-none focus:ring-2 focus:ring-indigo-500 sm:text-sm sm:leading-6">
                        <DirectorySelectorContent directory={props.folder}/>
                        <span
                            className="pointer-events-none absolute inset-y-0 right-0 ml-3 flex items-center pr-2">▾</span>
                    </ListboxButton>

                    <ListboxOptions
                        className="absolute z-10 mt-1 max-h-56 w-full overflow-auto rounded-md bg-white py-1 text-base shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none data-[closed]:data-[leave]:opacity-0 data-[leave]:transition data-[leave]:duration-100 data-[leave]:ease-in sm:text-sm"
                    >
                        {options}
                    </ListboxOptions>
                </div>
            </Listbox>
        </div>
        <ModalButtons>
            <ModalPrimaryButton
                onClick={() => {
                    if (name.includes("/")) {
                        alert("The name cannot contain a slash.");
                    }
                    (props.creatingDirectory ? fs.ops.createDir : fs.ops.createFile)(props.folder, name).then(() => {
                        props.closeModal();
                        props.onSuccess(name);
                    }).catch((error) => {
                        console.error(`Error while creating ${props.creatingDirectory ? "folder" : "file"}: ${error}`);
                        alert(`Error while creating ${props.creatingDirectory ? "folder" : "file"}: ${error}`);
                        props.closeModal();
                    });
                }}>Create
            </ModalPrimaryButton>
            <CloseModalButton closeModal={props.closeModal}>Cancel</CloseModalButton>
        </ModalButtons>
    </Modal>;
}

export function SaveProjectModal(props: { root: FsDir, closeModal: () => unknown }) {
    const [name, setName] = useState("");
    const fs = useContext(FsContext);
    return <Modal heading="Save Project" closeModal={props.closeModal}>
        <div className="p-4 md:p-5 space-y-4">
            <label htmlFor="name"
                className="block mb-2 font-medium text-gray-900 dark:text-white">Project Name</label>
            <input type="text" id="name"
                className="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                required={true} minLength={1} onChange={(e) => setName(e.target.value)}
                value={name}/>
            {fs.projectHandler!.projects[name] !== undefined && <div className="text-red-300 text-sm mt-2">Project with name {name} already exists and was last saved on {new Date(fs.projectHandler!.projects[name].lastModified).toLocaleString()}. Saving will overwrite any existing project data.</div>}
        </div>
        <ModalButtons>
            <ModalPrimaryButton
                onClick={() => {
                    fs.projectHandler!.saveProject(props.root, name).then(props.closeModal);
                }}>Save
            </ModalPrimaryButton>
            <CloseModalButton closeModal={props.closeModal}>Cancel</CloseModalButton>
        </ModalButtons>
    </Modal>;
}

function ProjectItem(props: {name: string, data: ProjectDataEntry, selected: boolean, onSelect: () => unknown}) {
    return (
        <button className={"block truncate font-normal cursor-pointer p-2 w-full border-2" + (props.selected ? " bg-purple-300 border-purple-300" : " ")} onClick={props.onSelect}>{props.name}</button>
    );
}

export function OpenProjectModal(props: { closeModal: () => unknown }) {
    const [selectedProjectName, setSelectedProjectName] = useState<string | null>(null);
    const fs = useContext(FsContext);
    return <Modal heading="Open Project" closeModal={props.closeModal}>
        <div className="p-4 md:p-5 space-y-4">
            <h2>Existing Projects</h2>
            {Object.entries(fs.projectHandler!.projects).map(([name, data]) => <ProjectItem key={name} name={name} data={data} selected={selectedProjectName===name} onSelect={() => selectedProjectName === name ? setSelectedProjectName(null) : setSelectedProjectName(name)} />)}
        </div>
        <div className="text-red-300 text-sm mt-2">Opening a project will delete <b>ALL</b> current project data if not saved. Make sure to save your existing project before opening a new project.</div>
        <ModalButtons>
            <ModalPrimaryButton
                disabled={selectedProjectName === null}
                onClick={() => {
                    fs.projectHandler!.closeProject().then(() => fs.projectHandler.getProject(selectedProjectName!)).then((newRoot) => {
                        if (newRoot) {
                            fs.setRoot(newRoot);
                        } else {
                            alert("Invalid project found.");
                        }
                        props.closeModal();
                    });
                }}>Open
            </ModalPrimaryButton>
            <CloseModalButton closeModal={props.closeModal}>Cancel</CloseModalButton>
        </ModalButtons>
    </Modal>;
}
