mod audio;

pub trait Builder {
    type FileAnalyzer;
    fn read_file_contents(self) -> std::io::Result<()>;
    fn write_file_contents(self) -> std::io::Result<()>;
    fn reconstruct_file(self) -> std::io::Result<()>;

    fn build(self) -> Self::FileAnalyzer;
}

