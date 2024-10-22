<div data-tauri-drag-region class="title-bar">
    <!-- 
    working : data-tauri-drag-region
    not working : javascript - drag code
    -->
    <span class="title">PathFinder</span>
    <div class="window-controls">
      <button on:click={minimizeWindow}>_</button>
      <button on:click={maximizeWindow}>[ ]</button>
      <button on:click={closeWindow}>X</button>
    </div>
</div>
  
<script lang="ts">
    import { appWindow } from "@tauri-apps/api/window";

    function minimizeWindow() {
        appWindow.minimize();
    }

    function maximizeWindow() {
        appWindow.toggleMaximize();
    }

    function closeWindow() {
        appWindow.close();
    }
</script>
<style>
.title{
    font-size: large;
}  
.title-bar {

    position: fixed;
    width: 98.2%;

    height: 30px;
    top: 0;

    display: flex;
    justify-content: space-between;
    /* background-color: #333; */
    /* background-color: transparent; */
    color: white;
    padding: 10px;

    z-index: 1000;
}


.window-controls button {
    background: none;
    border: none;
    /* color: white; */
    padding: 5px;
    cursor: pointer;

    /* -webkit-app-region: no-drag; */
}
</style>
  