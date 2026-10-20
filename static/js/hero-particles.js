const PARTICLE_STRIDE = 5;
const LINE_STRIDE = 5;

(() => {
    const hero = document.querySelector(".hero");
    const canvas = document.getElementById("hero-canvas");
    if (!hero || !canvas) {
        return;
    }
    const reducedMotion = window.matchMedia("(prefers-reduced-motion: reduce)").matches;
    const ctx = canvas.getContext("2d");
    const domSliders = {
        count: document.getElementById("sim-particle-count"),
        linkDistance: document.getElementById("sim-link-distance"),
        particleSpeed: document.getElementById("sim-particle-speed"),
        linkAlpha: document.getElementById("sim-link-alpha"),
    };

    let width = 0;
    let height = 0;
    let animationFrameId = null;
    let simInstance = null;

    function readConfigurationFromDom() {
        return {
            count: parseInt(domSliders.count.value, 10),
            linkDistance: parseFloat(domSliders.linkDistance.value),
            particleSpeed: parseFloat(domSliders.particleSpeed.value),
            linkAlpha: parseFloat(domSliders.linkAlpha.value),
        };
    }

    function resize() {
        const dpr = Math.min(window.devicePixelRatio || 1, 2);
        width = hero.clientWidth;
        height = hero.clientHeight;
        canvas.width = width * dpr;
        canvas.height = height * dpr;
        ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
    }

    function seed() {
        resize();
        const newSeed = (Date.now() ^ (width << 16) ^ height) >>> 0;
        simInstance.sim_init(readConfigurationFromDom().count, width, height, newSeed);
    }

    function applyConfig() {
        const configuration = readConfigurationFromDom();
        simInstance.sim_set_config(configuration.count, configuration.linkDistance, configuration.linkAlpha, configuration.particleSpeed);
    }

    function draw() {
        ctx.clearRect(0, 0, width, height);
        const lineCount = simInstance.sim_step();

        ctx.lineWidth = 1;
        const lines = new Float32Array(simInstance.memory.buffer, simInstance.lines_ptr(), lineCount * LINE_STRIDE);
        for (let i = 0; i < lineCount; i++) {
            const o = i * LINE_STRIDE;
            ctx.strokeStyle = "rgba(127, 209, 163, " + lines[o + 4].toFixed(3) + ")";
            ctx.beginPath();
            ctx.moveTo(lines[o], lines[o + 1]);
            ctx.lineTo(lines[o + 2], lines[o + 3]);
            ctx.stroke();
        }

        ctx.fillStyle = "rgba(127, 209, 163, 0.30)";
        const particles = new Float32Array(simInstance.memory.buffer, simInstance.particles_ptr(), readConfigurationFromDom().count * PARTICLE_STRIDE);
        for (let i = 0; i < particles.length / PARTICLE_STRIDE; i++) {
            const o = i * PARTICLE_STRIDE;
            ctx.beginPath();
            ctx.arc(particles[o], particles[o + 1], particles[o + 4], 0, Math.PI * 2);
            ctx.fill();
        }
    }

    function tick() {
        draw();
        animationFrameId = requestAnimationFrame(tick);
    }

    function start() {
        if (simInstance !== null && animationFrameId === null) {
            animationFrameId = requestAnimationFrame(tick);
        }
    }

    function stop() {
        if (animationFrameId !== null) {
            cancelAnimationFrame(animationFrameId);
            animationFrameId = null;
        }
    }

    fetch("/wasm/particles.wasm")
        .then((response) => {
            if (!response.ok) {
                throw new Error("failed to load particles.wasm: " + response.status);
            }
            return response.arrayBuffer();
        })
        .then((bytes) => WebAssembly.instantiate(bytes, {}))
        .then((result) => {
            simInstance = result.instance.exports;
            seed();
            applyConfig();
            draw();

            for (const slider of Object.values(domSliders)) {
                slider.addEventListener("input", applyConfig);
            }

            if (!reducedMotion) {
                start();

                new IntersectionObserver((entries) => {
                    for (const entry of entries) {
                        if (entry.isIntersecting) {
                            start();
                        } else {
                            stop();
                        }
                    }
                }).observe(hero);

                document.addEventListener("visibilitychange", () => {
                    if (document.hidden) {
                        stop();
                    } else {
                        start();
                    }
                });

                window.addEventListener("resize", seed);
            }
        })
        .catch(console.warn);
})();
