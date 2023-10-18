<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';

  import { createEventDispatcher } from 'svelte';

  const dispatch = createEventDispatcher();

  export let categoryId: number
  export let label: string
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
    invoke('update_category_label', { label: label, id: categoryId }).then((res) => {
      console.log(res);
    });
  }

  const deleteCategory = () => {
    invoke('delete_category', { id: categoryId }).then((res) => {
      // TODO should propagate to parent component, so it can refresh the list
      console.log(res);
    });
    dispatch('deleteHook');
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

