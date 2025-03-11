<script lang="ts">
  import Timeline from "$components/timeline/Component.svelte";
  import { onMount } from "svelte";
  import type { PageProps } from "./$types";

  const { data }: PageProps = $props();
  const timeline: Array<Array<string>> = $state([]);

  onMount(() => {
    if (!data.timeline) return;
    //TODO:Move this logic to the backend
    timeline.push(data.timeline.days.Sun);
    timeline.push(data.timeline.days.Mon);
    timeline.push(data.timeline.days.Tue);
    timeline.push(data.timeline.days.Wed);
    timeline.push(data.timeline.days.Thu);
    timeline.push(data.timeline.days.Fri);
    timeline.push(data.timeline.days.Sat);
  });
</script>

{#if data.timeline}
  {#if data.habits.length}
    <div class="space-y-6">
      {#each data.habits as habit}
        <Timeline {timeline} {habit} userSettings={data.userSettings} />
      {/each}
    </div>
  {:else}
    <div>No Habits</div>
  {/if}
{/if}
{#if data.error}
  <div class="flex-center justify-center p-4">
    <p class="text-sm text-rose-400">{data.error}</p>
  </div>
{/if}
