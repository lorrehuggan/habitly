<script>
  import { Window } from "@tauri-apps/api/window";
  const appWindow = new Window("main");
</script>

<div
  data-tauri-drag-region
  class="flex-center h-9 w-screen justify-between gap-2 bg-neutral-900 px-3"
>
  <div class="flex-center w-[50px] justify-center text-center">
    <p class="text-center text-[10px] tracking-wide text-neutral-600 uppercase">habitly</p>
  </div>
  <div class="flex-center gap-2">
    <button
      aria-label="minimize"
      onclick={() => appWindow.minimize()}
      class="transition-color rounded p-1 hover:cursor-pointer hover:bg-neutral-700"
    >
      <svg width="15" height="15" viewBox="0 0 15 15" fill="none" xmlns="http://www.w3.org/2000/svg"
        ><path
          d="M2.25 7.5C2.25 7.22386 2.47386 7 2.75 7H12.25C12.5261 7 12.75 7.22386 12.75 7.5C12.75 7.77614 12.5261 8 12.25 8H2.75C2.47386 8 2.25 7.77614 2.25 7.5Z"
          fill="currentColor"
          fill-rule="evenodd"
          clip-rule="evenodd"
        ></path></svg
      >
    </button>
    <button
      aria-label="close"
      onclick={() => appWindow.close()}
      class="transition-color rounded p-1 hover:cursor-pointer hover:bg-neutral-700"
    >
      <svg width="15" height="15" viewBox="0 0 15 15" fill="none" xmlns="http://www.w3.org/2000/svg"
        ><path
          d="M12.8536 2.85355C13.0488 2.65829 13.0488 2.34171 12.8536 2.14645C12.6583 1.95118 12.3417 1.95118 12.1464 2.14645L7.5 6.79289L2.85355 2.14645C2.65829 1.95118 2.34171 1.95118 2.14645 2.14645C1.95118 2.34171 1.95118 2.65829 2.14645 2.85355L6.79289 7.5L2.14645 12.1464C1.95118 12.3417 1.95118 12.6583 2.14645 12.8536C2.34171 13.0488 2.65829 13.0488 2.85355 12.8536L7.5 8.20711L12.1464 12.8536C12.3417 13.0488 12.6583 13.0488 12.8536 12.8536C13.0488 12.6583 13.0488 12.3417 12.8536 12.1464L8.20711 7.5L12.8536 2.85355Z"
          fill="currentColor"
          fill-rule="evenodd"
          clip-rule="evenodd"
        ></path></svg
      >
    </button>
  </div>
</div>
