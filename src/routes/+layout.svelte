<script>
	import '../app.css';
	import SideNav from '$lib/components/layout/SideNav.svelte';
	import BottomNav from '$lib/components/layout/BottomNav.svelte'

	let { children } = $props();

	let isNavOpen = $state(false)


	//let { children } = $props();
	
</script>

<div class="min-h-screen bg-gray-50 transition-colors duration-200 dark:bg-gray-900">
	<SideNav bind:isNavOpen />

	<!-- Main Content -->
	<div class="{isNavOpen ? 'sm:ml-64' : 'sm:ml-20'} min-h-screen transition-all duration-300">
		<main class="flex flex-col h-screen container mx-auto px-4 py-8 max-sm:mb-20 max-sm:mt-16">
			<div in:fade={{ duration: 200 }}>
				{@render children()}
			</div>
		</main>
		<BottomNav />
	</div>

	<!-- Notifications -->
	<!-- <Toast /> -->
</div>

