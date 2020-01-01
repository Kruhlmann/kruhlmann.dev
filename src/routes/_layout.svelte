<script context="module">
    export async function preload() {
        this.fetch("/api/code_backdrop").then((response) => {
            return response.json();
        }).then((response) => {
            console.log(response);
        });
    }
</script>

<div class="backdrop">
    <nav>
        <div class="filler"></div>
        <a href="/portfolio" class="item" rel="prefetch">
            Portfolio
        </a>
        <a href="/resume" class="item" rel="prefetch">
            Resume
        </a>
    </nav>

    <div class="content">
        <slot/>
    </div>
</div>


<style lang="scss">
@import "../scss/main";

:global(html, body) {
    height: 100%;
    margin: 0;
    background-color: $backdrop;

    a {
        color: inherit;
        text-decoration: inherit;
    }
}

.backdrop {
    display: flex;
    flex-direction: column;

    nav {
        color: $nav-color;
        font-size: 18px;
        width: 100%;
        display: grid;
        grid-template-columns: 1fr auto auto;
        grid-template-rows: 1fr;
        padding: 15px 30px;
        box-sizing: border-box;
        font-family: "Input Mono",monaco,monospace;

        .item {
            padding: 15px;
            text-transform: uppercase;
        }
    }

    .content {
        flex: 1;
    }
}

</style>
