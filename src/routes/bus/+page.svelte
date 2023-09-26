<script lang="ts">
	import { invoke } from "@tauri-apps/api";

	let data: Promise<[number, number[]]> = invoke("get_bus_now");
</script>

<a href="/">Go home</a>
<main class="flex h-full w-full flex-col items-center justify-center">
	{#await data then bus}
		<!-- promise was fulfilled -->
		<h1>{bus[0]}</h1>
		{#each bus[1] as stop}
			<p>{stop}</p>
		{/each}
	{/await}
</main>
