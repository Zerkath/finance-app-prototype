<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';

  let showDbReset = false;

  const resetDatabase = () => {
    invoke('reset_tables').then((res) => {
      console.log(res);
      showDbReset = false;
    });
  }
</script>

<p>DangerZone - Actions for managing the state in unrecoverable ways</p>

<button on:click|preventDefault={() => showDbReset = true}>Reset Database</button>

{#if showDbReset}
  <p>Are you sure you want to reset the database?</p>
    <button on:click|preventDefault={resetDatabase}>Yes</button>
  <button on:click|preventDefault={() => showDbReset = false}>No</button>
{/if}


