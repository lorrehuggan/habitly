<script lang="ts">
  import type { HTMLButtonAttributes } from "svelte/elements";
  import { cva, type VariantProps } from "class-variance-authority";

  const button = cva("rounded transition-colors cursor-pointer", {
    variants: {
      intent: {
        primary: "text-fg bg-neutral-800 hover:bg-neutral-900",
        secondary: "secondary",
        error: "text-fg bg-error hover:bg-error/50",
        muted: "text-fg hover:bg-neutral-800",
      },
      size: {
        button: "text-sm px-2 py-1",
        icon: "p-1",
      },
      disabled: {
        false: "opacity-100",
        true: "opacity-50 pointer-events-none",
      },
    },
    compoundVariants: [{ intent: "primary", size: "button", class: "primaryMedium" }],
  });

  interface $$Props extends HTMLButtonAttributes, VariantProps<typeof button> {}

  let { intent, size, disabled } = $$props;
</script>

<button {...$$props} class={button({ intent, size, disabled, class: $$props.class })} {disabled}>
  <slot />
</button>
