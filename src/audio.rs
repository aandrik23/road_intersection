use sdl2::audio::{AudioQueue, AudioSpecDesired};
use sdl2::Sdl;
use std::f32::consts::PI;

const SAMPLE_RATE: u32 = 44_100;

/// Short retro bleeps via SDL2 audio (no external sound files).
pub struct SoundEngine {
    queue: Option<AudioQueue<i16>>,
    muted: bool,
    signal_change: Vec<i16>,
    spawn: Vec<i16>,
    spawn_blocked: Vec<i16>,
}

impl SoundEngine {
    /// No audio device — for unit tests.
    pub fn silent() -> Self {
        Self {
            queue: None,
            muted: true,
            signal_change: Vec::new(),
            spawn: Vec::new(),
            spawn_blocked: Vec::new(),
        }
    }

    pub fn new(sdl: &Sdl) -> Self {
        let signal_change = mix_tone(&[(740.0, 55), (980.0, 70)], 0.22);
        let spawn = tone(520.0, 45, 0.18);
        let spawn_blocked = tone(180.0, 90, 0.12);

        match open_queue(sdl) {
            Ok(queue) => {
                let _ = queue.resume();
                Self {
                    queue: Some(queue),
                    muted: false,
                    signal_change,
                    spawn,
                    spawn_blocked,
                }
            }
            Err(e) => {
                eprintln!("Audio unavailable ({e}); running silent.");
                Self {
                    queue: None,
                    muted: true,
                    signal_change,
                    spawn,
                    spawn_blocked,
                }
            }
        }
    }

    pub fn toggle_mute(&mut self) {
        self.muted = !self.muted;
    }

    pub fn is_muted(&self) -> bool {
        self.muted
    }

    pub fn play_signal_change(&self) {
        self.play(&self.signal_change);
    }

    pub fn play_spawn(&self) {
        self.play(&self.spawn);
    }

    pub fn play_spawn_blocked(&self) {
        self.play(&self.spawn_blocked);
    }

    fn play(&self, samples: &[i16]) {
        if self.muted {
            return;
        }
        if let Some(queue) = &self.queue {
            let _ = queue.queue_audio(samples);
        }
    }
}

fn open_queue(sdl: &Sdl) -> Result<AudioQueue<i16>, String> {
    let audio = sdl.audio().map_err(|e| e.to_string())?;
    let desired = AudioSpecDesired {
        freq: Some(SAMPLE_RATE as i32),
        channels: Some(1),
        samples: Some(2048),
    };
    audio
        .open_queue(None, &desired)
        .map_err(|e| e.to_string())
}

fn tone(freq_hz: f32, duration_ms: u32, volume: f32) -> Vec<i16> {
    mix_tone(&[(freq_hz, duration_ms)], volume)
}

fn mix_tone(segments: &[(f32, u32)], volume: f32) -> Vec<i16> {
    let total_samples: u32 = segments
        .iter()
        .map(|(_, ms)| SAMPLE_RATE * ms / 1000)
        .sum();
    let mut out = Vec::with_capacity(total_samples as usize);
    let mut offset = 0u32;

    for &(freq_hz, duration_ms) in segments {
        let n = SAMPLE_RATE * duration_ms / 1000;
        for i in 0..n {
            let t = (offset + i) as f32 / SAMPLE_RATE as f32;
            let local = i as f32 / n.max(1) as f32;
            let attack = (local * 12.0).min(1.0);
            let release = ((1.0 - local) * 8.0).min(1.0);
            let env = attack * release;
            let sample =
                (env * volume * (2.0 * PI * freq_hz * t).sin() * i16::MAX as f32) as i16;
            out.push(sample);
        }
        offset += n;
    }
    out
}
