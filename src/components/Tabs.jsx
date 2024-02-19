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
                    `${ activeTab === child.props.label ? 'border-black' : '' }
                    flex-1 text-gray-700 py-2`}
                onClick = { e => changeTab(e, child.props.label)}
            >
            child.props.label
            </button>
        )
    }
    )
    return (
        <div className="max-w-md mx-auto">
        <div className="flex border-b border-gray-300">
        {tab_bar}
        </div>
        </div>
    )
}

export default Tabs;
