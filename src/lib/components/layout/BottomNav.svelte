<script>
    import { page } from "$app/state";
   import Home from 'lucide-svelte/icons/home'
   import Layers from 'lucide-svelte/icons/layers'
   import Play from 'lucide-svelte/icons/play'
   import Trophy from 'lucide-svelte/icons/trophy'
   import User from 'lucide-svelte/icons/user'
   
       //let { children } = $props();
       let navItems = [
           {
             name: 'Home',
             href: '/',
             icon: Home,
           },
           {
             name: 'Learn',
             href: '/decks',
             icon: Layers,
           },
           {
             name: 'Practice',
             href: '/practice',
             icon: Play,
           },
           {
             name: 'Ranking',
             href: '/leaderboard',
             icon: Trophy,
           },
           {
             name: 'Profile',
             href: '/profile',
             icon: User,
           },
         ]
   </script>
       <nav class="fixed bottom-0 inset-x-0 w-screen sm:hidden border-t border-t-amber-100 bg-background">
           <div class="flex justify-between items-center py-2">
             {#each navItems as item}
             <a 
               href={item.href}
               class="flex flex-col items-center px-4 py-2 {page.url.pathname === item.href ? 'text-primary' : 'text-muted-foreground'}"
             >
               <item.icon strokeWidth="1"/>
               <span class="text-xs mb-0">{item.name}</span>
            {#if page.url.pathname === item.href}
               <span  style="margin;-100px" class="text-xs font-extrabold mb-0">.</span>
           {/if}
             </a>
             {/each}
         </div>
       </nav>