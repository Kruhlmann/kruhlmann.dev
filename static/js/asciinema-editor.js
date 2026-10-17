(async function() {
    const useReducedMotion = window.matchMedia("(prefers-reduced-motion: reduce)").matches;
    let extraProps = {};
    if (useReducedMotion) {
        extraProps = {
            autoplay: false,
            poster: "npt:1",
        }
    }
    const fonts = await document.fonts.load('16px "Terminess"');

    console.log(fonts.map(font => ({
        family: font.family,
        status: font.status,
    })));
    AsciinemaPlayer.create(
        '/vim-session.cast',
        document.getElementById('editor-background'),
        {
            autoplay: true,
            loop: true,
            controls: false,
            idleTimeLimit: 0.5,
            terminalFontFaimly: '"Terminess"',
            fit: "height",
            ...extraProps,
        }
    );
})()