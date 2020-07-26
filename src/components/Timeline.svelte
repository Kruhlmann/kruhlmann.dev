<script lang="ts">
    import { Cookie } from "../lib/cookie";
    import { onMount } from "svelte";
    import { fly } from "svelte/transition";
    import * as config from "../../config/config.json";

    let theme_cookie: Cookie;
    let selected_theme = "";
    let cached_year = -1;
    let show_timeline_items = false;

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

    type TimelineItem = {
        date: {
            year: number;
            month: number;
            day?: number;
        };
        type: string;
        title: string;
        description: string;
        position: string;
        technologies: string[];
    };

    function get_technology_icon(technology_name: string): string {
        const icon = config.technologies.find((technology) => {
            return technology.name === technology_name;
        });

        if (icon) {
            return icon.icon;
        }
        return technology_name;
    }

    function pad_number(number_input: number): string {
        if (number_input > 9) {
            return `${number_input}`;
        }
        return `0${number_input}`;
    }

    function make_item_date(item: TimelineItem): string {
        const month_string = short_months[item.date.month - 1];
        if (item.date.day) {
            const padded_date = pad_number(item.date.day);
            return `${month_string} ${padded_date}`;
        }
        return month_string;
    }

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

    onMount(on_document_ready);
</script>

<div class="theme-wrapper theme-{selected_theme}">
    <div class="timeline">
        {#each config.activities as item, index}
            {#if is_new_year(item)}
                <div class="container hide small"></div>
                <div class="middle small">
                    <div class="new-year-block">{item.date.year}</div>
                </div>
                <div class="container hide small"></div>
            {/if}

            <div
                class="container left"
                in:fly="{{ x: -200, duration: 800, delay: 500 + index * 800 }}"
                class:hide="{index % 2 === 0}"
            >
                {#if index % 2 !== 0 && show_timeline_items}
                    <div class="content">
                        <h2 class="title">{item.title}</h2>
                        <h4 class="position">{item.position}</h4>
                        <div class="technologies">
                            {#each item.technologies as technology}
                                <div class="technology">
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
                <div class="date right">{make_item_date(item)}</div>
                <div class="icon-wrapper">
                    <span>{config.activity_types[item.type]}</span>
                </div>
                <div class="date left">{make_item_date(item)}</div>
            </div>

            <div
                class="container right"
                in:fly="{{ x: 200, duration: 800, delay: 500 + index * 800 }}"
                class:hide="{index % 2 !== 0}"
            >
                {#if index % 2 === 0 && show_timeline_items}
                    <div class="content">
                        <h2 class="title">{item.title}</h2>
                        <h4 class="position">{item.position}</h4>
                        <div class="technologies">
                            {#each item.technologies as technology}
                                <div class="technology">
                                    <div class="technology">
                                        {get_technology_icon(technology)}
                                    </div>
                                </div>
                            {/each}
                        </div>
                        <p class="description">{item.description}</p>
                    </div>
                {/if}
            </div>
        {/each}

        <div class="container hide small"></div>
        <div class="middle small">
            <div class="new-year-block">{new Date().getFullYear()}</div>
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

            &.small {
                padding-top: 10px;
                padding-bottom: 10px;
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

            .date {
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

                .date {
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

                .date {
                    &.right {
                        right: 40px;
                    }

                    &.left {
                        display: none;
                    }
                }
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
                }

                .title {
                    padding: 5px 15px;
                    margin-top: 10px;
                    @include themify() {
                        color: darken(themed(keyword-color), 5);
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
