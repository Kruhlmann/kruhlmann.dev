<script lang="ts">
    import Timeline from "../components/Timeline.svelte";
    import LanguageTile from "../components/LanguageTile.svelte";
    import { onMount } from "svelte";
    import { LanguageRecord, LanguageColors, Technology } from "../types";
    import { ContainerInfo } from "../types/index";
    import DockerContainer from "../components/DockerContainer.svelte";
    import * as _language_colors from "../../config/language_colors.json";
    import config from "../../config/config.json";

    const language_colors: LanguageColors = _language_colors;
    const user_base_url = `https://github.com/Kruhlmann/?tab=repositories`;

    let language_records: LanguageRecord[];
    let containers: ContainerInfo[];
    let total_hits: number;

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

    function get_language_url(language: string): string {
        const language_url_entry = language_colors[language];
        if (!language_url_entry) {
            return user_base_url;
        }

        const global_url = language_url_entry.url;
        const url_stub = global_url.split(/\?/)[1].replace(/^l=/, "");
        return `${user_base_url}&language=${url_stub}`;
    }

    function language_class_name(language: string): string {
        return language.replace(/[\W_]+/g, "-");
    }

    function get_language_color(language: string): string {
        const language_color_entry = language_colors[language];
        let background_color = "gray";
        if (language_color_entry) {
            background_color = language_color_entry.color || "gray";
        }
        return background_color;
    }

    function get_language_abbreviation(language: string): string {
        const technology = config.technologies.find(
            (technology: Technology) => {
                return technology.name.toLowerCase() === language.toLowerCase();
            },
        );

        if (technology) {
            return technology.icon;
        }

        return language.slice(0, 2);
    }

    function language_is_ignored(language: string): boolean {
        return config.excluded_technologies.includes(language);
    }

    onMount(async () => {
        const promises: Promise<any>[] = [];

        promises.push(
            get_language_stats().then((response: LanguageRecord[]) => {
                language_records = response;
                total_hits = language_records.reduce((total, record) => {
                    return total + record.hits;
                }, 0);
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
        <h1>GitHub Projects</h1>
        <h1>Active Docker Containers</h1>
        <div class="language-tiles">
            {#if language_records}
                {#each language_records as language_record}
                    {#if !language_is_ignored(language_record.language)}
                        <LanguageTile
                            abbreviation="{get_language_abbreviation(language_record.language)}"
                            href="{get_language_url(language_record.language)}"
                            classname="{language_class_name(language_record.language)}"
                            color="{get_language_color(language_record.language)}"
                            usage="{(language_record.hits / total_hits) * 100}"
                        />
                    {/if}
                {/each}
            {:else}
                {#each Array(16).fill(0) as _}
                    <LanguageTile
                        abbreviation="?"
                        color="#2c2c2c"
                        href=""
                        classname=""
                        usage="{-1}"
                    />
                {/each}
            {/if}
        </div>

        <div class="docker-containers">
            {#if containers}
                {#each containers as container}
                    <DockerContainer container="{container}" />
                {/each}
            {:else}
                {#each Array(4).fill(0) as _}
                    <DockerContainer
                        container="{{ Names: ['/loading...'], State: 'running' }}"
                    />
                {/each}
            {/if}
        </div>
    </div>
    <div class="spacer"></div>
    <h1>My Timeline</h1>
    <Timeline />
</div>

<style lang="scss">
    .container {
        width: 100%;
        display: flex;
        flex-direction: column;
        align-items: center;
        margin-top: 75px;

        h1 {
            text-align: center;
        }

        .top {
            display: grid;
            grid-template-columns: 1fr 1fr;
            grid-column-gap: 25px;
            width: 100%;
            box-sizing: border-box;
            padding: 0 25px;

            .docker-containers {
                display: grid;
                grid-template-columns: 1fr 1fr;
                grid-template-rows: repeat(auto-fill, 100px);
                grid-column-gap: 10px;
                grid-row-gap: 10px;
            }

            .language-tiles {
                display: grid;
                grid-template-columns: repeat(auto-fill, 100px);
                grid-template-rows: repeat(auto-fill, 100px);
                grid-column-gap: 10px;
                grid-row-gap: 10px;
            }
        }

        .spacer {
            height: 50px;
        }
    }
</style>
