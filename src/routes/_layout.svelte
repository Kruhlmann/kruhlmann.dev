<script>
    import { onMount } from "svelte";
    import { fade } from "svelte/transition";
    import { show_contact_modal, show_theme_modal } from "../stores.js";
    import { Cookie } from "../lib/cookie.js";
    import available_themes from "../themes.js"

    let mounted = false;
    let theme_cookie;
    let contact_modal;
    let theme_modal;
    let selected_theme;

    onMount(() => {
        theme_cookie = new Cookie("theme");
        if (theme_cookie.val() === "") {
            theme_cookie.val(available_themes[0]);
        }
        selected_theme = theme_cookie.val();
        document.onkeydown = (e) => {
            if (e.key === "Escape") {
                show_contact_modal.set(false);
                show_theme_modal.set(false);
            }
        }
        mounted = true;
    });

    function contact_modal_click(e) {
        if (e.target === contact_modal) {
            show_contact_modal.set(false);
        }
    }

    function theme_modal_click(e) {
        if (e.target === theme_modal) {
            show_theme_modal.set(false);
        }
    }

    function select_theme(theme) {
        selected_theme = theme_cookie.val(theme);
        show_theme_modal.set(false);
    }
</script>
{#if mounted}
    <div class="theme-container theme-{selected_theme}">
        <div class="container">
            <div class="backdrop">
                <nav>
                    <a href="/" class="title">
                        <span class="protocol">https://</span>
                        <span>kruhlmann.dev</span>
                        <span class="protocol">:443</span>
                    </a>
                    <span class="item" on:click={() => show_theme_modal.set(true)}>
                        Select theme
                    </span>
                    <a href="/portfolio" class="item" rel="prefetch">
                        Portfolio
                    </a>
                    <a href="/resume.pdf" class="item" target="_blank">
                        Resume
                    </a>
                    <span class="item" on:click={() => show_contact_modal.set(true)}>
                        Contact
                    </span>
                </nav>

                <div class="content">
                    <slot/>
                </div>
            </div>
        </div>
    </div>
{/if}

{#if $show_theme_modal}
    <div
        class="modal-container"
        on:click={theme_modal_click}>
        <div
            class="modal theme-modal"
            bind:this={theme_modal}
            transition:fade>
            <div class="close-btn" on:click={() => show_theme_modal.set(false)}>&times;</div>
            <div class="inner">
                {#each available_themes as theme}
                    <div class="theme theme-{theme}" on:click={() => select_theme(theme)}>
                        <span class="demo">
                            <span class="title">{theme.replace(/-/g, " ")}</span>
                            <span class="select-btn">select</span>
                        </span>
                    </div>
                {/each}
            </div>
        </div>
    </div>
{/if}

{#if $show_contact_modal}
    <div
        class="modal-container theme-{selected_theme}"
        on:click={contact_modal_click}>
        <div
            class="modal contact-modal"
            bind:this={contact_modal}
            transition:fade>
            <div class="close-btn" on:click={() => show_contact_modal.set(false)}>&times;</div>
            <div class="inner">
                <div class="top">
                    andreas s. krühlmann
                </div>
                <div class="bottom">
                    <div class="avatar">
                        <img src="/avatar.png" alt="avatar">
                    </div>
                    <div class="details">
                        <a class="item" title="GitHub" href="https://github.com/kruhlmann">
                            <span class="icon">
                                <img src="/github.png" alt="github-logo">
                            </span>
                            <span class="handle">kruhlmann</span>
                        </a>
                        <a class="item" title="Email" href="mailto:andreas@kruhlmann.dev">
                            <span class="icon">@</span>
                            <span class="handle">andreas@kruhlmann.dev</span>
                        </a>
                        <a class="item" title="LinkedIn" href="https://linkedin.com/in/andreaskruhlmann">
                            <span class="icon">
                                <img src="/linkedin.png" alt="linkedin-logo">
                            </span>
                            <span class="handle">andreaskruhlmann</span>
                        </a>
                        <a class="item" title="Discord" href="https://discordapp.com/invite/38wH62F">
                            <span class="icon">
                                <img src="/discord.png" alt="discord-logo">
                            </span>
                            <span class="handle">ges#0001</span>
                        </a>
                        <a class="item" title="GitLab" href="https://gitlab.com/Kruhlmann">
                            <span class="icon">
                                <img src="/gitlab.png" alt="gitlab-logo">
                            </span>
                            <span class="handle">kruhlmann</span>
                        </a>
                    </div>
                </div>
            </div>
        </div>
    </div>
{/if}

<style lang="scss">
@import "../scss/main";

:global(html, body) {
    height: 100%;
    margin: 0;
    font-family: "Input Mono",monaco,monospace;

    a {
        color: inherit;
        text-decoration: inherit;
        font-family: inherit;
    }
}

:global(#sapper) {
    height: 100%;
}

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

        @include themify($themes) {
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
    @include themify($themes) {
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
        @include themify($themes) {
            border: 2px solid themed(keyword-color);
        }

        &:hover {
            @include themify($themes) {
                background-color: themed(button-hl-color);
            }
        }
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
        @include themify($themes) {
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

.theme-container {
    height: 100%;
}

.container {
    height: 100%;

    @include themify($themes) {
        color: themed(keyword-color);
        background-color: themed(background-color);
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
                @include themify($themes) {
                    border-bottom: 2px solid themed(background-color);
                }
                cursor: pointer;
                transition: border-color 0.3s ease-in-out;

                &:not(:last-child) {
                    margin-right: 15px;
                }

                &:hover {
                    @include themify($themes) {
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
