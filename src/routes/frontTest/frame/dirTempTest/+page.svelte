<script lang="ts">
  import Folder from "$lib/components/Folder.svelte";
  import { drives } from "$lib/store";
</script>

<button
  on:click={() => {
    console.log($drives);
  }}>Show store</button
>
<div>
  {#each Object.keys($drives) as drive}
    <Folder path={drive} name={drive} items={$drives[drive]} />
  {/each}
</div>
