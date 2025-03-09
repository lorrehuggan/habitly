<script lang="ts">
  import type { PageProps } from "./$types";
  import { Dialog } from "bits-ui";
  import Trash from "$components/icons/Trash.svelte";
  import Calendar from "$components/icons/Calendar.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { redirect } from "@sveltejs/kit";
  import { goto } from "$app/navigation";
  let { data }: PageProps = $props();
  let isOpen = $state(false);

  async function deleteHabit(id: string) {
    const response: { message: string; error: boolean } = await invoke("delete_habit", { id });

    if (response.message === "success") {
      console.log(response);
      goto("/");
    }
  }
</script>

{#snippet dialog(id: string)}
  <Dialog.Root bind:open={isOpen}>
    <Dialog.Trigger>
      <button
        aria-label="delete habit"
        class="hover:bg-error cursor-pointer rounded bg-neutral-800 p-1 transition-colors"
      >
        <Trash />
      </button>
    </Dialog.Trigger>
    <Dialog.Portal>
      <Dialog.Overlay class="fixed inset-0 z-50 rounded-xl bg-black/80" />
      <Dialog.Content
        class="bg-bg border-fg/30 fixed top-[50%] left-[50%] z-50 w-full max-w-[calc(100%-2rem)] translate-x-[-50%] translate-y-[-50%] rounded-xl border-[1px] p-5 shadow outline-hidden sm:max-w-[490px] md:w-full"
      >
        <Dialog.Title>Are you sure you want to delete?</Dialog.Title>
        <Dialog.Description class="text-error text-sm"
          >This action cannot be undone</Dialog.Description
        >
        <div class="flex-center mt-8 justify-end gap-2">
          <button aria-label="cancel">Cancel</button>
          <button aria-label="archive">Archive</button>
          <button aria-label="delete" onclick={() => deleteHabit(id)}>Delete</button>
        </div>
      </Dialog.Content>
    </Dialog.Portal>
  </Dialog.Root>
{/snippet}

<div class="p-8">
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
        {@render dialog(data.habit.id)}
      </div>
    </div>
  {/if}
</div>
