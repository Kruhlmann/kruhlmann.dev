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
