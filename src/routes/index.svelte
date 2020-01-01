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
    <span class="title">I'm a <span class="current-language">{display_lang}</span> developer</span>
</div>

<style lang="scss">
    @import "../scss/main";

    .container {
        flex: 1;
        display: flex;
        align-items: center;
        justify-content: center;
        color: $font-color;
        box-sizing: border-box;

        .title {
            font-size: 56px;
            font-weight: bold;

            .current-language {
                color: #aaa;
                font-style: italic;
                text-transform: uppercase;
                text-shadow: 2px 1px 5px black;
            }
        }
    }
</style>
