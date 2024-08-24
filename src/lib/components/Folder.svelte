<script lang="ts">
  export let path: string;
  export let name: string = '';
  export let items: { [key: string]: string[] | null } | null = null;

  import { get } from 'svelte/store';
  import { drives } from '$lib/store';
  import { isDirectory, listFilesInDirectory } from '$lib/api';
  import type { DriveState } from '$lib/store';

  import Folder from './Folder.svelte';

  async function handleFolderClick() {
const currentDrives = get(drives) as DriveState;

if (items) {
  // 이미 펼쳐진 폴더를 접기
  currentDrives[path] = null;
} else {
  const dirCheck = await isDirectory(path);
  if (dirCheck) {
    const contents = await listFilesInDirectory(path);

    const folders: { [key: string]: string[] | null } = {};
    for (const item of contents) {
      if (await isDirectory(item)) {
        folders[item] = null;
      }
    }

    currentDrives[path] = folders;
  }
}

drives.set({ ...currentDrives }); // 상태를 명확히 업데이트
}

  function getFolderName(fullPath: string): string {
    return fullPath.split(/[/\\]/).pop() || '';
  }

  function convertToFolderItems(items: string[] | null): { [key: string]: string[] | null } | null {
    if (!items) return null;
    
    const folderItems: { [key: string]: string[] | null } = {};
    items.forEach(item => {
      folderItems[item] = null;
    });
    
    return folderItems;
  }
</script>

<style>
  .clickable {
    cursor: pointer;
    margin-left: 20px;
    display: flex;
    align-items: center;

    border:1px solid red;
  }

  .icon {
    margin-right: 5px;
  }

  .folder-item {
    margin-left: 20px;

    border:1px solid blue;
  }

  .forDebug{
      border: 2px solid yellow;
  }
</style>

<div class="forDebug">
  <div class="clickable" on:click={handleFolderClick}>
    <span class="icon">{items ? '▼' : '▶️'}</span>
    {name === 'C:\\' || name === 'D:\\' ? name : getFolderName(name)}
  </div>
  {#if items}
    <ul>
      {#each Object.entries(items || {}) as [subPath, subItems]}
          <li class="folder-item">
              <Folder 
                  path={subPath} 
                  name={getFolderName(subPath)} 
                  items={convertToFolderItems(subItems)} 
              />
          </li>
      {/each}
    </ul>
  {/if}
</div>