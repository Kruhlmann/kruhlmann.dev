use std::cell::RefCell;

pub const PARTICLE_STRIDE: usize = 5;
pub const LINE_STRIDE: usize = 5;

pub const DEFAULT_PARTICLE_COUNT: u32 = 42;
pub const DEFAULT_LINK_DISTANCE: f32 = 130.0;
pub const DEFAULT_LINK_LINE_ALPHA: f32 = 0.09;
pub const DEFAULT_PARTICLE_SPEED: f32 = 0.22;
pub const PARTICLE_RADIUS_MIN: f32 = 0.6;
pub const PARTICLE_RADIUS_MAX: f32 = 2.0;
pub const WRAP_MARGIN: f32 = 10.0;

#[derive(Clone, Copy)]
struct Config {
    link_distance: f32,
    link_line_alpha: f32,
    particle_speed: f32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            link_distance: DEFAULT_LINK_DISTANCE,
            link_line_alpha: DEFAULT_LINK_LINE_ALPHA,
            particle_speed: DEFAULT_PARTICLE_SPEED,
        }
    }
}

struct Sim {
    width: f32,
    height: f32,
    rng: u32,
    config: Config,
    particles: Vec<f32>,
    lines: Vec<f32>,
}

impl Sim {
    fn new() -> Self {
        Self {
            width: 0.0,
            height: 0.0,
            rng: 0,
            config: Config::default(),
            particles: Vec::new(),
            lines: Vec::new(),
        }
    }

    fn next_u32(&mut self) -> u32 {
        let mut x = self.rng;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.rng = x;
        x
    }

    fn next_f32(&mut self) -> f32 {
        (self.next_u32() >> 8) as f32 / (1u32 << 24) as f32
    }

    fn next_f32_range(&mut self, min: f32, max: f32) -> f32 {
        min + (max - min) * self.next_f32()
    }

    fn spawn_particle(&mut self) {
        let (width, height) = (self.width, self.height);
        let speed = self.config.particle_speed;
        let x = self.next_f32_range(0.0, width);
        let y = self.next_f32_range(0.0, height);
        let vx = self.next_f32_range(-speed, speed);
        let vy = self.next_f32_range(-speed, speed);
        let r = self.next_f32_range(PARTICLE_RADIUS_MIN, PARTICLE_RADIUS_MAX);
        self.particles.extend_from_slice(&[x, y, vx, vy, r]);
    }

    fn init(&mut self, count: usize, width: f32, height: f32, seed: u32) {
        self.width = width;
        self.height = height;
        self.rng = if seed == 0 { 0x9e37_79b9 } else { seed };
        self.config = Config::default();
        self.particles = Vec::with_capacity(count * PARTICLE_STRIDE);
        for _ in 0..count {
            self.spawn_particle();
        }
        self.grow_line_buffer(count);
    }

    fn set_config(
        &mut self,
        count: usize,
        link_distance: f32,
        link_line_alpha: f32,
        particle_speed: f32,
    ) {
        let previous_speed = self.config.particle_speed;
        self.config.link_distance = link_distance.max(1.0);
        self.config.link_line_alpha = link_line_alpha;
        self.config.particle_speed = particle_speed;

        let current = self.particles.len() / PARTICLE_STRIDE;
        if count < current {
            self.particles.truncate(count * PARTICLE_STRIDE);
        } else if count > current {
            for _ in current..count {
                self.spawn_particle();
            }
        }

        if previous_speed > 0.0 && particle_speed != previous_speed {
            let scale = particle_speed / previous_speed;
            for p in self.particles.chunks_mut(PARTICLE_STRIDE) {
                p[2] *= scale;
                p[3] *= scale;
            }
        }

        self.grow_line_buffer(count);
    }

    fn grow_line_buffer(&mut self, count: usize) {
        let pairs = count.saturating_mul(count.saturating_sub(1)) / 2;
        if self.lines.len() < pairs * LINE_STRIDE {
            self.lines = vec![0.0; pairs * LINE_STRIDE];
        }
    }

    fn step(&mut self) -> usize {
        let mut lines = 0;
        let count = self.particles.len() / PARTICLE_STRIDE;
        let link_distance = self.config.link_distance;
        let link_line_alpha = self.config.link_line_alpha;

        for i in 0..count {
            let base = i * PARTICLE_STRIDE;
            let (width, height) = (self.width, self.height);
            let mut x = self.particles[base] + self.particles[base + 2];
            let mut y = self.particles[base + 1] + self.particles[base + 3];
            if x < -WRAP_MARGIN {
                x = width + WRAP_MARGIN;
            } else if x > width + WRAP_MARGIN {
                x = -WRAP_MARGIN;
            }
            if y < -WRAP_MARGIN {
                y = height + WRAP_MARGIN;
            } else if y > height + WRAP_MARGIN {
                y = -WRAP_MARGIN;
            }
            self.particles[base] = x;
            self.particles[base + 1] = y;
        }

        for i in 0..count {
            let a = i * PARTICLE_STRIDE;
            for j in i + 1..count {
                let b = j * PARTICLE_STRIDE;
                let dx = self.particles[a] - self.particles[b];
                let dy = self.particles[a + 1] - self.particles[b + 1];
                let dist = (dx * dx + dy * dy).sqrt();
                if dist < link_distance {
                    let alpha = link_line_alpha * (1.0 - dist / link_distance);
                    let out = lines * LINE_STRIDE;
                    self.lines[out] = self.particles[a];
                    self.lines[out + 1] = self.particles[a + 1];
                    self.lines[out + 2] = self.particles[b];
                    self.lines[out + 3] = self.particles[b + 1];
                    self.lines[out + 4] = alpha;
                    lines += 1;
                }
            }
        }

        lines
    }
}

thread_local! {
    static SIM: RefCell<Sim> = RefCell::new(Sim::new());
}

#[unsafe(no_mangle)]
pub extern "C" fn sim_init(count: u32, width: f32, height: f32, seed: u32) {
    SIM.with(|sim| sim.borrow_mut().init(count as usize, width, height, seed));
}

#[unsafe(no_mangle)]
pub extern "C" fn sim_set_config(
    count: u32,
    link_distance: f32,
    link_line_alpha: f32,
    particle_speed: f32,
) {
    SIM.with(|sim| {
        sim.borrow_mut().set_config(
            count as usize,
            link_distance,
            link_line_alpha,
            particle_speed,
        )
    });
}

#[unsafe(no_mangle)]
pub extern "C" fn sim_step() -> u32 {
    SIM.with(|sim| sim.borrow_mut().step() as u32)
}

#[unsafe(no_mangle)]
pub extern "C" fn particles_ptr() -> u32 {
    SIM.with(|sim| sim.borrow().particles.as_ptr() as u32)
}

#[unsafe(no_mangle)]
pub extern "C" fn lines_ptr() -> u32 {
    SIM.with(|sim| sim.borrow().lines.as_ptr() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sim(count: usize, width: f32, height: f32, seed: u32) -> Sim {
        let mut s = Sim::new();
        s.init(count, width, height, seed);
        s
    }

    #[test]
    fn init_seeds_particles_in_bounds() {
        let s = sim(42, 800.0, 600.0, 1);
        assert_eq!(s.particles.len(), 42 * PARTICLE_STRIDE);
        for p in s.particles.chunks(PARTICLE_STRIDE) {
            assert!((0.0..=800.0).contains(&p[0]));
            assert!((0.0..=600.0).contains(&p[1]));
            assert!((PARTICLE_RADIUS_MIN..=PARTICLE_RADIUS_MAX).contains(&p[4]));
        }
    }

    #[test]
    fn init_is_deterministic_for_a_seed() {
        let a = sim(10, 800.0, 600.0, 7);
        let b = sim(10, 800.0, 600.0, 7);
        assert_eq!(a.particles, b.particles);
    }

    #[test]
    fn step_keeps_particles_near_bounds() {
        let mut s = sim(42, 300.0, 200.0, 3);
        for _ in 0..1000 {
            s.step();
        }
        for p in s.particles.chunks(PARTICLE_STRIDE) {
            assert!((-WRAP_MARGIN..=300.0 + WRAP_MARGIN).contains(&p[0]));
            assert!((-WRAP_MARGIN..=200.0 + WRAP_MARGIN).contains(&p[1]));
        }
    }

    #[test]
    fn step_reports_valid_line_count_and_alpha() {
        let mut s = sim(42, 300.0, 200.0, 5);
        let lines = s.step();
        assert!(lines <= 42 * 41 / 2);
        for l in s.lines.chunks(LINE_STRIDE).take(lines) {
            assert!((0.0..=DEFAULT_LINK_LINE_ALPHA).contains(&l[4]));
            let d = ((l[0] - l[2]).powi(2) + (l[1] - l[3]).powi(2)).sqrt();
            assert!(d < DEFAULT_LINK_DISTANCE);
        }
    }

    #[test]
    fn zero_seed_does_not_collapse_rng() {
        let s = sim(5, 100.0, 100.0, 0);
        let xs: Vec<f32> = s.particles.chunks(PARTICLE_STRIDE).map(|p| p[0]).collect();
        assert!(xs.windows(2).any(|w| w[0] != w[1]));
    }

    #[test]
    fn next_f32_range_stays_within_bounds() {
        let mut s = sim(1, 1.0, 1.0, 9);
        for _ in 0..1000 {
            let v = s.next_f32_range(-2.5, 7.5);
            assert!((-2.5..=7.5).contains(&v));
        }
    }

    #[test]
    fn set_config_grows_and_shrinks_particle_count() {
        let mut s = sim(10, 300.0, 200.0, 11);
        s.set_config(
            20,
            DEFAULT_LINK_DISTANCE,
            DEFAULT_LINK_LINE_ALPHA,
            DEFAULT_PARTICLE_SPEED,
        );
        assert_eq!(s.particles.len(), 20 * PARTICLE_STRIDE);
        s.set_config(
            5,
            DEFAULT_LINK_DISTANCE,
            DEFAULT_LINK_LINE_ALPHA,
            DEFAULT_PARTICLE_SPEED,
        );
        assert_eq!(s.particles.len(), 5 * PARTICLE_STRIDE);
    }

    #[test]
    fn set_config_rescales_velocities_and_honours_link_distance() {
        let mut s = sim(10, 300.0, 200.0, 13);
        s.set_config(
            10,
            1.0,
            DEFAULT_LINK_LINE_ALPHA,
            DEFAULT_PARTICLE_SPEED * 2.0,
        );
        for p in s.particles.chunks(PARTICLE_STRIDE) {
            assert!(p[2].abs() <= DEFAULT_PARTICLE_SPEED * 2.0);
        }
        let lines = s.step();
        assert_eq!(lines, 0);
    }
}
