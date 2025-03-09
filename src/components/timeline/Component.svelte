<script lang="ts">
  import Settings from "$components/icons/Settings.svelte";
  import Star from "$components/icons/Star.svelte";
  import StarFilled from "$components/icons/StarFilled.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import clsx from "clsx";
  import dayjs from "dayjs";
  import { onMount } from "svelte";
  import { actionGetCommits } from "../../actions/timeline";
  import type { Commit, Habit, UserSettings } from "../../types/timeline";

  let {
    timeline,
    habit,
    userSettings,
  }: { timeline: Array<Array<string>>; habit: Habit; userSettings?: UserSettings } = $props();
  let committedToday = $state(false);
  let commits: Commit[] | undefined = $state();
  let today = dayjs().format("YYYY-MM-DD");

  onMount(async () => {
    commits = await actionGetCommits(habit.id);
    const todaysCommit = commits?.find((commit) => commit.created === today);
    if (todaysCommit) {
      committedToday = true;
    }
  });

  async function addCommit(id: string) {
    switch (committedToday) {
      case false:
        committedToday = true;
        try {
          await invoke("create_commit", { id });
        } catch (e) {
          console.log(e);
          committedToday = false;
        }
        break;
      case true:
        committedToday = false;
        try {
          const commits = await actionGetCommits(id);
          const todaysCommit = commits?.find((commit) => commit.created === today);
          todaysCommit && (await invoke("delete_commit", { id: todaysCommit.id }));
          console.log(commits);
        } catch (e) {
          console.log(e);
          committedToday = true;
        }
    }
  }
</script>

{#snippet commitHeader()}
  <div
    class="w-timeline flex-center mx-auto mt-6 mb-2 justify-between rounded bg-neutral-800/40 p-2"
  >
    <div>
      <p class="text-xs">{habit.title}</p>
      <p class="text-[10px]">{habit.description}</p>
    </div>
    <div class="flex-center gap-2">
      <a
        href={`habit/${habit.id}`}
        aria-label="habit settings"
        class="cursor-pointer rounded bg-neutral-800 p-1 transition-colors"
      >
        <Settings />
      </a>
      <button
        onclick={() => addCommit(habit.id)}
        aria-label="commit"
        class={clsx("cursor-pointer rounded bg-neutral-800 p-1 transition-colors", {
          "bg-primary": committedToday,
        })}
      >
        {#if committedToday}
          <StarFilled />
        {:else}
          <Star />
        {/if}
      </button>
    </div>
  </div>
{/snippet}

{#snippet commitNode(commit: Commit | undefined, node: string)}
  {@const isNodeToday = node === today}
  {@const isAfterToday = dayjs(node).isAfter(dayjs(today), "day")}
  {@const ongoing = commit?.status === "ongoing"}
  {@const completed = commit?.status === "completed"}
  {@const previousCommit = Boolean(commit)}
  <button
    aria-label="node"
    class={clsx("size-2 cursor-pointer rounded-[2px] transition-colors", {
      "bg-primary": isNodeToday && committedToday,
      "bg-primary/45": !isNodeToday && completed,
      "bg-primary/25": !isNodeToday && ongoing,
      "bg-neutral-800/40": !isNodeToday && !previousCommit,
      "opacity-80": isAfterToday,
      "bg-neutral-800": isNodeToday && !committedToday && !userSettings?.highlightCurrentDay,
      "bg-rose-400": isNodeToday && userSettings?.highlightCurrentDay && !committedToday,
    })}
    data-date={node}
  ></button>
{/snippet}

{#snippet commitTimeline()}
  {#each timeline as weekday, i}
    <div
      class={clsx("flex-center w-timeline mx-auto gap-1 ", {
        "mt-1": i !== 0,
      })}
    >
      {#each weekday as node}
        {@const commit = commits?.find((commit) => commit.created === node)}
        {@render commitNode(commit, node)}
      {/each}
    </div>
  {/each}
{/snippet}

<div class="">
  {@render commitHeader()}
  {@render commitTimeline()}
</div>
