use logo::{DARK, render_svg};

#[test]
fn svg_has_vector_clip_path() {
    let dir = tempfile::tempdir().expect("tempdir");
    let path = dir.path().join("test.svg");

    render_svg(&path, 192.0, &DARK).expect("render_svg");

    let content = std::fs::read_to_string(&path).expect("read svg");

    assert!(
        content.contains("<clipPath"),
        "SVG contains no <clipPath> element — shape outline was rasterized"
    );

    let has_shape_clip = content.match_indices("<clipPath").any(|(start, _)| {
        let rest = &content[start..];
        let end = rest.find("</clipPath>").unwrap_or(rest.len());
        let body = &rest[..end];
        body.contains("<path") && (body.contains(" C ") || body.contains(" c "))
    });

    assert!(
        has_shape_clip,
        "no clipPath contains a Bézier curve <path> — shape outline was rasterized"
    );
}
