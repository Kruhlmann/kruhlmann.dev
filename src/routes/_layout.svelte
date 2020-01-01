<div class="container">
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
</div>

<style lang="scss">
@import "../scss/main";

:global(html, body) {
    height: 100%;
    margin: 0;
    background-color: $backdrop;
    font-family: "Input Mono",monaco,monospace;

    a {
        color: inherit;
        text-decoration: inherit;
        font-family: inherit;
    }
}

:global(#sapper) {
    height: 100%;
}

.container {
    position: relative;
    height: 100%;

    .backdrop {
        display: flex;
        flex-direction: column;
        flex: 1;
        height: 100%;
        position: relative;
        z-index: 1;

        nav {
            color: $font-color;
            font-size: 18px;
            width: 100%;
            display: grid;
            grid-template-columns: 1fr auto auto;
            grid-template-rows: 1fr;
            padding: 15px 30px;
            box-sizing: border-box;

            .item {
                padding: 15px;
                text-transform: uppercase;
            }
        }

        .content {
            display: flex;
            flex: 1;
        }
    }
}

</style>
