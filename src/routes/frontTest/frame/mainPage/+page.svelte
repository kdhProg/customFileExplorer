<script lang="ts">

import { onMount, afterUpdate } from 'svelte';

    import { isDirectory, listFilesInDirectory } from "$lib/api";
    import { invoke } from "@tauri-apps/api/tauri";
    import Folder from '$lib/components/Folder.svelte';
    import { drives } from '$lib/store';

    // import - css
    import "$lib/style/global_features.css"
    import "/src/lib/style/mainpage.css"

    // import - components
    import Navi from "$lib/components/navi.svelte";


    let showSettings = false;
    let activeTab = "interface";
    let viewMode = "single"; // 기본 모드는 single (하나의 파일 탐색기)
    let fileSize = 80; // 기본 파일 아이템 크기
    let selectedDriveLeft = null; // 왼쪽 패널에서 선택된 드라이브
    let selectedDriveRight = null; // 오른쪽 패널에서 선택된 드라이브
    let selectedFolderLeft = null; // 왼쪽 패널에서 선택된 폴더
    let selectedFolderRight = null; // 오른쪽 패널에서 선택된 폴더
    let filesInFolderLeft = []; // 왼쪽 패널에서 선택된 폴더의 파일들
    let filesInFolderRight = []; // 오른쪽 패널에서 선택된 폴더의 파일들
    let openedDrives = {}; // 드라이브 토글 상태 관리

    // 설정 모달 열기/닫기
    function toggleSettings() {
        showSettings = !showSettings;
    }

    // 탭 변경
    function changeTab(tab) {
        activeTab = tab;
    }

    // 보기 모드 변경
    function changeViewMode(mode) {
        viewMode = mode;
        showSettings = false; // 설정 모달 닫기
    }

    // 파일사이즈
    function updateFileSize(event: Event){
        const target = event.target as HTMLInputElement;
        fileSize = parseInt(target.value);
    }



    // 현재폴더 경로
    let curFolderName = '';
    let filesInCurrentFolder: string[] = []; // 현재 폴더의 파일 목록을 저장할 배열

    // 디렉토리 리스트에서 파일 클릭
    async function handleFolderSelected(event) {
        curFolderName = event.detail;
        filesInCurrentFolder = await listFilesInDirectory(curFolderName);
        // console.log(typeof filesInCurrentFolder[0])
    }



    // 파일 아이콘 설정 (파일 이름에 따라 아이콘을 다르게 설정하는 함수)
    function getFileIcon(file: string): string {
        if (file.includes(".txt")) return "📄";
        if (file.includes(".jpg") || file.includes(".png")) return "🖼️";
        if (file.includes(".mp4")) return "🎥";
        if (file.includes(".exe")) return "💻";
        return "📁";
    }

    // 파일명 추출
    function getFileName(filePath:string) {
        const parts = filePath.split(/[/\\]/);
        return parts[parts.length - 1];
    }


    // 테마
    // 기본 CSS 파일 로드
    let currentTheme = '/src/lib/style/themes/default_theme.css';

    // CSS 파일을 동적으로 변경하는 함수
    function applyTheme(themePath) {
        const existingLink = document.querySelector('#dynamic-theme');
        
        // 기존의 link 태그가 존재하면 경로를 변경
        if (existingLink) {
            existingLink.href = themePath;
        } else {
            // 새로운 link 태그를 생성하여 추가
            const linkElement = document.createElement('link');
            linkElement.rel = 'stylesheet';
            linkElement.id = 'dynamic-theme';
            linkElement.href = themePath;
            document.head.appendChild(linkElement);
        }

        // 현재 테마 경로 업데이트
        currentTheme = themePath;
    }

    // 페이지 로드 시 기본 테마 적용
    applyTheme(currentTheme);


    // 검색박스
    async function searchFilesInDirectory() {
        // console.log('clicked!')
        try {
            // 기본값 임시 설정
            // let C_directory: string = "D://entire_workspace//2024opensw_competition//pathFinder//src";
            let C_directory: string = "C://";
            let D_directory: string = "D://";
            const keyword = document.getElementById('searchInput');

            let C_searchRst: string[];
            let D_searchRst: string[];
            
            if (keyword instanceof HTMLInputElement) {
                const inputValue = keyword.value;
                // console.log(inputValue);

                console.log('searching.......')
                console.time("search_API_time_analysis");

                C_searchRst = await invoke("search_files", { directory:C_directory, keyword:inputValue });
                D_searchRst = await invoke("search_files", { directory:D_directory, keyword:inputValue });

                console.log('searching finished!')
                console.timeEnd("search_API_time_analysis");

                const C_fileNames = C_searchRst.map((item: any) => item.file_name);
                const D_fileNames = D_searchRst.map((item: any) => item.file_name);

                filesInCurrentFolder = C_fileNames.concat(D_fileNames);
                // console.log(typeof filesInCurrentFolder[0])
                // const temp = filesInCurrentFolder = C_searchRst.concat(D_searchRst);
                // console.log(temp);
            } else {
                console.error("Input element not found or is not of type HTMLInputElement");
            }
        } catch (error) {
            console.error("err:", error);
        }
    }

    
    //각 폴더 클릭
    async function eachFolderClick(file:string){
        curFolderName = file;
        filesInCurrentFolder = await listFilesInDirectory(curFolderName);
    }
    

    // 분할바 관련
    let sidebarWidth = 250; // 초기 사이드바 너비를 전역 변수로 관리

function updateSidebarWidth(width) {
    sidebarWidth = width;
    document.getElementById('sidebar').style.width = `${sidebarWidth}px`;
    document.getElementById('sidebar').style.minWidth = `${sidebarWidth}px`;
    document.getElementById('sidebar').style.maxWidth = `${sidebarWidth}px`;
}

onMount(() => {
    const sidebar = document.getElementById('sidebar');
    const resizer = document.getElementById('resizer');

    let startX;
    let startWidth;

    const mouseDownHandler = function (e) {
        startX = e.clientX;
        startWidth = sidebar.offsetWidth;

        document.addEventListener('mousemove', mouseMoveHandler);
        document.addEventListener('mouseup', mouseUpHandler);

        document.body.style.userSelect = 'none';
        document.body.style.cursor = 'col-resize';
    };

    const mouseMoveHandler = function (e) {
        const dx = e.clientX - startX;
        let newWidth = startWidth + dx;

        const minWidth = 150;
        const maxWidth = 500; // 사용자가 조정 가능한 최대 너비

        if (newWidth < minWidth) {
            newWidth = minWidth;
        } else if (newWidth > maxWidth) {
            newWidth = maxWidth;
        }

        updateSidebarWidth(newWidth);
    };

    const mouseUpHandler = function () {
        document.removeEventListener('mousemove', mouseMoveHandler);
        document.removeEventListener('mouseup', mouseUpHandler);

        document.body.style.userSelect = '';
        document.body.style.cursor = '';
    };

    resizer.addEventListener('mousedown', mouseDownHandler);

    // Ensure initial sidebar width is set
    updateSidebarWidth(sidebarWidth);
});

afterUpdate(() => {
    // 폴더를 펼치거나 접을 때 사이드바의 너비를 재설정
    updateSidebarWidth(sidebarWidth);
});



</script>

<!-- 메인 화면 -->
 <!-- <button on:click={()=>{console.log('curFolderName'+curFolderName)}}>testtest</button> -->
<div class="main-container">
    <!-- 상단 바 -->
    <header class="top-bar">
        <div class="logo">pathFinder</div>
        <div class="settings-icon" on:click={toggleSettings}>⚙️</div>
    </header>

    <!-- 네비게이션 바 -->
    <div class="navi-container">
        <Navi/>
    </div>

    <!-- 현재 디렉토리 -->
     <div class="current-directory-box">
       <input type="text" value={curFolderName}>
     </div>

    <!-- 검색박스 -->
    <div>
        <input id="searchInput" class="searchBoxInput" type="text">
        <button id="searchButton" class="searchBoxButton" on:click={searchFilesInDirectory}>검색</button>
    </div>

    <div class="content-wrapper {viewMode === 'dual' ? 'dual-view' : ''}">
        <!-- 좌측 패널: 드라이브 및 폴더 탐색기 -->
        <aside class="sidebar" id="sidebar">
            {#each Object.keys($drives) as drive}
                <Folder path={drive} name={drive} items={$drives[drive]} on:folderSelected={handleFolderSelected}/>
            {/each}
        </aside>


        <!-- 추가: 사이드바와 파일 뷰어 사이의 분할자 -->
        <div class="resizer" id="resizer"></div>

        <!-- 좌측 파일 탐색기 -->
        <div class="file-viewer" id="fileViewer">
            {#if filesInCurrentFolder.length > 0}
                {#each filesInCurrentFolder as file}
                    <div
                        class="file-item"
                        style="width: {fileSize}px; height: {fileSize}px;"
                        on:dblclick={() => eachFolderClick(file)}
                    >
                        <span class="file-icon">{getFileIcon(file)}</span>
                        <span class="file-name">{getFileName(file)}</span>
                    </div>
                {/each}
            {:else if selectedDriveLeft && selectedFolderLeft}
                <p>이 폴더는 비어 있습니다</p>
            {:else}
                <p>폴더를 선택하세요</p>
            {/if}
        </div>

        {#if viewMode === "dual"}
            <!-- 우측 패널: 드라이브 및 폴더 탐색기 -->
            <aside class="sidebar">
                {#each Object.keys($drives) as drive}
                    <Folder path={drive} name={drive} items={$drives[drive]} />
                {/each}
            </aside>

            <!-- 우측 파일 탐색기 -->
            <div class="file-viewer">
                {#if filesInCurrentFolder.length > 0}
                    {#each filesInCurrentFolder as file}
                        <div
                            class="file-item"
                            style="width: {fileSize}px; height: {fileSize}px;"
                        >
                            <span class="file-icon">{getFileIcon(file)}</span>
                            <span class="file-name">{file}</span>
                        </div>
                    {/each}
                {:else if selectedDriveRight && selectedFolderRight}
                    <p>이 폴더는 비어 있습니다</p>
                {:else}
                    <p>폴더를 선택하세요</p>
                {/if}
            </div>
        {/if}
    </div>

    <!-- 설정 모달 -->
    {#if showSettings}
        <div class="settings-modal">
            <div class="modal-content">
                <h2>설정</h2>
                <ul class="tabs">
                    <li
                        class:active={activeTab === "interface"}
                        on:click={() => changeTab("interface")}
                    >
                        인터페이스
                    </li>
                    <li
                        class:active={activeTab === "resize"}
                        on:click={() => changeTab("resize")}
                    >
                        화면 크기 조절
                    </li>
                    <li
                        class:active={activeTab === "themes"}
                        on:click={() => changeTab("themes")}
                    >
                        테마선택
                    </li>
                </ul>
                <div class="tab-content">
                    {#if activeTab === "interface"}
                        <h3>인터페이스 설정</h3>
                        <button on:click={() => changeViewMode("single")}
                            >화면 하나로 보기</button
                        >
                        <button on:click={() => changeViewMode("dual")}
                            >화면 두 개로 보기</button
                        >
                    {:else if activeTab === "resize"}
                        <h3>화면 크기 조절</h3>
                        <input
                            type="range"
                            min="50"
                            max="150"
                            value={fileSize}
                            on:input={updateFileSize}
                        />
                        <p>파일 아이콘 크기: {fileSize}px</p>
                    {:else if activeTab === "themes"}
                    <h3>테마 선택</h3>
                    <button on:click={() => applyTheme('/src/lib/style/themes/default_theme.css')}>디폴트 테마</button>
                    <button on:click={() => applyTheme('/src/lib/style/themes/retro_theme.css')}>레트로 테마</button>
                    <button on:click={() => applyTheme('/src/lib/style/themes/sf_style_theme.css')}>SF 테마</button>
                    {/if}
                </div>
                <button class="close-modal" on:click={toggleSettings}
                    >닫기</button
                >
            </div>
        </div>
    {/if}
</div>
<a href="/frontTest/frame">Go to previous page</a>
