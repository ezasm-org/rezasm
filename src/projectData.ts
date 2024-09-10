import {FsDir} from "./fsContext.ts";
import {fs} from "@tauri-apps/api";
import {BaseDirectory} from "@tauri-apps/api/fs";
import tauri_file_system from "./tauri_file_system.ts";

export interface ProjectDataEntry {
    lastModified: number; // Unix timestamp
    rootPath: string;
}

export type ProjectData = Record<string, ProjectDataEntry>;

export abstract class ProjectDataStore {
    protected savedProjects: ProjectData = {};
    abstract initDataStore(): Promise<void>;
    abstract saveProject(root: FsDir, projectName: string): Promise<void>;
    abstract closeProject(): Promise<void>;
    abstract getProject(projectName: string): Promise<FsDir | null>;

    public get projects(): Readonly<ProjectData> {
        return this.savedProjects;
    }
}

export class TauriProjectDataStore extends ProjectDataStore {
    async initDataStore(): Promise<void> {
        if (!await fs.exists("", {dir: BaseDirectory.AppLocalData})) {
            await fs.createDir("", {dir: BaseDirectory.AppLocalData, recursive: true});
        }
        if (await fs.exists("projects.json", {dir: BaseDirectory.AppLocalData})) {
            this.savedProjects = JSON.parse(await fs.readTextFile("projects.json", {dir: BaseDirectory.AppLocalData}));
        }
    }

    async saveProject(dir: FsDir, projectName: string): Promise<void> {
        this.savedProjects[projectName] = {lastModified: Date.now(), rootPath: dir.path()};
        await fs.writeTextFile("projects.json", JSON.stringify(this.savedProjects), {dir: BaseDirectory.AppLocalData});
    }

    async getProject(projectName: string): Promise<FsDir | null> {
        const projectData = this.savedProjects[projectName];
        if (!projectData) {
            return null;
        }
        try {
            await tauri_file_system.readDir({path: projectData.rootPath});
        } catch (e) {
            console.error(e);
            throw(e);
            // TODO: Handle project directory deleted
        }
        return new FsDir(projectData.rootPath, null);
    }

    async closeProject(): Promise<void> {
        // No-op
    }
}
