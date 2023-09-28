<script lang="ts">
	import type { Weather } from "$lib/types";
	import WeatherBlock from "$lib/weatherBlock.svelte";
	import { invoke } from "@tauri-apps/api";

	let data: Promise<Weather> = invoke("get_weather");

	let next_hours = async () => {
		const weather = await data;
		const now = new Date();
		const this_hour = now.getHours();
		return weather.hourly.slice(this_hour + 1, this_hour + 7);
	};
</script>

<main class="flex h-full w-full flex-col items-center justify-center">
	<div class="flex h-1/2 w-full flex-row items-center justify-center">
		{#await data}
			<!-- data is pending -->
			<h1 class="text-3xl">loading</h1>
		{:then weather}
			<h1>{weather.uv}</h1>
		{:catch error}
			<div class="text-quaternary">{error}</div>
			<!-- data was rejected -->
		{/await}
	</div>
	<div class="flex h-1/2 w-full flex-row items-center justify-center">
		{#await next_hours() then hourly}
			{#each hourly as item}
				<WeatherBlock
					hour={item.hour}
					rain={item.rain}
					clouds={item.cloudcover}
					temp={item.temperature}
				/>
			{/each}
		{/await}
	</div>
</main>
