<script lang="ts">
    import { createEventDispatcher } from "svelte";
    import { show_theme_modal } from "../lib/stores";
    import { fade } from "svelte/transition";
    import * as config from "../../config/config.json";

    const dispatch = createEventDispatcher();
    let theme_modal: HTMLElement | undefined = undefined;

    /**
     * Theme modal click event handling. Hides the contact modal.
     *
     * @param event - DOM click event.
     */
    function theme_modal_click(event: MouseEvent): void {
        if (event.target === theme_modal) {
            dispatch("close");
        }
    }

    /**
     * Dispatches a select theme event with the theme name key.
     *
     * @param theme - Name key of the theme to select.
     */
    function select_theme(theme: string): void {
        dispatch("theme", theme);
    }
</script>

<div class="modal-container" on:click="{theme_modal_click}">
    <div class="modal theme-modal" bind:this="{theme_modal}" transition:fade>
        <div class="close-btn" on:click="{() => show_theme_modal.set(false)}">
            &times;
        </div>
        <div class="inner">
            {#each config.themes as theme}
                <div
                    class="theme theme-{theme}"
                    on:click="{() => select_theme(theme)}"
                >
                    <span class="demo">
                        <span class="title">{theme.replace(/-/g, ' ')}</span>
                        <span class="select-btn">select</span>
                    </span>
                </div>
            {/each}
        </div>
    </div>
</div>

<style lang="scss">
    @import "../scss/main";

    .modal {
        display: flex;
        justify-content: center;
        align-items: center;
        width: 100%;
        height: 100%;
        position: fixed;
        z-index: 2;
        background-color: rgba(0, 0, 0, 0.7);
        top: 0;
        left: 0;

        .close-btn {
            position: fixed;
            top: 15px;
            right: 30px;
            cursor: pointer;
            font-size: 48px;
            color: gray;

            @include themify() {
                color: themed(keyword-color);
            }
        }

        .inner {
            width: 550px;
            background-color: white;
            -webkit-box-shadow: 5px 5px 15px 5px #000000;
            box-shadow: 5px 5px 15px 5px #000000;
        }
    }

    .theme-modal .inner {
        display: flex;
        flex-direction: column;
    }

    .demo {
        display: flex;
        padding: 15px;
        text-transform: uppercase;
        @include themify() {
            color: themed(keyword-color);
            background-color: themed(background-color);
        }

        .title {
            flex: 1;
            margin-right: 15px;
            font-size: 26px;
            line-height: 53px;
        }

        .select-btn {
            padding: 15px;
            cursor: pointer;
            transition: background-color 0.3s ease-in-out;
            @include themify() {
                border: 2px solid themed(keyword-color);
            }

            &:hover {
                @include themify() {
                    background-color: themed(button-hl-color);
                }
            }
        }
    }
</style>
