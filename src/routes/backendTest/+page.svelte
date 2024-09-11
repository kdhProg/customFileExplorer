<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";

    let cur_dir = "";

    let dir_lists = "";

    // 현재파일목록
    async function get_cur_dir() {
        cur_dir = await invoke("get_current_dir");
        dir_lists = await invoke("list_files_in_directory", { path: cur_dir });
        console.log(dir_lists)
    }
    
    let metaData = "";

    interface FileMetadata {
        file_name: string;
        file_size: number;
        last_modified: number;
        file_type: string;
    }

    let filePath =
        "D:\\entire_workspace\\2024opensw_competition\\pathFinder\\src-tauri";

    let metadata: FileMetadata | null = null;
    let error: string | null = null;

    // 메타데이터 호출
    async function getMetaData() {
        try {
            metadata = await invoke<FileMetadata>("get_file_metadata", {
                filePath,
            });
            console.log(metadata);
        } catch (err) {
            error = (err as Error).message;
        }
    }

    // 새폴더 생성
    async function mk_new_dir() {
        const folderPath =
            "D://entire_workspace//2024opensw_competition//pathFinder//src//routes//backendTest//test";

        try {
            await invoke("create_new_folder", { path: folderPath });
            console.log("Folder created successfully");
        } catch (error) {
            console.error("Failed to create folder:", error);
        }
    }

    // 검색 테스트
    let searchRst = {};
    let schfin = '';
    let schCount = 0;
    async function searchFilesInDirectory() {
        schfin = '검색중....';
        // const directory ="D://entire_workspace//2024opensw_competition//pathFinder//src//routes";
        const directory ="D://";
        const keyword = "테스트_찾기파일";
        try {
            console.time("search_API_time_analysis");
            searchRst = await invoke("search_files", { directory, keyword });
            console.timeEnd("search_API_time_analysis");
            schfin='완료';
            schCount = searchRst.length;
        } catch (error) {
            console.error("err:", error);
        }
    }

    let doOtherTasksVal = 1;
    function doOtherTasks(){
        doOtherTasksVal += 1;
    }

    // 휴지통 테스트
    async function deleteFile() {
        try {
            const del_path =
                "D://entire_workspace//2024opensw_competition//pathFinder//src//routes//backendTest//a.txt";
            await invoke("move_to_trash", { delPath: del_path });
            // 난점 --> 백엔드 인자명은 del_path인데 여기서는 카멜표기법으로 해야 인식함
            console.log("File moved to trash successfully");
        } catch (error) {
            console.error("Failed to delete file:", error);
        }
    }

    // 폴더여부 테스트
    let result = "";

    let isFileDirFilePath = "C:\\";

    async function checkIfDirectory() {
        try {
        const isDir = await invoke('is_directory', { path: isFileDirFilePath });
        result = isDir ? "This is a directory." : "This is not a directory.";
        } catch (error) {
        result = `Error: ${error}`;
        }
    }


    // 기본파일 실행 테스트

    let open_file_de_test_path = "D://entire_workspace//2024opensw_competition//pathFinder//static//mainLogo.png";
    let open_file_ret = "";
    async function open_file_with_default_program() {
        try {
        const ret = await invoke('open_file_with_default_program', { filePath: open_file_de_test_path });
        } catch (error) {
            open_file_ret = `Error: ${error}`;
        }
    }


    // 스토리지 디바이스 목록 가져오기 테스트
    //get_drive_info
    let drives_infos = [];

    async function get_drive_info() {
    try {
        drives_infos = await invoke('get_drive_info');
        console.log(drives_infos);
    } catch (error) {
        console.error('Failed to fetch drive list:', error);
    }
    }

</script>

<h1>backEnd API Test Page</h1>
<div>
    현재 디렉토리 cur_dir:{cur_dir}
</div>
<div>
    목록
    {dir_lists}
</div>
<hr />
<div>
    {#if metadata}
        <p><strong>File Path:</strong> {metadata.file_name}</p>
        <p><strong>File Size:</strong> {metadata.file_size} bytes</p>
        <p><strong>File type:</strong> {metadata.file_type}</p>
        <p>
            <strong>Last Modified:</strong>
            {new Date(metadata.last_modified * 1000).toLocaleString()}
        </p>
    {:else if error}
        <p style="color: red;">Error: {error}</p>
    {:else}
        <p>Loading...</p>
    {/if}
</div>
<button on:click={get_cur_dir}>test</button>
<button on:click={getMetaData}>metaData Test</button>
<button on:click={mk_new_dir}>new dir</button>
<hr />
<button on:click={searchFilesInDirectory}>searchFilesInDirectory</button>
<button on:click={doOtherTasks}>다른 task : {doOtherTasksVal}</button>
<div style="color: red">
    탐색여부 : {schfin}
</div>
<div>
    찾은 파일 개수 : {schCount}
</div>
<hr />
<button on:click={deleteFile}>deleteFile</button>
<hr>
<button on:click={checkIfDirectory}>Is Folder? Check</button>
<p>{result}</p>
<hr>
<button on:click={open_file_with_default_program}>EXE run test</button>
<p>{open_file_ret}</p>
<hr>
<p>스토리지 디바이스 가져오기</p>
<button on:click={get_drive_info}>get_drive_info</button>
<div>
    {drives_infos}
</div>

<a href="/">Go to previous page</a>
