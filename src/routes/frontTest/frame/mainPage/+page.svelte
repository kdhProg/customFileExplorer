<script lang="ts">

    import fs from 'fs';
    import path from 'path';

    import { onMount, afterUpdate } from 'svelte';

    import { isDirectory, listFilesInDirectory, openFileWithDefaultProgram } from "$lib/api";
    import { invoke } from "@tauri-apps/api/tauri";
    import Folder from '$lib/components/Folder.svelte';
    import { drives,updateDrives } from '$lib/store';

    import DiscInfo from '$lib/components/discInfo.svelte';

    import TitleBar from '$lib/components/titleBar.svelte';

    import { language } from '$lib/language';
    import { translations } from '$lib/i18n/translations';

    // import - css
    import "$lib/style/global_features.css"
    import "/src/lib/style/mainpage.css"


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

    // Set File icon
    function getFileIcon(file: string): string {
        if (file.includes(".txt")) return default_txt;
        if (file.includes(".jpg") || file.includes(".png")) return default_jpg;
        if (file.includes(".mp4")) return default_mp4;
        if (file.includes(".exe")) return default_exe;
        // return "📁";
        return currentLogo;
    }

    // Extract fileName
    function getFileName(filePath:string) {
        const parts = filePath.split(/[/\\]/);
        return parts[parts.length - 1];
    }


    // Theme
    // default theme ]
    let currentTheme = '/src/lib/style/themes/default_theme.css';

    // Change CSS
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

    
    // Click Each Folder / Files
    // Folder - update current folder list
    // File - execute with default enrolled programs
    async function eachFolderClick(file:string){
        curFolderName = file;
        let isDir = await isDirectory(curFolderName);
        if(isDir){
            // case : this is directory
            filesInCurrentFolder = await listFilesInDirectory(curFolderName);
        }else{
            // case : this is folder
            openFileWithDefaultProgram(curFolderName);
            
        }
    }


    // util bars
    async function load_util_buttons(){
        let jsonData = {};

        try {
            const response = await invoke('read_json_file');
            jsonData = JSON.parse(response);
        } catch (error) {
            console.error('JSON 파일을 가져오는 중 오류 발생:', error);
        }

    return jsonData;
    }   

    let utilButtons = []

    // util_buttons toggle checkbox
    function toggleItem(value, checked) {
        if (checked) {
        // 체크된 경우 배열에 추가
        utilButtons = [...utilButtons, value];
        } else {
        // 체크 해제된 경우 배열에서 제거
        utilButtons = utilButtons.filter(item => item !== value);
        }

        // apply when item changed
        util_apply();
    }

    // check if utilButtons have values
    function isChecked(value) {
     return utilButtons.includes(value);
    }

    // utilButtons apply button - backend
    async function util_apply() {
    try {
      await invoke('save_util_buttons', { buttons: utilButtons });
    } catch (error) {
      console.error('Failed to send buttons:', error);
    }
  }
    

// 분할바 관련
let sidebarWidth = 250; // 초기 사이드바 너비를 전역 변수로 관리

function updateSidebarWidth(width) {
    sidebarWidth = width;
    document.getElementById('sidebar').style.width = `${sidebarWidth}px`;
    document.getElementById('sidebar').style.minWidth = `${sidebarWidth}px`;
    document.getElementById('sidebar').style.maxWidth = `${sidebarWidth}px`;
}


// onMount -> load when page starts
onMount(() => {

    load_util_buttons().then(data => {
      utilButtons = data.buttons; // 받아온 데이터를 utilButtons에 할당
    });


    // 컴포넌트 첫 로드 시 드라이브 목록 업데이트
    updateDrives();


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


// Set Language
function switchLanguage(lang: string) {
    language.set(lang);
}

// Reactive derived store to get the current translations
$: currentTranslations = translations[$language];

// drives 스토어 구독
// 디버깅용 -> <button on:click={()=>{console.log(driveList)}}>test</button>와 같이 활용
// $: driveList = $drives;

// main logo click event (open github-repo)
function openGitgubRepo(){
    window.open('https://github.com/kdhProg/customFileExplorer', '_blank');
}


// ---- util bar ----




</script>

<!-- Main Screen -->
 <!-- <button on:click={()=>{console.log('curFolderName'+curFolderName)}}>test</button> -->
<div class="main-container">
    <TitleBar/>
    <!-- Current Directory / SearchBox / MovementButton -->
    <div class="header-container">

        <!-- logo -->
        <div class="logo-container">
            <!-- pathFinder -->
             <div class="main-logo-img-wrapper" on:click={openGitgubRepo}>
                <img class="main-logo-img" src="/mainLogo.png" alt="">
             </div>
        </div>

        <!-- movementBox -->
        <div class="movement-button-container">
            <div>
                <!-- ← -->
                 <img id="movement-btn-left" class="movement-button" src="/arrows/thick_arrows_left.png" alt="">
            </div>
            <div>
                <!-- → -->
                <img id="movement-btn-right" class="movement-button" src="/arrows/thick_arrows_right.png" alt="">
            </div>
            <div>
                <!-- ↑ -->
                <img id="movement-btn-up" class="movement-button" src="/arrows/thick_arrows_up.png" alt="">
            </div>
        </div>

        <!-- current directory -->
        <div class="current-directory-container">
            <input type="text" class="current-directory-inputbox" value={curFolderName} readonly>
            <div class="current-dir-inputBox-height">

            </div>
        </div>

        <!-- search box -->
        <!-- 🔍 -->
        <div class="search-container">
            <input id="searchInput" class="searchbox-input" type="text" placeholder="{curFolderName}">
            {#if isSearching}
            <!-- <button id="searchButton" class="searchbox-button-wrapper" disabled>
                <img class="searchBox-button-img" src="/icons/magnifying_glass.png" alt="">
            </button> -->
            <button class="searchbox-button-wrapper">❌</button>
            {:else}
            <button id="searchButton" class="searchbox-button-wrapper" on:click={searchFilesInDirectory}>
                <img class="searchBox-button-img" src="/icons/magnifying_glass.png" alt="">
            </button>
            {/if}
        </div>
    </div>


    <!-- util bar + settings -->
    <div class="util-container">

        <div class="util-btns-container">
            {#each utilButtons as btns}
                <div class="util-button-wrapper">
                    {#if btns === "Home"}
                        <img id={btns} class="util-button" src="/utilbuttons/util_home.png" alt="">
                    {:else if btns === "Cut"}
                        <img id={btns} class="util-button" src="/utilbuttons/util_cut.png" alt="">
                    {:else if btns === "Copy"}
                        <img id={btns} class="util-button" src="/utilbuttons/util_copy.png" alt="">
                    {:else if btns === "Paste"}
                        <img id={btns} class="util-button" src="/utilbuttons/util_paste.png" alt="">
                    {:else if btns === "Delete"}
                        <img id={btns} class="util-button" src="/utilbuttons/util_delete.png" alt="">
                    {/if}
                </div>
            {/each}
        </div>

        <!-- settings -->
        <div class="settings-icon-wrapper" on:click={toggleSettings}>
            <!-- ⚙️ -->
             <img class="gear-image" src="/icons/gear.png" alt="">
        </div>
    </div>
    
    <!-- Just for height set -->
    <div style="margin-bottom: 140px;"></div>

    <div class="content-wrapper {viewMode === 'dual' ? 'dual-view' : ''}">
        <!-- 좌측 패널: 드라이브 및 폴더 탐색기 -->
        <aside class="sidebar" id="sidebar">
            {#each Object.keys($drives) as drive}
                <Folder path={drive} name={drive} items={$drives[drive]} on:folderSelected={handleFolderSelected}/>
            {/each}
            <DiscInfo/>
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

    <!-- Setting Modal -->
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
                    <li
                        class:active={activeTab === "utils"}
                        on:click={() => changeTab("utils")}
                    >
                    {currentTranslations.utils}
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
                    {:else if activeTab === "utils"}
                    <h3>{currentTranslations.utils}</h3>
                    <label for="">{currentTranslations.util_home}</label><input type="checkbox" checked={isChecked("Home")} on:change="{(e) => toggleItem('Home', e.target.checked)}"> 
                    &nbsp;<label for="">{currentTranslations.util_cut}</label><input type="checkbox" checked={isChecked("Cut")} on:change="{(e) => toggleItem('Cut', e.target.checked)}">
                    &nbsp;<label for="">{currentTranslations.util_copy}</label><input type="checkbox" checked={isChecked("Copy")} on:change="{(e) => toggleItem('Copy', e.target.checked)}"> 
                    &nbsp;<label for="">{currentTranslations.util_paste}</label><input type="checkbox" checked={isChecked("Paste")} on:change="{(e) => toggleItem('Paste', e.target.checked)}">
                    <br/>
                    <label for="">{currentTranslations.util_delete}</label><input type="checkbox" checked={isChecked("Delete")} on:change="{(e) => toggleItem('Delete', e.target.checked)}">
                    <br/>
                    <!-- <button on:click={util_apply}>{currentTranslations.util_apply_button}</button> -->
                    {/if}
                </div>
                <button class="close-modal" on:click={toggleSettings}>{currentTranslations.modal_close}</button
                >
            </div>
        </div>
    {/if}
</div>
<!-- <a href="/frontTest/frame">Go to previous page</a> -->
