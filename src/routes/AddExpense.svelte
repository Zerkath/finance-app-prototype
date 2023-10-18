<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { onMount } from 'svelte';
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
	let link: string | undefined = undefined;
	let dateCreated: string = new Date().toISOString().split("T")[0]
	let expenseCategories: number[] = [];

  const insertExpense = () => {
    invoke('insert_expense', {
      value,
      name,
      description,
      link,
      dateCreated,
      expenseCategories,
    }).then((res) => {
      console.log(res);

      value = 0;
      name = '';
      description = undefined;
      link = undefined;
      dateCreated = new Date().toISOString().split("T")[0]
      expenseCategories = [];
    });
  };
</script>

<div class="expense-form">

  <input type="text" placeholder="Name" bind:value={name} />
  <input type="text" placeholder="Description" bind:value={description} />
  <input type="number" placeholder="Amount" bind:value />
  <input type="text" placeholder="Link" bind:value={link} />
  <input type="date" placeholder="Date" bind:value={dateCreated} />
  <!--
  <select bind:value={recurType}>
    <option value={undefined}>None</option>
    <option value="daily">Daily</option>
    <option value="weekly">Weekly</option>
    <option value="monthly">Monthly</option>
    <option value="yearly">Yearly</option>
  </select>
  <input type="date" placeholder="Date" bind:value={recurEnd} />
  -->
  <select multiple bind:value={expenseCategories}>
    {#each categories as category}
      <option value={category.id}>{category.label}</option>
    {/each}
  </select>

  <input type="submit" on:click={insertExpense} value="Add Expense" />
</div>

<style lang="scss">

  .expense-form {
    max-width: 500px;
    display: flex;
    flex-direction: column;
  }
</style>
