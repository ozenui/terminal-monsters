<script lang="ts">
    import "../app.css";
    import { open } from "@tauri-apps/plugin-shell";
    import DiscordIcon from "$lib/assets/discord-icon.svg";
    import BlueSkyIcon from "$lib/assets/bluesky-icon.svg";
    import GitHubIcon from "$lib/assets/github-icon.svg";

    let { children } = $props();

    const openLink = async (url: string) => {
        try {
            await open(url);
        } catch (err) {
            console.error("Failed to open link:", err);
        }
    };
</script>

<main
    class="h-screen bg-background/75 relative w-screen flex flex-col overflow-hidden"
>
    <header
        class="flex px-4 select-none pl-[72px] h-8 text-xs items-center justify-between"
        data-tauri-drag-region
    >
        <div
            class="flex items-center cursor-default justify-center gap-2"
            data-tauri-drag-region
        >
            Terminal Monsters
        </div>
        <div
            class="flex opacity-20 user-select-none items-center justify-center gap-3"
            data-tauri-drag-region
        >
            <button
                onclick={() =>
                    openLink("https://github.com/ozenui/terminal-monsters")}
            >
                <img src={DiscordIcon} alt="Discord" />
            </button>
            <button
                onclick={() =>
                    openLink("https://github.com/ozenui/terminal-monsters")}
            >
                <img src={BlueSkyIcon} alt="BlueSky" />
            </button>
            <button
                onclick={() =>
                    openLink("https://github.com/ozenui/terminal-monsters")}
            >
                <img src={GitHubIcon} alt="GitHub" />
            </button>
        </div>
    </header>
    {@render children()}
</main>
