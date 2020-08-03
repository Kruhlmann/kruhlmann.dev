<script lang="ts">
    import * as _language_colors from "../../config/language_colors.json";
    import { LanguageRecord, LanguageColors } from "../types";

    export let language_records: LanguageRecord[];

    const language_colors: LanguageColors = _language_colors;
    const user_base_url = `https://github.com/Kruhlmann/?tab=repositories`;
    const max_hits = Math.max(...language_records.map((record) => record.hits));

    function class_name(classname: string): string {
        return classname.replace(/[\W_]+/g, "-");
    }

    function get_language_color(language: string): string {
        const language_color_entry = language_colors[language];
        let background_color = "gray";
        if (language_color_entry) {
            background_color = language_color_entry.color || "gray";
        }
        return background_color;
    }

    function get_language_url(language: string) {
        const language_url_entry = language_colors[language];
        if (!language_url_entry) {
            return user_base_url;
        }

        const global_url = language_url_entry.url;
        const url_stub = global_url.split(/\?/)[1].replace(/^l=/, "");
        return `${user_base_url}&language=${url_stub}`;
    }

    function make_bar_style(language: string, hits: number) {
        return `
            width: ${(hits / max_hits) * 100}%;
            height: 100%;
            background-color: ${get_language_color(language)};
        `;
    }
</script>

<div class="container">
    {#each language_records as language_record}
        <a
            class="language {class_name(language_record.language)}"
            href="{get_language_url(language_record.language)}"
        >
            {language_record.language}
        </a>
        <div class="divider"></div>
        <div
            class="hits-count {class_name(language_record.language)}"
            style="color: {get_language_color(language_record.language)};"
        >
            {language_record.hits}
        </div>
        <div class="hits-wrapper">
            <div
                class="hits-bar {class_name(language_record.language)}"
                style="{make_bar_style(language_record.language, language_record.hits)}"
            ></div>
        </div>
    {/each}
</div>

<style lang="scss">
    .container {
        display: grid;
        grid-template-columns: auto 2px 35px 1fr;
        grid-auto-rows: 30px;
        grid-row-gap: 10px;
        width: 30%;
        font-size: 24px;

        * {
            line-height: 1.5em;
            color: inherit;
            text-decoration: none;
        }

        a:hover {
            text-decoration: underline;
        }

        .hits-count {
            text-align: right;
            padding-right: 5px;
        }

        .hits-wrapper > div {
            border-top-right-radius: 15px;
            border-bottom-right-radius: 15px;
            padding-left: 50px;
            box-sizing: border-box;
        }
    }
</style>
