<script>
    import * as _array from "../lib/array.js";
    import { onMount } from "svelte";

    const languages = _array.shuffle([
        "ruby",
        "javascript",
        "python",
        "sql",
        "c#",
        "css",
        "typescript",
        "visual basic",
        "java",
    ]);

    let cur_lang_idx = 0;
    let cur_lang_stub = 0;
    let cur_lang_grow = true;
    let cur_lang_grace = false;

    onMount(() => {
        setInterval(() => {
            if (!cur_lang_grace) {
                if (cur_lang_grow) {
                    cur_lang_stub ++;

                    if (cur_lang_stub >= languages[cur_lang_idx].length) {
                        cur_lang_grace = true;
                        cur_lang_grow = false;
                        setTimeout(() => {
                            cur_lang_grace = false;
                        }, 2000);
                    }
                } else {
                    cur_lang_stub --;
                    if (cur_lang_stub === 0) {
                        cur_lang_grace = true;
                        cur_lang_grow = true;
                        cur_lang_idx ++;
                        cur_lang_idx %= languages.length;
                        setTimeout(() => {
                            cur_lang_grace = false;
                        }, 1000);
                    }
                }
            }
        }, 85);
    });

    $: display_lang = languages[cur_lang_idx].substring(0, cur_lang_stub);
</script>

<div class="container">
    <span class="title">I'm a <span class="current-language">{display_lang}</span><span class="underscore">_</span> developer</span>
    <div class="btn-grp">
        <span>Contact</span>
        <a href="/resume">Resume</a>
    </div>
</div>

<style lang="scss">
    @import "../scss/main";

    @keyframes blink {
        0% { opacity: 1; }
        50% { opacity: 1 }
        51% { opacity: 0 }
        100% { opacity: 0; }
    }

    .container {
        flex: 1;
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        color: $font-color;
        box-sizing: border-box;

        .title {
            font-size: 56px;
            font-weight: bold;

            .current-language, .underscore {
                color: #aaa;
                font-style: italic;
                text-transform: uppercase;
                text-shadow: 2px 1px 5px black;
            }

            .underscore {
                animation: blink 1.1s infinite;
            }
        }

        .btn-grp {
            margin-top: 30px;
            display: flex;

            a, span {
                padding: 15px;
                text-transform: uppercase;
                border: 2px solid $font-color;
                color: $font-color;
                text-decoration: none;
                cursor: pointer;
                font-weight: bold;
                transition: background-color 0.3s ease-in-out;

                &:not(:last-child) {
                    margin-right: 20px;
                }

                &:hover {
                    background-color: $btn-hl;
                }
            }
        }
    }
</style>
