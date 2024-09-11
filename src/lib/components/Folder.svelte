<script lang="ts">

  export let path: string;
  export let name: string = "";
  export let items: { [key: string]: string[] | null } | null = null;

  import { isDirectory, listFilesInDirectory} from "$lib/api";
  import Folder from "./Folder.svelte";

  import '$lib/style/folder_svelte.css';

  import { createEventDispatcher } from 'svelte';
  
  const dispatch = createEventDispatcher();


  let expanded = false; // 현재 폴더가 확장되었는지 여부를 저장

  let curFolderName: string = '';

  async function handleFolderClick() {
    if (expanded) {
      // 폴더를 접기
      expanded = false;
    } else {
      // 폴더를 확장
      const dirCheck = await isDirectory(path);
      if (dirCheck) {
        const contents = await listFilesInDirectory(path);

        const folders: { [key: string]: string[] | null } = {};
        for (const item of contents) {
          if (await isDirectory(item)) {
            folders[item] = null;
          }
        }

        items = folders;
        expanded = true;
      }
    }
    curFolderName = path;
    dispatch('folderSelected', path);
    // console.log('path: '+curFolderName);
  }

  function getFolderName(fullPath: string): string {
    return fullPath.split(/[/\\]/).pop() || "";
    // return fullPath.split(/[/\\]/).pop() || "";
  }
</script>

<div class="folder">
  <div class="clickable" on:click={handleFolderClick}>
    <span class="icon">{expanded ? "▼" : "▶️"}</span>
    {name === "C:\\" || name === "D:\\" ? name : getFolderName(name)}
  </div>
  {#if expanded && items}
    <ul class="folder-contents">
      {#each Object.entries(items || {}) as [subPath, subItems]}
        <li class="folder-item">
          <Folder
            path={subPath}
            name={getFolderName(subPath)}
            items={subItems}
            on:folderSelected={event => dispatch('folderSelected', event.detail)}
          />
        </li>
      {/each}
    </ul>
  {/if}
</div>

<style>
  
</style>
