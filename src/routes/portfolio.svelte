<script>
    import technologies from "../technologies.js";

    const employments = [
        {
            firm: "Siemens Mobility A/S",
            position: "Software Developer",
            start: "FEB 2015",
            end: false,
            description: "Lorem Ipsum dolor sit amet",
            show: true,
            technologies: [
                "C#",
                "JavaScript",
                "TypeScript",
                "CSS/SASS",
                "Python",
                "Ruby",
                "VB6",
                "Svelte",
                "SQL",
            ].map((name) => technologies.find((t) => name === t.name)),
        },
        {
            firm: "Deeepgames",
            position: "Game Developer",
            start: "NOV 2013",
            end: "MAR 2017",
            description: "Lorem Ipsum dolor sit amet",
            show: true,
            technologies: [
                "Java",
                "JavaScript",
            ].map((name) => technologies.find((t) => name === t.name)),
        },
        {
            firm: "Copenhagen Judo",
            position: "Full-stack Developer",
            start: "MAR 2012",
            end: "AUG 2013",
            description: "Lorem Ipsum dolor sit amet",
            show: true,
            technologies: [
                "PHP",
                "SQL",
                "CSS/SASS",
                "JavaScript",
            ].map((name) => technologies.find((t) => name === t.name)),
        },
    ];

    const projects = [
        {
            name: "Building a 3D Game with LibGDX",
            img: "building_a_3d_game_with_libgdx.jpg",
            description: "Lorem Ipsum dolor sit amet",
            show: true,
            links: {
                "Publisher": "https://www.packtpub.com/game-development/building-3d-game-libgdx",
                "Book code": "https://github.com/PacktPublishing/Building-a-3D-Game-with-LibGDX/tree/master",
                "Game source": "https://github.com/DeeepGames/SpaceGladiators",
            },
            technologies: [
                "Java",
                "LibGDX",
            ].map((name) => technologies.find((t) => name === t.name)),
        },
        {
            name: "ClassicDB Bot",
            img: "classic_db_bot.png",
            description: "Lorem Ipsum dolor sit amet",
            show: true,
            links: {
                "Demo server": "https://discord.gg/38wH62F",
                "Source": "https://github.com/Kruhlmann/classicdb_bot",
            },
            technologies: [
                "TypeScript",
                "SQL",
                "Svelte",
                "CSS/SASS",
            ].map((name) => technologies.find((t) => name === t.name)),
        },
        {
            name: "VS Theme 2 SCSS",
            img: "ruby_gem.png",
            description: "Lorem Ipsum dolor sit amet",
            show: true,
            links: {
                "Gem": "https://rubygems.org/gems/vstheme2scss",
                "Source": "https://github.com/Kruhlmann/vs-theme-2-scss"
            },
            technologies: [
                "Ruby",
                "CSS/SASS",
            ].map((name) => technologies.find((t) => name === t.name)),
        },
        {
            name: "Mega Blobber",
            img: "mega_blobber.png",
            description: "Lorem Ipsum dolor sit amet",
            show: true,
            links: {
                "Source": "https://github.com/scanevaro/MegaBlobber",
            },
            technologies: [
                "Java",
                "LibGDX",
            ].map((name) => technologies.find((t) => name === t.name)),
        },
        {
            name: "Classic Parses",
            img: "",
            description: "Lorem Ipsum dolor sit amet",
            show: true,
            links: {
                "Demo server": "https://discord.gg/38wH62F",
                "Source": "https://github.com/Kruhlmann/classic-parses",
            },
            technologies: [
                "Ruby",
            ].map((name) => technologies.find((t) => name === t.name)),
        },
        {
            name: "Gatekeeper",
            img: "",
            description: "Lorem Ipsum dolor sit amet",
            show: true,
            links: {
                "Source": "https://github.com/Kruhlmann/gatekeeeper",
            },
            technologies: [
                "TypeScript",
                "SQL",
                "Svelte",
                "CSS/SASS",
            ].map((name) => technologies.find((t) => name === t.name)),
        },
        {
            name: "gasbot",
            img: "",
            description: "Lorem Ipsum dolor sit amet",
            show: true,
            links: {
                "Source": "https://github.com/Kruhlmann/gasbot",
            },
            technologies: [
                "Python",
            ].map((name) => technologies.find((t) => name === t.name)),
        },
    ]

</script>

<svelte:head>
    <title>Kruhlmann | Portfolio</title>
</svelte:head>
<div class="container">
    <h1>Where I've worked</h1>
    <section>
        {#each employments as employment}
            <header>
                <div class="title" on:click={() => employment.show = !employment.show}>
                    <span>{employment.show ? "-" : "+"}</span>
                    <span class="content">{employment.firm}</span>
                </div>
                <div class="tags">
                    {#each employment.technologies as technology}
                        <a
                            href="{technology.link}"
                            class="tag"
                            target="_blank">
                            {technology.name}
                        </a>
                    {/each}
                </div>
            </header>
            {#if employment.show}
                <div class="details">
                    <span>Position</span>
                    <span>{employment.position}</span>
                    <span>Employment start</span>
                    <span>{employment.start}</span>
                    <span>Employment end</span>
                    <span>{employment.end || "Ongoing"}</span>
                </div>
                <p>{employment.description}</p>
            {/if}
        {/each}
    </section>

    <h1>What I've made</h1>
    <section>
        {#each projects as project}
            <header>
                <div class="title" on:click={() => project.show = !project.show}>
                    <span>{project.show ? "-" : "+"}</span>
                    <span class="content">{project.name}</span>
                </div>
                <div class="tags">
                    {#each project.technologies as technology}
                        <a
                            href="{technology.link}"
                            class="tag"
                            target="_blank">
                            {technology.name}
                        </a>
                    {/each}
                </div>
            </header>
            {#if project.show}
                <div class="img-wrapper">
                    <img src="/img/portfolio/{project.img}" alt="{project.name}">
                </div>
                <p>{project.description}</p>
                <div class="links">
                    <span>Links:</span>
                    {#each Object.entries(project.links) as [name, link]}
                        <a href="{link}">{name}</a>
                    {/each}
                </div>
            {/if}
        {/each}
    </section>
</div>

<style lang="scss">
@import "../scss/main";

.container {
    width: 100%;
    max-width: 1080px;
    margin: 0 auto;

    section {
        width: 100%;
        display: grid;
        grid-template-columns: 450px 1fr;
        grid-row-gap: 25px;
        padding: 25px;
        @include themify($themes) {
            border: 2px solid themed(keyword-color);
        }

        &:not(:last-child) {
            margin-bottom: 85px;
        }

        header {
            grid-column: span 2;
            display: flex;
            justify-content: space-between;
            align-items: center;
            margin-bottom: 10px;

            .title {
                cursor: pointer;
                font-size: 24px;

                .content {
                    text-decoration: underline;
                }
            }

            .tags {
                display: flex;
                align-items: center;

                .tag {
                    margin: 0 5px;
                    border-radius: 12px;
                    padding: 4px 8px;
                    font-weight: bold;
                    cursor: pointer;
                    font-size: 14px;
                    text-decoration: none;
                    @include themify($themes) {
                        color: themed(invalid-color);
                        border: 1px solid themed(keyword-color);
                    }

                    &:hover {
                        text-decoration: underline;
                    }
                }
            }
        }

        .links {
            display: flex;
            grid-column: 2;
            padding: 0 30px;
            font-size: 18px;

            span {
                flex: 1;
            }

            a {
                text-decoration: none;
                font-weight: bold;
                @include themify($themes) {
                    color: themed(keyword-color);
                }

                &:hover {
                    text-decoration: underline;
                }
            }

            *:not(:first-child) {
                margin-left: 10px;
            }

            *:not(:last-child) {
                margin-right: 10px;
            }

        }

        .img-wrapper {
            width: 100%;
            display: flex;
            justify-content: center;

            img {
                min-width: 50%;
                min-height: 150px;
                max-width: 90%;
                max-height: 300px;
            }
        }

        .details {
            display: grid;
            grid-template-columns: 200px 1fr;
            grid-row-gap: 10px;
            grid-column-gap: 15px;
            font-size: 18px;

            span:nth-child(odd) {
                font-style: italic;
                @include themify($themes) {
                    color: themed(comment-color);
                }
            }

            span:nth-child(even) {
                @include themify($themes) {
                    color: themed(constant-color);
                }
            }
        }

        p {
            font-size: 18px;
            margin: 0;
            height: 100%;
            padding-left: 15px;
            margin-left: 15px;
            @include themify($themes) {
                border-left: 1px solid themed(keyword-color);
                color: themed(string-color);
            }
        }
    }

}
</style>
