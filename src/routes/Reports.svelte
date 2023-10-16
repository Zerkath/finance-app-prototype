<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
  import { onMount } from 'svelte';

  let name: string[] = [];

  onMount(() => {
    invoke('query_tables').then((res) => {
      name = res;
    });
  });

	let tables: string[] = [];

  console.log('Querying tables');

</script>

<p>Report View</p>
<p>Hello {name}!</p>

{#each tables as table}
	<p>{table}</p>
{/each}
