<script lang="ts">
	import List from './List.svelte';
	import AddExpense from './AddExpense.svelte';
  import Categories from './Categories.svelte';
	import Reports from './Reports.svelte';
  import DangerZone from './DangerZone.svelte';

	const Pages = ['list', 'reports', 'add expense', 'modify categories', 'DangerZone'] as const;

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
{:else if menu === 'reports'}
	<Reports />
{:else if menu === 'modify categories'}
  <Categories />
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
