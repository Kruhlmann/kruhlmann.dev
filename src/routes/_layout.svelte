<script lang="ts">
    import { onMount } from "svelte";
    import { fade, slide } from "svelte/transition";
    import { show_contact_modal, show_theme_modal } from "../lib/stores";
    import { Cookie } from "../lib/cookie";
    import * as config from "../../config/config.json";
    import ThemeModal from "../components/ThemeModal.svelte";
    import ContactModal from "../components/ContactModal.svelte";

    export let segment: string;

    let theme_cookie: Cookie;
    let selected_theme = "";
    let burger_menu_is_active = false;

    function theme_modal_close(): void {
        show_theme_modal.set(false);
    }

    function contact_modal_close(): void {
        show_contact_modal.set(false);
    }

    function select_theme(event: CustomEvent<string>): void {
        selected_theme = theme_cookie.val(event.detail);
        show_theme_modal.set(false);
    }

    function burger_menu_show_theme_modal(): void {
        burger_menu_is_active = false;
        show_theme_modal.set(true);
    }

    function burger_menu_show_contact_modal(): void {
        burger_menu_is_active = false;
        show_contact_modal.set(true);
    }

    function on_document_key_down(event: KeyboardEvent): void {
        if (event.key === "Escape") {
            show_contact_modal.set(false);
            show_theme_modal.set(false);
        }
    }

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
    {#if burger_menu_is_active}
        <div class="burger-menu" transition:slide>
            <nav>
                <a href="/" on:click="{() => (burger_menu_is_active = false)}">
                    <div class="icon"></div>
                    <span class="description">Home</span>
                </a>
                <div>
                    <div class="icon"></div>
                    <span
                        class="description"
                        on:click="{burger_menu_show_theme_modal}"
                    >Select a theme</span>
                </div>
                <div>
                    <div class="icon"></div>
                    <span
                        class="description"
                        on:click="{burger_menu_show_contact_modal}"
                    >View contact information</span>
                </div>
                <a
                    href="/resume.pdf"
                    on:click="{() => (burger_menu_is_active = false)}"
                >
                    <div class="icon"></div>
                    <span class="description">Download resume</span>
                </a>
                <a
                    href="/portfolio"
                    on:click="{() => (burger_menu_is_active = false)}"
                >
                    <div class="icon"></div>
                    <span class="description">View portfolio</span>
                </a>
            </nav>
        </div>
        <div
            class="burger-menu-overlay"
            transition:fade
            on:click="{() => (burger_menu_is_active = false)}"
        ></div>
    {/if}
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
                    <div
                        class="mobile-burger-menu-button"
                        on:click="{() => (burger_menu_is_active = true)}"
                    >
                        
                    </div>
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

    .burger-menu {
        position: absolute;
        z-index: 11;
        top: 0;
        left: 0;
        width: 100%;
        padding: 20px;
        font-size: 28px;
        border-bottom: 3px solid;

        nav {
            display: flex;
            flex-direction: column;

            a,
            div {
                display: grid;
                grid-template-columns: auto 1fr;
                grid-gap: 20px;
                align-items: center;
                cursor: pointer;

                &:hover .description {
                    text-decoration: underline;
                }

                .icon {
                    font-size: 36px;
                }

                &:not(:last-child) {
                    margin-bottom: 10px;
                }
            }
        }

        @include themify() {
            color: themed(keyword-color);
            background-color: themed(background-color);
            fill: themed(background-color);
        }
    }

    .burger-menu-overlay {
        position: absolute;
        z-index: 10;
        background-color: black;
        opacity: 0.5;
        top: 0;
        left: 0;
        width: 100%;
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

                .mobile-burger-menu-button {
                    display: none;
                    font-size: 50px;
                    cursor: pointer;
                    line-height: 31px;
                }

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

    @media only screen and (max-width: 1100px) {
        .container .backdrop nav {
            grid-template-columns: repeat(4, auto);
            padding: 15px 15px;
            text-align: center;

            .title {
                display: none;
            }
        }
    }

    @media only screen and (max-width: 768px) {
        .container .backdrop nav {
            grid-template-columns: auto 1fr;
            grid-template-rows: 31px;

            *:not(.mobile-burger-menu-button) {
                display: none;
            }

            .mobile-burger-menu-button {
                display: block;
            }
        }
    }
</style>
