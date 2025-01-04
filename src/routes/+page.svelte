<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import * as Table from "$lib/components/ui/table";
    import { Progress } from "$lib/components/ui/progress/index.js";
    import Separator from "$lib/components/ui/separator/separator.svelte";
    import RarityBadge from "$lib/assets/rarity-badge.svg";
    import MockMonsterImage from "$lib/assets/monster-placeholder-light.png";
    import type { DexMon, PartyMon } from "$lib/types";

    let dexMonsters = $state<DexMon[]>([]);
    let partyMonsters = $state<PartyMon[]>([]);
    let selectedMonster = $state<PartyMon>();

    async function loadParty() {
        const [dexResult, partyResult] = await Promise.all([
            invoke<DexMon[]>("get_dex"),
            invoke<string>("get_party"),
        ]);

        dexMonsters = dexResult;
        partyMonsters = JSON.parse(partyResult);
        selectedMonster = partyMonsters[0];
    }

    function handleKeydown(event: KeyboardEvent) {
        const currentIndex = partyMonsters.findIndex(
            (mon) => mon === selectedMonster,
        );

        if (event.key === "ArrowUp" && currentIndex > 0) {
            selectedMonster = partyMonsters[currentIndex - 1];
        } else if (
            event.key === "ArrowDown" &&
            currentIndex < partyMonsters.length - 1
        ) {
            selectedMonster = partyMonsters[currentIndex + 1];
        }
    }

    $effect(() => {
        loadParty();
    });
</script>

<svelte:window on:keydown={handleKeydown} />

<main class="h-screen relative w-screen flex overflow-hidden">
    {#if dexMonsters.length > 0 && partyMonsters.length > 0 && selectedMonster}
        <aside class="w-64 overflow-y-auto">
            <Table.Root>
                <Table.Header class="[&_tr]:border-0">
                    <Table.Row class="hover:bg-muted/0">
                        <Table.Head class="w-2 pr-0">#</Table.Head>
                        <Table.Head>Name</Table.Head>
                    </Table.Row>
                </Table.Header>
                <Table.Body>
                    {#each partyMonsters as mon}
                        <Table.Row
                            data-state={selectedMonster === mon
                                ? "selected"
                                : undefined}
                            on:click={() => (selectedMonster = mon)}
                        >
                            <Table.Cell class="w-2 pr-0">
                                {mon.dex_id}
                            </Table.Cell>
                            <Table.Cell>
                                {dexMonsters.find((m) => m.id === mon.dex_id)
                                    ?.name}
                            </Table.Cell>
                        </Table.Row>
                    {/each}

                    {#each dexMonsters.filter((d) => !partyMonsters.some((p) => p.dex_id === d.id)) as mon}
                        <Table.Row class="text-muted-foreground/60">
                            <Table.Cell class="w-2 pr-0">{mon.id}</Table.Cell>
                            <Table.Cell>???</Table.Cell>
                        </Table.Row>
                    {/each}
                </Table.Body>
            </Table.Root>
        </aside>

        <content
            class="w-full rounded-ss flex flex-col h-screen bg-primary/4 overflow-y-auto text-sm"
        >
            <section
                class="px-4 p-8 h-[256px] flex items-center justify-center"
            >
                <img
                    src={MockMonsterImage}
                    alt="Mock Monster Artwork"
                    class="h-full"
                />
            </section>

            <section class="px-4 py-2 flex w-full justify-between items-center">
                <div class="text-muted-foreground">Name</div>
                <div>
                    {dexMonsters.find((m) => m.id === selectedMonster?.dex_id)
                        ?.name}
                </div>
            </section>

            <section class="px-4 py-2 flex w-full justify-between items-center">
                <div class="text-muted-foreground">Family</div>
                <div>
                    {dexMonsters.find((m) => m.id === selectedMonster?.dex_id)
                        ?.family}
                </div>
            </section>

            <section class="px-4 py-2 flex w-full justify-between items-center">
                <div class="text-muted-foreground">Rarity</div>
                <div class="flex gap-1">
                    {#each Array(dexMonsters.find((m) => m.id === selectedMonster?.dex_id)?.rarity) as _}
                        <img src={RarityBadge} alt="Rarity Badge Artwork" />
                    {/each}
                </div>
            </section>

            <Separator class="my-2 bg-primary/5" />

            <section class="px-4 py-2 flex w-full justify-between items-center">
                <div class="text-muted-foreground">Level</div>
                <div>
                    {selectedMonster.level}
                </div>
            </section>

            <section class="px-4 py-2 flex w-full justify-between items-center">
                <div class="text-muted-foreground">Exp.</div>
                <div>
                    <Progress
                        value={selectedMonster.experience_range[0]}
                        max={selectedMonster.experience_range[1]}
                        class="w-[128px] bg-primary/10 rounded-none"
                    />
                </div>
            </section>
        </content>
    {/if}
</main>
