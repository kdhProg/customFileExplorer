<script lang="ts">
    import { onMount } from 'svelte';
    import { invoke } from '@tauri-apps/api/tauri';
  
    interface FileMetadata {
      file_name: string;
      file_size: number;
      last_modified: number;
    }
  
    let filePath = 'test_file.txt';
    let metadata: FileMetadata | null = null;
    let error: string | null = null;
  
    async function testTauriMethod() {
      try {
        metadata = await invoke<FileMetadata>('get_file_metadata', { filePath });
      } catch (err) {
        error = (err as Error).message;
      }
    }
  
    onMount(() => {
      testTauriMethod();
    });
  </script>
  
  <main>
    <h1>Tauri Method Test</h1>
    {#if metadata}
      <p><strong>File Path:</strong> {metadata.file_name}</p>
      <p><strong>File Size:</strong> {metadata.file_size} bytes</p>
      <p><strong>Last Modified:</strong> {new Date(metadata.last_modified * 1000).toLocaleString()}</p>
    {:else if error}
      <p style="color: red;">Error: {error}</p>
    {:else}
      <p>Loading...</p>
    {/if}
  </main>
  