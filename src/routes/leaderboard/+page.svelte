<script lang="ts">
    import { Button } from "$lib/components/ui/button";
  
    let currentView: 'challenge' | 'weekly' | 'daily' = 'challenge';
  
    const leaderboardData = [
      {
        rank: 1,
        name: "Motunrayo",
        totalCards: 313,
        points: 121.17,
        decksCompleted: 18,
        accuracy: 78,
        streak: 54
      },
      {
        rank: 2,
        name: "Prosam",
        totalCards: 342,
        points: 104.33,
        decksCompleted: 5,
        accuracy: 86,
        streak: 7
      },
      {
        rank: 3,
        name: "demigoddami",
        totalCards: 342,
        points: 101.67,
        decksCompleted: 4,
        accuracy: 86,
        streak: 21
      }
    ];
  </script>
  
  <div class="space-y-6">
    <div class="flex items-center gap-4">
      <a href="/" class="text-muted-foreground hover:text-foreground">
        <i class="ri-arrow-left-line text-xl"></i>
      </a>
      <h1 class="text-2xl font-bold">Leaderboard</h1>
    </div>
  
    <div class="flex gap-2 bg-secondary/50 p-1 rounded-full">
      {#each ['challenge', 'weekly', 'daily'] as view}
        <Button
          variant={currentView === view ? 'secondary' : 'ghost'}
          class="flex-1 rounded-full capitalize"
          on:click={() => currentView = view as typeof currentView}
        >
          {view}
        </Button>
      {/each}
    </div>
  
    <h2 class="text-xl font-semibold">Language Masters</h2>
  
    <div class="space-y-4">
      {#each leaderboardData as user}
        <div class="p-6 rounded-xl {user.rank === 1 ? 'bg-amber-50' : user.rank === 2 ? 'bg-gray-100' : 'bg-orange-50'}">
          <div class="flex items-center gap-4 mb-4">
            {#if user.rank === 1}
              <div class="w-10 h-10 flex items-center justify-center">
                <i class="ri-trophy-fill text-2xl text-amber-500"></i>
              </div>
            {:else}
              <div class="w-10 h-10 rounded-full bg-white flex items-center justify-center font-semibold">
                {user.rank}
              </div>
            {/if}
            
            <div class="flex-1">
              <div class="flex items-center gap-2">
                <i class="ri-user-3-line"></i>
                <span class="font-semibold">{user.name}</span>
              </div>
            </div>
            
            <div class="text-right">
              <div class="text-sm text-muted-foreground">Cards Mastered</div>
              <div class="font-semibold">{user.totalCards}</div>
            </div>
          </div>
  
          <div class="grid grid-cols-3 gap-4">
            <div class="bg-white rounded-lg p-3">
              <div class="flex items-center gap-2 text-sm text-muted-foreground mb-1">
                <i class="ri-coins-line"></i>
                <span>Learning Points</span>
              </div>
              <div class="font-semibold">{user.points} from {user.decksCompleted} decks</div>
            </div>
            
            <div class="bg-white rounded-lg p-3">
              <div class="flex items-center gap-2 text-sm text-muted-foreground mb-1">
                <i class="ri-star-line"></i>
                <span>Accuracy</span>
              </div>
              <div class="font-semibold">{user.accuracy}%</div>
            </div>
            
            <div class="bg-white rounded-lg p-3">
              <div class="flex items-center gap-2 text-sm text-muted-foreground mb-1">
                <i class="ri-fire-line"></i>
                <span>Streak</span>
              </div>
              <div class="font-semibold">{user.streak} days</div>
            </div>
          </div>
  
          <Button variant="outline" class="w-full mt-4">View Progress</Button>
        </div>
      {/each}
    </div>
  </div>