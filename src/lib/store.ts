import { writable } from 'svelte/store';

export type FolderStructure = {
  name: string;
  path: string;
  children: FolderStructure[] | null;
};

export type DriveState = {
  [key: string]: FolderStructure | null;
};

export const drives = writable<DriveState>({
  "C:\\": null,
  "D:\\": null,
});
