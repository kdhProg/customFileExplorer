<script>
import { invoke } from '@tauri-apps/api/tauri'
    
    let cur_dir = ''

    async function get_cur_dir() {
    cur_dir = await invoke('get_current_dir')
    }
</script>
<div class="entire-container">
    <p>
        cur_dir:{cur_dir}
    </p>
</div>

