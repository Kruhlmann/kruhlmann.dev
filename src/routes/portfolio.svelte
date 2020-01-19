<script>
    import technologies from "../technologies.js";

    function name_technology(name) {
        return technologies.find((t) => {
            return t.name === name;
        });
    }

    const employments = [
        {
            firm: "Siemens Mobility A/S",
            position: "Apprentice",
            start: "FEB 2015",
            end: false,
            description: "I decided to continue in Siemens Mobility during my internship from TEC Ballerup. During this period a lot of my work was focused on web services, shell scripts and introducing new technologies into our development stack.",
            show: true,
            technologies: [
                "CSS/SASS",
                "JavaScript",
                "Ruby",
                "Svelte",
                "TypeScript",
                "Bash",
            ].map((name) => technologies.find((t) => name === t.name)),
        },
        {
            firm: "Siemens Mobility",
            position: "Student Worker",
            start: "FEB 2015",
            end: "FEB 2019",
            description: "During my position as student worker in Siemens Mobility I worked with a wide array of languages and tools in order to solve a diverse array of challenges. Much of the work I did was designed to replace expensive manual labor with automization software created in the student talent pool.",
            show: true,
            technologies: [
                "C#",
                "JavaScript",
                "CSS/SASS",
                "Python",
                "VB6",
                "SQL",
                "Bash",
            ].sort().map(name_technology),
        },
        {
            firm: "Deeepgames",
            position: ["Author", "Game Developer"],
            start: "NOV 2013",
            end: "MAR 2017",
            description: "Deeepgames was a start-up initiative I created with my co-developers, Sebastian Di Giuseppe and Elmar van Rijnswou. We’ve since then released multiple games for Android using Java with LibGDX and written the book \"Building a 3D Game with LibGDX\" (published by Packt Publishing).",
            show: true,
            technologies: [
                "Java",
                "JavaScript",
            ].sort().map(name_technology),
        },
        {
            firm: "Copenhagen Judo",
            position: "Full-stack Developer",
            start: "MAR 2012",
            end: "AUG 2013",
            description: "Developed and designed an enrollment website using PHP/XAMPP, HTML, CSS, Javascript (JQuery) with a MySQL database.",
            show: true,
            technologies: [
                "PHP",
                "SQL",
                "CSS/SASS",
                "JavaScript",
            ].sort().map(name_technology),
        },
    ];

    const projects = [
        {
            name: "Building a 3D Game with LibGDX",
            img: "building_a_3d_game_with_libgdx.jpg",
            description: "After multiple game releases with my team at deeepgames we were approached by a publisher to write a book about 3D game development using the newly update LibGDX Java graphics framework. We targeted the book at intermediate Java developers taking them through the process of designing, coding and modeling everything that goes into a 3D game.",
            show: true,
            links: {
                "Publisher": "https://www.packtpub.com/game-development/building-3d-game-libgdx",
                "Book code": "https://github.com/PacktPublishing/Building-a-3D-Game-with-LibGDX/tree/master",
                "Game source": "https://github.com/DeeepGames/SpaceGladiators",
            },
            technologies: [
                "Java",
                "LibGDX",
            ].sort().map(name_technology),
        },
        {
            name: "ClassicDB Bot",
            img: "classic_db_bot.png",
            description: "ClassicDB Bot is a discord bot for embedding World of Warcraft: Classic items into discord chat channels. It allows players to discuss the merits of various equipment and in-game rewards without having to scour the often slow web archives. The project was created with typescript using the discord.js nodejs library with an sqlite database.",
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
            ].sort().map(name_technology),
        },
        {
            name: "Bird Hunter",
            img: "bird_hunter.jpeg",
            description: "Our final game release at deeepgames was a modernized version of the classic Duck Hunt game from 1984. We made the game using LibGDX and released it for Android, desktop and the web through a Java applet launcher.",
            show: true,
            links: {
                "Source": "https://github.com/scanevaro/JDuckHunt/commits/master",
            },
            technologies: [
                "Java",
                "LibGDX",
            ].sort().map(name_technology),
        },
        {
            name: "VS Theme 2 SCSS",
            img: "ruby_gem.png",
            description: "I created this ruby gem for this site specifically, which allows me to easily add new color themes by converting existing Visual Studio Code themes into SCSS themes.",
            show: true,
            links: {
                "Gem": "https://rubygems.org/gems/vstheme2scss",
                "Source": "https://github.com/Kruhlmann/vs-theme-2-scss"
            },
            technologies: [
                "Ruby",
                "CSS/SASS",
            ].sort().map(name_technology),
        },
        {
            name: "Mega Blobber",
            img: "mega_blobber.png",
            description: "Mega Blobber was the first game I created with my team at deeepgames. The original game was developed for a ludum dare game jam in less than 48 hours. We later polished the prototype and released it on the android store.",
            show: true,
            links: {
                "Source": "https://github.com/scanevaro/MegaBlobber",
            },
            technologies: [
                "Java",
                "LibGDX",
            ].sort().map(name_technology),
        },
        {
            name: "Classic Parses",
            img: "classic-parses.png",
            description: "Classic Parses allows discord users to easily keep track and compare their performance ratings in World of Warcraft: Classic. It uses the warcraftlogs.com API to serve this data.",
            show: true,
            links: {
                "Demo server": "https://discord.gg/38wH62F",
                "Source": "https://github.com/Kruhlmann/classic-parses",
            },
            technologies: [
                "Ruby",
            ].sort().map(name_technology),
        },
        {
            name: "Gatekeeper",
            img: "gatekeeper.png",
            description: "The gatekeeper discord bot was created after I was approached by a large World of Warcraft: Classic theorycrafting community. They needed a tool for keeping less experienced players from flooding their discussions. It was made simmilarly to a CAPTCHA system with the main difference being the challenges were related to the on-going theorycrafting.",
            show: true,
            links: {
                "Source": "https://github.com/Kruhlmann/gatekeeper",
                "Stats": "https://services.kruhlmann.dev/gatekeeeper",
            },
            technologies: [
                "TypeScript",
                "SQL",
                "Svelte",
                "CSS/SASS",
            ].sort().map(name_technology),
        },
        {
            name: "gasbot",
            img: "python.png",
            description: "gasbot is a modular Twitch.TV and IRC extension framework. It makes it simple for even novice developers to develop a custom feature-rich moderation and/or entertainment bot by use of its module system.",
            show: true,
            links: {
                "Source": "https://github.com/Kruhlmann/gasbot",
            },
            technologies: [
                "Python",
                "SQL",
            ].sort().map(name_technology),
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
                    <span class="key">Position{Array.isArray(employment.position) ? "s" : ""}</span>
                    {#if Array.isArray(employment.position)}
                        {#each employment.position as p}
                            <span class="val multiposition">{p}</span>
                        {/each}
                    {:else}
                        <span class="val">{employment.position}</span>
                    {/if}
                    <span class="key">Employment start</span>
                    <span class="val">{employment.start}</span>
                    <span class="key">Employment end</span>
                    <span class="val">{employment.end || "Ongoing"}</span>
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
@import url("https://fonts.googleapis.com/css?family=IBM+Plex+Sans&display=swap");

.container {
    width: 100%;
    max-width: 1080px;
    margin: 75px auto;

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
            grid-template-rows: repeat(auto-fit, 22px);
            grid-row-gap: 10px;
            grid-column-gap: 15px;
            font-size: 18px;

            span.key {
                font-style: italic;
                @include themify($themes) {
                    color: themed(comment-color);
                }
            }

            span.val {
                @include themify($themes) {
                    color: themed(constant-color);
                }
            }

            span.multiposition:not(:first-child) {
                grid-column: 2;
            }
        }

        p {
            font-size: 20px;
            margin: 0;
            height: 100%;
            line-height: 1.5em;
            padding-left: 15px;
            margin-left: 15px;
            font-family: "IBM Plex Sans", sans-serif;
            @include themify($themes) {
                border-left: 1px solid themed(keyword-color);
                color: themed(string-color);
            }
        }
    }

}
</style>
