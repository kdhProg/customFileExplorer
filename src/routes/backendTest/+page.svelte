<script lang="ts">
    import { invoke } from '@tauri-apps/api/tauri'
        
        let cur_dir = ''
    
        let dir_lists = ''
    
        async function get_cur_dir() {
        cur_dir = await invoke('get_current_dir');
        dir_lists = await invoke('list_files_in_directory', { path: cur_dir });
        }


        let metaData = '';

        interface FileMetadata {
        file_name: string;
        file_size: number;
        last_modified: number;
        file_type: string;
        }
    
        let filePath = 'D:\\entire_workspace\\2024opensw_competition\\pathFinder\\src-tauri\\tauri.conf.json';

        let metadata: FileMetadata | null = null;
        let error: string | null = null;
    
        async function getMetaData() {
        try {
            
            metadata = await invoke<FileMetadata>('get_file_metadata', { filePath });
            console.log(metadata);
        } catch (err) {
            error = (err as Error).message;
        }
        }

        async function mk_new_dir(){
            const folderPath = 'D://entire_workspace//2024opensw_competition//pathFinder//src//routes//backendTest//test';

            try {
                await invoke('create_new_folder', { path: folderPath });
                console.log('Folder created successfully');
            } catch (error) {
                console.error('Failed to create folder:', error);
            }
        }


        let searchRst = {};
        async function searchFilesInDirectory() {
            const directory = 'D://entire_workspace//2024opensw_competition//pathFinder//src//routes';
            const keyword = '+';
        try {
            searchRst = await invoke('search_files', { directory, keyword });
            console.log(searchRst);
        } catch (error) {
            console.error('err:', error);
        }
        }

    </script>
<h1>
    backEnd API Test Page
</h1>
<div>
    현재 디렉토리
    cur_dir:{cur_dir}
</div>
<div>
    목록
        {dir_lists}
</div>
<hr>
<div>
    {#if metadata}
      <p><strong>File Path:</strong> {metadata.file_name}</p>
      <p><strong>File Size:</strong> {metadata.file_size} bytes</p>
      <p><strong>File type:</strong> {metadata.file_type}</p>
      <p><strong>Last Modified:</strong> {new Date(metadata.last_modified * 1000).toLocaleString()}</p>
    {:else if error}
      <p style="color: red;">Error: {error}</p>
    {:else}
      <p>Loading...</p>
    {/if}
</div>
<button on:click="{get_cur_dir}">test</button>
<button on:click="{getMetaData}">metaData Test</button>
<button on:click="{mk_new_dir}">new dir</button>
<hr>
<button on:click="{searchFilesInDirectory}">searchFilesInDirectory</button>