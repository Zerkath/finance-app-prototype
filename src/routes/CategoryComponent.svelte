<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';

	import type { Category } from '$lib/types.ts';
  import { createEventDispatcher } from 'svelte';

  const dispatch = createEventDispatcher();

	export let category: Category;
  let label = category.label;
	let edit = false;
  let tempLabel = '';

  const startEdit = () => {
    edit = true;
    tempLabel = label;
  }

  const cancelEdit = () => {
    edit = false;
    label = tempLabel;
  }

  const saveEdit = () => {
    edit = false;
    invoke('update_category_label', { label: label, id: category.id }).then((res) => {
      console.log(res);
    });
  }

  const deleteCategory = () => {
    invoke('delete_category', { id: category.id }).then((res) => {
      // TODO should propagate to parent component, so it can refresh the list
      console.log(res);
    });
    dispatch('deleteHook', category);
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
    <button on:click={deleteCategory}>Delete</button>
  </div>
{/if}

