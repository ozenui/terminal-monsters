<script lang="ts">
    import type { Props } from "./index.js";
    import { cn } from "$lib/components/utils";
    import { open } from "@tauri-apps/plugin-shell";
    import DiscordIcon from "$lib/assets/discord-icon.svg";
    import BlueSkyIcon from "$lib/assets/bluesky-icon.svg";
    import GitHubIcon from "$lib/assets/github-icon.svg";

    type $$Props = Props;

    let className: $$Props["class"] = undefined;

    export { className as class };

    const DISCORD_URL = "";
    const BLUESKY_URL = "https://bsky.app/profile/ozenui.com";
    const GITHUB_URL = "https://github.com/ozenui/terminal-monsters";

    const openLink = async (url: string) => {
        try {
            await open(url);
        } catch (err) {
            console.error("Failed to open link:", err);
        }
    };
</script>

<header
    class={cn(
        "flex px-4 select-none pl-[72px] h-8 text-xs items-center justify-between",
        className,
    )}
    data-tauri-drag-region
    {...$$restProps}
>
    <div
        class="flex items-center cursor-default justify-center gap-2"
        data-tauri-drag-region
    >
        Terminal Monsters
    </div>
    <div
        class="flex opacity-20 items-center justify-center gap-3"
        data-tauri-drag-region
    >
        <button onclick={() => openLink(DISCORD_URL)}>
            <img src={DiscordIcon} alt="Discord" />
        </button>
        <button onclick={() => openLink(BLUESKY_URL)}>
            <img src={BlueSkyIcon} alt="BlueSky" />
        </button>
        <button onclick={() => openLink(GITHUB_URL)}>
            <img src={GitHubIcon} alt="GitHub" />
        </button>
    </div>
</header>
