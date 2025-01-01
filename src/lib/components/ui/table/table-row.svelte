<script lang="ts">
    import type { HTMLAttributes } from "svelte/elements";
    import { cn } from "$lib/components/utils.js";

    type $$Props = HTMLAttributes<HTMLTableRowElement> & {
        "data-state"?: unknown;
    };

    let className: $$Props["class"] = undefined;
    export { className as class };
</script>

<tr
    class={cn(
        "hover:bg-muted/40 border-l-[6px] border-primary border-opacity-0 data-[state=selected]:bg-muted/40 data-[state=selected]:border-opacity-100 data-[state=selected]:border-l-[6px] data-[state=selected]:border-primary cursor-default transition-colors",
        className,
    )}
    {...$$restProps}
    on:click
    on:keydown
>
    <slot />
</tr>
