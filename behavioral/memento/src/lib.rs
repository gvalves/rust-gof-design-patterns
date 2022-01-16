use std::{cell::RefCell, rc::Rc};

use chrono::prelude::*;

// pub trait ImageMemento {
//     fn name(&self) -> &str;
//     fn date(&self) -> DateTime<Local>;
// }

pub trait VariantIdent {
    fn variant_ident(&self) -> &str;
}

macro_rules! enum_variant_ident {
    (
        $(#[$meta:meta])*
        $vis:vis enum $enum_name:ident {
            $(
                $(#[$variant_meta:meta])*
                $variant_name:ident
            ),*$(,)?
        }
    ) => {
        $(#[$meta])*
        $vis enum $enum_name {
            $(
                $(#[$variant_meta])*
                $variant_name,
            )*
        }

        impl VariantIdent for $enum_name {
            fn variant_ident(&self) -> &str {
                match self {
                    $($enum_name::$variant_name => stringify!($variant_name),)*
                }
            }
        }
    };
}

enum_variant_ident! {
    #[derive(Clone, Copy, Debug)]
    pub enum ImageFormat {
        Jpg,
        Png,
        Gif,
    }
}

#[derive(Debug, Clone)]
pub struct Image {
    path: String,
    format: ImageFormat,
}

impl Image {
    pub fn new(path: &str, format: ImageFormat) -> Self {
        let path = path.to_string();
        Self { path, format }
    }

    pub fn convert_format_to(&mut self, format: ImageFormat) {
        self.format = format;

        let path = self.path.rsplit_once(".");
        let format_ext = self.format.variant_ident().to_lowercase();

        match path {
            Some((path, _)) => self.path = format!("{}.{}", path, format_ext),
            None => self.path.push_str(&format!(".{}", format_ext)),
        }
    }

    pub fn save(&self) -> ImageMemento {
        let date = Local::now();
        let name = date.to_rfc3339_opts(SecondsFormat::Secs, false);
        let path = self.path.clone();
        let format = self.format;

        let memento = ImageMemento::new(name, date, path, format);

        memento
    }

    pub fn restore(&mut self, memento: ImageMemento) {
        let memento = memento;

        self.path = memento.path().to_string();
        self.format = memento.format();
    }
}

impl From<Image> for Rc<RefCell<Image>> {
    fn from(image: Image) -> Self {
        Rc::new(RefCell::new(image))
    }
}

#[derive(Debug)]
pub struct ImageMemento {
    name: String,
    date: DateTime<Local>,
    path: String,
    format: ImageFormat,
}

impl ImageMemento {
    pub fn new(name: String, date: DateTime<Local>, path: String, format: ImageFormat) -> Self {
        Self {
            name,
            date,
            path,
            format,
        }
    }

    /// Get a reference to the image memento's name.
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    /// Get a reference to the image memento's date.
    pub fn date(&self) -> DateTime<Local> {
        self.date
    }

    /// Get a reference to the image memento impl's path.
    pub fn path(&self) -> &str {
        self.path.as_ref()
    }

    /// Get a reference to the image memento impl's format.
    pub fn format(&self) -> ImageFormat {
        self.format
    }
}

pub struct ImageBackupManager {
    image: Rc<RefCell<Image>>,
    mementos: Vec<ImageMemento>,
}

impl ImageBackupManager {
    pub fn new(image: &Rc<RefCell<Image>>) -> Self {
        let image = Rc::clone(image);
        Self {
            image,
            mementos: vec![],
        }
    }

    pub fn backup(&mut self) {
        println!("Backup: Salvando o estado de Image");
        self.mementos.push(self.image.borrow().save());
    }

    pub fn undo(&mut self) {
        let memento = self.mementos.pop();

        match memento {
            Some(memento) => {
                let name = memento.name().to_string();
                self.image.borrow_mut().restore(memento);
                println!("Backup: {} foi restaurado com sucesso!", name);
            }
            None => {
                println!("Backup: No mementos");
                return;
            }
        }
    }

    pub fn show_mementos(&self) {
        for memento in self.mementos.iter() {
            println!("{:?}", memento);
        }
    }
}
