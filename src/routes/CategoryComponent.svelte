<script lang="ts">
  import { invoke } from '@tauri-apps/api/tauri';

  import { createEventDispatcher } from 'svelte';

  const dispatch = createEventDispatcher();

  export let categoryId: number;
  export let label: string;

  let edit = false;
  let tempLabel = label;

  const startEdit = () => {
    tempLabel = label;
    edit = true;
  };

  const cancelEdit = () => {
    edit = false;
  };

  const saveEdit = () => {
    label = tempLabel;
    edit = false;
    invoke('update_category_label', { label: label, id: categoryId }).then(
      (res) => {
        console.log(res);
      }
    );
    dispatch('updateHook');
  };

  const deleteCategory = () => {
    invoke('delete_category', { id: categoryId }).then((res) => {
      // TODO should propagate to parent component, so it can refresh the list
      console.log(res);
    });
    dispatch('deleteHook');
  };
</script>

<div class="category">
  {#if edit}
    <input data-testid="category-input-open" bind:value={tempLabel}>
    <div>
      <button data-testid="cancel-button" on:click={cancelEdit}>Cancel</button>
      <button data-testid="save-button" on:click={saveEdit}>Save</button>
    </div>
  {:else}
    <input data-testid="category-input-closed" disabled bind:value={label} />
    <div>
      <button data-testid="edit-button" on:click={startEdit}>Edit</button>
      <button data-testid="delete-button" on:click={deleteCategory}>Delete</button>
    </div>
  {/if}
</div>

<style>
  .category {
    display: flex;
    flex-direction: row;
    justify-content: space-between;
    align-items: center;
    padding: 0.3rem 0;
  }
</style>
