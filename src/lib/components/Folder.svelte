<script lang="ts">
    export let path: string;
    export let name: string = '';
    export let items: { [key: string]: string[] | null } | null = null;
  
    import { listFilesInDirectory, isDirectory } from '$lib/api';
    import { get } from 'svelte/store';
    import { drives } from '$lib/store';
  
    import Folder from './Folder.svelte';
  
    async function handleFolderClick() {
      const currentDrives = get(drives);
  
      if (items) {
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
      drives.set(currentDrives);
    }
  </script>
  
  <style>
    .clickable {
      cursor: pointer;
      margin-left: 20px;
    }
  </style>
  
  <div>
    <div class="clickable" on:click={handleFolderClick}>{name}</div>
    {#if items}
      <ul>
        {#each Object.entries(items) as [key, subItems]}
          <li>
            <!-- 여기에 변환 로직 추가 -->
            <Folder 
              path={key} 
              name={key.split('/').pop() ?? ''} 
              items={Array.isArray(subItems) ? null : subItems} 
            />
          </li>
        {/each}
      </ul>
    {/if}
  </div>
  