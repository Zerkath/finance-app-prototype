<script lang="ts">
	import { writable } from 'svelte/store';

	import List from './List.svelte';
	import AddExpense from './AddExpense.svelte';
	import Reports from './Reports.svelte';
	import DangerZone from './DangerZone.svelte';

	const Pages = ['list', 'reports', 'add expense', 'DangerZone'] as const;

	type Page = (typeof Pages)[number];

	let menu: Page = localStorage.getItem('page') ? (localStorage.getItem('page') as Page) : 'list';

	const changePage = (page: Page) => {
		localStorage.setItem('page', page);
		menu = page;
	};
</script>

<ul id="menu" class="navbar">
	{#each Pages as page}
		<button on:click|preventDefault={() => changePage(page)}>{page}</button>
	{/each}
</ul>
{#if menu === 'list'}
	<List />
{:else if menu === 'add expense'}
	<AddExpense />
{:else if menu === 'reports'}
	<Reports />
{:else if menu === 'DangerZone'}
	<DangerZone />
{/if}

<style lang="scss">
	.navbar {
		display: flex;
		flex-direction: row;
		justify-content: space-around;
		align-items: center;
		list-style: none;
		padding: 0;
		margin: 0;
		background-color: #333;
		font-family: sans-serif;
		font-size: 1.5rem;
		height: 3rem;
		* {
			text-transform: capitalize;
		}
	}
</style>
