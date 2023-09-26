<script lang="ts">
	import { invoke } from "@tauri-apps/api/tauri";

	let msg = "";

	const handleClick = async () => {
		console.log("click");
		msg = await invoke("say_hello");
	};
</script>

<div class="flex h-full w-full flex-col items-center justify-center">
	<h1 class="text-tertiary">Welcome to SvelteKit</h1>
	<p class="text-green-600">Test</p>
	<a href="/bus">Go to buss</a>
	<button class="w-3/4 border border-black" on:click={handleClick}>Click me</button>
	<p>{msg}</p>
</div>
