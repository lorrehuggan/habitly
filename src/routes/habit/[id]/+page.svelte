<script lang="ts">
  import type { PageProps } from "./$types";
  import Trash from "$components/icons/Trash.svelte";
  import Calendar from "$components/icons/Calendar.svelte";
  import DeleteHabit from "$components/global/DeleteHabit.svelte";
  let { data }: PageProps = $props();
</script>

<div>
  {#if data.habit}
    <div class="flex-center justify-between">
      <div>
        <h1 class="text-2xl">{data.habit.title}</h1>
        <h3 class="text-neutral-400">{data.habit.description}</h3>
      </div>
      <div class="flex-center gap-2">
        <button aria-label="view calendar" class="rounded bg-neutral-800 p-1">
          <Calendar />
        </button>
        <DeleteHabit id={data.habit.id}>
          <button
            aria-label="delete habit"
            class="hover:bg-error cursor-pointer rounded bg-neutral-800 p-1 transition-colors"
          >
            <Trash />
          </button>
        </DeleteHabit>
      </div>
    </div>
  {/if}
</div>
