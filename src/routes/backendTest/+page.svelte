<script>
    import { invoke } from '@tauri-apps/api/tauri'
        
        let cur_dir = ''
    
        let dir_lists = ''
    
        async function get_cur_dir() {
        cur_dir = await invoke('get_current_dir');
        dir_lists = await invoke('list_files_in_directory', { path: cur_dir });
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
<button on:click="{get_cur_dir}">test</button>