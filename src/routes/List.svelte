<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { onMount } from 'svelte';

  import type { Category } from '$lib/types.ts';

	type Expense = {
		id: number; // unique identifier
		name: string;
		value: number; // decimal value most likely
		description?: string; // optional
		categories: Category[]; // array of strings
		link?: string; // optional
		date_created: string; // timestamp
		recur_type?: 'daily' | 'weekly' | 'monthly' | 'yearly'; // optional
		recur_end?: string | undefined;
	};

	let currentPage = 1;
	let maxPage = 1;

	onMount(() => {
		invoke('query_page', {pageSize: 50, currentPage: currentPage}).then((res) => {
      console.log(res)
      maxPage = res.total_pages
      expenses = res.expenses
		});
	});

	function changePage(page: number) {
		// TODO should change the page
		// and refetch data
		currentPage = page;
		return;
	}

	// Current view
  // should be
	// sorted
	// filtered
	// searchable
	let expenses: Expense[] = []

	function remove(id: number) {
		// TODO should remove the entry from DB and refetch data
		return;
	}

	function modify(id: number) {
		// TODO should open a modal
		return;
	}
</script>
<!-- TODO should be changed to a grid layout -->
<div class="tableview">
	<table>
		<tr>
			<th>Name</th>
			<th>Amount</th>
			<th>Description</th>
			<th>Categories</th>
			<th>Link</th>
			<th>Date Created</th>
			<th>Recur Type</th>
			<th>Recur End</th>
			<th>Actions</th>
		</tr>
		{#each expenses as expense}
			<tr>
				<td>{expense.name}</td>
				<td>{expense.value}</td>
				<td>{expense.description ? expense.description : ''}</td>
				<td>
        {#each expense.categories as category}
          <span style="padding: 4px; margin: 5px; background: lightgrey; border-radius: 5px;" id={category.id.toString()}>{category.label}</span>
        {/each}
        </td>
				<td>{expense.link ? expense.link : ''}</td>
				<td>{expense.date_created}</td>
				<td>{expense.recur_type ? expense.recur_type : ''}</td>
				<td>{expense.recur_end ? expense.recur_end : ''}</td>
				<td
					><button on:click={() => remove(expense.id)}>Remove</button><button
						on:click={() => modify(expense.id)}>Modify</button
					></td
				>
			</tr>
		{/each}
	</table>
	{#if expenses.length === 0}
		<div class="table-nocontent">No expenses found</div>
	{/if}
</div>

<div class="pagenav">
	{#if currentPage < 2}
		<button disabled>{'<<'}</button>
	{:else}
		<button on:click={() => changePage(currentPage - 1)}>{'<<'}</button>
	{/if}
	{`${currentPage}/${maxPage}`}
  {#if currentPage >= maxPage}
    <button disabled>{'>>'}</button>
  {:else}
    <button on:click={() => changePage(currentPage + 1)}>{'>>'}</button>
  {/if}
</div>

<style lang="scss">

	// full width table
	// with visible borders
	table {
		width: 100%;
		border-collapse: collapse;
		border: 1px solid #333;
	}
	// headers should have a background color
	th {
		border: 1px solid #000;
		background-color: #333;
		color: #fff;
	}
	// columns should be separated by small borders
	.table-nocontent {
		text-align: center;
		background: linear-gradient(180deg, #333, #fff);
		height: 14rem;
		padding: 3rem;
		border: 1px linear-gradient(180deg, #000, #fff);
		border-bottom: none;
	}

	td,
	th {
		border: 1px solid #000;
		padding: 0.5rem;
	}

	// navigation should be at the bottom
	// and centered
	// with a small margin
	// and padding
	.pagenav {
		display: flex;
		justify-content: center;
		align-items: center;
		* {
			margin: 0.8rem;
			padding: 0.3rem;
			min-width: 8rem;
		}
	}
</style>
