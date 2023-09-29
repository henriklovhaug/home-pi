<script lang="ts">
	export let hour: string;
	export let temp: number;
	export let rain: number;
	export let clouds: number;

	$: console.log(hour);
	$: hour_field = hour.slice(11, 13);
</script>

<section
	class="mx-2 flex flex-col items-center justify-center rounded-md border-2 bg-secondary p-2"
>
	<div class="flex flex-row items-center justify-center">
		<h1 class="text-6xl">kl. {hour_field}</h1>
	</div>
	<div class="flex flex-row items-center justify-center">
		<h1 class="text-4xl">{temp}&deg;</h1>
	</div>
	<div class="flex flex-row items-center justify-center">
		<h1 class="text-4xl">{rain}</h1>
	</div>
	<div class="flex flex-row items-center justify-center">
		<h1 class="text-4xl">{clouds}</h1>
	</div>
</section>
