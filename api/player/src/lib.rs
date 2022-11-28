struct Track {
    name: String,
}

impl Track {
    fn play(&self) {
        println!("Playing: {}", self.name);
    }
    fn pause(&self) {
        println!("Paused: {}", self.name);
    }
    fn load(&self) {
        println!("Loaded: {}", self.name);
    }
    fn unload(&self) {
        println!("Unloaded: {}", self.name);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn play() {
        let track = Track {
            name: String::from("Baby"),
        };

        track.play();
    }

    #[test]
    fn pause() {
        let track = Track {
            name: String::from("Baby"),
        };

        track.pause();
    }
}
