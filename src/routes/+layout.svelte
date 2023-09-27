<script lang="ts">
	import { fly } from "svelte/transition";
	import "../app.css";

	const pages = [
		{ name: "Home", path: "/" },
		{ name: "About", path: "/bus" }
	];
</script>

<div
	class="h-full w-full"
	in:fly={{ x: -200, duration: 300, delay: 300 }}
	out:fly={{ x: 200, duration: 300 }}
>
	<slot />
</div>
