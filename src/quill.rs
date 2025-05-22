//! Wraps the Typst crate to provide a more user-friendly interface.

use std::{
    collections::HashMap,
    env,
    fs,
    io::Read,
    path::PathBuf,
    str,
    sync::{Arc, Mutex},
};

use typst::{
    compile, 
    diag::{eco_format, 
        FileError, 
        FileResult, 
        PackageError, 
        PackageResult
    }, 
    foundations::{
        Bytes, 
        Datetime
    }, 
    layout::{
        PagedDocument,
        Abs,
    },
    syntax::{
        package::PackageSpec, 
        FileId, 
        Source
    }, 
    text::{
        Font, 
        FontBook
    }, 
    utils::LazyHash, 
    Library, 
    World
};
use ureq::Agent;
use time::OffsetDateTime;
use typst_kit::fonts::{FontSearcher, FontSlot};
use typst_pdf::{pdf, PdfOptions};
use typst_svg::svg_merged;

/// Main interface that determines the environment for Typst.
pub struct Quill {
    /// Root path to which files will be resolved.
    root: PathBuf,

    /// The content of a source.
    source: Source,

    /// The standard library.
    library: LazyHash<Library>,

    /// Metadata about all known fonts.
    book: LazyHash<FontBook>,

    /// Metadata about all known fonts.
    fonts: Vec<FontSlot>,

    /// Map of all known files.
    files: Arc<Mutex<HashMap<FileId, FileEntry>>>,

    /// Cache directory (e.g. where packages are downloaded to).
    cache_directory: PathBuf,

    /// http agent to download packages.
    http: Agent,

    /// Datetime.
    time: OffsetDateTime,
}

impl Quill {
    pub fn new(source_content: String) -> Self {
        let root = env::current_dir().expect("Failed to get current directory");
        let fonts = FontSearcher::new().include_system_fonts(true).search();

        Self {
            library: LazyHash::new(Library::default()),
            book: LazyHash::new(fonts.book),
            root,
            fonts: fonts.fonts,
            source: Source::detached(source_content),
            time: OffsetDateTime::now_utc(),
            cache_directory: env::var_os("CACHE_DIRECTORY")
                .map(|os_path| os_path.into())
                .unwrap_or(env::temp_dir()),
            http: ureq::agent(),
            files: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Creates a new Quill instance with the given source content and root path.
    fn compile(&self) -> PagedDocument {
        match compile(self).output {
            Ok(document) => document,
            Err(error) => panic!("Failed to compile: {:?}", error),
        }
    }

    /// Compiles the source into a pdf file to given filename.
    pub fn pdf(&self, filename: &str) {
        let document = self.compile();
        let pdf = pdf(&document, &PdfOptions::default())
            .expect("Error exporting PDF");
        if let Some(parent) = PathBuf::from(filename).parent() {
            fs::create_dir_all(parent).expect("Error creating directory");
        }
        fs::write(filename, pdf).expect("Error writing PDF.");
    }

    /// Compiles the source into a svg file to given filename.
    pub fn svg(&self, filename: &str) {
        let document = self.compile();
        let svg = svg_merged(&document, Abs::pt(2.0));
        if let Some(parent) = PathBuf::from(filename).parent() {
            fs::create_dir_all(parent).expect("Error creating directory");
        }
        fs::write(filename, svg).expect("Error writing SVG.");
    }

    /// Helper to handle file requests.
    ///
    /// Requests will be either in packages or a local file.
    fn file(&self, id: FileId) -> FileResult<FileEntry> {
        let mut files = self.files.lock().map_err(|_| FileError::AccessDenied)?;
        if let Some(entry) = files.get(&id) {
            return Ok(entry.clone());
        }
        let path = if let Some(package) = id.package() {
            // Fetching file from package
            let package_dir = self.download_package(package)?;
            id.vpath().resolve(&package_dir)
        } else {
            // Fetching file from disk
            id.vpath().resolve(&self.root)
        }
        .ok_or(FileError::AccessDenied)?;

        let content = fs::read(&path).map_err(|error| FileError::from_io(error, &path))?;
        Ok(files
            .entry(id)
            .or_insert(FileEntry::new(content, None))
            .clone())
    }

    /// Downloads the package and returns the system path of the unpacked package.
    fn download_package(&self, package: &PackageSpec) -> PackageResult<PathBuf> {
        let package_subdir = format!("{}/{}/{}", package.namespace, package.name, package.version);
        let path = self.cache_directory.join(package_subdir);

        if path.exists() {
            return Ok(path);
        }

        eprintln!("downloading {package}");
        let url = format!(
            "https://packages.typst.org/{}/{}-{}.tar.gz",
            package.namespace, package.name, package.version,
        );

        let response = retry(|| {
            let response = self
                .http
                .get(&url)
                .call()
                .map_err(|error| eco_format!("{error}"))?;

            let status = response.status().as_u16();
            if !http_successful(status) {
                return Err(eco_format!(
                    "response returned unsuccessful status code {status}",
                ));
            }

            Ok(response)
        })
        .map_err(|error| PackageError::NetworkFailed(Some(error)))?;

        let (_, body) = response.into_parts();

        let mut compressed_archive = Vec::with_capacity(1000);
        body
            .into_reader()
            .read_to_end(&mut compressed_archive)
            .map_err(|error| PackageError::NetworkFailed(Some(eco_format!("{error}"))))?;
        let raw_archive = zune_inflate::DeflateDecoder::new(&compressed_archive)
            .decode_gzip()
            .map_err(|error| PackageError::MalformedArchive(Some(eco_format!("{error}"))))?;
        let mut archive = tar::Archive::new(raw_archive.as_slice());
        archive.unpack(&path).map_err(|error| {
            _ = fs::remove_dir_all(&path);
            PackageError::MalformedArchive(Some(eco_format!("{error}")))
        })?;

        Ok(path)
    }
}

/// This is the interface we have to implement such that `typst` can compile it.
///
/// I have tried to keep it as minimal as possible
impl World for Quill {
    /// Standard library.
    fn library(&self) -> &LazyHash<Library> {
        &self.library
    }

    /// Metadata about all known Books.
    fn book(&self) -> &LazyHash<FontBook> {
        &self.book
    }

    /// Accessing the main source file.
    fn main(&self) -> FileId {
        self.source.id()
    }

    /// Accessing a specified source file (based on `FileId`).
    fn source(&self, id: FileId) -> FileResult<Source> {
        if id == self.source.id() {
            Ok(self.source.clone())
        } else {
            self.file(id)?.source(id)
        }
    }

    /// Accessing a specified file (non-file).
    fn file(&self, id: FileId) -> FileResult<Bytes> {
        self.file(id).map(|file| file.bytes.clone())
    }

    /// Accessing a specified font per index of font book.
    fn font(&self, id: usize) -> Option<Font> {
        self.fonts[id].get()
    }

    /// Get the current date.
    ///
    /// Optionally, an offset in hours is given.
    fn today(&self, offset: Option<i64>) -> Option<Datetime> {
        let offset = offset.unwrap_or(0);
        let offset = time::UtcOffset::from_hms(offset.try_into().ok()?, 0, 0).ok()?;
        let time = self.time.checked_to_offset(offset)?;
        Some(Datetime::Date(time.date()))
    }
}

/// A File that will be stored in the HashMap.
#[derive(Clone, Debug)]
struct FileEntry {
    bytes: Bytes,
    source: Option<Source>,
}

impl FileEntry {
    fn new(bytes: Vec<u8>, source: Option<Source>) -> Self {
        Self {
            bytes: Bytes::new(bytes),
            source,
        }
    }

    fn source(&mut self, id: FileId) -> FileResult<Source> {
        let source = if let Some(source) = &self.source {
            source
        } else {
            let contents = str::from_utf8(&self.bytes).map_err(|_| FileError::InvalidUtf8)?;
            let contents = contents.trim_start_matches('\u{feff}');
            let source = Source::new(id, contents.into());
            self.source.insert(source)
        };
        Ok(source.clone())
    }
}

fn retry<T, E>(mut f: impl FnMut() -> Result<T, E>) -> Result<T, E> {
    if let Ok(ok) = f() {
        Ok(ok)
    } else {
        f()
    }
}

fn http_successful(status: u16) -> bool {
    // 2XX
    status / 100 == 2
}