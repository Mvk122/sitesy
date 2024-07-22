use pulldown_cmark::{Options, Parser, TextMergeStream};

// Extracted to separate file to ensure consistency of options
pub fn get_pulldown_cmark_iterator(
    file_contents: &str,
) -> TextMergeStream<'_, pulldown_cmark::Parser<'_>> {
    let options = Options::from_bits_truncate(Options::ENABLE_YAML_STYLE_METADATA_BLOCKS.bits());
    let parser = Parser::new_ext(file_contents, options);
    return TextMergeStream::new(parser);
}
