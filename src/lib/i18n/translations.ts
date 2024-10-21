export const translations = {
    en: {
        settings: 'Settings',
        interface: 'Interface',
        resize: 'Resize',
        themes: 'Themes',
        language: 'Language',
        utils:'Utils',
        search:'Search Settings',
        close: 'Close',

        sel_folder:'Select Folder',
        no_folder:'Empty Folder',

        file_icon_size : 'File Icon Size',

        util_apply_button : 'Apply',
        util_home : 'Home',
        util_cut : 'Cut',
        util_copy : 'Copy',
        util_paste : 'Paste',
        util_delete: 'Delete',
        util_new_dir: 'New Directory',
        util_new_file: 'New File',

        copyClipboardText : 'Copy List',
        cutClipboardText:'Cut List',

        clearCopyClip : 'Clear',
        clearCutClip : 'Clear',

        pasteErr : 'Error has occurred during Paste : ',
        utilshotcutErr : 'Error has occurred during using shortcut : ',

        modal_close : 'Close',

        interface_set : 'Interface Setting',
        inter_one_panel: 'One Panel',
        inter_two_panel: 'Two Panel',

        default_theme:'default Theme',
        retro_theme:'XP style Theme',
        sf_style_theme:'SF style Theme',
        linux_theme:'Linux style Theme',

        paste_fail_msg : 'Problem has been occurred during copy/cut : ',

        mk_new_item_success : 'New Folder or File has been created. => ',
        mk_new_item_fail : 'New Folder or File has failed creation. => ',

        modal_sch_basic_title : 'Basic Features',
        modal_sch_basic_async : 'Async (Thread pools automatically set by CPU Core)',
        modal_sch_basic_filename : 'Search depends on file`s name',
        modal_sch_basic_realtime: 'Realtime show',
        modal_sch_basic_cache:'Use Cache',

        modal_sch_advanced_title:'Advanced Options',

        modal_thread_pool_title:'Thread Pool Settings',
        modal_thread_pool_use_box:'Modify Number of Thread Pools',
        modal_sch_advanced_thread_pool_set:'Async Thread pool custom set',
        modal_sch_advanced_thread_pool_size:'Thread pool size',
        modal_sch_advanced_thread_pool_size_default:'Default',

        modal_sch_advanced_sch_target:'Search Target',
        modal_sch_advanced_sch_target_fileNfolder:'Both File & Folder',
        modal_sch_advanced_sch_target_file:'Only File',
        modal_sch_advanced_sch_target_folder:'Only Folder',

        modal_sch_advanced_sch_options_title:'Search Options',
        modal_sch_advanced_file_content_sch:'File Content Search',

        modal_sch_advanced_date_start:'Start',
        modal_sch_advanced_date_end:'End',

        modal_sch_advanced_max:'Max',
        modal_sch_advanced_min:'Min',

        modal_sch_advanced_file_property:'File Property',
        modal_sch_advanced_file_property_size:'File Size',
        modal_sch_advanced_file_property_type:'File Type',
        modal_sch_advanced_file_property_modified:'Modified Date',
        modal_sch_advanced_file_property_creation:'Creation Date',
        modal_sch_advanced_file_property_owner:'File Owner',
         
        modal_sch_advanced_symbolic_link:'Search Symbolic Link',

        modal_sch_advanced_sch_methods:'Advanced Search Options',
        modal_sch_advanced_default:'Default(None)',
        modal_sch_advanced_regex:'Use Regex',
        modal_sch_advanced_fuzzy_dame:'Fuzzy Matching : Damerau Levenshtein',
        modal_sch_advanced_fuzzy_jac:'Fuzzy Matching : Jaccard Similarity',

        modal_sch_advanced_log_title:'Log Options',
        modal_sch_advanced_log_check:'Save Log',

        modal_sch_advanced_open_val_slots:'Value Slots',
        modal_sch_advanced_load_values:'Load Values',
        modal_sch_advanced_save_values:'Save Values',
        modal_sch_advanced_delete_values:'Delete Values',

        modal_sch_advanced_slots_slotTxt:'Slot',
        modal_sch_advanced_slots_name_ph:'Slot Name',
        modal_sch_advanced_slots_save:'Save',
        modal_sch_advanced_slots_load:'Load',
        modal_sch_advanced_slots_reset:'Reset',

        adv_slot_empty_name_alert:'Slot name cannot be empty. Please provide a valid name.',

        alt_modal_valid_empty_type_list: 'Enter the extension list.',
        alt_modal_valid_empty_owner_name: 'Enter the owner name.',
        alt_modal_valid_file_size_no_value: 'Enter both min & max size.',
        alt_modal_valid_file_size_must_int: 'Size must be an integer.',
        alt_modal_valid_file_size_bigger_than_0: 'Size must be ≥ 0 (max ≥ 1).',
        alt_modal_valid_file_size_min_bigger_than_max: 'Min size exceeds max size.',
        alt_modal_valid_file_size_lower_than_100GB: 'Size must be ≤ 100 GB.',
        alt_modal_valid_date_empty_val: 'Enter all dates.',
        alt_modal_valid_date_invalid_type: 'Invalid date format.',
        alt_modal_valid_date_start_faster_than_end: 'Start date is after end date.',
        alt_modal_valid_date_before_today: 'Dates must be today or earlier.'

    },
    ko: {
        settings: '설정',
        interface: '인터페이스',
        resize: '화면 크기 조절',
        themes: '테마선택',
        language: '언어선택',
        utils:'기능',
        search:'검색설정',
        close: '닫기',
        
        sel_folder:'폴더를 선택하세요',
        no_folder:'빈 폴더입니다',

        file_icon_size : '파일 아이콘 사이즈',

        util_apply_button : '적용',
        util_home : '홈',
        util_cut : '잘라내기',
        util_copy : '복사하기',
        util_paste : '붙여넣기',
        util_delete: '삭제하기',
        util_new_dir: '새 폴더',
        util_new_file: '새 파일',

        copyClipboardText : '복사',
        cutClipboardText:'잘라내기',

        clearCopyClip : '비우기',
        clearCutClip : '비우기',

        pasteErr : '붙여넣기 작업 중 오류발생 : ',
        utilshotcutErr : '단축키 오류 : ',

        modal_close : '닫기',

        interface_set : '인터페이스 설정',
        inter_one_panel: '패널1개',
        inter_two_panel: '패널2개',

        default_theme:'기본 테마',
        retro_theme:'XP 스타일 테마',
        sf_style_theme:'SF 스타일 테마',
        linux_theme:'리눅스 스타일 테마',

        paste_fail_msg : '복사/잘라내기 작업 중 문제 발생 : ',

        mk_new_item_success : '새 파일 또는 폴더를 생성하였습니다. => ',
        mk_new_item_fail : '새 파일 또는 폴더 생성에 실패했습니다. => ',

        modal_sch_basic_title : '기본 적용',
        modal_sch_basic_async : '비동기 처리(코어 수 기반 스레드 풀 자동 설정)',
        modal_sch_basic_filename : '파일명 기반 탐색',
        modal_sch_basic_realtime: '실시간 반영',
        modal_sch_basic_cache:'캐시 이용',

        modal_sch_advanced_title:'고급 옵션',

        modal_thread_pool_title:'스레드 풀 설정',
        modal_thread_pool_use_box:'사용자 지정 스레드 풀 지정',
        modal_sch_advanced_thread_pool_set:'비동기 처리 (스레드 풀 크기 사용자 설정)',
        modal_sch_advanced_thread_pool_size:'스레드 풀 크기',
        modal_sch_advanced_thread_pool_size_default:'자동설정값',

        modal_sch_advanced_sch_target:'탐색 대상',
        modal_sch_advanced_sch_target_fileNfolder:'파일/폴더 모두 탐색',
        modal_sch_advanced_sch_target_file:'파일만 탐색',
        modal_sch_advanced_sch_target_folder:'폴더만 탐색',

        modal_sch_advanced_sch_options_title:'탐색 옵션',
        modal_sch_advanced_file_content_sch:'파일 내용 탐색',
        
        modal_sch_advanced_date_start:'시작',
        modal_sch_advanced_date_end:'끝',

        modal_sch_advanced_max:'최대',
        modal_sch_advanced_min:'최소',

        modal_sch_advanced_file_property:'파일 속성(메타데이터) 탐색',
        modal_sch_advanced_file_property_size:'파일 크기',
        modal_sch_advanced_file_property_type:'파일 유형(확장자)',
        modal_sch_advanced_file_property_modified:'수정일',
        modal_sch_advanced_file_property_creation:'생성일',
        modal_sch_advanced_file_property_owner:'파일 소유자',

        modal_sch_advanced_symbolic_link:'심볼릭 링크 탐색',

        modal_sch_advanced_sch_methods:'고급 탐색 유형',
        modal_sch_advanced_default:'기본탐색',
        modal_sch_advanced_regex:'정규식 이용',
        modal_sch_advanced_fuzzy_dame:'퍼지 매칭법 : 다메라우-레벤슈타인 거리',
        modal_sch_advanced_fuzzy_jac:'퍼지 매칭법 : 자카드 유사도',

        modal_sch_advanced_log_title:'검색 로그 설정',
        modal_sch_advanced_log_check:'로그 기록',

        modal_sch_advanced_open_val_slots:'설정값 슬롯',
        modal_sch_advanced_load_values:'불러오기',
        modal_sch_advanced_save_values:'저장하기',
        modal_sch_advanced_delete_values:'삭제하기',

        modal_sch_advanced_slots_slotTxt:'슬롯',
        modal_sch_advanced_slots_name_ph:'슬롯 이름',
        modal_sch_advanced_slots_save:'저장',
        modal_sch_advanced_slots_load:'불러오기',
        modal_sch_advanced_slots_reset:'초기화',

        adv_slot_empty_name_alert:'슬롯에 저장할 이름을 입력하세요. 빈 문자열은 허용되지 않습니다.',

        alt_modal_valid_empty_type_list : '확장자 리스트를 입력하세요.',
        alt_modal_valid_empty_owner_name : '소유자명을 입력하세요.',
        alt_modal_valid_file_size_no_value : '최대 & 최소값을 모두 입력하세요.',
        alt_modal_valid_file_size_must_int : '파일 크기는 정수여야 합니다.',
        alt_modal_valid_file_size_bigger_than_0 : '입력값 크기는 0 이상이어야 합니다. (최대값은 1 이상이어야 합니다)',
        alt_modal_valid_file_size_min_bigger_than_max : '최소 파일 크기가 최대 파일 크기보다 큽니다.',
        alt_modal_valid_file_size_lower_than_100GB : '파일 크기는 100 GB를 초과할 수 없습니다.',
        alt_modal_valid_date_empty_val : '모든 날짜를 입력해야 합니다.',
        alt_modal_valid_date_invalid_type : '날짜 형식이 잘못되었습니다.',
        alt_modal_valid_date_start_faster_than_end : '시작 날짜가 끝 날짜보다 이후입니다.',
        alt_modal_valid_date_before_today : '시작과 끝 날짜는 오늘 이전이어야 합니다.'








    },
};
