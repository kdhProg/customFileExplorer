<script lang="ts">
    import { listen } from '@tauri-apps/api/event';
    import { onMount, afterUpdate, onDestroy } from 'svelte';
    import { writable, get } from 'svelte/store';
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
    import CurrentPath from '$lib/components/currentPath.svelte';


    let showSettings = false; /* Setting modal show bool */
    let activeTab = "resize";
    let viewMode = "single"; /* Default view mode(single) - not yet implemented */
    let fileSize = 80; /* Default file icon size */
    let selectedDriveLeft = null;
    let selectedDriveRight = null;
    let selectedFolderLeft = null;


    /* Current Folder Name */
    let curFolderName = '';

    /* File List on Current Path */
    let filesInCurrentFolder: string[] = [];

    // File Click Event - Directory List
    async function handleFolderSelected(event) {
        curFolderName = event.detail;
        nowAtCategoryView = false; // disable category viewing mode
        addPathHistory(curFolderName); // add to history

        filesInCurrentFolder = await listFilesInDirectory(curFolderName); // rerender
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
    if (pathHistory.length === 0 || pathHistory[pathHistory.length - 1] !== path) {
        if (pathHistory.length >= 10) {
            pathHistory.shift();
        }

        pathHistory.push(path);
        currentIndex = pathHistory.length - 1;

        console.log("pathHistory : " + pathHistory);
        console.log("currentIndex: " + currentIndex);
    }
}

// path move
async function moveToFolder(newFolder: string) {
    const isValid = await isPathValid(newFolder); // check path validation
    if (!isValid) {
        console.error(`Invalid path: ${newFolder}`);
        return;
    }

    pathHistory = pathHistory.slice(0, currentIndex + 1); // erase paths after current position
    pathHistory.push(newFolder); // add new path

    if (pathHistory.length > 10) {
        pathHistory.shift(); // erase first value when pathHistory's length exceeds 10
    }

    currentIndex = pathHistory.length - 1; // update current index
    curFolderName = newFolder; // update current folder name
    filesInCurrentFolder = await listFilesInDirectory(curFolderName);
}

// move to valid path
async function goToValidPath(index: number) {
    const targetPath = pathHistory[index];
    const isValid = await isPathValid(targetPath);

    if (!isValid) {
        console.warn(`Invalid path: ${targetPath}. Removing from history.`);
        pathHistory.splice(index, 1); // erase invalid path
        if (index <= currentIndex) {
            currentIndex--; // set index 
        }
        return false; // Invalid path
    }

    curFolderName = targetPath;
    filesInCurrentFolder = await listFilesInDirectory(curFolderName);
    currentIndex = index;
    return true; // Valid path
}

// go back (left arrow)
async function goBack() {
    if (currentIndex > 0) {
        const success = await goToValidPath(currentIndex - 1);
        if (!success && currentIndex > 0) {
            await goBack(); // recursive until find valid path
        }
    }
    console.log(pathHistory);
}

// go forward(right arrow)
async function goForward() {
    if (currentIndex < pathHistory.length - 1) {
        const success = await goToValidPath(currentIndex + 1);
        if (!success && currentIndex < pathHistory.length - 1) {
            await goForward(); // recursive until find valid path
        }
    }
    console.log(pathHistory);
}

// find parent path(dir)
function getParentFolder(path: string): string | null {
    const parts = path.split('\\');

    if (parts.length === 2 && /^[A-Z]:$/.test(parts[0])) {
        return `${parts[0]}\\`; // 'D:' -> 'D:\'
    }

    const parent = parts.slice(0, -1).join('\\');
    return parent || null;
}

// move to parent path
    async function goUp() {
        const isRoot = /^[A-Z]:\\?$/.test(curFolderName); // check root
        if (isRoot) {
            console.log('Already at the root folder. Cannot move up.');
            return;
        }

        const parentFolder = getParentFolder(curFolderName);
        console.log('Parent folder:', parentFolder);

        if (parentFolder) {
            await moveToFolder(parentFolder);
        } else {
            console.log('Already at the root folder.');
        }
    }





    // ------------------------------ File Icon ------------------------------

    let fileIcons: { [key: string]: string } = {}; // file icon depends on path

    // load file icon depends on path
    async function loadIcons(files) {
        const icons = {};
        for (const file of files) {
            icons[file] = await getFileIcon(file);
        }
        fileIcons = icons;
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
            // change_icon_default();
        } else if (themePath.includes('retro')) {
            currentLogo = themeLogos.retro;
            // change_icon_default();
        } else if (themePath.includes('sf')) {
            currentLogo = themeLogos.sf;
            // change_icon_cyan();
        } else if(themePath.includes('linux')) {
            currentLogo = themeLogos.linux;
            // change_icon_default();
        }
        // update current page theme
        currentTheme = themePath;

        filesInCurrentFolder = [...filesInCurrentFolder];
    }

    // Load default theme when page load
    // applyTheme(currentTheme);
    onMount(() => {
        applyTheme(currentTheme);
    });

// ------------------------------  Search ------------------------------

    // Check If searching is on
    let isSearching:boolean = false;
    let searchProcessId = null;
    let unlisten; // for real time listener
    let receivedFiles = new Set(); // set -> to filter duplication

    async function searchFilesInDirectory() {

        // before searhcing, adv props validation test should be passed
        if(searchValObj.customPropertyUse){
            if(searchValObj.customFileSizeUse){ // Check File size
                if( !validateFileSize(searchValObj.sizeMin,searchValObj.sizeMax) )return;
            }
            if(searchValObj.customFileTypeUse){
                if(!searchValObj.fileTypeList || !searchValObj.fileTypeList.trim()){
                    alert(currentTranslations.alt_modal_valid_empty_type_list);
                    return;
                }
            }
            if(searchValObj.customFileCrtDateUse){ // Check Creation Date
                if( !validateDateRange(searchValObj.crtStart,searchValObj.crtEnd) )return;
            }
            if(searchValObj.customFileModiDateUse){ // Check Modified Date
                if( !validateDateRange(searchValObj.modiStart,searchValObj.modiEnd) )return;
            }
            if(searchValObj.customFileOwnerUse){
                if( !searchValObj.ownerName || !searchValObj.ownerName.trim()){
                    alert(currentTranslations.alt_modal_valid_empty_owner_name)
                    return;
                }
            }
        }
        

        try {
            isSearching = true;
            const keyword = document.getElementById('searchInput');

            if(curFolderName === '' || curFolderName.length === 0){
                // No currentfolder (= when Initially project starts )
                

            }else{
                // searching from current directory
                if (keyword instanceof HTMLInputElement) {
                    // Get User-input Search Keyword
                    const inputValue = keyword.value;

                    // initialize listener and delete old one
                    if (unlisten) {
                        await unlisten();
                        unlisten = null;
                    }

                    receivedFiles.clear();
                    
                    // get search process ID
                    unlisten = await listen('process-info', (event) => {
                        const processInfo = event.payload;
                        if (processInfo && processInfo.id) {
                            searchProcessId = processInfo.id;  // save process ID
                            console.log("Process ID from backend:", searchProcessId);
                        }
                    });

                    // Search Result Array
                    let searchRst = [];
                    filesInCurrentFolder = []; // Clear filesInCurrentFolder

                    // Real time result listener
                    unlisten = await listen('search-result', (event) => {
                        const file = event.payload;

                        // filter duplicate file_path
                        if (!searchRst.includes(file.file_path)) {
                            searchRst.push(file.file_path);
                            filesInCurrentFolder = [...searchRst]; // update filesInCurrentFolder
                            console.log("Real-time search result:", file.file_path);  // file_path 출력
                        }
                    });

                    // search duration listener
                    await listen('search-time', (event) => {
                        const searchTime = event.payload;
                        console.log(`Search completed in ${searchTime} seconds`);
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
            alert(currentTranslations.search_path_error  );
            isSearching = false;
        }
    }

    // Request Cancel signal
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

// ------------------------------ keyword autocomplete ---------------------------------
    let isAutocompleteCheck:boolean = false;

    function isAutocompleteCheckToggle(){
        isAutocompleteCheck = !isAutocompleteCheck;
        if(!isAutocompleteCheck){
            blurredText = '';
        }
    }

    let inputValue = "";  // Input keyword
    let suggestions: string[] = [];  // suggestion list from backend( It can be empty )
    let blurredText = "";  // suggested blurred text 

    // get suggestions when page loads
    async function fetchKeywords() {
        const { keywords } = await invoke('get_keywords');
        if(keywords)suggestions = keywords;
    }

    // called when user input keywords
    function searchHandleInput(event) {
        if(!isAutocompleteCheck)return;
        // inputValue = event.target.value;
    // find suggestion keywords starts with input keywords
    const suggestion = suggestions.find((s) => s.startsWith(inputValue));
     blurredText = suggestion ? suggestion.slice(inputValue.length) : "";
    }

    // get suggestions when page loads
    onMount(fetchKeywords);

    // --------------------- change lang --------------------

    // (Ctrl + Space)
    function cngLanghandleKeydown(event: KeyboardEvent) {
        if (event.ctrlKey && event.code === 'Space') {
            schDoConvert(inputValue);
            // console.log("inputValue: ",inputValue)
        }
    }



    // --------------------- Validate Search Options ----------------------------

    function validateFileSize(minSize, maxSize) {
        const MAX_BYTES = 100 * 1024 ** 3; // 100 GB in bytes

        if (
            (minSize === '' || minSize === null || minSize === undefined) ||
            (maxSize === '' || maxSize === null || maxSize === undefined) ||
            !maxSize.toString().trim()
        ) {
            alert(currentTranslations.alt_modal_valid_file_size_no_value);
            return false;
        }

        if (!Number.isInteger(minSize) || !Number.isInteger(maxSize)) {
            alert(currentTranslations.alt_modal_valid_file_size_must_int);
            return false;
        }

        if (minSize < 0 || maxSize <= 0) {
            alert(currentTranslations.alt_modal_valid_file_size_bigger_than_0);
            return false;
        }

        if (minSize > maxSize) {
            alert(currentTranslations.alt_modal_valid_file_size_min_bigger_than_max);
            return false;
        }
        if (maxSize > MAX_BYTES) {
            alert(currentTranslations.alt_modal_valid_file_size_lower_than_100GB);
            return false;
        }

            return true; // valid
        }


    function validateDateRange(startDate, endDate) {
        const today = new Date();
        today.setHours(0, 0, 0, 0); // set time to 00:00:00 -> only calculate date

        if (!startDate || !endDate) {
            alert(currentTranslations.alt_modal_valid_date_empty_val);
            return false;
        }

        const start = new Date(startDate);
        const end = new Date(endDate);

        start.setHours(0, 0, 0, 0); // set time to 00:00:00 -> only calculate date
        end.setHours(0, 0, 0, 0);   // set time to 00:00:00 -> only calculate date

        if (isNaN(start.getTime()) || isNaN(end.getTime())) {
            alert(currentTranslations.alt_modal_valid_date_invalid_type);
            return false;
        }
        if (start > end) {
            alert(currentTranslations.alt_modal_valid_date_start_faster_than_end);
            return false;
        }
        if (start > today || end > today) {
            alert(currentTranslations.alt_modal_valid_date_before_today);
            return false;
        }

        return true; // valid
    }



    
    // ------------------------------ Click Each Folder / Files ------------------------------
    // Folder - update current folder list
    // File - execute with default enrolled programs
    async function eachFolderClick(file:string){
        // nowAtCategoryView = false; // disable category viewing mode
        // curFolderName = file;
        let isDir = await isDirectory(file);
        if(isDir){
            nowAtCategoryView = false; // disable category viewing mode
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
let isDragging = writable(false);  // drag state
let startX = 0, startY = 0;        // drag start pos
let endX = 0, endY = 0;            // drag end pos
let selectedFiles = writable<string[]>([]);  // selected file paths

const dragThreshold = 5;  // minimum movement distance
let rectStyle = writable('');  // rectangle css style
let container;  // file-viewer container

// mouse down : save first pos of cursor
function handleMouseDown(event: MouseEvent) {
  event.preventDefault(); // prevent default browser actions
  clearSelection();

  container = document.getElementById('fileViewer');

  const rect = container.getBoundingClientRect(); // parent container
  startX = event.pageX - rect.left + container.scrollLeft;
  startY = event.pageY - rect.top + container.scrollTop;
  endX = startX;
  endY = startY;

  rectStyle.set('');
  isDragging.set(true); // set drag on
}

// mouse move
function handleMouseMove(event: MouseEvent) {
  if (!$isDragging) return; // if not drag, ends.

  const rect = container.getBoundingClientRect();
  endX = event.pageX - rect.left + container.scrollLeft;
  endY = event.pageY - rect.top + container.scrollTop;

  updateRectStyle();
}

// mouse up : end drag
function handleMouseUp(event: MouseEvent) {
  isDragging.set(false);
  detectFilesInside();
}

// update select-rectangle-area
function updateRectStyle() {
  const x1 = Math.min(startX, endX);
  const y1 = Math.min(startY, endY);
  const width = Math.abs(endX - startX);
  const height = Math.abs(endY - startY);

  rectStyle.set(`left: ${x1}px; top: ${y1}px; width: ${width}px; height: ${height}px;`);
}

// clear old selected paths
function clearSelection() {
  const selectedElements = document.querySelectorAll('.file-item.selected');
  selectedElements.forEach((el) => el.classList.remove('selected'));
}

// detect file paths inside selection box
function detectFilesInside() {
  const container = document.getElementById('fileViewer'); // file-viewer 컨테이너 참조
  const containerRect = container.getBoundingClientRect(); // 컨테이너 기준 좌표 가져오기

  const selectionRect = {
    left: Math.min(startX, endX),
    top: Math.min(startY, endY),
    right: Math.max(startX, endX),
    bottom: Math.max(startY, endY),
  };

  const fileElements = document.querySelectorAll('.file-item');

  const selected = Array.from(fileElements).filter((el) => {
    const elRect = el.getBoundingClientRect();

    const elementLeft = elRect.left - containerRect.left + container.scrollLeft;
    const elementTop = elRect.top - containerRect.top + container.scrollTop;
    const elementRight = elementLeft + elRect.width;
    const elementBottom = elementTop + elRect.height;

    // check if elements inside selection box
    return !(
      selectionRect.right < elementLeft ||
      selectionRect.left > elementRight ||
      selectionRect.bottom < elementTop ||
      selectionRect.top > elementBottom
    );
  });

  // add 'selected' class to selected elements
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

  //   -------------------- Cut / Copy / Paste --------------------
  let copyClipboard: string[] = [];
  let cutClipboard: string[] = [];
  let ccp_message = writable<string>(''); // error msg

    function copyFiles() {
        const files = $selectedFiles;

        // erase duplicate from cutClipboard
        cutClipboard = cutClipboard.filter((file) => !files.includes(file));

        // save values to copyClipboard
        copyClipboard = [...files];

        console.log('Copy clipboard:', copyClipboard);
        console.log('Cut clipboard after removal:', cutClipboard);
    }

    function rmCopyClipFile(event) {
        const filePath = event.target.value; // each button's value
        copyClipboard = copyClipboard.filter(file => file !== filePath); //erase self from arr
    }

    function cutFiles() {
        const files = $selectedFiles;

        // erase duplicate from copyClipboard
        copyClipboard = copyClipboard.filter((file) => !files.includes(file));

        // save values to cutClipboard
        cutClipboard = [...files];

        console.log('Cut clipboard:', cutClipboard);
        console.log('copy clipboard after removal:', copyClipboard);

    }

    function rmCutClipFile(event) {
        const filePath = event.target.value;
        cutClipboard = cutClipboard.filter(file => file !== filePath);
    }

    async function pasteFiles(clipboard: string[], targetPath: string, isCut: boolean) {
        console.log("targetPath : "+targetPath);
        try {
            const result = await invoke('paste_files', { files: clipboard, targetPath, cut: isCut });
            // console.log("targetPath : "+targetPath);
        if (!result.success) {
            alert(currentTranslations.paste_fail_msg+result.message); 
            // if error occurred -> clear clipboard
            if (isCut){
                cutClipboard = [];
            }else{
                copyClipboard = [];
            }
        } else {
            ccp_message.set('Files pasted successfully.');
            if (isCut) cutClipboard = [];
                else copyClipboard = [];
            }
        } catch (err) {
            ccp_message.set(`Error: ${err}`);
            alert(currentTranslations.pasteErr + `${err}`);
        }
        filesInCurrentFolder = await listFilesInDirectory(curFolderName); // Rerending
    }

    

    // ----------------------- Copy / Paste expand toggle -----------------------------
    let isCopyExpanded = false;
    let isCutExpanded = false;

    function toggleCopyList() {
        isCopyExpanded = !isCopyExpanded;
    }

    

    function toggleCutList() {
        isCutExpanded = !isCutExpanded;
    }

    // ------ clear clips ----------
    function clearCopyClip(){
        copyClipboard = [];
    }

    function clearCutClip(){
        cutClipboard = []
    }


  //   -------------------- Delete --------------------

    const del_message = writable<string>(''); // error msg

    // move to recycle bin
    async function moveToTrash() {
    const files = $selectedFiles;

    try {
        const result = await invoke('move_files_to_trash', { paths: files });
        filesInCurrentFolder = await listFilesInDirectory(curFolderName); // Rerending
        if (!result.success) {
            del_message.set(result.message);
        } else {
            del_message.set('All files moved to trash successfully.');
            selectedFiles.set([]); // clear selection clipboard
        }
    } catch (err) {
            del_message.set(`Unexpected error: ${err}`);
    }
    }

    // ---------- Util Shortcut  ----------------

    function handleSelectAllShortCut(event) {
        if (event.ctrlKey && event.key.toLowerCase() === 'a') {
            event.preventDefault();
            selectAllFiles();
        }
    }
    function handleCopyCutPasteShortCut(event) {
        try {
            if (event.ctrlKey && event.key === 'c') {
            copyFiles();
            } else if (event.ctrlKey && event.key === 'x') {
            cutFiles();
            } else if (event.key === 'Delete') {
                moveToTrash();
            }else if (event.ctrlKey && event.key === 'v' && (copyClipboard.length > 0 || cutClipboard.length>0)) {
                pasteFiles(copyClipboard, curFolderName, false)
            } else if (event.ctrlKey && event.shiftKey && event.key.toLowerCase() === 'v') {
                // cut-paste
                console.log("cut-paste detected")
                pasteFiles(cutClipboard, curFolderName, true)
            }
        } catch (error) {
            alert(currentTranslations.utilshotcutErr + `${error.message}`);
        }
    }

    function selectAllFiles() {
        const fileItems = document.querySelectorAll('.file-item');
        fileItems.forEach(item => item.classList.add('selected'));

        const fileElements = document.querySelectorAll('.selected');

        const selected = Array.from(fileElements)
        selected.forEach((el) => el.classList.add('selected'));

        const selectedPaths = selected.map((el) => el.getAttribute('data-file-path') || '');
        selectedFiles.set(selectedPaths);
    }


    onMount(() => {
        window.addEventListener('keydown', handleSelectAllShortCut);
        window.addEventListener('keydown', handleCopyCutPasteShortCut);
    });

    onDestroy(() => {
        window.removeEventListener('keydown', handleSelectAllShortCut);
        window.addEventListener('keydown', handleCopyCutPasteShortCut);
    });


    // ------------ make new Folder / File ----------
    async function createItem(isFolder:boolean,basePath:String) {
        try {
            const result = await invoke('create_new_item', {
                isFolder,
                basePath,
            });
            alert(currentTranslations.mk_new_item_success + ` :  ${result}`);
            console.log(`Item crt at: ${result}`)

            filesInCurrentFolder = await listFilesInDirectory(curFolderName); // Rerending
        } catch (error) {
            alert(currentTranslations.mk_new_item_fail + ` : ${error}`);
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
// ------------ file viewer / dir viewer divide bar resizer
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


// ----------------------------- Set Language ----------------------------
function switchLanguage(lang: string) {
    language.set(lang);
}

// Reactive derived store to get the current translations
$: currentTranslations = translations[$language];


// ---------------------main logo click event (open github-repo)--------------
function openGitgubRepo(){
    window.open('https://github.com/kdhProg/customFileExplorer', '_blank');
}


// ---------------------------- modal -----------------------------------
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

// ------------------------------ modal draggable ------------------------------------

    let modal_set_modal;
    let modal_set_isDragging = false;
    let modal_set_startX = 0, modal_set_startY = 0;

    function modal_set_startDrag(e) {
        // Disable Drag at input range - resize tab
        if (e.target.tagName === 'INPUT' || e.target.closest('input[type="range"]')) return;

        // Disable Drag at Outside of modal
        if (e.target !== modal_set_modal && !modal_set_modal.contains(e.target)) return;

        modal_set_isDragging = true;

        const rect = modal_set_modal.getBoundingClientRect();
        modal_set_startX = e.clientX - rect.left;
        modal_set_startY = e.clientY - rect.top;

        // change cursor - grab
        document.body.style.cursor = 'grabbing';

        document.addEventListener('mousemove', modal_set_onDrag);
        document.addEventListener('mouseup', modal_set_stopDrag);
    }

    function modal_set_onDrag(e) {
        if (!modal_set_isDragging) return;

        const newLeft = e.clientX - modal_set_startX;
        const newTop = e.clientY - modal_set_startY;

        const maxX = window.innerWidth - modal_set_modal.offsetWidth;
        const maxY = window.innerHeight - modal_set_modal.offsetHeight;

        const finalLeft = Math.max(0, Math.min(newLeft, maxX));
        const finalTop = Math.max(0, Math.min(newTop, maxY));

        modal_set_modal.style.left = `${finalLeft}px`;
        modal_set_modal.style.top = `${finalTop}px`;
    }

    function modal_set_stopDrag() {
        modal_set_isDragging = false;

        // return cursor style
        document.body.style.cursor = 'default';

        document.removeEventListener('mousemove', modal_set_onDrag);
        document.removeEventListener('mouseup', modal_set_stopDrag);
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


// ------------------------ custom property -----------------------------
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


// ---------------------- file property - max & min calculation ----------------------
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

  // ------------------------- Slot Init -----------------------------------
  onMount(async () => {
    for (let i = 0; i < slots.length; i++) {
      const result = await invoke("load_settings", { slotNumber: slots[i].number });
      if (result.name.length !== 0) {
        slots[i].name = result.name;
        slots[i].hasValue = true;
      }
    }
  });

  // ------------------------- Save Custom Values -----------------------
  async function saveSlot(slotIndex:number) {
    const slot = slots[slotIndex];

    // not allow empty values
    if (!slot.name || slot.name.trim() === "") {
        alert(currentTranslations.adv_slot_empty_name_alert);
        return;
    }

    await invoke("save_settings", {
        slotNumber: slot.number,
        name: slot.name,
        settings: searchValObj
    });
    slots[slotIndex].hasValue = true; // activate load & reset btn after save
  }

// ----------------------------------- adv slot Load ------------------------------------ 
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

  // ----------------------- Slot reset(= delete custom values and reset) -------------------------
  async function deleteSlot(slotIndex:number) {
    const slot = slots[slotIndex];
    await invoke("delete_settings", { slotNumber: slot.number });
    slots[slotIndex].name = "";
    slots[slotIndex].hasValue = false;
  }

  // --------------- mouse rightclick option -------------------
  let right_click_visible = writable(false);
  let right_click_position = writable({ x: 0, y: 0 });

  // value will be saved this var
  let right_click_clip = writable([]);

  function closeMenu() {
    right_click_visible.set(false);
    right_click_clip.set([]);
  }

  function openMenu(e) {
    const target = e.target.closest('.file-item');
    if (!target) return;

    e.preventDefault();
    right_click_clip.update((files) => [
    //   ...files,
      target.dataset.filePath
    ]);
    right_click_position.set({ x: e.clientX, y: e.clientY });
    right_click_visible.set(true);
  }

    // ----------- right_click_copy & paste ------------------
    function rightCopyFiles() {
        const files = $right_click_clip;

        cutClipboard = cutClipboard.filter((file) => !files.includes(file));

        copyClipboard = [...files];
    }

    function rightCutFiles() {
        const files = $right_click_clip;

        copyClipboard = copyClipboard.filter((file) => !files.includes(file));

        cutClipboard = [...files];
    }

    onMount(() => {
    window.addEventListener('contextmenu', openMenu);
    document.addEventListener('click', closeMenu);
  });

  onDestroy(() => {
    window.removeEventListener('contextmenu', openMenu);
    document.removeEventListener('click', closeMenu);
  });

//   ------------ file Rename ----------
  let editingFile = writable(null);
  let newName = writable('');

    function getParentPath(filePath) {
        // Set separator (Windows: \, Unix: /)
        const separator = filePath.includes('\\') ? '\\' : '/';

        const parts = filePath.split(separator);
        parts.pop();

        return parts.join(separator);
    }

    function handleRenameClick() {
        const [file] = get(right_click_clip);
        startEditing(file);
    }


    function startEditing(file) {
        editingFile.set(file);
        newName.set(getFileName(file));
    }

  async function renameFile(file, parentPath) {
    const newFileName = $newName.trim();
    if (!newFileName) return;

    const oldFileName = getFileName(file);
    if (oldFileName === newFileName) {
      editingFile.set(null);
      return;
    }
    // console.log("parentPath : "+parentPath);
    // console.log("file: "+file);
    // console.log("newFileName : "+newFileName);
    try {
      await invoke('rename_file_or_directory', {
        oldPath: file,
        newPath: `${parentPath}/${newFileName}`,
      });

    filesInCurrentFolder = await listFilesInDirectory(curFolderName); // Rerending


    } catch (error) {
        // current Issue : Rename well worked but his alert statement always be called
        // err msg from backend : Invalid path
    //   alert(currentTranslations.alt_file_rename_failed + `   ${error}`);
    }

    editingFile.set(null);
  }


  // --------- right click properties ----------

  let fileMetadata = writable(null);
  let filePropsModalPos = writable({ x: 0, y: 0 });

  async function fetchFileMetadata() {
    try {
      const files = get(right_click_clip);
      if (files.length === 0) {
        // alert('파일 경로가 없습니다.');
        return;
      }

      const filePath = files[0];

      const metadata = await invoke('get_file_metadata', { filePath: filePath });
      fileMetadata.set(metadata);
    } catch (error) {
      console.error('메타데이터 가져오기 실패:', error);
    //   alert(`메타데이터 가져오기 실패: ${error}`);
    }
  }

  // modal close store
  let filePropModalToggle = writable(false);

  function filePropModalOpen(event) {
    event.preventDefault();
    const { clientX: x, clientY: y } = event;
    filePropsModalPos.set({ x, y });
    filePropModalToggle.set(true);
    fetchFileMetadata();
  }

  function filePropModalClose() {
    filePropModalToggle.set(false);
    fileMetadata.set(null);
  }

//   ---------------------------- search Lang Change ----------------------------------
// source : https://theyt.net/wiki/%ED%95%9C%EC%98%81%ED%83%80%EB%B3%80%ED%99%98%EA%B8%B0

const ENG_KEY = "rRseEfaqQtTdwWczxvgkoiOjpuPhynbml";
const KOR_KEY = "ㄱㄲㄴㄷㄸㄹㅁㅂㅃㅅㅆㅇㅈㅉㅊㅋㅌㅍㅎㅏㅐㅑㅒㅓㅔㅕㅖㅗㅛㅜㅠㅡㅣ";
const CHO_DATA = "ㄱㄲㄴㄷㄸㄹㅁㅂㅃㅅㅆㅇㅈㅉㅊㅋㅌㅍㅎ";
const JUNG_DATA = "ㅏㅐㅑㅒㅓㅔㅕㅖㅗㅘㅙㅚㅛㅜㅝㅞㅟㅠㅡㅢㅣ";
const JONG_DATA = "ㄱㄲㄳㄴㄵㄶㄷㄹㄺㄻㄼㄽㄾㄿㅀㅁㅂㅄㅅㅆㅇㅈㅊㅋㅌㅍㅎ";

function schDoConvert(searchQuery:String) {
	const hasKorean = /[ㄱ-ㅎ|ㅏ-ㅣ|가-힣]/.test(searchQuery);
    const hasEnglish = /[a-zA-Z]/.test(searchQuery);

    if (hasKorean && hasEnglish) {
        return;
    } else if (hasKorean) {
        inputValue = korTypeToEng(searchQuery);
    } else if (hasEnglish) {
        inputValue = engTypeToKor(searchQuery);
    } else {
        return;
    }

    
}

function engTypeToKor(src) {
	var res = "";
	if (src.length == 0)
		return res;

	var nCho = -1, nJung = -1, nJong = -1;		// 초성, 중성, 종성

	for (var i = 0; i < src.length; i++) {
		var ch = src.charAt(i);
		var p = ENG_KEY.indexOf(ch);
		if (p == -1) {				// 영자판이 아님
			// 남아있는 한글이 있으면 처리
			if (nCho != -1) {
				if (nJung != -1)				// 초성+중성+(종성)
					res += makeHangul(nCho, nJung, nJong);
				else							// 초성만
					res += CHO_DATA.charAt(nCho);
			} else {
				if (nJung != -1)				// 중성만
					res += JUNG_DATA.charAt(nJung);
				else if (nJong != -1)			// 복자음
					res += JONG_DATA.charAt(nJong);
			}
			nCho = -1;
			nJung = -1;
			nJong = -1;
			res += ch;
		} else if (p < 19) {			// 자음
			if (nJung != -1) {
				if (nCho == -1) {					// 중성만 입력됨, 초성으로
					res += JUNG_DATA.charAt(nJung);
					nJung = -1;
					nCho = CHO_DATA.indexOf(KOR_KEY.charAt(p));
				} else {							// 종성이다
					if (nJong == -1) {				// 종성 입력 중
						nJong = JONG_DATA.indexOf(KOR_KEY.charAt(p));
						if (nJong == -1) {			// 종성이 아니라 초성이다
							res += makeHangul(nCho, nJung, nJong);
							nCho = CHO_DATA.indexOf(KOR_KEY.charAt(p));
							nJung = -1;
						}
					} else if (nJong == 0 && p == 9) {			// ㄳ
						nJong = 2;
					} else if (nJong == 3 && p == 12) {			// ㄵ
						nJong = 4;
					} else if (nJong == 3 && p == 18) {			// ㄶ
						nJong = 5;
					} else if (nJong == 7 && p == 0) {			// ㄺ
						nJong = 8;
					} else if (nJong == 7 && p == 6) {			// ㄻ
						nJong = 9;
					} else if (nJong == 7 && p == 7) {			// ㄼ
						nJong = 10;
					} else if (nJong == 7 && p == 9) {			// ㄽ
						nJong = 11;
					} else if (nJong == 7 && p == 16) {			// ㄾ
						nJong = 12;
					} else if (nJong == 7 && p == 17) {			// ㄿ
						nJong = 13;
					} else if (nJong == 7 && p == 18) {			// ㅀ
						nJong = 14;
					} else if (nJong == 16 && p == 9) {			// ㅄ
						nJong = 17;
					} else {						// 종성 입력 끝, 초성으로
						res += makeHangul(nCho, nJung, nJong);
						nCho = CHO_DATA.indexOf(KOR_KEY.charAt(p));
						nJung = -1;
						nJong = -1;
					}
				}
			} else {								// 초성 또는 (단/복)자음이다
				if (nCho == -1) {					// 초성 입력 시작
					if (nJong != -1) {				// 복자음 후 초성
						res += JONG_DATA.charAt(nJong);
						nJong = -1;
					}
					nCho = CHO_DATA.indexOf(KOR_KEY.charAt(p));
				} else if (nCho == 0 && p == 9) {			// ㄳ
					nCho = -1;
					nJong = 2;
				} else if (nCho == 2 && p == 12) {			// ㄵ
					nCho = -1;
					nJong = 4;
				} else if (nCho == 2 && p == 18) {			// ㄶ
					nCho = -1;
					nJong = 5;
				} else if (nCho == 5 && p == 0) {			// ㄺ
					nCho = -1;
					nJong = 8;
				} else if (nCho == 5 && p == 6) {			// ㄻ
					nCho = -1;
					nJong = 9;
				} else if (nCho == 5 && p == 7) {			// ㄼ
					nCho = -1;
					nJong = 10;
				} else if (nCho == 5 && p == 9) {			// ㄽ
					nCho = -1;
					nJong = 11;
				} else if (nCho == 5 && p == 16) {			// ㄾ
					nCho = -1;
					nJong = 12;
				} else if (nCho == 5 && p == 17) {			// ㄿ
					nCho = -1;
					nJong = 13;
				} else if (nCho == 5 && p == 18) {			// ㅀ
					nCho = -1;
					nJong = 14;
				} else if (nCho == 7 && p == 9) {			// ㅄ
					nCho = -1;
					nJong = 17;
				} else {							// 단자음을 연타
					res += CHO_DATA.charAt(nCho);
					nCho = CHO_DATA.indexOf(KOR_KEY.charAt(p));
				}
			}
		} else {									// 모음
			if (nJong != -1) {						// (앞글자 종성), 초성+중성
				// 복자음 다시 분해
				var newCho;			// (임시용) 초성
				if (nJong == 2) {					// ㄱ, ㅅ
					nJong = 0;
					newCho = 9;
				} else if (nJong == 4) {			// ㄴ, ㅈ
					nJong = 3;
					newCho = 12;
				} else if (nJong == 5) {			// ㄴ, ㅎ
					nJong = 3;
					newCho = 18;
				} else if (nJong == 8) {			// ㄹ, ㄱ
					nJong = 7;
					newCho = 0;
				} else if (nJong == 9) {			// ㄹ, ㅁ
					nJong = 7;
					newCho = 6;
				} else if (nJong == 10) {			// ㄹ, ㅂ
					nJong = 7;
					newCho = 7;
				} else if (nJong == 11) {			// ㄹ, ㅅ
					nJong = 7;
					newCho = 9;
				} else if (nJong == 12) {			// ㄹ, ㅌ
					nJong = 7;
					newCho = 16;
				} else if (nJong == 13) {			// ㄹ, ㅍ
					nJong = 7;
					newCho = 17;
				} else if (nJong == 14) {			// ㄹ, ㅎ
					nJong = 7;
					newCho = 18;
				} else if (nJong == 17) {			// ㅂ, ㅅ
					nJong = 16;
					newCho = 9;
				} else {							// 복자음 아님
					newCho = CHO_DATA.indexOf(JONG_DATA.charAt(nJong));
					nJong = -1;
				}
				if (nCho != -1)			// 앞글자가 초성+중성+(종성)
					res += makeHangul(nCho, nJung, nJong);
				else                    // 복자음만 있음
					res += JONG_DATA.charAt(nJong);

				nCho = newCho;
				nJung = -1;
				nJong = -1;
			}
			if (nJung == -1) {						// 중성 입력 중
				nJung = JUNG_DATA.indexOf(KOR_KEY.charAt(p));
			} else if (nJung == 8 && p == 19) {            // ㅘ
				nJung = 9;
			} else if (nJung == 8 && p == 20) {            // ㅙ
				nJung = 10;
			} else if (nJung == 8 && p == 32) {            // ㅚ
				nJung = 11;
			} else if (nJung == 13 && p == 23) {           // ㅝ
				nJung = 14;
			} else if (nJung == 13 && p == 24) {           // ㅞ
				nJung = 15;
			} else if (nJung == 13 && p == 32) {           // ㅟ
				nJung = 16;
			} else if (nJung == 18 && p == 32) {           // ㅢ
				nJung = 19;
			} else {			// 조합 안되는 모음 입력
				if (nCho != -1) {			// 초성+중성 후 중성
					res += makeHangul(nCho, nJung, nJong);
					nCho = -1;
				} else						// 중성 후 중성
					res += JUNG_DATA.charAt(nJung);
				nJung = -1;
				res += KOR_KEY.charAt(p);
			}
		}
	}

	// 마지막 한글이 있으면 처리
	if (nCho != -1) {
		if (nJung != -1)			// 초성+중성+(종성)
			res += makeHangul(nCho, nJung, nJong);
		else                		// 초성만
			res += CHO_DATA.charAt(nCho);
	} else {
		if (nJung != -1)			// 중성만
			res += JUNG_DATA.charAt(nJung);
		else {						// 복자음
			if (nJong != -1)
				res += JONG_DATA.charAt(nJong);
		}
	}

	return res;
}

function makeHangul(nCho, nJung, nJong) {
	return String.fromCharCode(0xac00 + nCho * 21 * 28 + nJung * 28 + nJong + 1);
}

function korTypeToEng(src) {
	var res = "";
	if (src.length == 0)
		return res;

	for (var i = 0; i < src.length; i++) {
		var ch = src.charAt(i);
		var nCode = ch.charCodeAt(0);
		var nCho = CHO_DATA.indexOf(ch), nJung = JUNG_DATA.indexOf(ch), nJong = JONG_DATA.indexOf(ch);
		var arrKeyIndex = [-1, -1, -1, -1, -1];

		if (0xac00 <= nCode && nCode <= 0xd7a3) {
			nCode -= 0xac00;
			arrKeyIndex[0] = Math.floor(nCode / (21 * 28));			// 초성
			arrKeyIndex[1] = Math.floor(nCode / 28) % 21;			// 중성
			arrKeyIndex[3] = nCode % 28 - 1;						// 종성
		} else if (nCho != -1)			// 초성 자음
			arrKeyIndex[0] = nCho;
		else if (nJung != -1)			// 중성
			arrKeyIndex[1] = nJung;
		else if (nJong != -1)			// 종성 자음
			arrKeyIndex[3] = nJong;
		else							// 한글이 아님
			res += ch;

		// 실제 Key Index로 변경. 초성은 순서 동일
		if (arrKeyIndex[1] != -1) {
			if (arrKeyIndex[1] == 9) {					// ㅘ
				arrKeyIndex[1] = 27;
				arrKeyIndex[2] = 19;
			} else if (arrKeyIndex[1] == 10) {			// ㅙ
				arrKeyIndex[1] = 27;
				arrKeyIndex[2] = 20;
			} else if (arrKeyIndex[1] == 11) {			// ㅚ
				arrKeyIndex[1] = 27;
				arrKeyIndex[2] = 32;
			} else if (arrKeyIndex[1] == 14) {			// ㅝ
				arrKeyIndex[1] = 29;
				arrKeyIndex[2] = 23;
			} else if (arrKeyIndex[1] == 15) {			// ㅞ
				arrKeyIndex[1] = 29;
				arrKeyIndex[2] = 24;
			} else if (arrKeyIndex[1] == 16) {			// ㅟ
				arrKeyIndex[1] = 29;
				arrKeyIndex[2] = 32;
			} else if (arrKeyIndex[1] == 19) {			// ㅢ
				arrKeyIndex[1] = 31;
				arrKeyIndex[2] = 32;
			} else {
				arrKeyIndex[1] = KOR_KEY.indexOf(JUNG_DATA.charAt(arrKeyIndex[1]));
				arrKeyIndex[2] = -1;
			}
		}
		if (arrKeyIndex[3] != -1) {
			if (arrKeyIndex[3] == 2) {					// ㄳ
				arrKeyIndex[3] = 0
				arrKeyIndex[4] = 9;
			} else if (arrKeyIndex[3] == 4) {			// ㄵ
				arrKeyIndex[3] = 2;
				arrKeyIndex[4] = 12;
			} else if (arrKeyIndex[3] == 5) {			// ㄶ
				arrKeyIndex[3] = 2;
				arrKeyIndex[4] = 18;
			} else if (arrKeyIndex[3] == 8) {			// ㄺ
				arrKeyIndex[3] = 5;
				arrKeyIndex[4] = 0;
			} else if (arrKeyIndex[3] == 9) {			// ㄻ
				arrKeyIndex[3] = 5;
				arrKeyIndex[4] = 6;
			} else if (arrKeyIndex[3] == 10) {			// ㄼ
				arrKeyIndex[3] = 5;
				arrKeyIndex[4] = 7;
			} else if (arrKeyIndex[3] == 11) {			// ㄽ
				arrKeyIndex[3] = 5;
				arrKeyIndex[4] = 9;
			} else if (arrKeyIndex[3] == 12) {			// ㄾ
				arrKeyIndex[3] = 5;
				arrKeyIndex[4] = 16;
			} else if (arrKeyIndex[3] == 13) {			// ㄿ
				arrKeyIndex[3] = 5;
				arrKeyIndex[4] = 17;
			} else if (arrKeyIndex[3] == 14) {			// ㅀ
				arrKeyIndex[3] = 5;
				arrKeyIndex[4] = 18;
			} else if (arrKeyIndex[3] == 17) {			// ㅄ
				arrKeyIndex[3] = 7;
				arrKeyIndex[4] = 9;
			} else {
				arrKeyIndex[3] = KOR_KEY.indexOf(JONG_DATA.charAt(arrKeyIndex[3]));
				arrKeyIndex[4] = -1;
			}
		}

		for (var j = 0; j < 5; j++) {
			if (arrKeyIndex[j] != -1)
				res += ENG_KEY.charAt(arrKeyIndex[j]);
		}
	}

	return res;
}

// ---------------------- file category --------------------------------

let fileCateList = writable([]); // category list from server

let showCateCrtModal = false;

let fileCateCrtObj = {
    "name":"Name Here",
    "description":"Description Here",
    "color":""
}

async function loadFileCategories(){
    const categories = await invoke('get_categories');
    fileCateList.set(categories);
}

function toggleSHowCateCrtModal(){
    showCateCrtModal = !showCateCrtModal;
}

  // 페이지 로드 시 카테고리를 가져옴
onMount(async () => {
    loadFileCategories();
});

async function fileCateCreateCategory(name:string,description:string,color:string) {
    const newCategories = await invoke('create_category', {
        name,
        description,
        color,
    });
    alert(currentTranslations.alt_file_cate_new_cate_crt);
    loadFileCategories(); //reload
    toggleSHowCateCrtModal(); //close modal
}

async function fileCateDeleteCategory(num) {
    const newCategories = await invoke('delete_category', {
        num: parseInt(num),
    });
    alert(currentTranslations.alt_file_cate_del_cate);

    // set fileviwer to empty space
    nowAtCategoryView = false;
    curFolderName = '';
    filesInCurrentFolder = [];
}

async function fileCateAddPath(num:string,path:string) {
    const newCategories = await invoke('add_to_category', {
        num: parseInt(num),
        path,
    });
    // console.log('Categories after adding path:', newCategories);
}


async function fileCateRemovePath(num:string,path:string) {
    const newCategories = await invoke('remove_from_category', {
        num: parseInt(num),
        path,
    });

    await loadFileCategories();
    if(nowAtCategoryView){
        // if category tab, reload each units when units removed
        const category = $fileCateList.find(cat => cat.num === parseInt(num));
        
        if (category) {
            filesInCurrentFolder = category.list;
        }
        
    }

}

// check fileviwer is viewing category
let nowAtCategoryView = false;

let currentCategoryInfo = writable({
    "num" : 0,
    "name":"Name Here",
    "description":"Description Here",
    "list":[]
});

// Each Category Btn
function setFileViwerToCategory(category){
    nowAtCategoryView = true
    curFolderName = '';
    currentCategoryInfo.set({
        num : category.num,
        name: category.name,
        description: category.description,
        list : category.list
    });
    filesInCurrentFolder = category.list;
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
                    <!-- <img id="movement-btn-img-left" class="movement-button" src="/arrows/thick_arrows_left.png" alt=""> -->
                    <img id="movement-btn-img-left" class="movement-button" 
                        src={currentTheme === '/src/lib/style/themes/sf_style_theme.css' ? '/arrows/cyan_theme/thick_arrows_left.png' : '/arrows/thick_arrows_left.png'} alt=""
                    >
                </button>
            </div>
            <div>
                <!-- → -->
                <button class="movement-button-wrapper-btn" on:click={() => goForward()} disabled={currentIndex >= pathHistory.length - 1}>
                    <!-- <img id="movement-btn-img-right" class="movement-button" src="/arrows/thick_arrows_right.png" alt=""> -->
                    <img id="movement-btn-img-right" class="movement-button" 
                        src={currentTheme === '/src/lib/style/themes/sf_style_theme.css' ? '/arrows/cyan_theme/thick_arrows_right.png' : '/arrows/thick_arrows_right.png'} alt=""
                    >
                </button>
            </div>
            <div>
                <!-- ↑ -->
                <button class="movement-button-wrapper-btn" on:click={() => goUp()}>
                    <!-- <img id="movement-btn-img-up" class="movement-button" src="/arrows/thick_arrows_up.png" alt=""> -->
                    <img id="movement-btn-img-up" class="movement-button" 
                        src={currentTheme === '/src/lib/style/themes/sf_style_theme.css' ? '/arrows/cyan_theme/thick_arrows_up.png' : '/arrows/thick_arrows_up.png'} alt=""
                    >
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
        <div class="search-container">
            <div class="search-autocomplete-wrapper">
                <input 
                id="searchInput" 
                class="searchbox-input" 
                type="text" 
                bind:value={inputValue} 
                on:input={searchHandleInput}
                on:keydown={cngLanghandleKeydown}
                placeholder={curFolderName} 
                />
                <div class="searchbox-placeholder">
                {#if inputValue}
                <span class="search-invisible-text">{inputValue}</span><span>{blurredText}</span>
                {/if}
                </div>
            </div>
            {#if isSearching}
            <button class="searchbox-button-wrapper" on:click={cancelSearch}>❌</button>
            {:else}
            <button id="searchButton" class="searchbox-button-wrapper" on:click={searchFilesInDirectory}>
                <img id="searchBox-button-img" class="util-button-img" 
                    src={currentTheme === '/src/lib/style/themes/sf_style_theme.css' ? '/icons/cyan_theme/magnifying_glass.png' : '/icons/magnifying_glass.png'} alt=""
                >
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
                            <img id="util-btn-img-home" class="util-button-img" 
                            src={currentTheme === '/src/lib/style/themes/sf_style_theme.css' ? '/utilbuttons/cyan_theme/util_home.png' : '/utilbuttons/util_home.png'} alt="">
                            
                        </button>
                    </div>
                    {:else if btns === "Cut"}
                    <div class="util-buttons-wrapper">
                        <button class="util-button" on:click={cutFiles}>
                            <img id="util-btn-img-cut" class="util-button-img" 
                                src={currentTheme === '/src/lib/style/themes/sf_style_theme.css' ? '/utilbuttons/cyan_theme/util_cut.png' : '/utilbuttons/util_cut.png'} alt=""
                            >
                        </button>
                        <button 
                            class="util-button"
                            on:click={() => pasteFiles(cutClipboard, curFolderName, true)} 
                            disabled={cutClipboard.length === 0}
                        >
                            <!-- <img id="util-btn-img-cut-paste" class="util-button-img" src="/utilbuttons/util_cut_paste.png" alt=""> -->
                            <img id="util-btn-img-cut-paste" class="util-button-img" 
                                src={currentTheme === '/src/lib/style/themes/sf_style_theme.css' ? '/utilbuttons/cyan_theme/util_cut_paste.png' : '/utilbuttons/util_cut_paste.png'} alt=""
                            >
                        </button>
                    </div>
                    {:else if btns === "Copy"}
                    <div class="util-buttons-wrapper">
                        <button class="util-button"  on:click={copyFiles}>
                            <!-- <img id="util-btn-img-copy" class="util-button-img" src="/utilbuttons/util_copy.png" alt=""> -->
                            <img id="util-btn-img-copy" class="util-button-img" 
                                src={currentTheme === '/src/lib/style/themes/sf_style_theme.css' ? '/utilbuttons/cyan_theme/util_copy.png' : '/utilbuttons/util_copy.png'} alt=""
                            >
                        </button>
                        <button 
                            class="util-button" 
                            on:click={() => pasteFiles(copyClipboard, curFolderName, false)} 
                            disabled={copyClipboard.length === 0}
                        >
                            <!-- <img id="util-btn-img-copy-paste" class="util-button-img" src="/utilbuttons/util_copy_paste.png" alt=""> -->
                            <img id="util-btn-img-copy-paste" class="util-button-img" 
                            src={currentTheme === '/src/lib/style/themes/sf_style_theme.css' ? '/utilbuttons/cyan_theme/util_copy_paste.png' : '/utilbuttons/util_copy_paste.png'} alt=""
                        >
                        </button>
                    </div>
                    {:else if btns === "Delete"}
                    <div class="util-buttons-wrapper">
                        <button class="util-button" on:click={moveToTrash} disabled={$selectedFiles.length === 0}>
                            <!-- <img id="util-btn-img-delete" class="util-button-img" src="/utilbuttons/util_delete.png" alt=""> -->
                        <img id="util-btn-img-delete" class="util-button-img" 
                            src={currentTheme === '/src/lib/style/themes/sf_style_theme.css' ? '/utilbuttons/cyan_theme/util_delete.png' : '/utilbuttons/util_delete.png'} alt=""
                        >
                        </button>
                    </div>
                    {:else if btns === "New_Dir"}
                    <div class="util-buttons-wrapper">
                        <button class="util-button" on:click={()=>{createItem(true,curFolderName)}}>
                        <img class="util-button-img" 
                            src={currentTheme === '/src/lib/style/themes/sf_style_theme.css' ? '/utilbuttons/cyan_theme/new_dir.png' : '/utilbuttons/new_dir.png'} alt=""
                        >
                        </button>
                    </div>
                    {:else if btns === "New_File"}
                    <div class="util-buttons-wrapper">
                        <button class="util-button" on:click={()=>{createItem(false,curFolderName)}}>
                        <img class="util-button-img" 
                            src={currentTheme === '/src/lib/style/themes/sf_style_theme.css' ? '/utilbuttons/cyan_theme/new_file.png' : '/utilbuttons/new_file.png'} alt=""
                        >
                        </button>
                    </div>
                    {/if}
                </div>
            {/each}
        </div>

        <!-- settings -->
        <div class="settings-icon-wrapper" on:click={toggleSettings}>
            <!-- ⚙️ -->
             <!-- <img id="settings-icon-img" class="gear-image" src="/icons/gear.png" alt=""> -->
             <img id="settings-icon-img" class="gear-image" 
                src={currentTheme === '/src/lib/style/themes/sf_style_theme.css' ? '/icons/cyan_theme/gear.png' : '/icons/gear.png'} alt=""
            >
        </div>
    </div>
    
    <!-- Just for height set -->
    <div style="margin-bottom: 150px;"></div>

    <!-- Copy / Paste List -->
    {#if copyClipboard.length > 0}
        <div class="copy-items-container" style="max-height: {isCopyExpanded ? 'none' : '30px'};">
            <div class="copy-banner">
                <span class="copy-banner-text">{currentTranslations.copyClipboardText}</span>
            </div>
            <div class="copy-item-list" 
                style="max-height: {isCopyExpanded ? 'none' : '30px'}; 
                overflow: {isCopyExpanded ? 'visible' : 'hidden'};"
            >
            {#each copyClipboard as file}
                <div class="copy-item">
                    <div class="copy-item-file-name">{getFileName(file)}</div>
                    <button class="copy-item-cancle-btn" value={file} on:click={rmCopyClipFile}>❌</button>
                </div>
            {/each}
            </div>
            <div class="copy-list-empty-wrapper">
                <button class="copy-list-empty-btn" on:click={clearCopyClip}>
                    {currentTranslations.clearCutClip}
                </button>
            </div>
            <div class="copy-list-expand-wrapper">
                <button class="copy-list-expand-btn" on:click={toggleCopyList}>
                    {isCopyExpanded ? '▲' : '▼'}
                </button>
            </div>
        </div>
    {/if}
    {#if copyClipboard.length > 0 && cutClipboard.length > 0}
    <hr>
    {/if}
    {#if cutClipboard.length > 0}
        <div class="cut-items-container" style="max-height: {isCutExpanded ? 'none' : '30px'};">
            <div class="cut-banner">
                <span class="cut-banner-text">{currentTranslations.cutClipboardText}</span>
            </div>
            <div class="cut-item-list" 
                style="max-height: {isCutExpanded ? 'none' : '30px'}; 
                overflow: {isCutExpanded ? 'visible' : 'hidden'};"
            >
            {#each cutClipboard as file}
                <div class="cut-item">
                    <div class="cut-item-file-name">{getFileName(file)}</div>
                    <button class="cut-item-cancle-btn" value={file} on:click={rmCutClipFile}>❌</button>
                </div>
            {/each}
            </div>
            <div class="cut-list-empty-wrapper">
                <button class="cut-list-empty-btn" on:click={clearCutClip}>
                    {currentTranslations.clearCopyClip}
                </button>
            </div>
            <div class="cut-list-expand-wrapper">
                <button class="cut-list-expand-btn" on:click={toggleCutList}>
                    {isCutExpanded ? '▲' : '▼'}
                </button>
            </div>
        </div>
    {/if}
    
    
    <div class="content-wrapper {viewMode === 'dual' ? 'dual-view' : ''}">
        <!-- Directory List -->
        <aside class="sidebar" id="sidebar">
            {#each Object.keys($drives) as drive}
                <Folder path={drive} name={drive} items={$drives[drive]} currentTheme={currentTheme} on:folderSelected={handleFolderSelected}/>
            {/each}
            <hr>
            <div>{currentTranslations.file_category_title}</div>
            <button class="dirview-category-add-cate-btn" on:click={toggleSHowCateCrtModal} >{currentTranslations.file_category_add_cate_btn}</button>
            {#if $fileCateList.length > 0}
                {#each $fileCateList as category}
                    <div>
                        <button 
                            class="dirview-category-button" 
                            style="border-color: {category.color};"
                            on:click={setFileViwerToCategory(category)}
                        >
                            {category.name}
                        </button>
                    </div>
                {/each}
            {/if}
            <hr>
            <DiscInfo/>
        </aside>


        <!-- division bar between sidebar & file-viewer -->
        <div class="resizer" id="resizer"></div>

        <!-- file viewer -->
        <div class="file-viewer-container">
        {#if nowAtCategoryView}
            <div class="file-viewer-category-desc-container">
                <div>
                    <h4>{currentTranslations.file_category_file_viewer_name}</h4>
                    <p>{$currentCategoryInfo.name}</p>
                </div>
                <div>
                    <h4>{currentTranslations.file_category_file_viewer_desc}</h4>
                    <p>{$currentCategoryInfo.description}</p>
                </div>
                <div>
                    <button 
                        class="file-viewer-del-cate-btn"
                        on:click={()=>{fileCateDeleteCategory($currentCategoryInfo.num);loadFileCategories();}}
                    >
                        {currentTranslations.file_category_delete_cate_btn}
                    </button>
                </div>
            </div>
        {/if}
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
                            {#if $editingFile === file}
                                <input
                                    type="text"
                                    bind:value={$newName}
                                    on:focusout={() => renameFile(file, getParentPath(file))}
                                    on:keydown={(e) => e.key === 'Enter' && renameFile(file, getParentPath(file))}
                                    class="file-name-input"
                                    autofocus
                                />
                            {:else}
                                <span class="file-name">{getFileName(file)}</span>
                            {/if}
                        </div>
                    {/each}
                {:else if selectedDriveLeft && selectedFolderLeft}
                    <p>{currentTranslations.no_folder}</p>
                {:else}
                    <p>{currentTranslations.sel_folder}</p>
                {/if}
            </div>
        </div>
        
    </div>

    <!------------------------ Setting Modal ------------------------>
    {#if showSettings}
        <div 
        id="modal-settings" 
        class="settings-modal"
        bind:this={modal_set_modal} 
        on:mousedown={modal_set_startDrag}
        >
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
                            min="80"
                            max="120"
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
                        &nbsp;<label for="">{currentTranslations.util_delete}</label><input type="checkbox" checked={isChecked("Delete")} on:change="{(e) => toggleItem('Delete', e.target.checked)}">
                        <br>
                        &nbsp;<label for="">{currentTranslations.util_new_dir}</label><input type="checkbox" checked={isChecked("New_Dir")} on:change="{(e) => toggleItem('New_Dir', e.target.checked)}">
                        &nbsp;<label for="">{currentTranslations.util_new_file}</label><input type="checkbox" checked={isChecked("New_File")} on:change="{(e) => toggleItem('New_File', e.target.checked)}">
                        <br/>
                    <!-- Modal : Search Tab -->
                    {:else if activeTab === "search"}
                        <h3>{currentTranslations.search}</h3>
                        <div class="modal-sch-wrapper">

                            <div class="modal-sch-autocomplete-wrapper">
                                <h3>{currentTranslations.modal_sch_autocomplete_title}</h3>
                                <label>
                                    <input type="checkbox" on:change={isAutocompleteCheckToggle} bind:checked={isAutocompleteCheck}>
                                    {currentTranslations.modal_sch_autocomplete_check}
                                </label>
                            </div>

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
                                    <select id="thread_pool" class="modal-thread-pool-selectbox" bind:value={searchValObj.threadPoolNum}>
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
                                                 <div class="modal-file-size-row">
                                                    <div class="modal-file-size-txt">
                                                        {currentTranslations.modal_sch_advanced_max}
                                                     </div>
                                                     <div class="modal-file-size-numbox">
                                                        <label>
                                                            <input class="modal-file-size-input" type="number" bind:value={fileMaxRawVal} on:change={fileSizeCalc}>
                                                        </label>
                                                     </div>
                                                    <div>
                                                        <select class="modal-set-file-sch-size-unit" bind:value={fileMaxUnit} on:change={fileSizeCalc}>
                                                            <option value="B">B</option>
                                                            <option value="KB">KB</option>
                                                            <option value="MB">MB</option>
                                                            <option value="GB">GB</option>
                                                        </select>
                                                    </div>
                                                 </div>
                                                 <div class="modal-file-size-row">
                                                    <div class="modal-file-size-txt">
                                                        {currentTranslations.modal_sch_advanced_min}
                                                    </div>
                                                    <div class="modal-file-size-numbox">
                                                        <label>
                                                            <input class="modal-file-size-input" type="number" bind:value={fileMinRawVal} on:change={fileSizeCalc}>
                                                        </label>
                                                    </div>
                                                    <div>
                                                        <select class="modal-set-file-sch-size-unit" bind:value={fileMinUnit} on:change={fileSizeCalc}>
                                                            <option value="B">B</option>
                                                            <option value="KB">KB</option>
                                                            <option value="MB">MB</option>
                                                            <option value="GB">GB</option>
                                                        </select>
                                                    </div>
                                                 </div>
                                            </div>
                                            {/if}
                                            <br>
                                            <label>
                                                {currentTranslations.modal_sch_advanced_file_property_type}
                                                <input type="checkbox" on:change={isFileTypeChkToggle} bind:checked={isFileTypeChk}>
                                            </label>
                                            {#if isFileTypeChk}
                                                <input class="modal-file-type-input" type="text" bind:value={searchValObj.fileTypeList} placeholder=".txt .pdf .docx ....">
                                            {/if}
                                            <br>
                                            <label>
                                                {currentTranslations.modal_sch_advanced_file_property_creation}
                                                <input type="checkbox" on:change={isFileCrtDateChkToggle} bind:checked={isFileCrtDateChk}>
                                            </label>
                                            {#if isFileCrtDateChk}
                                            <div class="modal-crt-date-row">
                                                <!-- Creation Date -->
                                                 <div class="modal-crt-date-start-row">
                                                    <div class="modal-crt-date-txt">
                                                        {currentTranslations.modal_sch_advanced_date_start}
                                                    </div>
                                                    <div>
                                                        <label>
                                                            <input class="date-input" type="date" bind:value={searchValObj.crtStart}>
                                                        </label>
                                                    </div>
                                                 </div>
                                                 <div class="modal-crt-date-end-row">
                                                    <div class="modal-crt-date-txt">
                                                        {currentTranslations.modal_sch_advanced_date_end}
                                                    </div>
                                                    <div>
                                                        <label>
                                                            <input class="date-input" type="date" bind:value={searchValObj.crtEnd}>
                                                        </label>
                                                    </div>
                                                 </div>                                           
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
                                                 <div class="modal-modi-date-start-row">
                                                    <div class="modal-modi-date-txt">
                                                        {currentTranslations.modal_sch_advanced_date_start}
                                                    </div>
                                                    <div>
                                                        <label>
                                                            <input class="date-input" type="date" bind:value={searchValObj.modiStart}>
                                                        </label>
                                                    </div>
                                                 </div>
                                                 <div class="modal-modi-date-end-row">
                                                    <div class="modal-modi-date-txt">
                                                        {currentTranslations.modal_sch_advanced_date_end}
                                                    </div>
                                                    <div>
                                                        <label>
                                                            <input class="date-input" type="date" bind:value={searchValObj.modiEnd}>
                                                        </label>
                                                    </div>
                                                 </div>
                                            </div>
                                            {/if}
                                            <br>
                                            <label>
                                                {currentTranslations.modal_sch_advanced_file_property_owner}
                                                <input type="checkbox" on:change={isFileOwnerChkToggle} bind:checked={isFileOwnerChk}>
                                            </label>
                                            {#if isFileOwnerChk}
                                                <input class="modal-file-owner-input" type="text" bind:value={searchValObj.ownerName} placeholder="steve...">
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
                        <!-- <br>
                        <button on:click={()=>{console.log(searchValObj)}}>DEBUG</button> -->
                        <br>
                        <button class="modal-sch-val-slots" on:click={advModalToggle}>
                            {currentTranslations.modal_sch_advanced_open_val_slots}
                        </button>
                        <br>
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
                                    <input class="adv-slot-name-btn" type="text" bind:value={slot.name} placeholder="{currentTranslations.modal_sch_advanced_slots_name_ph}">
                                  </label>
                                </div>
                                <button class="adv-slot-save-btn" on:click={() => saveSlot(index)}>{currentTranslations.modal_sch_advanced_slots_save}</button>
                                <button class="adv-slot-load-btn" on:click={() => loadSlot(index)} disabled={!slot.hasValue}>{currentTranslations.modal_sch_advanced_slots_load}</button>
                                <button class="adv-slot-reset-btn" on:click={() => deleteSlot(index)} disabled={!slot.hasValue}>{currentTranslations.modal_sch_advanced_slots_reset}</button>
                              </div>
                            {/each}
                        </div>
                        {/if}
                    {/if}
                </div>
                <button class="close-modal" on:click={toggleSettings}>{currentTranslations.modal_close}</button>
            </div>
        </div>
    {/if}

    <!-- ------------- Mouse right click file viewer option -------------------- -->
    {#if $right_click_visible}
        <div
        class="file-item-menu"
        style="top: {$right_click_position.y}px; left: {$right_click_position.x}px;"
        >
            <div class="file-item-btn-wrapper">
                <button on:click={rightCopyFiles} class="file-item-btn">{currentTranslations.file_item_right_set_copy}</button>
            </div>
            <div class="file-item-btn-wrapper">
                <button on:click={rightCutFiles} class="file-item-btn">{currentTranslations.file_item_right_set_cut}</button>
            </div>
            <div class="file-item-btn-wrapper">
                <button on:click={handleRenameClick} class="file-item-btn">{currentTranslations.file_item_right_set_rename}</button>
            </div>
            <div class="file-item-btn-wrapper">
                <button on:click={filePropModalOpen} class="file-item-btn">{currentTranslations.file_item_right_set_metadata}</button>
            </div>
            {#each $fileCateList as category}
                {#if $right_click_clip.length > 0 && category.list.includes($right_click_clip[0])}
                    <div class="file-item-btn-wrapper">
                        <button 
                            class="file-item-btn" 
                            style="color : {category.color}"
                            on:click={()=>{fileCateRemovePath(category.num,$right_click_clip[0]);}}
                        >
                            {currentTranslations.file_cate_remove_list} {category.name}
                        </button>
                    </div>
                {:else}
                    <div class="file-item-btn-wrapper">
                        <button 
                            class="file-item-btn" 
                            style="color : {category.color}" 
                            on:click={()=>{fileCateAddPath(category.num,$right_click_clip[0]);loadFileCategories();}}
                        >
                            {currentTranslations.file_cate_add_list} {category.name}
                        </button>
                    </div>
                {/if}
            {/each}
        </div>
    {/if}

    <!-- File Right-click Properties modal -->
    {#if $filePropModalToggle}
    <div class="file-props-modal" on:click={filePropModalClose} style="left: {$filePropsModalPos.x}px; top: {$filePropsModalPos.y}px">
        <div class="file-props-content">
            <div>
                <h2>{currentTranslations.file_prop_title}</h2>
            </div>
            {#if $fileMetadata}
            <div>
                <p>{currentTranslations.file_prop_name}: {$fileMetadata.file_name}</p>
                <p>{currentTranslations.file_prop_size}: {$fileMetadata.file_size}</p>
                <p>{currentTranslations.file_prop_date}: {new Date($fileMetadata.last_modified * 1000).toLocaleString()}</p>
                <!-- <p>{currentTranslations.file_prop_type}: {$fileMetadata.file_type}</p> -->
            </div>
            {:else}
                <p>{currentTranslations.file_prop_loading}</p>
            {/if}
            <div>
                <button class="file-prop-modal-btn" on:click={filePropModalClose}>{currentTranslations.file_prop_close}</button>
            </div>
        </div>
    </div>
    {/if}

    <!-- File Category Creation Modal -->
     {#if showCateCrtModal}
        <div class="category-creation-modal-wrapper">
            <div class="category-creation-each-row">
                <h3>{currentTranslations.file_category_creation_title}</h3>
            </div>
            <div class="category-creation-each-row">
                <p>{currentTranslations.file_category_crt_name_txt}</p>
                <input type="text" bind:value={fileCateCrtObj.name}>
            </div>
            <div class="category-creation-each-row">
                <p>{currentTranslations.file_category_crt_desp_txt}</p>
                <textarea class="category-creation-desp-area" bind:value={fileCateCrtObj.description}/>
            </div>
            <div class="category-creation-each-row">
                <p>{currentTranslations.file_category_crt_sel_color_txt}</p>
                <input type="color" class="category-creation-color-btn" bind:value={fileCateCrtObj.color}>
            </div>
            <div class="category-creation-each-row">
                <button 
                    class="category-creation-btn" 
                    on:click={()=>{fileCateCreateCategory(fileCateCrtObj.name,fileCateCrtObj.description,fileCateCrtObj.color)}}
                >
                    {currentTranslations.file_category_crt_creation_btn}
                </button>
            </div>
            <div class="category-creation-each-row">
                <button on:click={toggleSHowCateCrtModal} class="category-creation-btn">
                    {currentTranslations.file_category_crt_close}
                </button>
            </div>
        </div>
     {/if}

</div>