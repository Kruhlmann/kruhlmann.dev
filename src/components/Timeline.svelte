<script lang="ts">
    import { Cookie } from "../lib/cookie";
    import { onMount } from "svelte";
    import { fly } from "svelte/transition";
    import { TimelineItem } from "../types";
    import * as config from "../../config/config.json";

    let theme_cookie: Cookie;
    let selected_theme = "";
    let cached_year = -1;
    let show_timeline_items = false;

    const activity_types: Record<string, string> = config.activity_types;
    const short_months = [
        "Jan",
        "Feb",
        "Mar",
        "Apr",
        "May",
        "Jun",
        "Jul",
        "Aug",
        "Sep",
        "Oct",
        "Nov",
        "Dec",
    ];

    /**
     * Returns the corresponding icon for a technology name. The name must exist
     * within the technologies section on the config, else the name is simply
     * returned.
     *
     * @see config/config.json
     * @param technology_name - Name of technology to find icon for.
     * @returns - The corresponding icon if it was found, else technology_name.
     */
    function get_technology_icon(technology_name: string): string {
        const icon = config.technologies.find((technology) => {
            return technology.name === technology_name;
        });

        if (icon) {
            return icon.icon;
        }
        return technology_name;
    }

    /**
     * Pads an integer with 0 if it is lower than 9.
     *
     * @param number_input - Integer to pad.
     * @returns - Padded integer.
     */
    function pad_number(number_input: number): string {
        if (number_input > 9) {
            return `${number_input}`;
        }
        return `0${number_input}`;
    }

    /**
     * Creates a short string representation of a date on a TimelineItem.
     *
     * @see pad_number
     * @param item - Item to create date string for.
     * @returns - String representation. If no day was specified in the date
     * object of the item thenthis is simply the 3-letter representation of the
     * month (e.g. Jan for January). If the day was included it is added as a
     * suffix, padded to fill 2 digits.
     */
    function make_item_date(item: TimelineItem): string {
        const month_string = short_months[item.date.month - 1];
        if (item.date.day) {
            const padded_date = pad_number(item.date.day);
            return `${month_string} ${padded_date}`;
        }
        return month_string;
    }

    /*
     * Checks the cache for the last recorded year shown in the DOM, and updates
     * it if a newer one is available. Returns the resulting boolean.
     *
     * @param item - New item to render in the DOM.
     * @returns - Whether the cache was updated.
     */
    function is_new_year(item: TimelineItem): boolean {
        if (item.date.year !== cached_year) {
            cached_year = item.date.year;
            return true;
        }
        return false;
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
        show_timeline_items = true;
    }

    /**
     * Takes a possibly undefined array of unknown items and returns a list of
     * their string representation.
     *
     * @param array Array of items to convert to strings.
     * @returns Array of string items. If the passed array was falsy an empty
     * array is returned in its place.
     */
    function as_strings(array: unknown[] | undefined): string[] {
        if (!array) {
            return [];
        }
        return array.map((item: unknown) => {
            return `${item}`;
        });
    }

    onMount(on_document_ready);
</script>

<div class="theme-wrapper theme-{selected_theme}">
    <div class="timeline">
        {#each config.activities as item, index}
            {#if is_new_year(item)}
                <div class="container hide small"></div>
                <div class="middle small" class:first="{index === 0}">
                    <div class="new-year-block">{item.date.year}</div>
                </div>
                <div class="container hide small"></div>
            {/if}

            {#if item.is_small}
                <div class="container hide small"></div>
                <div class="middle small">
                    {#if index % 2 === 0}
                        <div class="flag right">{make_item_date(item)}</div>
                        <div class="icon-wrapper">
                            <span>{activity_types[item.type]}</span>
                        </div>
                        <div class="flag left large">{item.title}</div>
                    {:else}
                        <div class="flag right large">{item.title}</div>
                        <div class="icon-wrapper">
                            <span>{activity_types[item.type]}</span>
                        </div>
                        <div class="flag left">{make_item_date(item)}</div>
                    {/if}
                </div>
                <div class="container hide small"></div>
            {:else}
                <div
                    class="container left"
                    in:fly="{{ x: -200, duration: 800, delay: 500 + index * 800 }}"
                    class:hide="{index % 2 === 0}"
                >
                    {#if index % 2 !== 0 && show_timeline_items}
                        <div class="content">
                            <h2
                                class="title"
                                class:small="{item.title.length > 20}"
                            >
                                {item.title}
                            </h2>
                            <h4 class="position">{item.position}</h4>
                            <div class="technologies">
                                {#each as_strings(item.technologies) as technology}
                                    <div
                                        class="technology"
                                        title="{technology}"
                                    >
                                        {get_technology_icon(technology)}
                                    </div>
                                {/each}
                            </div>
                            <p class="description">{item.description}</p>
                        </div>
                    {/if}
                </div>

                <div
                    class="middle"
                    class:right="{index % 2 === 0}"
                    class:left="{index % 2 !== 0}"
                >
                    <div class="flag right">{make_item_date(item)}</div>
                    <div class="icon-wrapper">
                        <span>{activity_types[item.type]}</span>
                    </div>
                    <div class="flag left">{make_item_date(item)}</div>
                </div>

                <div
                    class="container right"
                    in:fly="{{ x: 200, duration: 800, delay: 500 + index * 800 }}"
                    class:hide="{index % 2 !== 0}"
                >
                    {#if index % 2 === 0 && show_timeline_items}
                        <div class="content">
                            <h2
                                class="title"
                                class:small="{item.title.length > 20}"
                            >
                                {item.title}
                            </h2>
                            <h4 class="position">{item.position}</h4>
                            <div class="technologies">
                                {#each as_strings(item.technologies) as technology}
                                    <div class="technology">
                                        <div
                                            class="technology"
                                            title="{technology}"
                                        >
                                            {get_technology_icon(technology)}
                                        </div>
                                    </div>
                                {/each}
                            </div>
                            <p class="description">{item.description}</p>
                        </div>
                    {/if}
                </div>
            {/if}
        {/each}

        <div class="container hide small"></div>
        <div class="middle small last">
            <div class="new-year-block">1996</div>
        </div>
        <div class="container hide small"></div>
    </div>
</div>

<style lang="scss">
    @import "../scss/main";

    .theme-wrapper {
        margin: 0 auto;
        width: 1000px;
    }

    .timeline {
        display: grid;
        grid-template-columns: 1fr 5px 1fr;
        align-items: center;
        width: 100%;

        .middle.small,
        .container.small {
            height: 25px;
        }

        .middle {
            display: flex;
            align-items: center;
            justify-content: center;
            height: 275px;
            z-index: 1;
            padding-top: 25px;
            padding-bottom: 25px;
            @include themify() {
                background-color: themed(keyword-color);
            }

            .icon-wrapper {
                position: relative;
                display: flex;
                justify-content: center;
                align-items: center;
                text-align: center;
                height: 80px;
                width: 80px;
                box-sizing: border-box;
                font-size: 46px;
                padding: 20px;
                border-radius: 40px;

                @include themify() {
                    background-color: themed(background-color);
                    border: 5px solid themed(keyword-color);
                    color: themed(comment-color);
                }
            }

            .new-year-block {
                padding: 10px 30px;
                font-size: 22px;
                @include themify() {
                    background-color: themed(keyword-color);
                    color: themed(background-color);
                }
            }

            .flag {
                position: relative;
                min-width: 90px;
                max-width: 90px;
                padding: 10px;
                height: 40px;
                box-sizing: border-box;
                text-align: center;
                font-weight: bold;
                text-transform: uppercase;
                @include themify() {
                    background-color: themed(keyword-color);
                    color: themed(background-color);
                }

                &.large {
                    min-width: 240px;
                    max-width: 240px;

                    &.left {
                        margin-right: -145px;
                    }

                    &.right {
                        margin-left: -145px;
                    }
                }

                &.left {
                    border-top-right-radius: 20px;
                    border-bottom-right-radius: 20px;
                }

                &.right {
                    border-top-left-radius: 20px;
                    border-bottom-left-radius: 20px;
                }
            }

            &.left {
                .icon-wrapper {
                    left: 45px;
                }

                .flag {
                    &.right {
                        display: none;
                    }

                    &.left {
                        left: 40px;
                    }
                }
            }

            &.right {
                .icon-wrapper {
                    right: 45px;
                }

                .flag {
                    &.right {
                        right: 40px;
                    }

                    &.left {
                        display: none;
                    }
                }
            }

            &.small {
                padding-top: 60px;
                padding-bottom: 60px;

                .right {
                    right: -5px;
                }

                .left {
                    left: -5px;
                }
            }

            &.first {
                padding-top: 0;
            }

            &.last {
                padding-bottom: 0;
            }
        }

        .container {
            z-index: 0;
            display: flex;
            flex-direction: column;
            justify-content: center;
            box-sizing: border-box;
            height: 225px;

            .content {
                display: grid;
                grid-template-columns: 1fr 45px;
                grid-template-rows: 50px 25px 1fr;
                box-sizing: border-box;
                height: 100%;

                * {
                    margin: 0;
                }

                .technologies {
                    display: flex;
                    flex-direction: column;
                    align-items: center;
                    grid-column: 2 / span 1;
                    grid-row: 1 / span 3;
                    font-size: 25px;
                    @include themify() {
                        border-left: 1px solid themed(keyword-color);
                        color: themed(constant-color);
                    }

                    * {
                        cursor: help;
                    }
                }

                .title {
                    padding: 5px 15px;
                    margin-top: 10px;
                    @include themify() {
                        color: darken(themed(keyword-color), 5);
                    }

                    &.small {
                        font-size: 19px;
                        line-height: 30px;
                    }
                }

                .position {
                    padding: 5px 15px;
                    font-style: italic;
                    @include themify() {
                        color: themed(comment-color);
                    }
                }

                .description {
                    font-size: 14px;
                    padding: 5px 15px;
                    @include themify() {
                        color: themed(string-color);
                    }
                }
            }

            @include themify() {
                border: 2px solid themed(keyword-color);
            }

            &.right {
                margin-left: 60px;
            }

            &.left {
                margin-right: 60px;
            }

            &.hide {
                border-color: rgba(0, 0, 0, 0);
            }
        }
    }
</style>
