<script lang="ts">
    import Timeline from "../components/Timeline.svelte";
    import LanguageStats from "../components/LanguageStats.svelte";
    import { onMount } from "svelte";
    /*import { LanguageRecord } from "../types";*/

    /*let language_data: Promise<LanguageRecord[]>;*/

    /*async function get_language_stats(): Promise<LanguageRecord[]> {*/
    /*const response = await fetch("/api/language_stats");*/
    /*return response.json();*/
    /*}*/

    const langs = [
        { language: "C++", hits: 1 },
        { language: "TeX", hits: 1 },
        { language: "Vim Snippet", hits: 1 },
        { language: "CSS", hits: 1 },
        { language: "VBA", hits: 1 },
        { language: "Visual Basic", hits: 1 },
        { language: "Vim script", hits: 2 },
        { language: "Go", hits: 2 },
        { language: "Shell", hits: 2 },
        { language: "C", hits: 3 },
        { language: "C#", hits: 5 },
        { language: "Java", hits: 5 },
        { language: "Ruby", hits: 7 },
        { language: "TypeScript", hits: 8 },
        { language: "HTML", hits: 8 },
        { language: "Python", hits: 12 },
        { language: "JavaScript", hits: 15 },
    ]
        .reverse()
        .map((x) => {
            if (x.language === "HTML") {
                x.language = "Svelte";
            }
            return x;
        });

    onMount(() => {
        /*language_data = get_language_stats();*/
    });

    /*{#await language_data    <Timeline />*/
    /*Waiting...*/
    /*{:then data}*/
    /*<LanguageStats data="{data}"/>*/
    /*{:catch}*/
    /*Error...*/
    /*{/await}*/
</script>

<div class="container">
    <LanguageStats language_records="{langs}" />
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
