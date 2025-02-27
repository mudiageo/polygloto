<script lang="ts">
    import { Button } from "$lib/components/ui/button";
    import type { Deck } from "$lib/types";
  
    const decks: Deck[] = [
      {
        id: "1",
        name: "Basic Spanish",
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
  </script>
  
  <div class="space-y-6">
    <div class="flex items-center justify-between">
      <h1 class="text-2xl font-bold">My Decks</h1>
      <Button variant="outline">
        <i class="ri-add-line mr-2"></i>
        New Deck
      </Button>
    </div>
  
    <div class="grid gap-4">
      {#each decks as deck}
        <a 
          href="/decks/{deck.id}"
          class="block p-4 rounded-lg border bg-card hover:bg-accent/50 transition-colors"
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