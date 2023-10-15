<script lang="ts">
	import List from './List.svelte';
	import AddExpense from './AddExpense.svelte';
	import ModifyIncome from './ModifyIncome.svelte';
	import Reports from './Reports.svelte';

	const Pages = ['list', 'reports', 'add expense', 'modify income'] as const;

	type Page = (typeof Pages)[number];

	let menu: Page = 'list';
</script>

<ul id="menu" class="navbar">
	{#each Pages as page}
		<button on:click|preventDefault={() => (menu = page)}>{page}</button>
	{/each}
</ul>
{#if menu === 'list'}
	<List />
{:else if menu === 'add expense'}
	<AddExpense />
{:else if menu === 'modify income'}
	<ModifyIncome />
{:else if menu === 'reports'}
	<Reports />
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
	}
</style>
