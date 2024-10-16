<script lang="ts">
  import { writable } from 'svelte/store';

  // 상태 관리
  let isDragging = writable(false);  // 드래그 상태
  let startX = 0, startY = 0;        // 드래그 시작 좌표
  let endX = 0, endY = 0;            // 드래그 끝 좌표
  let selectedFiles = writable<string[]>([]);  // 선택된 파일 경로 저장

  let filesInCurrentFolder = ["a","b","c","d","e"]

  const dragThreshold = 5;  // 최소 이동 거리 (5px 이상만 드래그로 처리)
  let rectStyle = writable('');  // 직사각형 CSS 스타일
  // let isClick = true;  // 클릭 여부 추적

  // 마우스 다운: 클릭/드래그 시작 지점 초기화
  function handleMouseDown(event: MouseEvent) {
    event.preventDefault();  // 기본 브라우저 동작 방지
    clearSelection();

    // 클릭 및 드래그 초기화
    startX = event.clientX;
    startY = event.clientY;
    endX = startX;
    endY = startY;
    // isClick = true;  // 초기 상태는 클릭으로 설정
    
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
      // isClick = false;  // 클릭이 아닌 드래그로 전환
      endX = event.clientX;
      endY = event.clientY;
      updateRectStyle();  // 직사각형 스타일 업데이트
    }
  }

  // 마우스 업: 클릭 또는 드래그 종료 처리
  function handleMouseUp(event: MouseEvent) {
    // if (isClick) {
    //   console.log('Click detected, no drag');
    //   isDragging.set(false);  // 드래그 상태 해제
    //   return;  // 직사각형 없이 종료
    // }
// ss
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

  selectedFiles.subscribe((files) => {
  console.log('Selected files:', files);
});
</script>


<style>
  .selection-rect {
    position: absolute;
    background-color: rgba(0, 123, 255, 0.3);
    border: 1px dashed rgba(0, 123, 255, 0.8);
    pointer-events: none; /* 직사각형 클릭 방지 */
    user-select: none; /* 텍스트 선택 방지 */
  }

  .file-item {
    position: relative;
    display: inline-block;
    margin: 5px;
    /* cursor: pointer; */
    user-select: none; /* 텍스트 선택 방지 */
    /* transition: border 0.3s ease; */

    width: 100px; height: 100px; border: 1px dashed red;
  }

  .file-item .selected {
    border: 2px solid #007bff; /* 선택된 파일 강조 */
    background-color: rgba(0, 123, 255, 0.1); /* 선택된 파일 배경 강조 */
  }
</style>

<!-- 드래그 영역 -->
<div 
  role="application"
  on:mousedown={handleMouseDown} 
  on:mousemove={handleMouseMove} 
  on:mouseup={handleMouseUp}
  style="position: relative; width: 100%; height: 100vh; border: 1px solid black;"
>
  <!-- 드래그 중 보이는 직사각형 -->
  {#if $isDragging}
    <div class="selection-rect" style={$rectStyle}></div>
  {/if}

  <!-- 파일 목록 -->
  {#each filesInCurrentFolder as file}
    <div
      class="file-item"
      data-file-path={file}
    >
      <span class="file-name">{file}</span>
    </div>
  {/each}
</div>

  
<a href="/frontTest">Go to previous page</a>
<!-- <button on:click={()=>{console.log(selectedFiles);}}>DEBUG</button> -->