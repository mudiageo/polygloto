<script lang="ts">
	import { page } from '$app/state';
	 import Home from 'lucide-svelte/icons/home';
	 import Layers from 'lucide-svelte/icons/layers';
	 import Play from 'lucide-svelte/icons/play';
	 import Trophy from 'lucide-svelte/icons/trophy';
	 import User from 'lucide-svelte/icons/user';

	let { isNavOpen = $bindable() } = $props();


	const navItems = [
		{ label: 'Home', path: '/', icon: Home },
		{ path: '/decks', icon: Layers, label: 'Learn' },
		{ label: 'Practice', path: '/practice', icon: Play },
		{ label: 'Ranking', path: '/leaderboard', icon: Trophy },
		{ label: 'Profile', path: '/profile', icon: User }
	];
</script>

<!-- Desktop Sidebar -->
<aside
	class="fixed inset-y-0 left-0  bg-white shadow-lg dark:bg-gray-800 hidden  sm:block
    {isNavOpen ? 'w-64' : 'w-20'} z-50 transform transition-all duration-300 ease-in-out"
>
	<div class="flex h-full flex-col">
		<div
			class="flex items-center justify-between border-b border-gray-200 p-4 dark:border-gray-700"
		>
			{#if isNavOpen}
				<a href="/" class="text-2xl font-bold text-primary-600 dark:text-primary-400">Polygloto</a>
			{/if}
			<button
				class="rounded-lg p-2 transition-colors hover:bg-gray-100 dark:hover:bg-gray-700"
				onclick={() => (isNavOpen = !isNavOpen)}
				aria-label="Open Side Navigation Menu"
			>
				<svg
					class="h-6 w-6 text-gray-600 dark:text-gray-300"
					fill="none"
					stroke="currentColor"
					viewBox="0 0 24 24"
				>
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d={isNavOpen ? 'M15 19l-7-7 7-7' : 'M9 5l7 7-7 7'}
					/>
				</svg>
			</button>
		</div>

		<nav class="flex-1 space-y-1 p-4">
			{#each navItems as item}
				<a
					href={item.path}
					class="flex items-center rounded-lg px-4 py-2  transition-colors
                dark:text-gray-300 dark:hover:bg-accent
              {page.url.pathname === item.path
						? 'bg-primary text-white dark:bg-primary-900 dark:text-primary-400'
						: 'hover:bg-accent/50'}"
					role="menuitem"
				>
					<span class="mr-2 flex h-6 w-6 items-center {isNavOpen ? 'mr-3' : ''}">
						<item.icon  strokeWidth="1"/>
					</span>
					<span class="flex items-center">
						{#if isNavOpen}
							<span>{item.label}</span>
						{/if}
					</span>
				</a>
			{/each}
		</nav>

		<div class="border-t border-gray-200 p-4 dark:border-gray-700">
			
			<a
				href="/settings"
				class="flex items-center rounded-lg px-4 py-2 text-gray-600 transition-colors
	  hover:bg-gray-100 dark:text-gray-300 dark:hover:bg-gray-700
	  {page.url.pathname === '/app/settings'
					? 'bg-primary-50 text-primary-600 dark:bg-primary-900 dark:text-primary-400'
					: ''}"
			>
				<svg
					class="h-6 w-6 {isNavOpen ? 'mr-3' : ''}"
					fill="none"
					stroke="currentColor"
					viewBox="0 0 24 24"
				>
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"
					/>
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"
					/>
				</svg>
				{#if isNavOpen}
					<span>Setting</span>
				{/if}
			</a>

		
		</div>
	</div>
</aside>
