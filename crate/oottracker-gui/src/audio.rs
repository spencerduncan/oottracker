use {
    rodio::{Decoder, OutputStream, OutputStreamHandle, Sink},
    std::{
        fs::File,
        io::BufReader,
        path::Path,
        sync::{Arc, Mutex},
    },
};

/// Audio player that handles playback of fanfare sounds when items are collected.
pub struct AudioPlayer {
    _stream: OutputStream,
    stream_handle: OutputStreamHandle,
    current_sink: Arc<Mutex<Option<Sink>>>,
}

impl AudioPlayer {
    /// Creates a new AudioPlayer instance.
    /// Returns None if audio output cannot be initialized.
    pub fn new() -> Option<Self> {
        let (stream, stream_handle) = OutputStream::try_default().ok()?;
        Some(Self {
            _stream: stream,
            stream_handle,
            current_sink: Arc::new(Mutex::new(None)),
        })
    }

    /// Plays the audio file at the given path.
    /// If a sound is already playing, it will be stopped and the new sound will play.
    pub fn play(&self, path: &Path) {
        // Open the audio file
        let file = match File::open(path) {
            Ok(f) => f,
            Err(_) => return,
        };

        let reader = BufReader::new(file);

        // Create decoder for the audio file
        let source = match Decoder::new(reader) {
            Ok(s) => s,
            Err(_) => return,
        };

        // Stop any currently playing sound
        if let Ok(mut sink_guard) = self.current_sink.lock() {
            if let Some(sink) = sink_guard.take() {
                sink.stop();
            }

            // Create a new sink and play the sound
            if let Ok(sink) = Sink::try_new(&self.stream_handle) {
                sink.append(source);
                *sink_guard = Some(sink);
            }
        }
    }
}

impl std::fmt::Debug for AudioPlayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AudioPlayer").finish_non_exhaustive()
    }
}
