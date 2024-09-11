<script lang="ts">

import { onMount, afterUpdate } from 'svelte';

    import { isDirectory, listFilesInDirectory } from "$lib/api";
    import { invoke } from "@tauri-apps/api/tauri";
    import Folder from '$lib/components/Folder.svelte';
    import { drives } from '$lib/store';

    import { language } from '$lib/language';
    import { translations } from '$lib/i18n/translations';

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
        // Todo : 더블클릭 대상이 폴더면 해당 파일리스트 반환 수행 / 일반파일이면 기본설정프로그램으로 실행


        filesInCurrentFolder = await listFilesInDirectory(curFolderName);
        // console.log(typeof filesInCurrentFolder[0])
    }


    // 테마에 따라 로고 이미지를 설정하는 객체
    const themeLogos = {
    default: "/icons/dir_logo_default.png",
    retro: "/icons/dir_logo_retro.png",
    sf: "/icons/dir_logo_sf.png",
    linux: "/icons/dir_logo_linux.png"
    };

    // 기본 로고
    let currentLogo = themeLogos.default; 

    let default_txt = "/icons/exe_txt.png";
    let default_jpg = "/icons/exe_jpg.png";
    let default_mp4 = "/icons/exe_mp4.png";
    let default_exe = "/icons/exe_default.png";

    // 파일 아이콘 설정 (파일 이름에 따라 아이콘을 다르게 설정하는 함수)
    function getFileIcon(file: string): string {
        if (file.includes(".txt")) return default_txt;
        if (file.includes(".jpg") || file.includes(".png")) return default_jpg;
        if (file.includes(".mp4")) return default_mp4;
        if (file.includes(".exe")) return default_exe;
        // return "📁";
        return currentLogo;
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

            // 테마에 따라 로고 이미지를 변경
        if (themePath.includes('default')) {
            currentLogo = themeLogos.default;
        } else if (themePath.includes('retro')) {
            currentLogo = themeLogos.retro;
        } else if (themePath.includes('sf')) {
            currentLogo = themeLogos.sf;
        } else if(themePath.includes('linux')) {
            currentLogo = themeLogos.linux;
        }
        // 현재 테마 경로 업데이트
        currentTheme = themePath;

        filesInCurrentFolder = [...filesInCurrentFolder];
    }

    // 페이지 로드 시 기본 테마 적용
    applyTheme(currentTheme);


    // 검색실행여부 변수
    let isSearching:boolean = false;

    // 검색박스
    async function searchFilesInDirectory() {
        // console.log('clicked!')
        try {
            isSearching = true;
            const keyword = document.getElementById('searchInput');

            if(curFolderName === '' || curFolderName.length === 0){
                //현재 파일 경로가 없다면(=초기화면) 기본값은 C,D에서 모두 탐색
                let C_directory: string = "C://";
                let D_directory: string = "D://";

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

            }else{
                // 현재 디렉토리에서 검색
                if (keyword instanceof HTMLInputElement) {
                const inputValue = keyword.value;

                let searchRst: string[];

                console.log('searching.......')
                console.time("search_API_time_analysis");

                searchRst = await invoke("search_files", { directory:curFolderName, keyword:inputValue });

                console.log('searching finished!')
                console.timeEnd("search_API_time_analysis");

                const searchRstmapped = searchRst.map((item: any) => item.file_name);

                filesInCurrentFolder = searchRstmapped;
                } else {
                    console.error("Input element not found or is not of type HTMLInputElement");
                }

            }
            
            isSearching = false;

            
            
            
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


// 언어설정
function switchLanguage(lang: string) {
    language.set(lang);
}

// Reactive derived store to get the current translations
$: currentTranslations = translations[$language];




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
        <div>
            {currentTranslations.nav_file}
        </div>
        <div>
            {currentTranslations.nav_home}
        </div>
        <div>
            {currentTranslations.nav_view}
        </div>
        <div>
            {currentTranslations.nav_help}
        </div>
    </div>

    <!-- 현재 디렉토리 / 검색박스 / 이동버튼 -->
    <div class="util-bar-container">
        <!-- 이동버튼 -->
        <div class="movement-button-container">
            <div>
                ←
            </div>
            <div>
                →
            </div>
            <div>
                ↑
            </div>
        </div>

        <!-- 현재 디렉토리 -->
        <div class="current-directory-container">
            <input type="text" class="current-directory-inputbox" value={curFolderName}>
        </div>

        <!-- 검색박스 -->
        <div class="search-container">
            <input id="searchInput" class="searchbox-input" type="text" placeholder="{curFolderName}">
            {#if isSearching}
            <button id="searchButton" class="searchbox-button" disabled>🔍</button>
            {:else}
            <button id="searchButton" class="searchbox-button" on:click={searchFilesInDirectory}>🔍</button>
            {/if}
        </div>
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
                        <!-- <span class="file-icon">{getFileIcon(file)}</span> -->
                        <img src="{getFileIcon(file)}" alt="File Icon" class="file-icon">
                        <span class="file-name">{getFileName(file)}</span>
                    </div>
                {/each}
            {:else if selectedDriveLeft && selectedFolderLeft}
                <p>{currentTranslations.no_folder}</p>
            {:else}
                <p>{currentTranslations.sel_folder}</p>
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
                    <p>{currentTranslations.no_folder}</p>
                {:else}
                    <p>{currentTranslations.sel_folder}</p>
                {/if}
            </div>
        {/if}
    </div>

    <!-- 설정 모달 -->
    {#if showSettings}
        <div class="settings-modal">
            <div class="modal-content">
                <h2>{currentTranslations.settings}</h2>
                <ul class="tabs">
                    <li
                        class:active={activeTab === "interface"}
                        on:click={() => changeTab("interface")}
                    >
                    {currentTranslations.interface}
                    </li>
                    <li
                        class:active={activeTab === "resize"}
                        on:click={() => changeTab("resize")}
                    >
                    {currentTranslations.resize}
                    </li>
                    <li
                        class:active={activeTab === "themes"}
                        on:click={() => changeTab("themes")}
                    >
                    {currentTranslations.themes}
                    </li>
                    <li
                        class:active={activeTab === "language"}
                        on:click={() => changeTab("language")}
                    >
                    {currentTranslations.language}
                    </li>
                </ul>
                <div class="tab-content">
                    {#if activeTab === "interface"}
                        <h3>{currentTranslations.interface_set}</h3>
                        <button on:click={() => changeViewMode("single")}>{currentTranslations.inter_one_panel}</button>
                        <button on:click={() => changeViewMode("dual")}>{currentTranslations.inter_two_panel}</button>
                    {:else if activeTab === "resize"}
                        <h3>{currentTranslations.resize}</h3>
                        <input
                            type="range"
                            min="50"
                            max="150"
                            value={fileSize}
                            on:input={updateFileSize}
                        />
                        <p>{currentTranslations.file_icon_size}: {fileSize}px</p>
                    {:else if activeTab === "themes"}
                    <h3>{currentTranslations.themes}</h3>
                    <button class="theme_btn" on:click={() => applyTheme('/src/lib/style/themes/default_theme.css')}>{currentTranslations.default_theme}</button>
                    <button class="theme_btn" on:click={() => applyTheme('/src/lib/style/themes/retro_theme.css')}>{currentTranslations.retro_theme}</button>
                    <button class="theme_btn" on:click={() => applyTheme('/src/lib/style/themes/sf_style_theme.css')}>{currentTranslations.sf_style_theme}</button>
                    <button class="theme_btn" on:click={() => applyTheme('/src/lib/style/themes/linux_style_theme.css')}>{currentTranslations.linux_theme}</button>
                    {:else if activeTab === "language"}
                    <h3>{currentTranslations.language}</h3>
                    <button id="lang_btn_en" class="lang_btn" on:click={() => switchLanguage('en')}>English</button>
                    <button class="lang_btn" on:click={() => switchLanguage('ko')}>한국어</button>
                    {/if}
                </div>
                <button class="close-modal" on:click={toggleSettings}>{currentTranslations.modal_close}</button
                >
            </div>
        </div>
    {/if}
</div>
<a href="/frontTest/frame">Go to previous page</a>
