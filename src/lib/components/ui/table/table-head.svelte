<script lang="ts">
    import type { HTMLThAttributes } from "svelte/elements";
    import { cn } from "$lib/components/utils.js";

    type $$Props = HTMLThAttributes;

    let className: $$Props["class"] = undefined;
    export { className as class };
</script>

<th
    class={cn(
        "text-muted-foreground/50 h-12 px-4 text-left align-middle font-normal [&:has([role=checkbox])]:pr-0",
        className,
    )}
    {...$$restProps}
>
    <slot />
</th>
