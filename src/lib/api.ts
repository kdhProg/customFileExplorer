import { invoke } from '@tauri-apps/api/tauri';

export async function listFilesInDirectory(path: string): Promise<string[]> {
    return invoke('list_files_in_directory', { path });
}

interface Result<T, E> {
    Ok?: T;
    Err?: E;
}

export async function isDirectory(path: string): Promise<boolean> {
    // console.log("API path : "+path );
    const result = await invoke<Result<boolean, string>>('is_directory', { path });
    // console.log("RST : " + result );
    // console.log(typeof(result));
    if (result) {
        return true;
    } else {
        console.error({ result });
        return false;
    }
}