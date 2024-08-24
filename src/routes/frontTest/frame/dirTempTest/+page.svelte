<script lang="ts">
<<<<<<< HEAD
    import Folder from '$lib/components/Folder.svelte';
    import { drives } from '$lib/store';
  </script>
  <div>
    {#each Object.keys($drives) as drive}
      <Folder path={drive} name={drive} items={$drives[drive]} />
    {/each}
  </div>
  
=======
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
>>>>>>> 21c21625ac5d068dd4a88d9246165af68b17615b
