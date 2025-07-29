<script lang="ts">
    import "../app.css";
    import { TitleBar } from "$lib/components/ui/title-bar";

    let { children } = $props();
</script>

<main class="h-screen bg-background/75 w-screen flex flex-col overflow-hidden">
    <TitleBar></TitleBar>
    {@render children()}
</main>
