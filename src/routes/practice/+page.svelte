<script lang="ts">
    import { Button } from "$lib/components/ui/button";
    import type { Deck } from "$lib/types";
  
    const recentDecks: Deck[] = [
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
  
    const practiceTypes = [
      {
        name: "Quick Review",
        description: "5-minute practice session",
        icon: "ri-timer-line",
        color: "bg-blue-500"
      },
      {
        name: "Challenge Mode",
        description: "Test your knowledge",
        icon: "ri-trophy-line",
        color: "bg-amber-500"
      },
      {
        name: "Spaced Repetition",
        description: "Review cards based on confidence",
        icon: "ri-brain-line",
        color: "bg-green-5"
      }
    ];
  </script>
  
  <div class="space-y-6">
    <div class="flex items-center justify-between">
      <h1 class="text-2xl font-bold">Practice</h1>
      <Button variant="outline">
        <i class="ri-history-line mr-2"></i>
        History
      </Button>
    </div>
  
    <div class="space-y-4">
      <h2 class="text-lg font-semibold">Practice Modes</h2>
      <div class="grid gap-4">
        {#each practiceTypes as type}
          <div class="p-4 rounded-xl border bg-card hover:bg-accent/50 transition-colors">
            <div class="flex items-center gap-4">
              <div class="w-12 h-12 {type.color} rounded-full flex items-center justify-center">
                <i class="{type.icon} text-2xl text-white"></i>
              </div>
              <div class="flex-1">
                <h3 class="font-semibold">{type.name}</h3>
                <p class="text-sm text-muted-foreground">{type.description}</p>
              </div>
              <Button variant="ghost" size="icon">
                <i class="ri-arrow-right-s-line text-xl"></i>
              </Button>
            </div>
          </div>
        {/each}
      </div>
    </div>
  
    <div class="space-y-4">
      <h2 class="text-lg font-semibold">Recent Decks</h2>
      <div class="grid gap-4">
        {#each recentDecks as deck}
          <a 
            href="/decks/{deck.id}"
            class="block p-4 rounded-xl border bg-card hover:bg-accent/50 transition-colors"
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
  
    <div class="rounded-xl border bg-card p-6">
      <h2 class="text-lg font-semibold mb-4">Quick Actions</h2>
      <div class="grid grid-cols-2 gap-4">
        <Button variant="outline" class="h-auto py-4 flex flex-col gap-2">
          <i class="ri-shuffle-line text-2xl"></i>
          <span>Random Cards</span>
        </Button>
        <Button variant="outline" class="h-auto py-4 flex flex-col gap-2">
          <i class="ri-time-line text-2xl"></i>
          <span>Review Due</span>
        </Button>
      </div>
      
      </div>
      
      </div>