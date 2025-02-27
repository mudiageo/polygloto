<script lang="ts">
    import { Button } from "$lib/components/ui/button";
  
    const stats = {
      totalPoints: 1250,
      rank: 42,
      testsCompleted: 25,
      averageScore: 85
    };
  
    const achievements = [
      {
        name: "Quick Learner",
        description: "Complete 10 lessons",
        progress: 8,
        total: 10,
        icon: "ri-rocket-line"
      },
      {
        name: "Perfect Score",
        description: "Get 100% on any test",
        progress: 0,
        total: 1,
        icon: "ri-medal-line"
      }
    ];
  </script>
  
  <div class="space-y-6">
    <div class="flex items-center justify-between">
      <h1 class="text-2xl font-bold">Profile</h1>
      <Button variant="ghost" size="icon">
        <i class="ri-settings-4-line text-xl"></i>
      </Button>
    </div>
  
    <div class="flex items-center gap-4">
      <div class="w-20 h-20 bg-primary/10 rounded-full flex items-center justify-center">
        <i class="ri-user-3-line text-3xl"></i>
      </div>
      <div>
        <h2 class="text-xl font-semibold">Mudia</h2>
        <p class="text-sm text-muted-foreground">Student</p>
      </div>
    </div>
  
    <div class="grid grid-cols-2 gap-4">
      <div class="p-4 rounded-xl bg-card border">
        <div class="text-sm text-muted-foreground mb-1">Total Points</div>
        <div class="text-2xl font-bold">{stats.totalPoints}</div>
      </div>
      <div class="p-4 rounded-xl bg-card border">
        <div class="text-sm text-muted-foreground mb-1">Rank</div>
        <div class="text-2xl font-bold">#{stats.rank}</div>
      </div>
      <div class="p-4 rounded-xl bg-card border">
        <div class="text-sm text-muted-foreground mb-1">Tests Completed</div>
        <div class="text-2xl font-bold">{stats.testsCompleted}</div>
      </div>
      <div class="p-4 rounded-xl bg-card border">
        <div class="text-sm text-muted-foreground mb-1">Average Score</div>
        <div class="text-2xl font-bold">{stats.averageScore}%</div>
      </div>
    </div>
  
    <div class="space-y-4">
      <h2 class="text-xl font-semibold">Achievements</h2>
      {#each achievements as achievement}
        <div class="p-4 rounded-xl bg-card border">
          <div class="flex items-center gap-4">
            <div class="w-12 h-12 bg-primary/10 rounded-full flex items-center justify-center">
              <i class="{achievement.icon} text-2xl"></i>
            </div>
            <div class="flex-1">
              <h3 class="font-semibold">{achievement.name}</h3>
              <p class="text-sm text-muted-foreground">{achievement.description}</p>
            </div>
          </div>
          <div class="mt-4">
            <div class="flex justify-between text-sm mb-2">
              <span>{achievement.progress}/{achievement.total}</span>
              <span>{Math.round(achievement.progress/achievement.total * 100)}%</span>
            </div>
            <div class="w-full bg-secondary rounded-full h-2">
              <div 
                class="bg-primary rounded-full h-2" 
                style="width: {achievement.progress/achievement.total * 100}%"
              ></div>
            </div>
          </div>
        </div>
      {/each}
      
    </div>
  </div>