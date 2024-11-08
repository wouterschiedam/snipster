#[derive(Debug, Clone)]
pub struct FzfOptions {
    multi: bool,
    ansi: bool,
    reverse: bool,
    header: Option<u8>,
    delimiter: Option<String>,
    with_nth: Option<String>,
    preview: Option<String>,
    preview_window: Option<String>,
    bind: Option<String>,
    height: Option<String>,
}

impl FzfOptions {
    fn new() -> Self {
        FzfOptions {
            multi: false,
            ansi: false,
            reverse: false,
            header: None,
            delimiter: None,
            with_nth: None,
            preview: None,
            preview_window: None,
            bind: None,
            height: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FzfBuilder {
    options: FzfOptions,
}

impl FzfBuilder {
    pub fn new() -> Self {
        FzfBuilder {
            options: FzfOptions::new(),
        }
    }

    // Methods to set various options
    pub fn multi(mut self) -> Self {
        self.options.multi = true;
        self
    }

    pub fn ansi(mut self) -> Self {
        self.options.ansi = true;
        self
    }

    pub fn reverse(mut self) -> Self {
        self.options.reverse = true;
        self
    }

    pub fn header(mut self, header: u8) -> Self {
        self.options.header = Some(header);
        self
    }

    pub fn delimiter(mut self, delimiter: &str) -> Self {
        self.options.delimiter = Some(delimiter.to_string());
        self
    }

    pub fn with_nth(mut self, with_nth: &str) -> Self {
        self.options.with_nth = Some(with_nth.to_string());
        self
    }

    pub fn preview(mut self, preview: &str) -> Self {
        self.options.preview = Some(preview.to_string());
        self
    }

    pub fn preview_window(mut self, window: &str) -> Self {
        self.options.preview_window = Some(window.to_string());
        self
    }

    pub fn bind(mut self, bind: &str) -> Self {
        self.options.bind = Some(bind.to_string());
        self
    }

    pub fn height(mut self, height: &str) -> Self {
        self.options.height = Some(height.to_string());
        self
    }

    // Method to execute FZF with the built options
    pub fn build(self) -> String {
        let mut command = "fzf ".to_string();

        if self.options.multi {
            command.push_str("--multi ");
        }
        if self.options.ansi {
            command.push_str("--ansi ");
        }
        if self.options.reverse {
            command.push_str("--reverse ");
        }
        if let Some(ref header) = self.options.header {
            command.push_str(&format!(" --header-lines={} ", header));
        }
        if let Some(ref delimiter) = self.options.delimiter {
            command.push_str(&format!(" --delimiter={} ", delimiter));
        }
        if let Some(ref with_nth) = self.options.with_nth {
            command.push_str(&format!("--with-nth={} ", with_nth));
        }
        if let Some(ref preview) = self.options.preview {
            command.push_str(&format!("--preview={} ", preview));
        }
        if let Some(ref preview_window) = self.options.preview_window {
            command.push_str(&format!("--preview-window={} ", preview_window));
        }
        if let Some(ref bind) = self.options.bind {
            command.push_str(&format!("--bind='{}' ", bind));
        }
        if let Some(ref height) = self.options.height {
            command.push_str(&format!("--height={} ", height));
        }

        command
    }
}
