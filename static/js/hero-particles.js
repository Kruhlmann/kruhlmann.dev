const PARTICLE_COUNT = 60;
const PARTICLE_LINK_DISTANCE = 130;
const PARTICLE_SPEED = 0.22;

(function() {
    const hero = document.querySelector(".hero");
    const canvas = document.getElementById("hero-canvas");
    if (!hero || !canvas) {
        return;
    }
    const reducedMotion = window.matchMedia("(prefers-reduced-motion: reduce)").matches;
    const ctx = canvas.getContext("2d");

    let width = 0;
    let height = 0;
    let particles = [];
    let animationFrameId = null;

    function resize() {
        const dpr = Math.min(window.devicePixelRatio || 1, 2);
        width = hero.clientWidth;
        height = hero.clientHeight;
        canvas.width = width * dpr;
        canvas.height = height * dpr;
        ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
    }

    function seed() {
        particles = Array.from({ length: PARTICLE_COUNT }, function() {
            return {
                x: Math.random() * width,
                y: Math.random() * height,
                vx: (Math.random() - 0.5) * PARTICLE_SPEED * 2,
                vy: (Math.random() - 0.5) * PARTICLE_SPEED * 2,
                r: Math.random() * 1.4 + 0.6,
            };
        });
    }

    function draw() {
        ctx.clearRect(0, 0, width, height);
        ctx.lineWidth = 1;
        for (let i = 0; i < particles.length; i++) {
            for (let j = i + 1; j < particles.length; j++) {
                const a = particles[i];
                const b = particles[j];
                const dx = a.x - b.x;
                const dy = a.y - b.y;
                const dist = Math.sqrt(dx * dx + dy * dy);
                if (dist < PARTICLE_LINK_DISTANCE) {
                    const alpha = 0.09 * (1 - dist / PARTICLE_LINK_DISTANCE);
                    ctx.strokeStyle = "rgba(127, 209, 163, " + alpha.toFixed(3) + ")";
                    ctx.beginPath();
                    ctx.moveTo(a.x, a.y);
                    ctx.lineTo(b.x, b.y);
                    ctx.stroke();
                }
            }
        }
        ctx.fillStyle = "rgba(127, 209, 163, 0.30)";
        for (const p of particles) {
            ctx.beginPath();
            ctx.arc(p.x, p.y, p.r, 0, Math.PI * 2);
            ctx.fill();
        }
    }

    function tick() {
        for (const particle of particles) {
            particle.x += particle.vx;
            particle.y += particle.vy;
            if (particle.x < -10) {
                particle.x = width + 10;
            } else if (particle.x > width + 10) {
                particle.x = -10;
            }
            if (particle.y < -10) {
                particle.y = height + 10;
            } else if (particle.y > height + 10) {
                particle.y = -10;
            }
        }
        draw();
        animationFrameId = requestAnimationFrame(tick);
    }

    function start() {
        if (animationFrameId === null) {
            animationFrameId = requestAnimationFrame(tick);
        }
    }

    function stop() {
        if (animationFrameId !== null) {
            cancelAnimationFrame(animationFrameId);
            animationFrameId = null;
        }
    }

    resize();
    seed();
    draw();

    if (!reducedMotion) {
        start();

        new IntersectionObserver(function(entries) {
            for (const entry of entries) {
                if (entry.isIntersecting) {
                    start();
                } else {
                    stop();
                }
            }
        }).observe(hero);

        document.addEventListener("visibilitychange", function() {
            if (document.hidden) {
                stop();
            } else {
                start();
            }
        });

        window.addEventListener("resize", function() {
            resize();
            seed();
        });
    }
})();
