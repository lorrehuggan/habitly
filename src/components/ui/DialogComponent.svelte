<script>
  import Close from "$components/icons/Close.svelte";
  import { Dialog } from "bits-ui";
  let { trigger, body, title, description } = $props();
  let isOpen = $state(false);
</script>

<Dialog.Root bind:open={isOpen}>
  <Dialog.Trigger>
    {@render trigger()}
  </Dialog.Trigger>
  <Dialog.Portal>
    <Dialog.Overlay class="fixed inset-0 z-50 rounded-xl bg-black/80" />
    <Dialog.Content
      class="bg-bg border-fg/30 fixed top-[50%] left-[50%] z-50 w-full max-w-[calc(100%-2rem)] translate-x-[-50%] translate-y-[-50%] rounded-xl border-[1px] p-5 shadow outline-hidden sm:max-w-[490px] md:w-full"
    >
      <Dialog.Title>{title}</Dialog.Title>
      <Dialog.Description class="text-error text-sm">{description}</Dialog.Description>
      {@render body()}
      <Dialog.Close class="absolute top-4 right-4 cursor-pointer"><Close /></Dialog.Close>
    </Dialog.Content>
  </Dialog.Portal>
</Dialog.Root>
