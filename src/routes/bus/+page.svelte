<script lang="ts">
	import type { BusTime } from "$lib/types";
	import { invoke } from "@tauri-apps/api";
	import { onMount } from "svelte";

	const n = 10;
	let bus_times: Promise<BusTime[]> = invoke("get_next_n_bus", { n: n });
</script>

<a href="/">Go home</a>
<main class="flex h-full w-full flex-col items-center justify-center">
	<h1
		class="h-12 w-1/2 rounded-t-md border-4 border-quaternary text-center text-3xl text-quaternary"
	>
		A
	</h1>
	{#await bus_times}
		<h1>LOADING</h1>
	{:then bus}
		<div
			class="flex w-1/2 flex-col items-center justify-center rounded-b-md border-4 border-quaternary py-4"
		>
			{#each bus as bus_time}
				<h1 class="my-1 text-3xl">
					{bus_time.hour}:{bus_time.minute.toLocaleString("en-US", {
						minimumIntegerDigits: 2,
						useGrouping: false
					})}
				</h1>
			{/each}
		</div>
	{/await}
</main>
