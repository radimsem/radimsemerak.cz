<script lang="ts">
	// ui
	import Header from '$lib/components/Header.svelte';
	import Hero from '$lib/components/Hero.svelte';
	import Experience from '$lib/components/Experience.svelte';
	import Projects from '$lib/components/Projects.svelte';
	import Contact from '$lib/components/Contact.svelte';
    import type { PageServerData } from './$types';

	export let data: PageServerData;
</script>

<svelte:head>
	<title>Home</title>
	<meta name="description" content="Open-source Full Stack Developer" />
</svelte:head>

<div class="relative space-y-10 px-[5%] py-4 text-slate-300 md:px-[10%]">
	<Header />

	<main class="w-full min-h-screen pb-10">
		<section class="space-y-10 lg:space-y-20">
			{#if data.projects && data.obfuscHandler}
				<Hero />
				<Experience />
				<Projects data={data.projects}/>
				<Contact obfuscHandler={data.obfuscHandler} />
			{:else}
				{data.err}
			{/if}
		</section>		
	</main>

	<footer>
		<p class="text-center text-sm text-slate-400">
			© {new Date().getFullYear()} Radim Semerák. 
			Open-source on <a href="https://github.com/radimsem" class="text-slate-400">Github</a>.
		</p>
	</footer>
</div>