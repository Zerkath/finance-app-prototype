<script lang="ts">
  import { invoke } from '@tauri-apps/api/tauri';
  import { onMount } from 'svelte';

  let total = 0;
  let uncategorized = 0;
  let dates: Map<string, number> = new Map();
  let categories: Map<string, number> = new Map();

  let supportedReports: string[] = [];
  let selectedReportType: string | undefined = undefined;
  let selectedDate: string = new Date().toISOString().split('T')[0];

  onMount(() => {
    invoke('get_supported_report_types').then((res) => {
      supportedReports = res;
    });
  });

  const randomColor = () => {
    return Math.floor(Math.random() * 16777215).toString(16);
  };

  const getCSSForCategory = (value: number) => {
    return `height: ${
      (value / total) * 25
    }rem; background-color: #${randomColor()}`;
  };

  const getCSSForDate = (value: number) => {
    return `width: ${
      (value / total) * 100
    }%; background-color: #${randomColor()}`;
  };

  const getReport = () => {
    if (selectedReportType == undefined) {
      return;
    }
    invoke('get_basic_report', {
      reportType: selectedReportType,
      selectedDate: selectedDate
    }).then((res) => {
      total = res.total;
      uncategorized = res.uncategorized;
      dates = res.dates;
      categories = res.categories;
    });
  };
</script>

<div class="controls">
  <input aria-label="Date Selector" type="date" bind:value={selectedDate} />
  <select bind:value={selectedReportType}>
    {#each supportedReports as report}
      <option value={report}>{report}</option>
    {/each}
  </select>
  <button on:click={getReport}>Get Report</button>
</div>

<div class="parent-dataview">
  <div class="block-container">
    <h3>By Category</h3>
    <div class="container">
      <div style={getCSSForCategory(uncategorized)}>
        <span>Uncategorized</span>
        <span>{uncategorized}€</span>
      </div>
      {#each Object.entries(categories) as [key, value]}
        <div style={getCSSForCategory(value)}>
          <span>{key}</span>
          <span>{value}€</span>
        </div>
      {/each}
    </div>
  </div>

  <div class="block-container">
    <h3>By Date</h3>
    <div class="dateview">
      {#each Object.entries(dates) as [key, value]}
        <div>{key}</div>
        <div style={getCSSForDate(value)}>{value}</div>
      {/each}
    </div>
    <div>Total expenditure: {total}</div>
  </div>
</div>

<style lang="scss">
  .controls {
    display: flex;
    flex-direction: column;
    max-width: 20rem;
    margin: 0.5rem 0;
    > * {
      display: flex;
      padding: 0.3rem;
      margin: 0.2rem;
    }
  }

  .parent-dataview {
    display: flex;
    flex-direction: row;
    justify-content: space-between;
  }

  .block-container {
    width: 50%;
    outline: 1px solid black;
    padding: 4px;

    > * {
      padding: 10px;
    }
  }

  .dateview {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    * {
      padding: 4px;
      border: 5px;
    }
  }

  .container {
    div {
      display: block;
      padding: 4px;
      min-height: 2rem;
      text-wrap: pretty;
      text-transform: capitalize;
      display: flex;

      :last-child {
        margin-left: auto;
      }
    }
  }
</style>
