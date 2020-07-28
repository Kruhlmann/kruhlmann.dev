<script lang="ts">
    import { onMount } from "svelte";
    import { Technology } from "../types/index";
    import * as config from "../../config/config.json";
    import { show_contact_modal } from "../lib/stores";

    const languages = config.technologies
        .filter((technology: Technology) => {
            return technology.show_on_index;
        })
        .map((technology: Technology) => {
            return technology.name;
        });

    let grace_period_active = false;
    let language_slot_growing = true;
    let language_slot_length = 0;
    let language_index = 0;

    function grow_language_slot(current_language: string): void {
        language_slot_length++;

        if (language_slot_length >= current_language.length) {
            grace_period_active = true;
            language_slot_growing = false;

            setTimeout(() => {
                grace_period_active = false;
            }, config.language_slot_grace_timeout);
        }
    }

    function shrink_language_slot(): void {
        language_slot_length--;

        if (language_slot_length === 0) {
            grace_period_active = true;
            language_slot_growing = true;
            language_index = (language_index + 1) % languages.length;

            setTimeout(() => {
                grace_period_active = false;
            }, config.language_slot_grace_timeout / 2);
        }
    }

    function update_language_slot(): void {
        if (!grace_period_active) {
            const current_language = languages[language_index];

            if (language_slot_growing) {
                grow_language_slot(current_language);
            } else {
                shrink_language_slot();
            }
        }
    }

    function on_document_ready(): void {
        setInterval(update_language_slot, config.language_slot_update_interval);
    }

    function get_current_language(index: number, length: number): string {
        return languages[index].substring(0, length);
    }

    $: lang_string = get_current_language(language_index, language_slot_length);

    onMount(on_document_ready);
</script>

<svelte:head>
    <title>Kruhlmann | Home</title>
</svelte:head>

<main>
    <div class="title">
        <span>I'm a</span>
        <span class="current-language">{lang_string}</span>
        <span class="underscore">_</span>
        <span>developer</span>
    </div>
    <div class="btn-grp">
        <span on:click="{() => show_contact_modal.set(true)}">Contact</span>
        <a href="/resume.pdf" download>Resume</a>
    </div>
</main>

<style lang="scss">
    @import "../scss/themes";

    @keyframes blink {
        0% {
            opacity: 1;
        }
        50% {
            opacity: 1;
        }
        51% {
            opacity: 0;
        }
        100% {
            opacity: 0;
        }
    }

    .title {
        font-size: 1em;
    }

    main {
        flex: 1;
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        box-sizing: border-box;

        * {
            @include themify() {
                color: themed(keyword-color);
            }
        }

        .title {
            display: flex;
            align-items: center;
            justify-content: center;
            font-size: 56px;
            font-weight: bold;

            .current-language,
            .underscore {
                font-style: italic;
                text-transform: uppercase;
                @include themify() {
                    color: themed(comment-color);
                }
            }

            .underscore {
                font-style: normal;
                animation: blink 1.1s infinite;
            }

            *:not(.underscore):not(.current-language) {
                margin: 0 35px;
            }
        }

        .btn-grp {
            margin-top: 30px;
            display: flex;

            a,
            span {
                padding: 15px;
                text-transform: uppercase;
                border: 2px solid;
                text-decoration: none;
                cursor: pointer;
                font-weight: bold;
                transition: color 0.3s ease-in-out,
                    border-color 0.3s ease-in-out;
                @include themify() {
                    color: themed(keyword-color);
                    border-color: themed(keyword-color);
                }

                &:not(:last-child) {
                    margin-right: 20px;
                }

                &:hover {
                    @include themify() {
                        color: themed(comment-color);
                        border-color: themed(comment-color);
                    }
                }
            }
        }
    }
</style>
