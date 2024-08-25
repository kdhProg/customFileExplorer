<script lang="ts">

// Legacy file(어디에도 사용되지 않음)

// import { invoke } from "@tauri-apps/api/tauri";

// let cur_dir = "";

// let dir_lists = "";


// // 현재파일목록
// async function get_cur_dir() {
//     cur_dir = await invoke("get_current_dir");
//     dir_lists = await invoke("list_files_in_directory", { path: cur_dir });
// }
// get_cur_dir();

let defaultPath = 'c:\\';

</script>

<div class="currentPath-container">
    <input class="currentPath-box" type="text" bind:value={defaultPath}>
</div>

<style>
.currentPath-box{
    width: 700px;
}
</style>