<script lang="ts">
	import type { BusTime } from "$lib/types";
	import { invoke } from "@tauri-apps/api";

	const n = 10;
	let bus_times: Promise<BusTime[]> = invoke("get_next_n_bus", { n: n });

	let list_1 = async () => {
		const bus = await bus_times;
		return bus.slice(0, 5);
	};

	let list_2 = async () => {
		const bus = await bus_times;
		return bus.slice(5, 10);
	};
</script>

<!-- <a href="/">Go home</a> -->
<main class="flex h-full w-full flex-col items-center justify-center">
	<h1
		class="w-1/2 rounded-t-md border-4 border-quaternary py-4 text-center text-8xl text-quaternary"
	>
		A
	</h1>
	<div
		class="flex w-1/2 flex-row items-center justify-center rounded-b-md border-4 border-quaternary py-4"
	>
		<div class="flex h-full w-1/2 flex-col items-center justify-center">
			{#await list_1() then value}
				{#each value as bus_time}
					<h1 class="my-1 text-6xl">
						{bus_time.hour}:{bus_time.minute.toLocaleString("en-US", {
							minimumIntegerDigits: 2,
							useGrouping: false,
						})}
					</h1>
				{/each}
			{/await}
		</div>
		<div class="flex h-full w-1/2 flex-col items-center justify-center">
			{#await list_2() then value}
				{#each value as bus_time}
					<h1 class="my-1 text-6xl">
						{bus_time.hour}:{bus_time.minute.toLocaleString("en-US", {
							minimumIntegerDigits: 2,
							useGrouping: false,
						})}
					</h1>
				{/each}
			{/await}
		</div>
	</div>
</main>
