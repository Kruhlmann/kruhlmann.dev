<script lang="ts">
    import { Cookie } from "../lib/cookie";
    import { onMount } from "svelte";
    import * as config from "../../config/config.json";

    let theme_cookie: Cookie;
    let selected_theme = "";

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
    const item_type_icons: Record<string, string> = {
        employment_enter: "",
        employment_position_change: "",
    };

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

    const items: TimelineItem[] = [
        {
            date: { year: 2015, month: 2, day: 16 },
            type: "employment_enter",
            title: "Siemens Mobility A/S",
            description:
                "Worked at a student worker in the student talent pool of Siemens Mobility A/S",
            position: "Student Worker",
            technologies: [
                "C#",
                "Java",
                "VB6",
                "JavaScript",
                "CSS/SCSS",
                "SQL",
                "Python",
            ],
        },
        {
            date: { year: 2018, month: 2, day: 16 },
            type: "employment_position_change",
            title: "Siemens Mobility A/S",
            description:
                "Worked at a student worker in the student talent pool of Siemens Mobility A/S",
            position: "Intern",
            technologies: [
                "C#",
                "Java",
                "VB6",
                "JavaScript",
                "CSS/SCSS",
                "SQL",
                "Python",
            ],
        },
    ];

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

<div class="theme-wrapper theme-{selected_theme}">
    <div class="timeline">
        {#each items as item, index}
            <div class="container left" class:hide="{index % 2 === 0}">
                {#if index % 2 !== 0}
                    <div class="content">
                        <h2>{item.title}</h2>
                        <p>{item.description}</p>
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
                    <span>{item_type_icons[item.type]}</span>
                </div>
                <div class="date left">{make_item_date(item)}</div>
            </div>

            <div class="container right" class:hide="{index % 2 !== 0}">
                {#if index % 2 === 0}
                    <div class="content">
                        <h2>{item.title}</h2>
                        <p>{item.description}</p>
                    </div>
                {/if}
            </div>
        {/each}
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
        grid-template-rows: repeat(auto-fill, 225px);
        width: 100%;

        & > * {
            height: 225px;
        }

        .middle {
            display: flex;
            align-items: center;
            justify-content: center;
            z-index: 1;
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
                    color: themed(keyword-color);
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
            padding: 20px;
            box-sizing: border-box;

            &.right {
                margin-left: 40px;
            }

            &.left {
                margin-right: 40px;
            }

            &.hide {
                border-color: rgba(0, 0, 0, 0);
            }
        }
    }
</style>
