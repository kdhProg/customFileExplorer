<script lang="ts">
    // import "/src/lib/style/mainPage.css";

    import { invoke } from "@tauri-apps/api/tauri";


    // import - css
    import "/src/lib/style/global_features.css"

    // import - components
    import Navi from "$lib/components/navi.svelte";
    import CurrentPath from "$lib/components/currentPath.svelte";


    let c_default_path = 'c:\\';
    let d_default_path = 'd:\\';

    let c_drive_default_files: string[] = [];
    let d_drive_default_files: string[] = [];

    async function c_d_file_lists() {
        c_drive_default_files = await invoke("list_files_in_directory", { path: c_default_path });
        d_drive_default_files = await invoke("list_files_in_directory", { path: d_default_path });
    }


    interface FileMetadata {
        file_name: string;
        file_size: number;
        last_modified: number;
        file_type: string;
    }

    let metadata: FileMetadata;
    let error: string | null = null;

    // 메타데이터 호출
    async function getMetaData(filePath: string): Promise<FileMetadata> {
        try {
            metadata = await invoke<FileMetadata>("get_file_metadata", {
                filePath,
            });
        } catch (err) {
            error = (err as Error).message;
        } finally{
            return metadata;
        }
    }
   


    // 페이지 로드 후 비동기적으로 파일 목록을 가져와서 업데이트
    async function initializeDrives() {
        await c_d_file_lists();

        // C 드라이브의 폴더4에 파일 목록 업데이트
        drives["C 드라이브"].C폴더4 = c_drive_default_files;

        console.log(drives); // 업데이트된 drives 객체 출력
    }

    // 페이지 로드 시 비동기 함수 실행
    initializeDrives();


    /////////////////////////////////////////////////////////////
    let drives = {
        "C 드라이브": {
            C폴더1: ["텍스트파일", "이미지파일"],
            C폴더2: ["동영상파일", "텍스트파일"],
            C폴더3: ["실행파일", "이미지파일"],
            C폴더4: c_drive_default_files
        },
        "D 드라이브": {
            D폴더1: ["텍스트파일", "텍스트파일"],
            D폴더2: ["이미지파일", "동영상파일"],
        },
    };


        // "C 드라이브": {
        //     C폴더1: ["텍스트파일", "이미지파일"],
        //     C폴더2: ["동영상파일", "텍스트파일"],
        //     C폴더3: ["실행파일", "이미지파일"],
        //     C폴더4: c_drive_default_files
        // },
        // "D 드라이브": {
        //     D폴더1: ["텍스트파일", "텍스트파일"],
        //     D폴더2: ["이미지파일", "동영상파일"],
        // },


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

    // 파일 크기 조절
    function updateFileSize(event) {
        fileSize = event.target.value;
    }

    // 드라이브 선택 시 하위 폴더 표시/숨김
    function toggleDrive(drive, panel) {
        if (openedDrives[drive]) {
            delete openedDrives[drive];
            if (panel === "left") {
                selectedDriveLeft = null;
                selectedFolderLeft = null;
                filesInFolderLeft = [];
            } else {
                selectedDriveRight = null;
                selectedFolderRight = null;
                filesInFolderRight = [];
            }
        } else {
            openedDrives = {}; // 다른 드라이브 닫기
            openedDrives[drive] = true;
            if (panel === "left") {
                selectedDriveLeft = drive;
                selectedFolderLeft = null;
                filesInFolderLeft = [];
            } else {
                selectedDriveRight = drive;
                selectedFolderRight = null;
                filesInFolderRight = [];
            }
        }
    }

    // 폴더 선택 시 파일 표시
    function selectFolder(folder, panel) {
        if (panel === "left") {
            selectedFolderLeft = folder;
            filesInFolderLeft = drives[selectedDriveLeft]?.[folder] || [];
        } else {
            selectedFolderRight = folder;
            filesInFolderRight = drives[selectedDriveRight]?.[folder] || [];
        }
    }

    // 파일 아이콘 설정
    function getFileIcon(file) {
        if (file.includes("텍스트파일")) return "📄";
        if (file.includes("이미지파일")) return "🖼️";
        if (file.includes("동영상파일")) return "🎥";
        if (file.includes("실행파일")) return "💻";
        return "📁";
    }


    // 테마
    // 기본 CSS 파일 로드
    let currentTheme = '/src/lib/style/mainPage.css';

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


    // -------------- tauri API --------------------

    


</script>

<!-- 메인 화면 -->
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
     <div>
       <CurrentPath/>
     </div>

    <div class="content-wrapper {viewMode === 'dual' ? 'dual-view' : ''}">
        <!-- 좌측 패널: 드라이브 및 폴더 탐색기 -->
        <aside class="sidebar">
            <ul class="folder-list">
                {#each Object.keys(drives) as drive}
                    <li>
                        <button on:click={() => toggleDrive(drive, "left")}>
                            {drive}
                        </button>
                        {#if selectedDriveLeft === drive && openedDrives[drive]}
                            <ul class="folder-sublist">
                                {#each Object.keys(drives[drive]) as folder}
                                    <li
                                        on:click={() =>
                                            selectFolder(folder, "left")}
                                    >
                                        {folder}
                                    </li>
                                {/each}
                            </ul>
                        {/if}
                    </li>
                {/each}
            </ul>
        </aside>

        <!-- 좌측 파일 탐색기 -->
        <div class="file-viewer">
            {#if filesInFolderLeft.length > 0}
                {#each filesInFolderLeft as file}
                    <div
                        class="file-item"
                        style="width: {fileSize}px; height: {fileSize}px;"
                    >
                        <span class="file-icon">{getFileIcon(file)}</span>
                        <span class="file-name">{file}</span>
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
                <ul class="folder-list">
                    {#each Object.keys(drives) as drive}
                        <li>
                            <span on:click={() => toggleDrive(drive, "right")}>
                                {drive}
                            </span>
                            {#if selectedDriveRight === drive && openedDrives[drive]}
                                <ul class="folder-sublist">
                                    {#each Object.keys(drives[drive]) as folder}
                                        <li
                                            on:click={() =>
                                                selectFolder(folder, "right")}
                                        >
                                            {folder}
                                        </li>
                                    {/each}
                                </ul>
                            {/if}
                        </li>
                    {/each}
                </ul>
            </aside>

            <!-- 우측 파일 탐색기 -->
            <div class="file-viewer">
                {#if filesInFolderRight.length > 0}
                    {#each filesInFolderRight as file}
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
