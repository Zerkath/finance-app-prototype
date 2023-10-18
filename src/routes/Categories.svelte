<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { onMount } from 'svelte';
  
  import CategoryComponent from './CategoryComponent.svelte';

	import type { Category } from '$lib/types.ts';

	let categories: Category[] = [];

	const updateCategoriesList = () => {
		invoke('get_categories').then((res) => {
			categories = res;
		});
	};

	onMount(updateCategoriesList);

	let categoryLabel = '';
	const upsertCategory = () => {
		if (categoryLabel == '') {
			return;
		}
		invoke('upsert_category', { label: categoryLabel }).then((res) => {
			updateCategoriesList();
		});
	};

  $: categories, console.log(categories);
</script>

<input bind:value={categoryLabel} /><button on:click={upsertCategory}>Add</button>
<div style="display: grid; grid-template-columns: repeat(2, auto);">
	<div>Category Label</div>
	<div>Actions</div>
	{#each categories as category}
    <CategoryComponent on:deleteHook={updateCategoriesList} categoryId={category.id} label={category.label} />
	{/each}
</div>
