<script>
import { onMount } from 'svelte';

let modal;

onMount(() => {
  modal = document.getElementById('modal');
  if (modal) {
    const header = modal.querySelector('.modal-header');
    let isDragging = false;
    let offsetX, offsetY;

    header.addEventListener('mousedown', (e) => {
      isDragging = true;
      offsetX = e.clientX - modal.offsetLeft;
      offsetY = e.clientY - modal.offsetTop;
      document.body.style.cursor = 'grabbing';
    });

    document.addEventListener('mousemove', (e) => {
      if (isDragging) {
        let x = e.clientX - offsetX;
        let y = e.clientY - offsetY;

        const maxX = window.innerWidth - modal.offsetWidth;
        const maxY = window.innerHeight - modal.offsetHeight;

        x = Math.max(0, Math.min(x, maxX));
        y = Math.max(0, Math.min(y, maxY));

        modal.style.left = `${x}px`;
        modal.style.top = `${y}px`;
      }
    });

    document.addEventListener('mouseup', () => {
      isDragging = false;
      document.body.style.cursor = 'default';
    });
  } else {
    console.error('Modal element not found');
  }
});
</script>
<style>
  .modal {
  position: fixed;
  top: 50px;
  left: 50px;
  width: 300px;
  height: 200px;
  background-color: white;
  border: 1px solid black;
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
  z-index: 1000;
  cursor: grab; /* 드래그할 수 있는 커서 */
}

.modal-header {
  background-color: #f1f1f1;
  padding: 10px;
  cursor: move; /* 모달 상단에서 드래그 가능하게 */
}

.modal-content {
  padding: 20px;
}

</style>
<div id="modal" class="modal">
  <div class="modal-header">
    <span>모달 창</span>
  </div>
  <div class="modal-content">
    <p>모달 내용입니다.</p>
  </div>
</div>
