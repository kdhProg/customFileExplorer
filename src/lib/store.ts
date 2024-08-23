import { writable } from 'svelte/store';

export const drives = writable({
    "C:\\": null,
    "D:\\": null,
});
