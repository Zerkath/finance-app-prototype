<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { onMount } from 'svelte';

	type Report = {
		total: number;
		dates: Map<string, number>;
	};

	let reportData: Report = {
		total: 0,
		dates: new Map()
	};

	let supportedReports: string[] = [];
	let selectedReportType: string | undefined = undefined;
	let selectedDate: string = new Date().toISOString().split('T')[0];

	onMount(() => {
		invoke('get_supported_report_types').then((res) => {
			supportedReports = res;
		});
	});

	const getReport = () => {
		if (selectedReportType == undefined) {
			return;
		}
		invoke('get_basic_report', {
			reportType: selectedReportType,
			selectedDate: selectedDate
		}).then((res) => {
			reportData = res;
		});
	};
</script>

<input type="date" bind:value={selectedDate} />

<select bind:value={selectedReportType}>
	{#each supportedReports as report}
		<option value={report}>{report}</option>
	{/each}
</select>

<button on:click={getReport}>Get Report</button>

{#each Object.entries(reportData.dates) as [key, value]}
	<div>{key}: {value}</div>
{/each}
