<script lang="ts">
    import { listen } from '@tauri-apps/api/event';
    import { onMount, afterUpdate } from 'svelte';
    import { writable } from 'svelte/store';
    import { invoke } from "@tauri-apps/api/tauri";

    import { isDirectory, listFilesInDirectory, openFileWithDefaultProgram } from "$lib/api";
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
    let activeTab = "resize";
    let viewMode = "single"; // 기본 모드는 single (하나의 파일 탐색기)
    let fileSize = 80; // 기본 파일 아이템 크기
    let selectedDriveLeft = null; // 왼쪽 패널에서 선택된 드라이브
    let selectedDriveRight = null; // 오른쪽 패널에서 선택된 드라이브
    let selectedFolderLeft = null; // 왼쪽 패널에서 선택된 폴더


    // Current Path
    let curFolderName = '';

    // File List on Current Path
    let filesInCurrentFolder: string[] = [];

    // File Click Event - Directory List
    async function handleFolderSelected(event) {
        curFolderName = event.detail;

        addPathHistory(curFolderName); // add to history

        filesInCurrentFolder = await listFilesInDirectory(curFolderName);
        // console.log(typeof filesInCurrentFolder[0])
    }

    // ------------------------------ Back / Forward / Pre movement Button ------------------------------
    let pathHistory: string[] = []; // Maximum size : 10
    let currentIndex = -1;

    // Check Existence of input Directory or File path
    async function isPathValid(path: string): Promise<boolean> {
        return await invoke('path_exists', { dirPath: path });
    }

    // Add New Member -> used at EachFolderClick / handleFolderSelected
    function addPathHistory(path: string) {
    // 배열이 비어 있거나 이전 경로와 다른 경우에만 추가
        if (pathHistory.length === 0 || pathHistory[pathHistory.length - 1] !== path) {
            if (pathHistory.length >= 10) {
                pathHistory.shift();
            }

            // Add Path & MoveIndex
            // Set Index to last of Array
            
            pathHistory.push(path);
            currentIndex = pathHistory.length - 1;
            
            console.log("pathHistory : "+pathHistory);
            console.log("currentIndex: "+currentIndex);
            
        }
    }


    // 폴더 이동 함수
    async function moveToFolder(newFolder: string) {

        const isValid = await isPathValid(newFolder); // 경로 유효성 검사
        if (!isValid) {
        console.error(`Invalid path: ${newFolder}`);
        return; // 경로가 유효하지 않으면 이동 중단
        }

        pathHistory = pathHistory.slice(0, currentIndex + 1); // 현재 인덱스 이후 경로 제거
        pathHistory.push(newFolder); // 새 경로 추가

        // 최대 크기가 10을 초과하면 첫 번째 경로 삭제
        if (pathHistory.length > 10) {
            pathHistory.shift();
        }

        currentIndex = pathHistory.length - 1; // 현재 인덱스 업데이트
        curFolderName = newFolder; // 현재 폴더 경로 업데이트
        filesInCurrentFolder = await listFilesInDirectory(curFolderName);
    }

    // 뒤로 가기 함수
    async function goBack() {
        if (currentIndex > 0) {
            currentIndex--;
            curFolderName = pathHistory[currentIndex];
            filesInCurrentFolder = await listFilesInDirectory(curFolderName);
        }
        console.log(pathHistory);
    }

    // 앞으로 가기 함수
    async function goForward() {
        if (currentIndex < pathHistory.length - 1) {
            currentIndex++;
            curFolderName = pathHistory[currentIndex];
            filesInCurrentFolder = await listFilesInDirectory(curFolderName);
        }
        console.log(pathHistory);
    }

    // 상위 폴더 계산 함수
    function getParentFolder(path: string): string | null {
        const parts = path.split('\\'); // '\'로 경로 분리

        if (parts.length === 2 && /^[A-Z]:$/.test(parts[0])) {
            // 현재 또는 부모 폴더가 드라이브 루트 ('D:')
            return `${parts[0]}\\`; // 'D:' → 'D:\' 형태로 반환
        }

        const parent = parts.slice(0, -1).join('\\'); // 상위 경로 계산
        return parent || null; // 유효하지 않으면 null 반환
    }

    // 상위 폴더 이동 함수
    async function goUp() {
        const isRoot = /^[A-Z]:\\?$/.test(curFolderName); // 현재 위치가 루트인지 확인
        if (isRoot) {
            console.log('Already at the root folder. Cannot move up.');
            return; // 루트에서 더 상위로 이동할 수 없음
        }

        const parentFolder = getParentFolder(curFolderName); // 상위 폴더 계산
        console.log('Parent folder:', parentFolder);

        if (parentFolder) {
            await moveToFolder(parentFolder); // 상위 폴더로 이동
        } else {
            console.log('Already at the root folder.'); // 더 이상 상위로 이동 불가
        }
    }





    // ------------------------------ File Icon ------------------------------

    let fileIcons: { [key: string]: string } = {}; // 파일 경로별 아이콘 저장 객체

    // 파일 경로에 따른 아이콘 비동기 로드
    async function loadIcons(files) {
        const icons = {};
        for (const file of files) {
            icons[file] = await getFileIcon(file); // 비동기 아이콘 로드
        }
        fileIcons = icons; // 아이콘 저장
    }

    // load icons when filesInCurrentFolder change
    $: if (filesInCurrentFolder.length > 0) {
        loadIcons(filesInCurrentFolder);
    }

    // Icon depends
    const themeLogos = {
        default: "/icons/dir_logo_default.png",
        retro: "/icons/dir_logo_retro.png",
        sf: "/icons/dir_logo_sf.png",
        linux: "/icons/dir_logo_linux.png"
    };

    // Default file icon
    let currentLogo = themeLogos.default; 

    let file_exe = "/icons/file_exe.png";
    let file_jpg = "/icons/file_jpg.png";
    let file_mp4 = "/icons/file_mp4.png";
    let file_txt = "/icons/file_txt.png";
    let file_default = "/icons/file_default.png";

    // Set File icon
    async function getFileIcon(file_path: string):Promise<string> {
        let isDir:boolean = await invoke('is_directory', { path: file_path });
        if (isDir){
            return currentLogo;
        }else{
            if (file_path.includes(".txt")) return file_txt;
            if (file_path.includes(".jpg") || file_path.includes(".png")) return file_jpg;
            if (file_path.includes(".mp4")) return file_mp4;
            if (file_path.includes(".exe")) return file_exe;
            return file_default;
        }
    }

    // Extract fileName
    function getFileName(filePath:string) {
        const parts = filePath.split(/[/\\]/);
        return parts[parts.length - 1];
    }


    // Theme
    // default theme
    let currentTheme = '/src/lib/style/themes/default_theme.css';

    // Change CSS
    function applyTheme(themePath) {
        const existingLink = document.querySelector('#dynamic-theme');
        
        // Change line path if [link] already exists
        if (existingLink) {
            existingLink.href = themePath;
        } else {
            // create new [link] tag
            const linkElement = document.createElement('link');
            linkElement.rel = 'stylesheet';
            linkElement.id = 'dynamic-theme';
            linkElement.href = themePath;
            document.head.appendChild(linkElement);
        }

            // change logo image by themes
        if (themePath.includes('default')) {
            currentLogo = themeLogos.default;
        } else if (themePath.includes('retro')) {
            currentLogo = themeLogos.retro;
        } else if (themePath.includes('sf')) {
            currentLogo = themeLogos.sf;
        } else if(themePath.includes('linux')) {
            currentLogo = themeLogos.linux;
        }
        // update current page theme
        currentTheme = themePath;

        filesInCurrentFolder = [...filesInCurrentFolder];
    }

    // Load default theme when page load
    applyTheme(currentTheme);

// ------------------------------  Search ------------------------------

    // Check If searching is on
    let isSearching:boolean = false;
    let searchProcessId = null;
    let unlisten;
    let receivedFiles = new Set();

    async function searchFilesInDirectory() {
        // console.log('clicked!')
        try {
            isSearching = true;
            const keyword = document.getElementById('searchInput');

            if(curFolderName === '' || curFolderName.length === 0){
                // 현재 파일 경로가 없다면(=초기화면) 검색불가
                

            }else{
                // 현재 디렉토리에서 검색
                if (keyword instanceof HTMLInputElement) {
                    // Get User-input Search Keyword
                    const inputValue = keyword.value;

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

                    // Search Result Array
                    let searchRst = [];
                    filesInCurrentFolder = []; // filesInCurrentFolder를 비움

                    // 실시간 탐색 결과 리스너 등록
                    unlisten = await listen('search-result', (event) => {
                        const file = event.payload;

                        // file_path가 중복되지 않도록 확인 후 추가
                        if (!searchRst.includes(file.file_path)) {
                            searchRst.push(file.file_path);  // file_path만 추가
                            filesInCurrentFolder = [...searchRst]; // filesInCurrentFolder를 업데이트
                            console.log("Real-time search result:", file.file_path);  // file_path 출력
                        }
                    });

                    // 검색 수행 시간 리스너
                    await listen('search-time', (event) => {
                        const searchTime = event.payload;  // 전달된 검색 수행 시간
                        console.log(`Search completed in ${searchTime} seconds`);
                        // // 수행 시간을 화면에 표시하거나 다른 로직에 활용할 수 있음
                        // document.getElementById('search-time-display').textContent = `Search time: ${searchTime.toFixed(2)} seconds`;
                    });


                    await invoke("search_files", { 
                        keyword:inputValue,
                        directory:curFolderName,
                        options:searchValObj 
                    });
                    

                    // filesInCurrentFolder = searchRst;
                } else {
                    console.error("Input element not found or is not of type HTMLInputElement");
                }

            }
            
            isSearching = false;

            
            
            
        } catch (error) {
            console.error("err:", error);
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

    
    // ------------------------------ Click Each Folder / Files ------------------------------
    // Folder - update current folder list
    // File - execute with default enrolled programs
    async function eachFolderClick(file:string){
        // curFolderName = file;
        let isDir = await isDirectory(file);
        if(isDir){
            // case : this is directory
            curFolderName = file;

            filesInCurrentFolder = await listFilesInDirectory(curFolderName);

            addPathHistory(curFolderName); // add to history
        }else{
            // case : this is file -> not update currentDirectory
            openFileWithDefaultProgram(file);
            
        }
    }

    // ------------------------------  File Drag  ------------------------------
    // 상태 관리
  let isDragging = writable(false);  // 드래그 상태
  let startX = 0, startY = 0;        // 드래그 시작 좌표
  let endX = 0, endY = 0;            // 드래그 끝 좌표
  let selectedFiles = writable<string[]>([]);  // 선택된 파일 경로 저장

  const dragThreshold = 5;  // 최소 이동 거리 (5px 이상만 드래그로 처리)
  let rectStyle = writable('');  // 직사각형 CSS 스타일

  // 마우스 다운: 클릭/드래그 시작 지점 초기화
  function handleMouseDown(event: MouseEvent) {
    event.preventDefault();  // 기본 브라우저 동작 방지
    clearSelection();

    // 클릭 및 드래그 초기화
    startX = event.clientX;
    startY = event.clientY;
    endX = startX;
    endY = startY;
    
    rectStyle.set('');
    isDragging.set(true);  // 드래그 상태 시작
  }

  // 마우스 이동: 일정 거리 이상 이동하면 드래그로 간주
  function handleMouseMove(event: MouseEvent) {
    if (!$isDragging) return;  // 드래그 상태가 아니면 종료

    const dx = Math.abs(event.clientX - startX);
    const dy = Math.abs(event.clientY - startY);

    // 드래그로 간주되는 최소 거리 이상 이동했을 때만 처리
    if (dx > dragThreshold || dy > dragThreshold) {
      endX = event.clientX;
      endY = event.clientY;
      updateRectStyle();  // 직사각형 스타일 업데이트
    }
  }

  // 마우스 업: 클릭 또는 드래그 종료 처리
  function handleMouseUp(event: MouseEvent) {
    isDragging.set(false);  // 드래그 상태 해제
    detectFilesInside();  // 직사각형 내 파일 탐지
  }

  // 직사각형 스타일 업데이트
  function updateRectStyle() {
    const x1 = Math.min(startX, endX);
    const y1 = Math.min(startY, endY);
    const width = Math.abs(endX - startX);
    const height = Math.abs(endY - startY);

    rectStyle.set(`left: ${x1}px; top: ${y1}px; width: ${width}px; height: ${height}px;`);
  }

  // 기존 선택된 파일 해제
  function clearSelection() {
    const selectedElements = document.querySelectorAll('.file-item.selected');
    selectedElements.forEach((el) => el.classList.remove('selected'));
  }

  // 직사각형 내 포함된 파일 탐지
  function detectFilesInside() {
    const fileElements = document.querySelectorAll('.file-item');
    const rect = new DOMRect(
      Math.min(startX, endX) + window.scrollX,
      Math.min(startY, endY) + window.scrollY,
      Math.abs(endX - startX),
      Math.abs(endY - startY)
    );

    const selected = Array.from(fileElements).filter((el) => {
      const elRect = el.getBoundingClientRect();
      return (
        rect.left <= elRect.right &&
        rect.right >= elRect.left &&
        rect.top <= elRect.bottom &&
        rect.bottom >= elRect.top
      );
    });

    selected.forEach((el) => el.classList.add('selected'));
    const selectedPaths = selected.map((el) => el.getAttribute('data-file-path') || '');
    console.log(`Selected items count: ${selected.length}`);
    selectedFiles.set(selectedPaths);
  }


    // ------------------------------ util bars ------------------------------
    async function load_util_buttons(){
        let jsonData = {};

        try {
            const response = await invoke('read_json_file');
            jsonData = JSON.parse(response);
        } catch (error) {
            console.error('Error while loading util json:', error);
        }

    return jsonData;
    }   

    // Util Button Lists
    let utilButtons = []

    // util_buttons toggle checkbox
    function toggleItem(value, checked) {
        if (checked) {
        // check - add to array
        utilButtons = [...utilButtons, value];
        } else {
        // unchekced - remove from array
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


  // ------------------------------ util buttons detail ------------------------------
  
  //   -------------------- Home --------------------
  

  //   -------------------- Cut / Copy / Paste --------------------
  let copyClipboard: string[] = [];
  let cutClipboard: string[] = [];
  let ccp_message = writable<string>(''); // 에러 메시지 상태

    function copyFiles() {
        const files = $selectedFiles;

        // cutClipboard에서 중복된 파일 제거
        cutClipboard = cutClipboard.filter((file) => !files.includes(file));

        // copyClipboard에 selectedFiles의 값 저장
        copyClipboard = [...files];

        console.log('Copy clipboard:', copyClipboard);
        console.log('Cut clipboard after removal:', cutClipboard);
    }

    function cutFiles() {
        const files = $selectedFiles;

        // copyClipboard에서 중복된 파일 제거
        copyClipboard = copyClipboard.filter((file) => !files.includes(file));

        // cutClipboard에 selectedFiles의 값 저장
        cutClipboard = [...files];

        console.log('Cut clipboard:', cutClipboard);
        console.log('copy clipboard after removal:', copyClipboard);

    }
    async function pasteFiles(clipboard: string[], targetPath: string, isCut: boolean) {
        try {
            const result = await invoke('paste_files', { files: clipboard, targetPath, cut: isCut });
            // console.log("targetPath : "+targetPath);
        if (!result.success) {
            ccp_message.set(result.message);
        } else {
            ccp_message.set('Files pasted successfully.');
            if (isCut) cutClipboard = [];
                else copyClipboard = [];
            }
        } catch (err) {
            ccp_message.set(`Error: ${err}`);
        }
        filesInCurrentFolder = await listFilesInDirectory(curFolderName); // Rerending
    }

  //   -------------------- Delete --------------------

    const del_message = writable<string>(''); // 에러 메시지 상태

    // 휴지통으로 파일 이동 함수
    async function moveToTrash() {
    const files = $selectedFiles; // 최신 선택된 파일 목록 가져오기

    try {
        const result = await invoke('move_files_to_trash', { paths: files });
        filesInCurrentFolder = await listFilesInDirectory(curFolderName); // Rerending
        if (!result.success) {
            del_message.set(result.message); // 에러 메시지 설정
        } else {
            del_message.set('All files moved to trash successfully.');
        selectedFiles.set([]); // 선택된 파일 목록 초기화
        }
    } catch (err) {
            del_message.set(`Unexpected error: ${err}`);
    }
    }



  
    

  // ------------------------------ divide bar ------------------------------
let sidebarWidth = 250;

function updateSidebarWidth(width) {
    sidebarWidth = width;
    document.getElementById('sidebar').style.width = `${sidebarWidth}px`;
    document.getElementById('sidebar').style.minWidth = `${sidebarWidth}px`;
    document.getElementById('sidebar').style.maxWidth = `${sidebarWidth}px`;
}


// ------------------------------ onMount -> load when page starts ------------------------------
onMount(() => {

    load_util_buttons().then(data => {
      utilButtons = data.buttons;
    });


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
        const maxWidth = 500;

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
    // re-setting sidebar width when folder open/close
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


// ------------ modal ------------------------
// Settings Modal On / Off
function toggleSettings() {
        showSettings = !showSettings;
    }

// Changing Tab
function changeTab(tab) {
    activeTab = tab;
}

// File size
function updateFileSize(event: Event){
    const target = event.target as HTMLInputElement;
    fileSize = parseInt(target.value);
}

// ------------------------------ advanced modal sch options ------------------------------

// custom thread pools
let isThreadPoolsChk : boolean = false;

function isThreadPoolsChkToggle(){
    isThreadPoolsChk = !isThreadPoolsChk;

    // Change searchObj's boolean value
    searchValObj.customThreadPoolUse = !searchValObj.customThreadPoolUse;

    if(!isThreadPoolsChk){
        searchValObj.threadPoolNum = "0"; // reset to "default" val
    }
}


// custom property
let isFilePropertyChk : boolean = false;

function isPropertyChkToggle(){
    isFilePropertyChk = !isFilePropertyChk;

    // Change searchObj's boolean value
    searchValObj.customPropertyUse = !searchValObj.customPropertyUse;
    
    // reset All detail booleans & property custom values;
    if(!isFilePropertyChk){

        isFileSizeChk = false;
        isFileCrtDateChk = false;
        isFileModifiedDateChk = false;
        isFileTypeChk = false;
        isFileOwnerChk = false;

        searchValObj.customFileSizeUse = false;
        searchValObj.sizeMax = 0; 
        searchValObj.sizeMin = 0;
        searchValObj.fileMaxRawVal = 0;
        searchValObj.fileMinRawVal = 0;
        searchValObj.fileMaxUnit = 'B';
        searchValObj.fileMinUnit = 'B';
        fileMaxRawVal = 0;
        fileMinRawVal = 0;
        fileMaxUnit = 'B';
        fileMinUnit = 'B';

        searchValObj.customFileCrtDateUse = false;
        searchValObj.crtStart = ''; 
        searchValObj.crtEnd = '';

        searchValObj.customFileModiDateUse = false;
        searchValObj.modiStart = ''; 
        searchValObj.modiEnd = '';

        searchValObj.customFileTypeUse = false;
        searchValObj.fileTypeList = ''; 

        searchValObj.customFileOwnerUse = false;
        searchValObj.ownerName = '';

    }
}

let isFileSizeChk : boolean = false;
let isFileCrtDateChk : boolean = false;
let isFileModifiedDateChk : boolean = false;
let isFileTypeChk : boolean = false;
let isFileOwnerChk : boolean = false;


function isFileSizeChkToggle(){
    isFileSizeChk = !isFileSizeChk;

    // Change searchObj's boolean value
    searchValObj.customFileSizeUse = !searchValObj.customFileSizeUse;
    
    // reset max & min size;
    if(!isFileSizeChk){
        searchValObj.sizeMax = 0; 
        searchValObj.sizeMin = 0;

        fileMaxRawVal = 0;
        fileMinRawVal = 0;
        fileMaxUnit = 'B';
        fileMinUnit = 'B';
    }
}


function isFileCrtDateChkToggle(){
    isFileCrtDateChk = !isFileCrtDateChk;

    // Change searchObj's boolean value
    searchValObj.customFileCrtDateUse = !searchValObj.customFileCrtDateUse;

    // reset date value;
    if(!isFileCrtDateChk){
        searchValObj.crtStart = ''; 
        searchValObj.crtEnd = '';
    }

}

function isFileModifiedDateChkToggle(){
    isFileModifiedDateChk = !isFileModifiedDateChk;

    // Change searchObj's boolean value
    searchValObj.customFileModiDateUse = !searchValObj.customFileModiDateUse;

    // reset date value;
    if(!isFileCrtDateChk){
        searchValObj.modiStart = ''; 
        searchValObj.modiEnd = '';
    }
}

function isFileTypeChkToggle(){
    isFileTypeChk = !isFileTypeChk;

    // Change searchObj's boolean value
    searchValObj.customFileTypeUse = !searchValObj.customFileTypeUse;

    // reset fileType value;
    if(!isFileTypeChk){
        searchValObj.fileTypeList = ''; 
    }
}

function isFileOwnerChkToggle(){
    isFileOwnerChk = !isFileOwnerChk;

    // Change searchObj's boolean value
    searchValObj.customFileOwnerUse = !searchValObj.customFileOwnerUse;

    // reset fileOwner value;
    if(!isFileOwnerChk){
        searchValObj.ownerName = ''; 
    }
}


// file property - max & min calculation
let fileMaxRawVal = 0;
let fileMinRawVal = 0;
let fileMaxUnit = 'B';
let fileMinUnit = 'B';

function fileSizeCalc(){
    const unitValArr: { [key in 'B' | 'KB' | 'MB' | 'GB']: number } = {
        'B': 1,
        'KB': 1024,
        'MB': 1024 * 1024,
        'GB': 1024 * 1024 * 1024
    };
     
    searchValObj.sizeMax = fileMaxRawVal * unitValArr[fileMaxUnit as 'B' | 'KB' | 'MB' | 'GB'];
    searchValObj.sizeMin  = fileMinRawVal * unitValArr[fileMinUnit as 'B' | 'KB' | 'MB' | 'GB'];

    // save settings
    searchValObj.fileMaxRawVal = fileMaxRawVal;
    searchValObj.fileMinRawVal = fileMinRawVal;
    searchValObj.fileMaxUnit = fileMaxUnit;
    searchValObj.fileMinUnit = fileMinUnit;
}



// File Search - Advanced settings values Object
// All of selectable options are false when init
let searchValObj = {

    /*
    Policy : "0" means default
    (default => Use Automatic Settings => backend will calculate appropriate number of thread nums depends CPU core)
    */
    customThreadPoolUse:false,
        threadPoolNum : "0",

    /* 
    Policy : This Type should be "String" not "Number"
    0 : Folder + File   
    1 : File Only
    2 : Folder Only
    */
    searchScope : "0",

    /*
    File Content
    */
    customFileContUse:false,

    /*
    File Detail Property
    */
    customPropertyUse:false,
        customFileSizeUse:false,
            sizeMax:0,
            sizeMin:0,
        customFileCrtDateUse:false,
            crtStart:'',
            crtEnd:'',
        customFileModiDateUse:false,
            modiStart:'',
            modiEnd:'',
        customFileOwnerUse:false,
            ownerName:'',
        customFileTypeUse:false,
            fileTypeList:'',

    customSymbolicChk : false,

    /* 
    Policy : This Type should be "String" not "Number"
    0 : Default(Not Use advanced Methods)
    1 : Regex
    2 : Fuzzy - Damerau Levenshtein
    3 : Fuzzy - Jaccard Similarity
    */
    customSchMethod:"0",


    /*
    Search Log
    */
    customLogUse:false,


    /*
     Just used to load_value
    */
    fileMaxRawVal : 0,
    fileMinRawVal : 0,
    fileMaxUnit : "B",
    fileMinUnit : "B",
}



// ------------------------------ Sch Advanced value Slot Modal -------------------------------

let showAdvSlotModal:boolean = false;

function advModalToggle(){
    showAdvSlotModal = !showAdvSlotModal;
}

let slots = [
    { number: 1, name: "", hasValue: false },
    { number: 2, name: "", hasValue: false },
    { number: 3, name: "", hasValue: false },
    { number: 4, name: "", hasValue: false },
    { number: 5, name: "", hasValue: false }
  ];

  // Slot Init
  onMount(async () => {
    for (let i = 0; i < slots.length; i++) {
      const result = await invoke("load_settings", { slotNumber: slots[i].number });
      if (result.name.length !== 0) {
        slots[i].name = result.name;
        slots[i].hasValue = true;
      }
    }
  });

  // Save Custom Values
  async function saveSlot(slotIndex:number) {
    const slot = slots[slotIndex];
    await invoke("save_settings", {
        slotNumber: slot.number,
        name: slot.name,
        settings: searchValObj
    });
    slots[slotIndex].hasValue = true; // activate load & reset btn after save
  }

// Load Slot 
  async function loadSlot(slotIndex:number) {
    const slot = slots[slotIndex];
    const result = await invoke("load_settings", { slotNumber: slot.number });
    if (result) {
      slots[slotIndex].name = result.name;
      const loaded_obj = result.val;
      // -- update searchValObj
      searchValObj = loaded_obj;

      // -- update bind - vars
    isThreadPoolsChk = loaded_obj.customThreadPoolUse;
    isFilePropertyChk = loaded_obj.customPropertyUse;
    isFileSizeChk = loaded_obj.customFileSizeUse;
    isFileCrtDateChk = loaded_obj.customFileCrtDateUse;
    isFileModifiedDateChk = loaded_obj.customFileModiDateUse;
    isFileTypeChk = loaded_obj.customFileTypeUse;
    isFileOwnerChk = loaded_obj.customFileOwnerUse;

    fileMaxRawVal = loaded_obj.fileMaxRawVal;
    fileMinRawVal = loaded_obj.fileMinRawVal;
    fileMaxUnit = loaded_obj.fileMaxUnit;
    fileMinUnit = loaded_obj.fileMinUnit;
    }
  }

  // Slot reset(= delete custom values and reset)
  async function deleteSlot(slotIndex:number) {
    const slot = slots[slotIndex];
    await invoke("delete_settings", { slotNumber: slot.number });
    slots[slotIndex].name = "";
    slots[slotIndex].hasValue = false;
  }

</script>

<!-- Main Screen -->
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
                <button class="movement-button-wrapper-btn" on:click={() => goBack()} disabled={currentIndex <= 0}>
                    <img id="movement-btn-left" class="movement-button" src="/arrows/thick_arrows_left.png" alt="">
                </button>
            </div>
            <div>
                <!-- → -->
                <button class="movement-button-wrapper-btn" on:click={() => goForward()} disabled={currentIndex >= pathHistory.length - 1}>
                    <img id="movement-btn-right" class="movement-button" src="/arrows/thick_arrows_right.png" alt="">
                </button>
            </div>
            <div>
                <!-- ↑ -->
                <button class="movement-button-wrapper-btn" on:click={() => goUp()}>
                    <img id="movement-btn-up" class="movement-button" src="/arrows/thick_arrows_up.png" alt="">
                </button>
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
            <button class="searchbox-button-wrapper" on:click={cancelSearch}>❌</button>
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
                    <div class="util-each-button-wrapper">
                        <button class="util-button">
                            <img class="util-button-img" src="/utilbuttons/util_home.png" alt="">
                        </button>
                    </div>
                    {:else if btns === "Cut"}
                    <div class="util-buttons-wrapper">
                        <button class="util-button" on:click={cutFiles}>
                            <img class="util-button-img" src="/utilbuttons/util_cut.png" alt="">
                        </button>
                        <button class="util-button"
                            on:click={() => pasteFiles(cutClipboard, curFolderName, true)} 
                            disabled={cutClipboard.length === 0}
                        >
                            <img class="util-button-img" src="/utilbuttons/util_cut_paste.png" alt="">
                        </button>
                    </div>
                    {:else if btns === "Copy"}
                    <div class="util-buttons-wrapper">
                        <button class="util-button"  on:click={copyFiles}>
                            <img class="util-button-img" src="/utilbuttons/util_copy.png" alt="">
                        </button>
                        <button class="util-button" 
                            on:click={() => pasteFiles(copyClipboard, curFolderName, false)} 
                            disabled={copyClipboard.length === 0}
                        >
                            <img class="util-button-img" src="/utilbuttons/util_copy_paste.png" alt="">
                        </button>
                    </div>
                    {:else if btns === "Delete"}
                    <div class="util-buttons-wrapper">
                        <button class="util-button" on:click={moveToTrash} disabled={$selectedFiles.length === 0}>
                            <img class="util-button-img" src="/utilbuttons/util_delete.png" alt="">
                        </button>
                    </div>
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
        <!-- Directory List -->
        <aside class="sidebar" id="sidebar">
            {#each Object.keys($drives) as drive}
                <Folder path={drive} name={drive} items={$drives[drive]} on:folderSelected={handleFolderSelected}/>
            {/each}
            <hr>
            <DiscInfo/>
        </aside>


        <!-- division bar between sidebar & file-viewer -->
        <div class="resizer" id="resizer"></div>

        <!-- file viewer -->
        <div 
            class="file-viewer" 
            id="fileViewer"
            on:mousedown={handleMouseDown} 
            on:mousemove={handleMouseMove} 
            on:mouseup={handleMouseUp}
        >
            {#if $isDragging}
                <div class="selection-rect" style={$rectStyle}></div>
            {/if}
            {#if filesInCurrentFolder.length > 0}
                {#each filesInCurrentFolder as file}
                    <div
                        class="file-item"
                        data-file-path={file}
                        style="width: {fileSize}px; height: {fileSize}px;"
                        on:dblclick={() => eachFolderClick(file)}
                    >
                        <img src="{fileIcons[file] || ''}" alt="File Icon" class="file-icon" />
                        <span class="file-name">{getFileName(file)}</span>
                    </div>
                {/each}
            {:else if selectedDriveLeft && selectedFolderLeft}
                <p>{currentTranslations.no_folder}</p>
            {:else}
                <p>{currentTranslations.sel_folder}</p>
            {/if}
        </div>
    </div>

    <!------------------------ Setting Modal ------------------------>
    {#if showSettings}
        <div class="settings-modal">
            <div class="modal-content">
                <h2>{currentTranslations.settings}</h2>
                <ul class="tabs">
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
                    <li
                        class:active={activeTab === "search"}
                        on:click={() => changeTab("search")}
                    >
                    {currentTranslations.search}
                    </li>
                </ul>
                <div class="tab-content">
                    {#if activeTab === "resize"}
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
                        <!-- <label for="">{currentTranslations.util_home}</label><input type="checkbox" checked={isChecked("Home")} on:change="{(e) => toggleItem('Home', e.target.checked)}">  -->
                        &nbsp;<label for="">{currentTranslations.util_cut}</label><input type="checkbox" checked={isChecked("Cut")} on:change="{(e) => toggleItem('Cut', e.target.checked)}">
                        &nbsp;<label for="">{currentTranslations.util_copy}</label><input type="checkbox" checked={isChecked("Copy")} on:change="{(e) => toggleItem('Copy', e.target.checked)}">
                        <label for="">{currentTranslations.util_delete}</label><input type="checkbox" checked={isChecked("Delete")} on:change="{(e) => toggleItem('Delete', e.target.checked)}">
                        <br/>
                    <!-- Modal : Search Tab -->
                    {:else if activeTab === "search"}
                        <h3>{currentTranslations.search}</h3>
                        <div class="modal-sch-wrapper">
                            <div class="modal-sch-basic-wrapper">
                                <h3>{currentTranslations.modal_sch_basic_title}</h3>
                                <p>- {currentTranslations.modal_sch_basic_async}</p>
                                <p>- {currentTranslations.modal_sch_basic_filename}</p>
                                <p>- {currentTranslations.modal_sch_basic_realtime}</p>
                                <p>- {currentTranslations.modal_sch_basic_cache}</p>
                            </div>
                            
                            <div class="modal-sch-advanced-wrapper">
                                <h3>{currentTranslations.modal_sch_advanced_title}</h3>
                                <!-- Async -->
                                <h4>{currentTranslations.modal_thread_pool_title}</h4>
                                <label>
                                    {currentTranslations.modal_thread_pool_use_box}
                                    <input type="checkbox" on:change={isThreadPoolsChkToggle} bind:checked={isThreadPoolsChk}>
                                </label>
                                {#if isThreadPoolsChk}
                                <div class="modal-set-thread-pool-wrapper">
                                    <label for="thread_pool">
                                        {currentTranslations.modal_sch_advanced_thread_pool_size}
                                    </label>
                                    <select id="thread_pool" bind:value={searchValObj.threadPoolNum}>
                                        <option value="0">{currentTranslations.modal_sch_advanced_thread_pool_size_default}</option>
                                        <option value="4">4</option>
                                        <option value="8">8</option>
                                        <option value="16">16</option>
                                        <option value="32">32</option>
                                    </select>
                                    <br>
                                </div>
                                {/if}
                                <hr>
                                <!-- Search Target : both file + folder / only file / only folder -->
                                <h4>{currentTranslations.modal_sch_advanced_sch_target}</h4>
                                <div>
                                    <label>
                                        <input type="radio" name="modal-adv-sch-target" value="0" bind:group={searchValObj.searchScope}>
                                        {currentTranslations.modal_sch_advanced_sch_target_fileNfolder}
                                    </label>
                                    <br>
                                    <label>
                                        <input type="radio" name="modal-adv-sch-target" value="1" bind:group={searchValObj.searchScope}>
                                        {currentTranslations.modal_sch_advanced_sch_target_file}
                                    </label>
                                    <br>
                                    <!-- If this option chekced -> File Search Option - property-size will be disabled -->
                                    <label>
                                        <input type="radio" name="modal-adv-sch-target" value="2" bind:group={searchValObj.searchScope}>
                                        {currentTranslations.modal_sch_advanced_sch_target_folder}
                                    </label>
                                </div>
                                <hr>
                                <!-- File Search Option -->
                                <h4>{currentTranslations.modal_sch_advanced_sch_options_title}</h4>
                                <div class="modal-set-file-sch-option-wrapper">
                                    <!-- Alert : file content check may cause decrease of speed -->
                                    <label>
                                        {currentTranslations.modal_sch_advanced_file_content_sch}
                                        <input type="checkbox" id="content-search-btn" bind:checked={searchValObj.customFileContUse}>
                                    </label>
                                    <br>
                                    <label>{currentTranslations.modal_sch_advanced_file_property}
                                        <input type="checkbox" id="recent-modified-search-btn" on:change={isPropertyChkToggle} bind:checked={isFilePropertyChk}>  
                                    </label>
                                    {#if isFilePropertyChk}
                                        <div class="modal-set-file-sch-property-wrapper">
                                            <!-- File Property size / type / modified date / creation date / owner -->
                                            <label for="">
                                                {currentTranslations.modal_sch_advanced_file_property_size}
                                                <input type="checkbox" on:change={isFileSizeChkToggle} bind:checked={isFileSizeChk}>
                                            </label>
                                            {#if isFileSizeChk}
                                            <div>
                                                <!-- fileSize max / min -->
                                                <label>
                                                    {currentTranslations.modal_sch_advanced_max}
                                                    <input type="number" bind:value={fileMaxRawVal} on:change={fileSizeCalc}>
                                                </label>
                                                <select class="modal-set-file-sch-size-unit" bind:value={fileMaxUnit} on:change={fileSizeCalc}>
                                                    <option value="B">B</option>
                                                    <option value="KB">KB</option>
                                                    <option value="MB">MB</option>
                                                    <option value="GB">GB</option>
                                                </select>
                                                <br>
                                                <label>
                                                    {currentTranslations.modal_sch_advanced_min}
                                                    <input type="number" bind:value={fileMinRawVal} on:change={fileSizeCalc}>
                                                </label>
                                                <select class="modal-set-file-sch-size-unit" bind:value={fileMinUnit} on:change={fileSizeCalc}>
                                                    <option value="B">B</option>
                                                    <option value="KB">KB</option>
                                                    <option value="MB">MB</option>
                                                    <option value="GB">GB</option>
                                                </select>
                                            </div>
                                            {/if}
                                            <br>
                                            <label>
                                                {currentTranslations.modal_sch_advanced_file_property_type}
                                                <input type="checkbox" on:change={isFileTypeChkToggle} bind:checked={isFileTypeChk}>
                                            </label>
                                            {#if isFileTypeChk}
                                                <input type="text" bind:value={searchValObj.fileTypeList}>
                                            {/if}
                                            <br>
                                            <label>
                                                {currentTranslations.modal_sch_advanced_file_property_creation}
                                                <input type="checkbox" on:change={isFileCrtDateChkToggle} bind:checked={isFileCrtDateChk}>
                                            </label>
                                            {#if isFileCrtDateChk}
                                            <div>
                                                <!-- Creation Date -->
                                                <label>
                                                    {currentTranslations.modal_sch_advanced_date_start}
                                                    <input type="date" bind:value={searchValObj.crtStart}>
                                                </label>
                                                <br>
                                                <label>
                                                    {currentTranslations.modal_sch_advanced_date_end}
                                                    <input type="date" bind:value={searchValObj.crtEnd}>
                                                </label>
                                            </div>
                                            {/if}
                                            <br>
                                            <label>
                                                {currentTranslations.modal_sch_advanced_file_property_modified}
                                                <input type="checkbox" on:change={isFileModifiedDateChkToggle} bind:checked={isFileModifiedDateChk}>
                                            </label>
                                            {#if isFileModifiedDateChk}
                                            <div>
                                                <!-- Modified Date -->
                                                <label>
                                                    {currentTranslations.modal_sch_advanced_date_start}
                                                    <input type="date" bind:value={searchValObj.modiStart}>
                                                </label>
                                                <br>
                                                <label>
                                                    {currentTranslations.modal_sch_advanced_date_end}
                                                    <input type="date" bind:value={searchValObj.modiEnd}>
                                                </label>
                                            </div>
                                            {/if}
                                            <br>
                                            <label>
                                                {currentTranslations.modal_sch_advanced_file_property_owner}
                                                <input type="checkbox" on:change={isFileOwnerChkToggle} bind:checked={isFileOwnerChk}>
                                            </label>
                                            {#if isFileOwnerChk}
                                                <input type="text" bind:value={searchValObj.ownerName}>
                                            {/if}
                                        </div>
                                    {/if}
                                    <br>
                                    <label>{currentTranslations.modal_sch_advanced_symbolic_link}
                                        <input type="checkbox" id="symbolic-link-search-btn" bind:checked={searchValObj.customSymbolicChk}>  
                                    </label>
                                </div>
                                <hr>
                                <!-- Search Method -->
                                <h4>{currentTranslations.modal_sch_advanced_sch_methods}</h4>
                                <div class="modal-set-sch-method-wrapper">
                                    <label>
                                        <input type="radio" id="none-search-btn" class="modal-adv-sch-method" name="modal-adv-sch-method" value="0" bind:group={searchValObj.customSchMethod}>
                                        {currentTranslations.modal_sch_advanced_default}
                                    </label>
                                    <br>
                                    <label>
                                        <input type="radio" id="regex-search-btn" class="modal-adv-sch-method" name="modal-adv-sch-method" value="1" bind:group={searchValObj.customSchMethod}>
                                        {currentTranslations.modal_sch_advanced_regex}
                                    </label>
                                    <br>
                                    <label>
                                        <input type="radio" id="fuzzy-matching-btn" class="modal-adv-sch-method" name="modal-adv-sch-method" value="2" bind:group={searchValObj.customSchMethod}>
                                        {currentTranslations.modal_sch_advanced_fuzzy_dame}
                                    </label>
                                    <br>
                                    <label>
                                        <input type="radio" id="index-search-btn" class="modal-adv-sch-method" name="modal-adv-sch-method" value="3" bind:group={searchValObj.customSchMethod}>
                                        {currentTranslations.modal_sch_advanced_fuzzy_jac}
                                    </label>
                                </div>
                                <hr>
                                <!-- Search Log -->
                                <h4>{currentTranslations.modal_sch_advanced_log_title}</h4>
                                <div>
                                    <label>
                                        {currentTranslations.modal_sch_advanced_log_check}
                                        <input type="checkbox" bind:checked={searchValObj.customLogUse}>
                                    </label>
                                </div>
                            </div>
                        </div>
                        <br>
                        <button on:click={()=>{console.log(searchValObj)}}>DEBUG</button>
                        <br>
                        <button on:click={advModalToggle}>
                            {currentTranslations.modal_sch_advanced_open_val_slots}
                        </button>
                        <!-- Search Advanced custom Value Slot Modal -->
                        {#if showAdvSlotModal}
                        <div class="adv-slot-wrapper">
                            {#each slots as slot, index}
                              <div class="adv-slot">
                                <div>
                                    {currentTranslations.modal_sch_advanced_slots_slotTxt} {slot.number}
                                </div>
                                <div>
                                  <label>
                                    <input type="text" bind:value={slot.name} placeholder="{currentTranslations.modal_sch_advanced_slots_name_ph}">
                                  </label>
                                </div>
                                <button on:click={() => saveSlot(index)}>{currentTranslations.modal_sch_advanced_slots_save}</button>
                                <button on:click={() => loadSlot(index)} disabled={!slot.hasValue}>{currentTranslations.modal_sch_advanced_slots_load}</button>
                                <button on:click={() => deleteSlot(index)} disabled={!slot.hasValue}>{currentTranslations.modal_sch_advanced_slots_reset}</button>
                              </div>
                            {/each}
                        </div>
                        {/if}
                    {/if}
                </div>
                <button class="close-modal" on:click={toggleSettings}>{currentTranslations.modal_close}</button
                >
            </div>
        </div>
    {/if}
</div>
<!-- <a href="/frontTest/frame">Go to previous page</a> -->
