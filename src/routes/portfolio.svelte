<script lang="ts">
    import Timeline from "../components/Timeline.svelte";
    import LanguageStats from "../components/LanguageStats.svelte";
    import { onMount } from "svelte";
    import { LanguageRecord } from "../types";

    let language_records: LanguageRecord[];

    async function get_language_stats(): Promise<LanguageRecord[]> {
        const response = await fetch("/api/language_stats");
        const json = await response.json();
        if (response.ok) {
            return json;
        } else {
            throw new Error(json);
        }
    }

    onMount(async () => {
        language_records = await get_language_stats();
    });
</script>

<div class="container">
    {#if language_records}
        <LanguageStats language_records="language_records" />
    {:else}Waiting...{/if}
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

        .spacer {
            height: 50px;
        }
    }
</style>
