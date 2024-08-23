<script lang="ts">
  import Folder from './Folder.svelte';
  import { drives } from '$lib/store';
</script>

{#each Object.keys($drives) as drive}
  <Folder path={drive} name={drive} items={$drives[drive]} />
{/each}
