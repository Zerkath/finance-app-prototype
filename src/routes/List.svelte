<script lang="ts">
	type Expense = {
		id: number; // unique identifier
		name: string;
		amount: number; // decimal value most likely
		description?: string; // optional
		categories: string[]; // array of strings
		link?: string; // optional
		dateCreated: number; // timestamp
		recurType?: 'daily' | 'weekly' | 'monthly' | 'yearly'; // optional
		recurEnd?: Date; // optional
	};

  function toDateString(timestamp: number) {
    // iso format
    const date = new Date(timestamp);
    return date.toISOString().split('T')[0];
  }

  function fakeDate() {
    return Date.now() - Math.floor(Math.random() * 100000000000);
  }

	// generate a list of expenses for mocks
	const expenses: Expense[] = [
   { id: 1, name: "Foo", amount: 1, dateCreated: fakeDate(), categories: ["Food"] },
   { id: 2, name: "Bar", amount: 2, dateCreated: fakeDate(), categories: ["Rent"] },
   { id: 3, name: "Baz", amount: 3, dateCreated: fakeDate(), categories: ["Food", "Pet"] },
   { id: 4, name: "Qux", amount: 4, dateCreated: fakeDate(), categories: ["Hobbies"] },
   { id: 5, name: "Quux", amount: 5, dateCreated: fakeDate(), categories: ["Games"] },
   { id: 6, name: "Corge", amount: 6, dateCreated: fakeDate(), categories: [] },
   { id: 7, name: "Grault", amount: 7, dateCreated: fakeDate(), categories: [] },
   { id: 8, name: "Garply", amount: 8, dateCreated: fakeDate(), categories: [] },
   { id: 9, name: "Waldo", amount: 9, dateCreated: fakeDate(), categories: [] },
   { id: 10, name: "Fred", amount: 10, dateCreated: fakeDate(), categories: [] },
   { id: 11, name: "Plugh", amount: 11, dateCreated: fakeDate(), categories: [] },
   { id: 12, name: "Xyzzy", amount: 12, dateCreated: fakeDate(), categories: [] },
   { id: 13, name: "Thud", amount: 13, dateCreated: fakeDate(), categories: [] },
   { id: 14, name: "Foo", amount: 14, dateCreated: fakeDate(), categories: [] },
   { id: 15, name: "Bar", amount: 15, dateCreated: fakeDate(), categories: [] },
   { id: 16, name: "Baz", amount: 16, dateCreated: fakeDate(), categories: [] },
   { id: 17, name: "Qux", amount: 17, dateCreated: fakeDate(), categories: [] },
   { id: 18, name: "Quux", amount: 18, dateCreated: fakeDate(), categories: [] },
  ].sort((a, b) => b.dateCreated - a.dateCreated);

	function remove(id: number) {
		// TODO should remove the entry from DB and refetch data
		return;
	}

	function modify(id: number) {
		// TODO should open a modal
		return;
	}
</script>

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
				<td>{expense.amount}</td>
        <td>{expense.description ? expense.description : ""}</td>
        <td>{expense.categories}</td>
        <td>{expense.link ? expense.link : ""}</td>
        <td>{toDateString(expense.dateCreated)}</td>
        <td>{expense.recurType ? expense.recurType : ""}</td>
        <td>{expense.recurEnd ? expense.recurEnd : ""}</td>
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
	<button>Previous Page</button>{'1 / 1'}<button>Next Page</button>
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
		height: 10rem;
		padding: 1rem;
		border: 1px solid #000;
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
