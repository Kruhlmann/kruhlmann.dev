<script lang="ts">
    export let abbreviation: string;
    export let color = "gray";
    export let href = "";
    export let classname = "";
    export let usage = 100;
</script>

{#if href !== ''}
    <a href="{href}" class="card {classname}" style="background-color: {color};">
        <span class="abbreviation">{abbreviation}</span>
        <span class="usage">{Math.floor(usage)}%</span>
    </a>
{:else}
    <div class="card {classname}" style="background-color: {color};">
        <span class="abbreviation">{abbreviation}</span>
    </div>
{/if}

<style lang="scss">
    .card {
        display: flex;
        flex-direction: column;
        width: 100px;
        height: 100px;
        box-shadow: 5px 5px 10px -5px rgba(0, 0, 0, 0.75);
        text-decoration: none;
        text-align: center;

        .abbreviation {
            font-size: 70px;
            height: 80%;
            line-height: 100%;
        }

        .usage {
            height: 20%;
            line-height: 20%;
            font-size: 22px;
        }

        .abbreviation,
        .usage {
            color: black;
            opacity: 0.5;
            transition: opacity 0.3s ease-in-out;
        }

        &:hover {
            .abbreviation,
            .usage {
                opacity: 0.8;
            }
        }
    }
</style>
