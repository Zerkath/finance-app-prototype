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

	let value = 0;
	let name: string = '';
	let description: string | undefined = undefined;
	let dateCreated: string = new Date().toISOString().split('T')[0];
	let expenseCategories: number[] = [];

	const insertExpense = () => {
		invoke('insert_expense', {
			value,
			name,
			description,
			dateCreated,
			expenseCategories
		}).then((res) => {
			console.log(res);

			value = 0;
			name = '';
			description = undefined;
			dateCreated = new Date().toISOString().split('T')[0];
			expenseCategories = [];
		});
	};


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

<div class="creation-forms">
	<div class="expense-form">
		<h3>Expense Creation Form</h3>
		<input type="text" placeholder="Name" bind:value={name} />
		<input type="text" placeholder="Description" bind:value={description} />
		<input type="number" placeholder="Amount" bind:value />
		<input type="date" placeholder="Date" bind:value={dateCreated} />
		<select multiple bind:value={expenseCategories}>
			{#each categories as category}
				<option value={category.id}>{category.label}</option>
			{/each}
		</select>

		<input type="submit" on:click={insertExpense} value="Add Expense" />
	</div>

	<div class="category-form">
		<h3>Categories</h3>
		<input bind:value={categoryLabel} />
    <button style="margin-bottom: 1rem;"on:click={upsertCategory}>Add</button>
    {#each categories as category}
      <CategoryComponent
        on:deleteHook={updateCategoriesList}
        on:updateHook={updateCategoriesList}
        categoryId={category.id}
        label={category.label}
      />
    {/each}
	</div>
</div>

<style lang="scss">
  .creation-forms {
    display: flex;
    flex-direction: row;
    justify-content: left;
    > * {
      min-width: 20rem;
      margin: 1rem;
      border: 1px solid black;
      padding: 1rem;
    }
  }

  input { 
    margin: 0.5rem 0;
  }

	.category-form {
		display: flex;
		flex-direction: column;
	}

	.expense-form {
		display: flex;
		flex-direction: column;
	}
</style>
