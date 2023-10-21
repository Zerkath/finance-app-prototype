<script lang="ts">
  import { invoke } from '@tauri-apps/api/tauri';
  import { onMount } from 'svelte';

  import type { Category } from '$lib/types.ts';

  type Expense = {
    id: number;
    name: string;
    value: number;
    description?: string;
    categories: Category[];
    date_created: string; // of format YYYY-MM-DD
  };

  let currentPage = 1;
  let maxPage = 1;

  const updatePage = () =>
    invoke('query_page', { pageSize: 25, currentPage: currentPage }).then(
      (res) => {
        maxPage = res.total_pages;
        expenses = res.expenses;
      }
    );

  onMount(() => {
    updatePage();
  });

  const changePage = (page: number) => {
    currentPage = page;
    updatePage();
  };

  let expenses: Expense[] = [];

  const remove = (id: number) => {
    invoke('delete_expense', { id: id }).then(() => {
      updatePage();
    });
  };
</script>

<!-- TODO should be changed to a grid layout -->
<div class="tableview">
  <table>
    <tr>
      <th>Name</th>
      <th>Amount</th>
      <th>Description</th>
      <th>Categories</th>
      <th>Date Created</th>
      <th>Actions</th>
    </tr>
    {#each expenses as expense}
      <tr>
        <td>{expense.name}</td>
        <td>{expense.value}</td>
        <td>{expense.description ? expense.description : ''}</td>
        <td>
          {#each expense.categories as category}
            <span
              style="padding: 4px; margin: 5px; background: lightgrey; border-radius: 5px;"
              id={category.id.toString()}>{category.label}</span
            >
          {/each}
        </td>
        <td>{expense.date_created}</td>
        <td>
          <button on:click={() => remove(expense.id)}>Remove</button>
          <button disabled>Modify</button>
        </td>
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
  table {
    width: 100%;
    border-collapse: collapse;
    border: 1px solid #333;
  }

  th {
    border: 1px solid #000;
    background-color: #333;
    color: #fff;
  }

  .table-nocontent {
    text-align: center;
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
