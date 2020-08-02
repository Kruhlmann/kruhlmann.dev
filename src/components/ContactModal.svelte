<script lang="ts">
    import { fade } from "svelte/transition";
    import { createEventDispatcher, onMount } from "svelte";
    import { Cookie } from "../lib/cookie";
    import * as config from "../../config/config.json";

    const dispatch = createEventDispatcher();

    let theme_cookie: Cookie;
    let selected_theme = "";
    let contact_modal: HTMLElement | undefined = undefined;

    /**
     * Contact modal click event handling. Hides the contact modal.
     *
     * @param event - DOM click event.
     */
    function contact_modal_click(event: MouseEvent): void {
        if (event.target === contact_modal) {
            dispatch("close");
        }
    }

    /**
     * Document ready logic. Sets the cookie value and addes keydown event
     * listener.
     */
    function on_document_ready(): void {
        theme_cookie = new Cookie("theme");
        if (theme_cookie.val() === "") {
            theme_cookie.val(config.themes[0]);
        }
        selected_theme = theme_cookie.val();
    }

    onMount(on_document_ready);
</script>

<div
    class="modal-container theme-{selected_theme}"
    on:click="{contact_modal_click}"
>
    <div
        class="modal contact-modal"
        bind:this="{contact_modal}"
        transition:fade
    >
        <div class="close-btn" on:click="{() => dispatch('close')}">
            &times;
        </div>
        <div class="inner">
            <div class="top">andreas s. krühlmann</div>
            <div class="bottom">
                <div class="avatar">
                    <img src="/avatar.png" alt="avatar" />
                </div>
                <div class="details">
                    <a
                        class="item"
                        title="GitHub"
                        href="https://github.com/kruhlmann"
                    >
                        <span class="icon">
                            <img src="/github.png" alt="github-logo" />
                        </span>
                        <span class="handle">kruhlmann</span>
                    </a>
                    <a
                        class="item"
                        title="Email"
                        href="mailto:andreas@kruhlmann.dev"
                    >
                        <span class="icon">@</span>
                        <span class="handle">andreas@kruhlmann.dev</span>
                    </a>
                    <a
                        class="item"
                        title="LinkedIn"
                        href="https://linkedin.com/in/andreaskruhlmann"
                    >
                        <span class="icon">
                            <img src="/linkedin.png" alt="linkedin-logo" />
                        </span>
                        <span class="handle">andreaskruhlmann</span>
                    </a>
                    <a
                        class="item"
                        title="Discord"
                        href="https://discord.gg/Y5eA3dW"
                    >
                        <span class="icon">
                            <img src="/discord.png" alt="discord-logo" />
                        </span>
                        <span class="handle">ges#0001</span>
                    </a>
                    <a
                        class="item"
                        title="GitLab"
                        href="https://gitlab.com/Kruhlmann"
                    >
                        <span class="icon">
                            <img src="/gitlab.png" alt="gitlab-logo" />
                        </span>
                        <span class="handle">kruhlmann</span>
                    </a>
                </div>
            </div>
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

    .contact-modal .inner {
        .top {
            padding: 30px 0;
            font-size: 32px;
            text-align: center;
            text-transform: uppercase;
            font-weight: bold;
            font-family: "Arial";
            @include themify() {
                color: themed(keyword-color);
                background-color: themed(background-color);
            }
        }

        .bottom {
            display: grid;
            grid-template-columns: 35% 65%;

            .avatar {
                display: flex;
                justify-content: center;
                align-items: center;
                padding: 20px 0;
            }

            .details {
                display: flex;
                flex-direction: column;
                justify-content: center;
                padding: 15px 0;

                .item {
                    display: grid;
                    grid-template-columns: 30px 1fr;
                    color: black;
                    text-decoration: none;

                    & {
                        color: inherit;
                        text-decoration: inherit;
                    }

                    &:not(:last-child) {
                        margin-bottom: 10px;
                    }

                    .icon {
                        display: flex;
                        align-items: center;
                        font-size: 25px;
                        font-family: "Arial";
                        font-weight: bold;

                        img {
                            width: 25px;
                            height: 25px;
                        }
                    }

                    .handle {
                        font-size: 20px;
                        line-height: 29px;
                    }
                }
            }
        }
    }
</style>
