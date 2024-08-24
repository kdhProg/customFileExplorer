<script lang="ts">
  export let path: string;
  export let name: string = '';
  export let items: { [key: string]: { [key: string]: any } | null } | null = null;

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

        const folders: { [key: string]: { [key: string]: any } | null } = {};
        for (const item of contents) {
          const fullPath = `${path}\\${item}`;
          if (await isDirectory(fullPath)) {
            folders[fullPath] = null;
          }
        }

        // 현재 경로를 분해하여 각 수준에서 부모 객체를 찾아서 업데이트
        const pathParts = path.split(/[/\\]/);
        let currentLevel = currentDrives;

        for (let i = 0; i < pathParts.length; i++) {
          const part = pathParts.slice(0, i + 1).join('\\');
          if (i === pathParts.length - 1) {
            currentLevel[part] = folders;
          } else {
            currentLevel = currentLevel[part] as { [key: string]: { [key: string]: any } | null };
          }
        }
      }
    }

    drives.set({ ...currentDrives }); // 상태를 명확히 업데이트
  }

  function getFolderName(fullPath: string): string {
    return fullPath.split(/[/\\]/).pop() || '';
  }
</script>

<style>
  .clickable {
    cursor: pointer;
    margin-left: 20px;
    display: flex;
    align-items: center;

    border: 1px solid red;
  }

  .icon {
    margin-right: 5px;
  }

  .folder-item {
    margin-left: 20px;

    border: 1px solid blue;
  }

  .forDebug {
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
            items={subItems} 
          />
        </li>
      {/each}
    </ul>
  {/if}
</div>
