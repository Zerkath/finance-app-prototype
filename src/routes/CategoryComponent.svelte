<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';

	import type { Category } from '$lib/types.ts';

	export let category: Category;
  let label = category.label;
	let edit = false;
  let tempLabel = '';

  export const startEdit = () => {
    edit = true;
    tempLabel = label;
  }

  export const cancelEdit = () => {
    edit = false;
    label = tempLabel;
  }

  export const saveEdit = () => {
    edit = false;
    invoke('update_category_label', { label: label, id: category.id }).then((res) => {
      console.log(res);
    });
  }

</script>

{#if edit}
	<input bind:value={label} />
  <div>
	<button on:click={cancelEdit}>Cancel</button>
	<button on:click={saveEdit}>Save</button>
  </div>
{:else}
 <input disabled bind:value={label} />
  <div>
    <button on:click={startEdit}>Edit</button>
    <button>Delete</button>
  </div>
{/if}

