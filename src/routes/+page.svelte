<script>
	import { fade, slide } from 'svelte/transition';

  import { invoke } from "@tauri-apps/api/core";
	import { Card } from '$lib/components/ui/card';
	import { HoverCard } from '$lib/components/ui/hover-card';
	import { Progress } from '$lib/components/ui/progress';
	import { Sheet } from '$lib/components/ui/sheet';
  import { Badge } from '$lib/components/ui/badge'
  import { Button } from "$lib/components/ui/button";
  import { Avatar } from "$lib/components/ui/avatar";
  import Flame from 'lucide-svelte/icons/flame'
  import User from 'lucide-svelte/icons/user'

  let name = $state("");
  let greetMsg = $state("");
  let cards = $state([]);
  const recentDecks = [
      {
        id: "1",
        name: "Spanish Basics",
        description: "Essential Spanish vocabulary",
        cardCount: 50,
        progress: 35,
        language: "Spanish"
      },
      {
        id: "2",
        name: "French Phrases",
        description: "Common French expressions",
        cardCount: 30,
        progress: 15,
        language: "French"
      }
    ];

  async function greet(event) {
    event.preventDefault();

    greetMsg = await invoke("greet", { name });
  }

  async function getCards(event) {
    event.preventDefault();

    cards = await invoke("flashcards");
  }
  let decks = $state([{name:"Deck1", description:"description"}, {name:"Deck2", description:"description"}, {name:"Deck3", description:"description"}])



</script>
<main class="flex flex-col">

   <header class="px-4 pt-6 pb-2">
            <div class="flex justify-between items-center">
              <div class="flex items-center">
                <div class="flex flex-col">
                  <span class="text-lg font-medium">Mavor Polyglotarian</span>
                </div>
              </div>
              <div class="flex items-center">
                <div class="flex items-center bg-orange-100 px-2 py-1 rounded-full">
                  <Flame size={16} class="text-orange-500 mr-1" />
                  <span class="text-sm font-medium text-orange-700">12</span>
                </div>
                <div class="h-10 p-2 rounded-full bg-secondary ml-3 flex items-center justify-center text-white font-medium">
     Urhobo
                  
                </div>
              </div>
            </div>
          </header>
 
  <section>
  <Card class="flex flex-row p-4">
    <div in:fade>
    <h1 class="text-2xl font-bold">Welcome Polyglotarian</h1>
      <p class="mt-4 text-primary">Keep up the great work</p>
    </div>    
              <div in:slide class="flex flex-col justify-end">
                  <div class="w-36 h-36 rounded-full border-4 border-white/30 flex items-center justify-center">
                    <span class="text-xl font-bold">15/25</span>
                  </div>
                  <p class="mt-2">Today's goal</p>
                  <button class="mt-4 bg-gradient-to-r from-blue-600 to-indigo-600 text-white px-8 py-3 rounded-lg font-medium shadow-sm hover:bg-blue-50 w-full max-w-xs">
                    Continue Learning
                  </button>
                </div>
      
    
  </Card>
</section>

  

<div class="space-y-4">
  <h2 class="text-xl font-semibold">Recent Decks</h2>
  <div class="flex flex-row gap-4">
    {#each recentDecks as deck}
      <a 
        href="/decks/{deck.id}"
        class="p-4 rounded-xl border bg-card hover:bg-accent/50 transition-colors"
      >
        <div class="flex justify-between items-start mb-2">
          <div>
            <h3 class="font-semibold">{deck.name}</h3>
            <p class="text-sm text-muted-foreground">{deck.description}</p>
          </div>
          <span class="text-sm font-medium bg-primary/10 text-primary px-2 py-1 rounded">
            {deck.language}
          </span>
        </div>
        
        <div class="space-y-2">
          <div class="flex justify-between text-sm">
            <span>{deck.cardCount} cards</span>
            <span>{deck.progress}% complete</span>
          </div>
          <div class="w-full bg-secondary rounded-full h-2">
            <div 
              class="bg-primary rounded-full h-2" 
              style="width: {deck.progress}%"
            ></div>
          </div>
        </div>
      </a>
    {/each}
  </div>
</div>


  <form class="row" onsubmit={getCards}>
    <input id="greet-input" placeholder="Enter a name..." bind:value={name} />
    <button type="submit">Greet</button>
  </form>
  <p>{greetMsg}</p>
  <p>{cards}</p>
</main>
