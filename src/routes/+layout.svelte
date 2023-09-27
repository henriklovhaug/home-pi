<script lang="ts">
	import { fly } from "svelte/transition";
	import "../app.css";
	import { onMount } from "svelte";
	import { goto } from "$app/navigation";

	const pages = [
		{ name: "Home", path: "/" },
		{ name: "About", path: "/bus" }
	];

  let index = 0;

	onMount(() => {
		setInterval(() => {
			// goto(pages[Math.floor(Math.random() * pages.length)].path);
      goto(pages[index++ % pages.length].path);
		}, 10_000);
	});
</script>

<div
	class="h-full w-full"
	in:fly={{ x: -200, duration: 300, delay: 300 }}
	out:fly={{ x: 200, duration: 300 }}
>
	<slot />
</div>
