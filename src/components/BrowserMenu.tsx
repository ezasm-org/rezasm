import {Menu, MenuButton, MenuItem, MenuItems, Transition} from "@headlessui/react";
import React, {PropsWithChildren} from "react";

export function MenuHeading(props: PropsWithChildren) {
    return (
        <MenuButton
            className="inline-flex w-full justify-center gap-x-1.5 bg-white px-3 py-2 text-sm font-semibold text-gray-900 shadow-sm hover:bg-gray-50">
            {props.children}
        </MenuButton>
    );
}

export function MenuOption(props: PropsWithChildren<React.JSX.IntrinsicElements["a"]>) {
    const {children, ...otherProps} = props;
    return <MenuItem>
        {({focus}) => (
            <a
                {...otherProps}
                role="menuitem"
                href="#"
                className={
                    (focus ? "bg-gray-100 text-gray-900" : "text-gray-700") +
                    " block px-4 py-2 text-sm"
                }
            >
                {children}
            </a>
        )}
    </MenuItem>;
}

function MenuSection(props: PropsWithChildren) {
    return <div className="menu-section">
        {props.children}
    </div>;
}

function SectionMenu(props: PropsWithChildren<{ heading: string }>) {
    return <Menu as="div" className="relative inline-block text-left">
        <div>
            <MenuHeading>{props.heading}</MenuHeading>
        </div>
        <Transition
            enter="transition ease-out duration-100"
            enterFrom="transform opacity-0 scale-95"
            enterTo="transform opacity-100 scale-100"
            leave="transition ease-in duration-75"
            leaveFrom="transform opacity-100 scale-100"
            leaveTo="transform opacity-0 scale-95"
        >
            <MenuItems
                className="absolute z-10 mt-2 w-56 divide-y divide-gray-100 origin-top-right bg-white shadow-lg focus:outline-none">
                {props.children}
            </MenuItems>
        </Transition>
    </Menu>;
}

function FileMenu() {
    return <SectionMenu heading="File">
        <MenuSection>
            <MenuOption>
                Open Folder
            </MenuOption>
            <MenuOption>
                Open File
            </MenuOption>
        </MenuSection>
        <MenuSection>
            <MenuOption>
                Save
            </MenuOption>
        </MenuSection>
        <MenuSection>
            <MenuOption>
                Export File
            </MenuOption>
            <MenuOption>
                Export Folder
            </MenuOption>
            <MenuOption>
                Export Project
            </MenuOption>
        </MenuSection>
    </SectionMenu>;
}

function EditMenu() {
    return <SectionMenu heading="Edit">
        <MenuSection>
            <MenuOption>
                Undo
            </MenuOption>
            <MenuOption>
                Redo
            </MenuOption>
        </MenuSection>
        <MenuSection>
            <MenuOption>
                Cut
            </MenuOption>
            <MenuOption>
                Copy
            </MenuOption>
            <MenuOption>
                Paste
            </MenuOption>
        </MenuSection>
    </SectionMenu>;
}

export default function BrowserMenu() {
    return (
        <header className="menu-bar px-4 space-x-1 flex">
            <FileMenu/>
            <EditMenu/>
        </header>
    );
}
