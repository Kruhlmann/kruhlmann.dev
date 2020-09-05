<script lang="ts">
    import { onMount } from "svelte";
    import { show_contact_modal, show_theme_modal } from "../lib/stores";
    import { Cookie } from "../lib/cookie";
    import * as config from "../../config/config.json";
    import ThemeModal from "../components/ThemeModal.svelte";
    import ContactModal from "../components/ContactModal.svelte";

    export let segment: string;

    let theme_cookie: Cookie;
    let selected_theme = "";

    /**
     * Theme modal click event handling. Hides the theme modal.
     */
    function theme_modal_close(): void {
        show_theme_modal.set(false);
    }

    /**
     * Contact modal click event handling. Hides the contact modal.
     */
    function contact_modal_close(): void {
        show_contact_modal.set(false);
    }

    /**
     * Updates the cookie with a selected theme and closes the theme modal.
     *
     * @param event - CustomEvent with detail field as the selected theme.
     */
    function select_theme(event: any): void {
        selected_theme = theme_cookie.val(event.detail);
        show_theme_modal.set(false);
    }

    /**
     * Document key down event handling. Closes both modals when the Escape keys
     * is pressed down.
     *
     * @param event - KeyboardEvent.
     */
    function on_document_key_down(event: KeyboardEvent): void {
        if (event.key === "Escape") {
            show_contact_modal.set(false);
            show_theme_modal.set(false);
        }
    }

    /**
     * Document ready logic. Sets the cookie value and addes keydown event
     * listener.
     */
    function on_document_ready(): void {
        theme_cookie = new Cookie("theme");
        if (theme_cookie.val() === "" || theme_cookie.val() === undefined) {
            theme_cookie.val(config.themes[0]);
        }
        selected_theme = theme_cookie.val();

        document.onkeydown = on_document_key_down;
    }

    onMount(on_document_ready);
</script>

<div class="theme-container theme-{selected_theme}">
    <div class="container">
        <div class="backdrop">
            {#if segment !== 'startpage'}
                <nav>
                    <a href="/" class="title">
                        <span class="protocol">https://</span>
                        <span>kruhlmann.dev</span>
                        <span class="protocol">:443</span>
                    </a>
                    <span
                        class="item"
                        on:click="{() => show_theme_modal.set(true)}"
                    >
                        Select theme
                    </span>
                    <a
                        href="/portfolio"
                        class="item"
                        rel="prefetch"
                    >Portfolio</a>
                    <a
                        href="/resume.pdf"
                        class="item"
                        target="_blank"
                    >Resume</a>
                    <span
                        class="item"
                        on:click="{() => show_contact_modal.set(true)}"
                    >
                        Contact
                    </span>
                </nav>
            {/if}
            <div class="content">
                <slot />
            </div>
        </div>
    </div>
</div>

{#if $show_theme_modal}
    <ThemeModal on:close="{theme_modal_close}" on:theme="{select_theme}" />
{/if}

{#if $show_contact_modal}
    <ContactModal on:close="{contact_modal_close}" />
{/if}

<style lang="scss">
    @import "../scss/main";

    :global(html, body) {
        height: 100%;
        margin: 0;
        font-family: "Fira Code Mono", monaco, monospace;

        a {
            color: inherit;
            text-decoration: inherit;
            font-family: inherit;
        }
    }

    .theme-container {
        height: 100%;
    }

    .container {
        height: 100%;

        @include themify() {
            color: themed(keyword-color);
            background-color: themed(background-color);
            fill: themed(background-color);
        }

        .backdrop {
            display: flex;
            flex-direction: column;
            flex: 1;
            height: 100%;
            position: relative;
            z-index: 1;
            overflow: auto;

            nav {
                font-size: 18px;
                width: 100%;
                display: grid;
                grid-template-columns: 1fr auto auto auto auto;
                grid-template-rows: 1fr;
                padding: 15px 50px;
                box-sizing: border-box;

                .title {
                    display: flex;
                    font-weight: bold;
                    font-size: 26px;

                    .protocol {
                        opacity: 0.6;
                        font-weight: normal;
                    }
                }

                .item {
                    padding: 5px 15px;
                    text-transform: uppercase;
                    @include themify() {
                        border-bottom: 2px solid themed(background-color);
                    }
                    cursor: pointer;
                    transition: border-color 0.3s ease-in-out;

                    &:not(:last-child) {
                        margin-right: 15px;
                    }

                    &:hover {
                        @include themify() {
                            border-color: themed(keyword-color);
                        }
                    }
                }
            }

            .content {
                display: flex;
                flex: 1;
            }
        }
    }
</style>
