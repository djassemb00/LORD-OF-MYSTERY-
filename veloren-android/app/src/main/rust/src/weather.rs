//! Weather and Day/Night Cycle System
//!
//! Handles weather effects, time of day, and lighting changes.

use vek::Vec3;

// ========================
// Time of Day
// ========================

/// Time of day
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TimeOfDay {
    pub hour: f32,    // 0-24
    pub minute: f32,  // 0-60
    pub day: u32,
}

impl TimeOfDay {
    pub fn new(hour: f32, minute: f32, day: u32) -> Self {
        Self {
            hour: hour % 24.0,
            minute: minute % 60.0,
            day,
        }
    }

    /// Get time as fraction of day (0-1)
    pub fn day_fraction(&self) -> f32 {
        (self.hour + self.minute / 60.0) / 24.0
    }

    /// Check if it's daytime
    pub fn is_daytime(&self) -> bool {
        self.hour >= 6.0 && self.hour < 18.0
    }

    /// Check if it's nighttime
    pub fn is_nighttime(&self) -> bool {
        !self.is_daytime()
    }

    /// Check if it's dawn
    pub fn is_dawn(&self) -> bool {
        self.hour >= 5.0 && self.hour < 7.0
    }

    /// Check if it's dusk
    pub fn is_dusk(&self) -> bool {
        self.hour >= 17.0 && self.hour < 19.0
    }

    /// Get sun position (normalized)
    pub fn sun_position(&self) -> Vec3<f32> {
        let fraction = self.day_fraction();
        let angle = fraction * std::f32::consts::TAU - std::f32::consts::FRAC_PI_2;
        
        Vec3::new(
            angle.cos(),
            angle.sin(),
            0.5,
        )
    }

    /// Get sky color based on time
    pub fn sky_color(&self) -> Vec3<f32> {
        if self.is_daytime() {
            // Day sky - blue
            Vec3::new(0.4, 0.6, 0.9)
        } else if self.is_dawn() {
            // Dawn - orange/pink
            Vec3::new(0.9, 0.5, 0.3)
        } else if self.is_dusk() {
            // Dusk - purple/orange
            Vec3::new(0.6, 0.3, 0.5)
        } else {
            // Night - dark blue
            Vec3::new(0.05, 0.05, 0.15)
        }
    }

    /// Get ambient light level (0-1)
    pub fn ambient_light(&self) -> f32 {
        if self.is_daytime() {
            1.0
        } else if self.is_dawn() || self.is_dusk() {
            0.5
        } else {
            0.2
        }
    }

    /// Advance time
    pub fn advance(&mut self, seconds: f32, time_scale: f32) {
        let minutes_passed = seconds * time_scale / 60.0;
        self.minute += minutes_passed;
        
        while self.minute >= 60.0 {
            self.minute -= 60.0;
            self.hour += 1.0;
        }
        
        while self.hour >= 24.0 {
            self.hour -= 24.0;
            self.day += 1;
        }
    }
}

// ========================
// Weather Types
// ========================

/// Weather condition
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WeatherType {
    Clear,
    Cloudy,
    Rain,
    Thunderstorm,
    Snow,
    Blizzard,
    Fog,
}

impl WeatherType {
    /// Get visibility multiplier (0-1)
    pub fn visibility(&self) -> f32 {
        match self {
            WeatherType::Clear => 1.0,
            WeatherType::Cloudy => 0.9,
            WeatherType::Rain => 0.7,
            WeatherType::Thunderstorm => 0.5,
            WeatherType::Snow => 0.6,
            WeatherType::Blizzard => 0.3,
            WeatherType::Fog => 0.2,
        }
    }

    /// Get rain intensity (0-1)
    pub fn rain_intensity(&self) -> f32 {
        match self {
            WeatherType::Clear | WeatherType::Cloudy | WeatherType::Fog => 0.0,
            WeatherType::Rain => 0.5,
            WeatherType::Thunderstorm => 1.0,
            WeatherType::Snow => 0.3,
            WeatherType::Blizzard => 0.8,
        }
    }

    /// Check if has precipitation
    pub fn has_precipitation(&self) -> bool {
        matches!(self, 
            WeatherType::Rain | 
            WeatherType::Thunderstorm | 
            WeatherType::Snow | 
            WeatherType::Blizzard
        )
    }
}

// ========================
// Weather System
// ========================

/// Weather state
pub struct WeatherSystem {
    pub current_weather: WeatherType,
    pub target_weather: WeatherType,
    pub transition_progress: f32,
    pub transition_duration: f32,
    pub wind_direction: f32,
    pub wind_speed: f32,
    pub temperature: f32,
    pub humidity: f32,
}

impl WeatherSystem {
    pub fn new() -> Self {
        Self {
            current_weather: WeatherType::Clear,
            target_weather: WeatherType::Clear,
            transition_progress: 1.0,
            transition_duration: 60.0, // 1 minute transition
            wind_direction: 0.0,
            wind_speed: 0.0,
            temperature: 20.0,
            humidity: 0.5,
        }
    }

    /// Update weather
    pub fn update(&mut self, delta_time: f32) {
        // Update transition
        if self.transition_progress < 1.0 {
            self.transition_progress += delta_time / self.transition_duration;
            if self.transition_progress >= 1.0 {
                self.transition_progress = 1.0;
                self.current_weather = self.target_weather;
            }
        }

        // Randomly change weather
        if self.transition_progress >= 1.0 && rand::random::<f32>() < 0.001 {
            self.change_weather();
        }

        // Update wind
        self.wind_direction += (rand::random::<f32>() - 0.5) * delta_time * 0.1;
        self.wind_speed += (rand::random::<f32>() - 0.5) * delta_time * 0.01;
        self.wind_speed = self.wind_speed.clamp(0.0, 1.0);
    }

    /// Change to random weather
    fn change_weather(&mut self) {
        let weathers = [
            WeatherType::Clear,
            WeatherType::Cloudy,
            WeatherType::Rain,
            WeatherType::Thunderstorm,
            WeatherType::Snow,
            WeatherType::Fog,
        ];
        
        self.target_weather = weathers[rand::random::<usize>() % weathers.len()];
        self.transition_progress = 0.0;
    }

    /// Get current visibility
    pub fn visibility(&self) -> f32 {
        let current_vis = self.current_weather.visibility();
        let target_vis = self.target_weather.visibility();
        
        current_vis * (1.0 - self.transition_progress) + target_vis * self.transition_progress
    }

    /// Get current rain intensity
    pub fn rain_intensity(&self) -> f32 {
        let current = self.current_weather.rain_intensity();
        let target = self.target_weather.rain_intensity();
        
        current * (1.0 - self.transition_progress) + target * self.transition_progress
    }
}

// ========================
// Day/Night Cycle
// ========================

/// Day/night cycle manager
pub struct DayNightCycle {
    pub time: TimeOfDay,
    pub time_scale: f32, // 1 = real time, 60 = 1 hour per minute
    pub weather: WeatherSystem,
}

impl DayNightCycle {
    pub fn new() -> Self {
        Self {
            time: TimeOfDay::new(12.0, 0.0, 1), // Start at noon
            time_scale: 60.0, // 1 game hour = 1 real minute
            weather: WeatherSystem::new(),
        }
    }

    /// Update cycle
    pub fn update(&mut self, delta_time: f32) {
        self.time.advance(delta_time, self.time_scale);
        self.weather.update(delta_time);
    }

    /// Get sun direction for lighting
    pub fn sun_direction(&self) -> Vec3<f32> {
        self.time.sun_position()
    }

    /// Get sky color
    pub fn sky_color(&self) -> Vec3<f32> {
        self.time.sky_color()
    }

    /// Get fog color
    pub fn fog_color(&self) -> Vec3<f32> {
        let sky = self.sky_color();
        let weather_factor = 1.0 - self.weather.visibility() * 0.5;
        sky * weather_factor
    }

    /// Get ambient light multiplier
    pub fn ambient_light(&self) -> f32 {
        self.time.ambient_light() * self.weather.visibility()
    }
}
