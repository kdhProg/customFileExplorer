<script>
    import "/src/routes/frontTest/style/mainPage.css";

    let drives = {
        "C 드라이브": {
            C폴더1: ["텍스트파일", "이미지파일"],
            C폴더2: ["동영상파일", "텍스트파일"],
            C폴더3: ["실행파일", "이미지파일"],
        },
        "D 드라이브": {
            D폴더1: ["텍스트파일", "텍스트파일"],
            D폴더2: ["이미지파일", "동영상파일"],
        },
    };

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
</script>

<!-- 메인 화면 -->
<div class="main-container">
    <!-- 상단 바 -->
    <header class="top-bar">
        <div class="logo">pathFinder</div>
        <div class="settings-icon" on:click={toggleSettings}>⚙️</div>
    </header>

    <div class="content-wrapper {viewMode === 'dual' ? 'dual-view' : ''}">
        <!-- 좌측 패널: 드라이브 및 폴더 탐색기 -->
        <aside class="sidebar">
            <ul class="folder-list">
                {#each Object.keys(drives) as drive}
                    <li>
                        <span on:click={() => toggleDrive(drive, "left")}>
                            {drive}
                        </span>
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
