<script lang="ts">
  import type { PageProps } from "./$types";
  import Button from "$components/ui/Button.svelte";
  import TrashIcon from "$components/icons/TrashIcon.svelte";
  import RestoreIcon from "$components/icons/RestoreIcon.svelte";
  import { restoreArchivedHabit } from "$actions/timeline";
  import ArchiveIcon from "$components/icons/ArchiveIcon.svelte";
  import DeleteHabit from "$components/global/DeleteHabit.svelte";

  const { data }: PageProps = $props();
</script>

<div class="">
  <div class="flex-center gap-2">
    <span><ArchiveIcon /></span>
    <h1 class="text-xl">Archived Habits</h1>
  </div>
  {#if data.habits && data.habits.length > 0}
    {#each data.habits as habit}
      <div
        class="w-timeline flex-center mx-auto mt-6 mb-2 justify-between rounded bg-amber-600 p-2"
      >
        <div>
          <p class="text-xs">{habit.title}</p>
          <p class="text-[10px]">{habit.description}</p>
        </div>
        <div class="flex-center gap-2">
          <Button intent="primary" size="icon" onclick={() => restoreArchivedHabit(habit.id)}>
            <RestoreIcon />
          </Button>
          <DeleteHabit archive id={habit.id}>
            <Button intent="primary" size="icon">
              <TrashIcon />
            </Button>
          </DeleteHabit>
        </div>
      </div>
    {/each}
  {:else}
    <p class="text-warning text-sm">You have no archived habits</p>
  {/if}
</div>
