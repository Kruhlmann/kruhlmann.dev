<script lang="ts">
    import Timeline from "../components/Timeline.svelte";
    import LanguageStats from "../components/LanguageStats.svelte";
    import { onMount } from "svelte";
    import { LanguageRecord } from "../types";
    import { ContainerInfo } from "../types/index";
    import DockerContainer from "../components/DockerContainer.svelte";

    let language_records: LanguageRecord[];
    let containers: ContainerInfo[];

    async function get_language_stats(): Promise<LanguageRecord[]> {
        const response = await fetch("/api/language_stats");
        const json = await response.json();
        if (response.ok) {
            return json;
        } else {
            throw new Error(json);
        }
    }

    async function get_containers(): Promise<ContainerInfo[]> {
        const response = await fetch("/api/docker");
        const json = await response.json();
        if (response.ok) {
            return json;
        } else {
            throw new Error(json);
        }
    }

    onMount(async () => {
        const promises: Promise<any>[] = [];

        promises.push(
            get_language_stats().then((response: LanguageRecord[]) => {
                language_records = response;
            }),
        );
        promises.push(
            get_containers().then((response: ContainerInfo[]) => {
                containers = response;
            }),
        );

        await Promise.all(promises);
    });
</script>

<div class="container">
    <div class="top">
        {#if language_records}
            <LanguageStats language_records="{language_records}" />
        {:else}Waiting...{/if}

        {#if containers}
            <div class="docker-containers">
                {#each containers as container}
                    <DockerContainer container="{container}" />
                {/each}
            </div>
        {:else}Waiting...{/if}
    </div>
    <div class="spacer"></div>
    <Timeline />
</div>

<style lang="scss">
    .container {
        width: 100%;
        display: flex;
        flex-direction: column;
        align-items: center;
        margin-top: 75px;

        .top {
            display: grid;
            grid-template-columns: 35% 65%;
            width: 80%;

            .docker-containers {
                display: grid;
                grid-template-columns: 1fr 1fr 1fr;
                grid-template-rows: 85px;
                grid-column-gap: 20px;
                grid-row-gap: 10px;
            }
        }

        .spacer {
            height: 50px;
        }
    }
</style>
