<script lang="ts">
    import { page } from "$app/stores";
    import { Button } from "$lib/components/ui/button";
    import type { Flashcard } from "$lib/types";
  
    const deckId = $page.params.id;
    
    const cards: Flashcard[] = [
      {
        id: "1",
        deckId: "1",
        front: "Hello",
        back: "Hola",
        confidence: 3
      },
      {
        id: "2",
        deckId: "1",
        front: "Goodbye",
        back: "Adiós",
        confidence: 4
      }
    ];
  
    let currentCardIndex = 0;
    let showingFront = true;
  
    function flipCard() {
      showingFront = !showingFront;
    }
  
    function nextCard() {
      if (currentCardIndex < cards.length - 1) {
        currentCardIndex++;
        showingFront = true;
      }
    }
  
    function previousCard() {
      if (currentCardIndex > 0) {
        currentCardIndex--;
        showingFront = true;
      }
    }
  
    function updateConfidence(level: 1 | 2 | 3 | 4 | 5) {
      cards[currentCardIndex].confidence = level;
    }
  </script>
  
  <div class="space-y-6">
    <div class="flex items-center gap-4">
      <a href="/decks" class="text-muted-foreground hover:text-foreground">
        <i class="ri-arrow-left-line text-xl"></i>
      </a>
      <h1 class="text-2xl font-bold">Study Deck</h1>
    </div>
  
    <div class="flex flex-col items-center gap-6">
      <div 
        class="w-full max-w-md aspect-[3/2] perspective-[1000px]"
        onclick={flipCard}
      >
        <div class="relative w-full h-full transition-transform duration-500 transform-3d" 
             style="transform: rotateY({showingFront ? 0 : 180}deg)">
          <div class="absolute w-full h-full backface-hidden">
            <div class="w-full h-full p-6 rounded-xl border bg-amber-700 bg-card flex items-center justify-center">
              <p class="text-2xl">{cards[currentCardIndex].front}</p>
            </div>
          </div>
          <div class="absolute w-full h-full backface-hidden"
               style="transform: rotateY(180deg)">
            <div class="w-full h-full p-6 rounded-xl border bg-card flex items-center justify-center">
              <p class="text-2xl">{cards[currentCardIndex].back}</p>
            </div>
          </div>
        </div>
      </div>
  
      <div class="flex gap-4">
        <Button 
          variant="outline" 
          onclick={previousCard}
          disabled={currentCardIndex === 0}
        >
          <i class="ri-arrow-left-s-line mr-2"></i>
          Previous
        </Button>
        <Button 
          variant="outline"
          onclick={nextCard}
          disabled={currentCardIndex === cards.length - 1}
        >
          Next
          <i class="ri-arrow-right-s-line ml-2"></i>
        </Button>
      </div>
  
      <div class="flex gap-2">
        {#each [1, 2, 3, 4, 5] as level}
          <Button 
            variant="ghost" 
            size="sm"
            class="w-10 h-10 rounded-full {cards[currentCardIndex].confidence === level ? 'bg-primary text-primary-foreground' : ''}"
            onclick={() => updateConfidence(level)}
          >
            {level}
          </Button>
        {/each}
        
      </div>
  
      <div class="w-full max-w-md flex justify-between text-sm text-muted-foreground">
        <span>Card {currentCardIndex + 1} of {cards.length}</span>
        <span>Confidence Level: {cards[currentCardIndex].confidence}</span>
      </div>
    </div>
  </div>
  