<script lang="ts">
	import type { Weather } from "$lib/types";
	import { invoke } from "@tauri-apps/api";

	let data: Promise<Weather> = invoke("get_weather");
</script>

<main class="flex h-full w-full items-center justify-center">
	{#await data}
		<!-- data is pending -->
		<div class="text-quaternary">loading</div>
	{:then weather}
		<div>{weather.uv}</div>
		<div>Success</div>
		<!-- data was fulfilled -->
	{:catch error}
		<div class="text-quaternary">{error}</div>
		<!-- data was rejected -->
	{/await}
</main>
