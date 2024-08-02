import {FsDir} from "./fsContext.ts";
import {fs} from "@tauri-apps/api";
import {BaseDirectory} from "@tauri-apps/api/fs";

export interface ProjectDataEntry {
    lastModified: number; // Unix timestamp
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
        if (await fs.exists("projects.json", {dir: BaseDirectory.AppLocalData})) {
            this.savedProjects = JSON.parse(await fs.readTextFile("projects.json", {dir: BaseDirectory.AppLocalData}));
        }
    }

    async saveProject(_: FsDir, projectName: string): Promise<void> {
        this.savedProjects[projectName] = {lastModified: Date.now()};
        await fs.writeTextFile("projects.json", JSON.stringify(this.savedProjects), {dir: BaseDirectory.AppLocalData});
    }

    async getProject(projectName: string): Promise<FsDir | null> {
        if (!this.savedProjects[projectName]) {
            return null;
        }
        return new FsDir("/", null);
    }

    async closeProject(): Promise<void> {
        // No-op
    }
}
