<script lang="ts">
    import * as config from "../../config/config.json";
    import { Cookie } from "../lib/cookie";
    import { time } from "../lib/stores";
    import { onMount } from "svelte";

    const categories: Record<string, Record<string, string>> = config.startpage;

    let theme_cookie: Cookie;
    let selected_theme = "";

    function on_document_ready(): void {
        theme_cookie = new Cookie("theme");
        if (theme_cookie.val() === "" || theme_cookie.val() === undefined) {
            theme_cookie.val(config.themes[0]);
        }
        selected_theme = theme_cookie.val();
    }

    onMount(on_document_ready);
</script>

<div class="container">
    <div id="time" class="time">{$time}</div>

    <form method="GET" action="http://www.google.com/search">
        <input name="as_q" class="search-field" />
        <button
            type="submit"
            class="search-submit"
            value=""
            style="fill: white"
        ></button>
    </form>

    <div class="box">
        {#each Object.keys(categories) as category_name}
            <div class="box-container">
                <div class="heading">{category_name}</div>
                {#each Object.entries(categories[category_name]) as [name, href]}
                    <a href="{href}">
                        <div class="link" tabindex="0">{name}</div>
                    </a>
                {/each}
            </div>
        {/each}
    </div>
</div>

<div class="wave-container">
    <div class="wave wave1"></div>
    <div class="wave wave2"></div>
    <div class="wave wave3"></div>
    <div class="wave wave4"></div>
</div>

<style lang="scss">
    @import "../scss/main.scss";

    .container {
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        text-align: center;
        min-height: 100vh;
        margin: 0 auto;
        width: 80%;
        font-family: "Start Page";

        form {
            background-color: #2e3441;
            border-radius: 5px;
            width: 450px;
            margin-bottom: 80px;
            display: grid;
            grid-template-columns: 1fr auto;
            align-items: center;

            .search-submit {
                outline: none;
                border: none;
                position: relative;
                cursor: pointer;
                color: inherit;
                font-family: inherit;
                background-color: inherit;
                background-image: url("/img/skarp_search.png");
                background-repeat: no-repeat;
                background-size: 32px 32px;
                height: 32px;
                width: 32px;
                border-radius: 15px;
                margin-right: 10px;
            }
            .search-field {
                box-sizing: border-box;
                color: white;
                font-size: 35px;
                background-color: transparent;
                border: none;
                padding: 15px;
                font-family: "Start Page";

                &:focus {
                    outline: none;
                }
            }
        }

        .time {
            font-size: 120px;
            margin-bottom: 20px;
        }

        a,
        .box {
            transition: all 150ms ease-in-out;
        }

        .box {
            display: grid;
            grid-template-columns: repeat(auto-fit, 200px);
            grid-gap: 50px;
            justify-content: center;
            width: 100%;
            text-transform: capitalize;
        }

        .box-container {
            box-shadow: 4px 4px 15px -5px rgba(0, 0, 0, 0.25);
            background-color: #2e3441;
            width: 200px;
            height: 230px;
            float: left;
            border-radius: 5px;

            &:hover {
                box-shadow: 4px 4px 15px -5px rgba(0, 0, 0, 0.75);
            }

            .heading {
                margin: 15px 25px;
                font-size: 33px;
                text-align: left;
            }

            .link {
                font-size: 28px;
                line-height: 28px;
                margin: 8px 25px;
                text-align: left;
            }
        }

        a {
            color: #6c7789;
            text-decoration: none;

            &:hover {
                color: white;
            }
        }
    }

    .wave-container .wave {
        position: absolute;
        bottom: 0;
        left: 0;
        width: 100%;
        height: 100px;
        background: url("/svg/wave.svg");
        background-size: 990px 100px;
        background-repeat: repeat-x;
    }

    .wave-container .wave.wave1 {
        animation: animate 30s linear infinite;
        z-index: 1000;
        opacity: 1;
        animation-delay: 0s;
        bottom: 0;
    }

    .wave-container .wave.wave2 {
        animation: animate2 15s linear infinite;
        z-index: 999;
        opacity: 0.5;
        animation-delay: -5s;
        bottom: 10px;
    }

    .wave-container .wave.wave3 {
        animation: animate 30s linear infinite;
        z-index: 998;
        opacity: 0.2;
        animation-delay: -2s;
        bottom: 15px;
    }
    .wave-container .wave.wave4 {
        animation: animate2 5s linear infinite;
        z-index: 997;
        opacity: 0.7;
        animation-delay: -5s;
        bottom: 20px;
    }

    @keyframes animate {
        0% {
            background-position-x: 0;
        }
        100% {
            background-position-x: 990px;
        }
    }

    @keyframes animate2 {
        0% {
            background-position-x: 0;
        }
        100% {
            background-position-x: -990px;
        }
    }
</style>
