import { writable } from 'svelte/store';

export type DriveState = {
  [key: string]: { [key: string]: string[] | null } | null;
};

export const drives = writable<DriveState>({
  "C:\\": null,
  "D:\\": null,
});