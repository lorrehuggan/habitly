<script lang="ts">
  import clsx from "clsx";
  import type { Commit, Habit } from "../../types/timeline";
  import { actionGetCommits } from "../../actions/timeline";
  import dayjs from "dayjs";
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

  let { timeline, habit }: { timeline: Array<Array<string>>; habit: Habit } = $props();
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
    class="w-timeline flex-center mx-auto mt-4 mb-2 justify-between rounded bg-neutral-800/20 p-2"
  >
    <div>
      <p class="text-xs">{habit.title}</p>
      <p class="text-[10px]">{habit.description}</p>
    </div>
    <div>
      <button
        onclick={() => addCommit(habit.id)}
        aria-label="commit"
        class={clsx("rounded bg-neutral-800 p-1 transition-colors", {
          "bg-primary": committedToday,
        })}
      >
        <svg
          width="15"
          height="15"
          viewBox="0 0 15 15"
          fill="none"
          xmlns="http://www.w3.org/2000/svg"
          class={clsx("transition-colors", {
            "text-bg": committedToday,
          })}
          ><path
            d="M11.4669 3.72684C11.7558 3.91574 11.8369 4.30308 11.648 4.59198L7.39799 11.092C7.29783 11.2452 7.13556 11.3467 6.95402 11.3699C6.77247 11.3931 6.58989 11.3355 6.45446 11.2124L3.70446 8.71241C3.44905 8.48022 3.43023 8.08494 3.66242 7.82953C3.89461 7.57412 4.28989 7.55529 4.5453 7.78749L6.75292 9.79441L10.6018 3.90792C10.7907 3.61902 11.178 3.53795 11.4669 3.72684Z"
            fill="currentColor"
            fill-rule="evenodd"
            clip-rule="evenodd"
          ></path></svg
        >
      </button>
    </div>
  </div>
{/snippet}

{#snippet commitNode(commit: Commit | undefined, node: string)}
  {@const isNodeToday = node === today}
  {@const isThisMonth = dayjs(node).isSame(dayjs(today), "month")}
  {@const isAfterToday = dayjs(node).isAfter(dayjs(today), "day")}
  {@const ongoing = commit?.status === "ongoing"}
  {@const completed = commit?.status === "completed"}
  {@const previousCommit = Boolean(commit)}
  {@debug isThisMonth, isNodeToday}
  <button
    aria-label="node"
    class={clsx("size-2 cursor-pointer rounded-[2px] transition-colors", {
      "bg-primary": isNodeToday && committedToday,
      "bg-neutral-800": isNodeToday && !committedToday,
      "bg-primary/45": !isNodeToday && completed,
      "bg-primary/25": !isNodeToday && ongoing,
      "bg-neutral-800/40": !isNodeToday && !previousCommit,
      "opacity-80": isAfterToday,
    })}
    data-date={node}
  ></button>
{/snippet}

{#snippet commitTimeline()}
  {#each timeline as weekday, i}
    <div
      class={clsx("flex-center w-timeline mx-auto gap-1", {
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

{#snippet commitFooter()}
  <div class="w-timeline m-2 mx-auto rounded bg-neutral-700/20 p-2">
    <div>
      <p class="text-xs"></p>
    </div>
  </div>
{/snippet}

<div class="">
  {@render commitHeader()}
  {@render commitTimeline()}
  {@render commitFooter()}
</div>
