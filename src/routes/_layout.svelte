<script>
    import { fade } from "svelte/transition";
    import { show_contact_modal } from "../stores.js";

    let contact_modal;

    function contact_modal_click(e) {
        if (e.target === contact_modal) {
            show_contact_modal.set(false);
        }
    }
</script>

<div class="container">
    <div class="backdrop">
        <nav>
            <a href="/" class="title">
                <span class="protocol">http://</span>
                <span>kruhlmann.dev</span>
            </a>
            <a href="https://github.com/Kruhlmann/" class="item" rel="prefetch">
                Portfolio
            </a>
            <a href="/resume.pdf" class="item" download>
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

{#if $show_contact_modal}
    <div
        class="modal"
        bind:this={contact_modal}
        transition:fade
        on:click={contact_modal_click}>
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
                    <a class="item" title="Email" href="mailto:kruhlmann@protonmail.com">
                        <span class="icon">@</span>
                        <span class="handle">kruhlmann@protonmail.com</span>
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
{/if}

<style lang="scss">
@import "../scss/main";

:global(html, body) {
    height: 100%;
    margin: 0;
    background-color: $backdrop;
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

    .inner {
        width: 550px;
        background-color: white;
        -webkit-box-shadow: 5px 5px 15px 5px #000000;
        box-shadow: 5px 5px 15px 5px #000000;

        .top {
            padding: 30px 0;
            font-size: 32px;
            background-color: $backdrop;
            text-align: center;
            text-transform: uppercase;
            color: $font-color;
            font-weight: bold;
            font-family: "Arial";
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
}

.container {
    position: relative;
    height: 100%;

    .backdrop {
        display: flex;
        flex-direction: column;
        flex: 1;
        height: 100%;
        position: relative;
        z-index: 1;

        nav {
            color: $font-color;
            font-size: 18px;
            width: 100%;
            display: grid;
            grid-template-columns: 1fr auto auto auto;
            grid-template-rows: 1fr;
            padding: 15px 50px;
            box-sizing: border-box;

            .title {
                display: flex;
                color: $font-color;
                font-weight: bold;
                font-size: 26px;
                
                .protocol {
                    color: $protocol-color;
                    font-weight: normal;
                }
            }

            .item {
                padding: 15px;
                padding-bottom: 5px;
                text-transform: uppercase;
                border-bottom: 2px solid $backdrop;
                cursor: pointer;
                transition: border-color 0.3s ease-in-out;

                &:not(:last-child) {
                    margin-right: 15px;
                }

                &:hover {
                    border-color: $font-color;
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
