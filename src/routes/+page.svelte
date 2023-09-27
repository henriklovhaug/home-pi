<script lang="ts">
	import { invoke } from "@tauri-apps/api/tauri";

	let msg = "";

	const handleClick = async () => {
		console.log("click");
		msg = await invoke("say_hello");
	};
</script>

<div class="flex h-full w-full flex-col items-center justify-evenly">
	<a href="/bus" class="text-3xl">Go to buss</a>
	<button class="w-3/4 rounded border border-quaternary text-quaternary" on:click={handleClick}
		>Click me</button
	>
	<p>{msg}</p>
</div>
