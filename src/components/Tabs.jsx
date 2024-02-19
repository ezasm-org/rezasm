import React, { useState } from "react";
function Tabs({children}) {
    const [activeTab, setActiveTab] = useState(children[0].props.label);

    const changeTab = (e, newTab) => {
        e.preventDefault();
        setActiveTab(newTab);
    }

    let tab_bar = children.map(child => {
        return (
            <button 
                key = {child.props.label}
                className = {
                    `${ activeTab === child.props.label ? 'active-tab' : '' }
                    flex-1 text-gray-700 py-2`}
                onClick = { e => changeTab(e, child.props.label)}
            >
            {child.props.label}
            </button>
        )
    }
    );

    let visible_tab = children.map(child => {
            return (
                <div className = {`${ activeTab !== child.props.label ? 'hidden' : ''} py-3`}>
                    {child.props.children}
                </div>
            )
        })
    return (
        <div className="fill mx-auto">
        <div className="flex border-b border-gray-300">
        {tab_bar}
        </div>
        {visible_tab}
        </div>
    )
}

function Tab({label, children}) {
    return (
        <div className = "visible" label = {label}>
        {children}
        </div>
    )
}

export {Tabs, Tab};
