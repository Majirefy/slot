use slot_ui::text;

const CJK: &str = "中文显示测试游戏设置确定返回";

#[test]
fn label_font_lacks_cjk_and_the_fallback_covers_it() {
    let label = text::label_font().expect("label font");
    let cjk = text::cjk_font().expect("CJK fallback font");

    let missing_in_label: Vec<char> = CJK
        .chars()
        .filter(|ch| label.lookup_glyph_index(*ch) == 0)
        .collect();
    assert_eq!(
        missing_in_label.len(),
        CJK.chars().count(),
        "this test needs Chinese absent from the Latin font"
    );
    for ch in CJK.chars() {
        assert_ne!(
            cjk.lookup_glyph_index(ch),
            0,
            "CJK fallback lacks {ch:?}"
        );
    }
}

#[test]
fn chinese_layout_renders_real_ink() {
    let font = text::label_font().expect("label font");
    let (w, h, px) = (320.0, 64.0, 16.0);
    let layout = text::fit(font, "中文显示", w, 1, px, px);

    assert_eq!(layout.lines, vec!["中文显示"], "a short Chinese phrase stays whole");
    assert!(
        text::line_width(font, "中文显示", px, layout.tracking) > 0.0,
        "Chinese must have a measurable advance width"
    );

    let cov = text::coverage(w as u32, h as u32, &layout);
    let inked = cov.iter().filter(|&&a| a > 0).count();
    assert!(inked > 0, "Chinese glyphs rasterized to nothing");
}

#[test]
fn mixed_latin_and_chinese_keeps_both_visible() {
    let font = text::label_font().expect("label font");
    let (w, h, px) = (320.0, 64.0, 16.0);
    let layout = text::fit(font, "Slot 中文", w, 1, px, px);
    let cov = text::coverage(w as u32, h as u32, &layout);

    let cols: Vec<u32> = (0..w as u32)
        .filter(|x| (0..h as u32).any(|y| cov[(y * w as u32 + x) as usize] > 0))
        .collect();
    let (first, last) = (
        *cols.first().expect("some ink"),
        *cols.last().expect("some ink"),
    );
    assert!(
        last - first > px as u32,
        "mixed string rendered too narrow to hold Latin + CJK"
    );
}
