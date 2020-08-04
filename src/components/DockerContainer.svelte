<script lang="ts">
    import { ContainerInfo } from "../types/index";
    export let container: ContainerInfo;

    const running = container.State === "running";
    const name = container.Names[0].slice(1);
</script>

<a href="https://hub.docker.com/r/kruhlmann/{name}">
    <div class="status" class:running></div>
    <div class="about">
        <div class="name">{name}</div>
        <div class="ref">View image</div>
    </div>
</a>

<style lang="scss">
    a {
        display: grid;
        grid-template-columns: 100px 1fr;
        height: 100px;
        background-color: #2c2c2c;
        box-shadow: 5px 5px 10px -5px rgba(0, 0, 0, 0.75);
        color: inherit;
        text-decoration: none;
        transition: transform 0.4s ease-in-out;

        .status {
            display: grid;
            display: flex;
            justify-content: center;
            align-items: center;
            flex-direction: column;
            background-color: #333;
            font-size: 115px;
            overflow: hidden;
            background-color: #771921;
            color: darken(#771921, 12);

            &.running {
                background-color: #44883f;
                color: darken(#44884f, 12);
            }
        }

        .about {
            font-size: 25px;
            overflow: hidden;

            & > * {
                display: flex;
                justify-content: center;
                align-items: center;
            }

            & > * {
                position: relative;
                height: 100px;
                top: 0;
                transition: top 0.3s ease-in-out;
            }
        }

        &:hover {
            .about > * {
                top: -100px;
            }
        }
    }
</style>
