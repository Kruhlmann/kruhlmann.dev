<script>
    import technologies from "../technologies.js";

    const employments = [
        {
            firm: "Siemens Mobility A/S",
            position: "Software Developer",
            start: "FEB 2015",
            end: false,
            description: "Lorem Ipsum dolor sit amet",
            technologies: [
                "C#",
                "JavaScript",
                "TypeScript",
                "CSS/SASS",
                "Python",
                "Ruby",
                "VB8",
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
            technologies: [
                "Java",
                "LibGDX",
            ].map((name) => technologies.find((t) => name === t.name)),
        },
        {
            name: "ClassicDB Bot",
            img: "",
            description: "Lorem Ipsum dolor sit amet",
            technologies: [
                "TypeScript",
                "SQL",
                "Svelte",
                "CSS/SASS",
            ].map((name) => technologies.find((t) => name === t.name)),
        },
        {
            name: "Flying Cavemen",
            img: "",
            description: "Lorem Ipsum dolor sit amet",
            technologies: [
                "Java",
                "LibGDX",
            ].map((name) => technologies.find((t) => name === t.name)),
        },
        {
            name: "Mega Blobber",
            img: "mega_blobber.png",
            description: "Lorem Ipsum dolor sit amet",
            technologies: [
                "Java",
                "LibGDX",
            ].map((name) => technologies.find((t) => name === t.name)),
        },
        {
            name: "Classic Parses",
            img: "",
            description: "Lorem Ipsum dolor sit amet",
            technologies: [
                "Ruby",
            ].map((name) => technologies.find((t) => name === t.name)),
        },
        {
            name: "Gatekeeper",
            img: "",
            description: "Lorem Ipsum dolor sit amet",
            technologies: [
                "TypeScript",
                "SQL",
                "Svelte",
                "CSS/SASS",
            ].map((name) => technologies.find((t) => name === t.name)),
        },
        {
            name: "ND World Chat",
            img: "",
            description: "Lorem Ipsum dolor sit amet",
            technologies: [
                "Python",
            ].map((name) => technologies.find((t) => name === t.name)),
        },
        {
            name: "gasbot",
            img: "",
            description: "Lorem Ipsum dolor sit amet",
            technologies: [
                "Python",
            ].map((name) => technologies.find((t) => name === t.name)),
        },
    ]

</script>

<div class="container">
    <h1>Where I've worked</h1>
    <section>
        {#each employments as employment}
            <header>
                <h2>{employment.firm}</h2>
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
            <div class="details">
                <span>Position</span>
                <span>{employment.position}</span>
                <span>Employment start</span>
                <span>{employment.start}</span>
                <span>Employment end</span>
                <span>{employment.end || "Ongoing"}</span>
            </div>
            <p>{employment.description}</p>
        {/each}
    </section>

    <h1>What I've made</h1>
    <section>
        {#each projects as project}
            <header>
                <h2>{project.name}</h2>
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
            <div class="img-wrapper">
                <img src="/img/portfolio/{project.img}" alt="{project.name}">
            </div>
            <p>{project.description}</p>
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
        grid-template-columns: 375px 1fr;
        grid-row-gap: 25px;
        padding: 25px;
        @include themify($themes) {
            border: 2px solid themed(keyword-color);
        }

        header {
            grid-column: span 2;
            display: flex;
            justify-content: space-between;
            align-items: center;
            margin-bottom: 10px;

            h2 {
                margin: 0;
                font-size: 24px;
                text-decoration: underline;
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
            grid-template-columns: 160px 1fr;
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
