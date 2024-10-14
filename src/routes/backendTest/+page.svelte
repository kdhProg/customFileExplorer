<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import { listen } from '@tauri-apps/api/event';
    import { onMount } from "svelte";
    import SelectBox from "$lib/components/selectBox.svelte";

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

    // 검색 테스트 - Deprecated
    // let searchRst = {};
    // let schfin = '';
    // let schCount = 0;
    // async function searchFilesInDirectory() {
    //     schfin = '검색중....';
    //     // const directory ="D://entire_workspace//2024opensw_competition//pathFinder//src//routes";
    //     const directory ="D://";
    //     const keyword = "테스트_찾기파일";
    //     try {
    //         console.time("search_API_time_analysis");
    //         searchRst = await invoke("search_files", { directory, keyword });
    //         console.timeEnd("search_API_time_analysis");
    //         schfin='완료';
    //         schCount = searchRst.length;
    //     } catch (error) {
    //         console.error("err:", error);
    //     }
    // }

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

    let mount_point = '';
    let total_space = 0;
    let available_space = 0;

    async function get_drive_info() {
        try {
            drives_infos = await invoke('get_drive_info');
            mount_point = drives_infos[0].mount_point;
            total_space = drives_infos[0].total_space;
            available_space = drives_infos[0].available_space;
        } catch (error) {
            console.error('Failed to fetch drive list:', error);
        }
    }

// ----------------------- new 검색테스트 -----------------------
let searchProcessId = null;
let unlisten;
let receivedFiles = new Set();

async function startSearch() {
    // 기존 리스너 제거 및 초기화
    if (unlisten) {
        await unlisten();
        unlisten = null;
    }

    receivedFiles.clear();
    
    // 탐색 프로세스 ID를 미리 받아오기 위한 리스너
    unlisten = await listen('process-info', (event) => {
        const processInfo = event.payload;
        if (processInfo && processInfo.id) {
            searchProcessId = processInfo.id;  // Process ID를 저장
            console.log("Process ID from backend:", searchProcessId);
        }
    });

    // 실시간 탐색 결과 리스너 등록
    unlisten = await listen('search-result', (event) => {
        const file = event.payload;
        if (!receivedFiles.has(file.file_path)) {
            receivedFiles.add(file.file_path);
            console.log("Real-time search result:", file);
        }
    });

    try {
        await invoke('search_files', {
            keyword: 'temp', 
            directory: 'D://entire_workspace//2024opensw_competition',
            options: {
                customThreadPoolUse: false,
                threadPoolNum: "0",
                searchScope: "0",
                customFileContUse: false,
                customPropertyUse: false,
                customFileSizeUse: false,
                sizeMax: 0,
                sizeMin: 0,
                customFileCrtDateUse: false,
                crtStart: '',
                crtEnd: '',
                customFileModiDateUse: false,
                modiStart: '',
                modiEnd: '',
                customFileOwnerUse: false,
                ownerName: '',
                customFileTypeUse: false,
                fileTypeList: '',
                customSymbolicChk: false,
                customSchMethod: "0",
            }
        });
    } catch (error) {
        console.error("Search failed:", error);
        searchProcessId = null;
    }
}

async function cancelSearch() {
    console.log("searchProcessId ID: ", searchProcessId);
    if (searchProcessId) {
        try {
            console.log("Cancel request sent for process ID: ", searchProcessId);
            await invoke('cancel_search', { processId: searchProcessId });
            console.log(`Cancel request for process: ${searchProcessId} has been acknowledged`);
        } catch (error) {
            console.error("Failed to invoke cancel_search:", error);
        }
    } else {
        console.log("No active search process to cancel.");
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
<!-- <button on:click={searchFilesInDirectory}>searchFilesInDirectory</button> -->
<button on:click={doOtherTasks}>다른 task : {doOtherTasksVal}</button>
<!-- <div style="color: red">
    탐색여부 : {schfin}
</div>
<div>
    찾은 파일 개수 : {schCount}
</div> -->
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
    <p>
        mount_point : {mount_point}
    </p>
    <p>
        total_space : {total_space}
    </p>
    <p>
        available_space : {available_space}
    </p>
</div>
<hr>
<!-- NEW 검색 테스트 -->
<button on:click={startSearch}>Start Search</button>
<button on:click={cancelSearch}>Cancel Search</button>
<hr>
<a href="/">Go to previous page</a>
