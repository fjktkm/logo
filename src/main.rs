use std::path::Path;

use anyhow::{Context as _, Result};

use logo::{ALL_THEMES, render_pdf, render_png, render_svg};

const PNG_SIZES: &[u32] = &[256, 512, 1024];
const VECTOR_SIZE: f64 = 576.0;
const OUTPUT_DIR: &str = "dist";

fn main() -> Result<()> {
    let output_dir = Path::new(OUTPUT_DIR);
    std::fs::create_dir_all(output_dir).with_context(|| format!("create {OUTPUT_DIR}"))?;

    for theme in ALL_THEMES {
        for &size in PNG_SIZES {
            let path = output_dir.join(format!("logo_{}_{}.png", theme.name, size));
            render_png(&path, size, theme).with_context(|| format!("render {}", path.display()))?;
            println!("  wrote {}", path.display());
        }

        let svg = output_dir.join(format!("logo_{}.svg", theme.name));
        render_svg(&svg, VECTOR_SIZE, theme)
            .with_context(|| format!("render {}", svg.display()))?;
        println!("  wrote {}", svg.display());

        let pdf = output_dir.join(format!("logo_{}.pdf", theme.name));
        render_pdf(&pdf, VECTOR_SIZE, theme)
            .with_context(|| format!("render {}", pdf.display()))?;
        println!("  wrote {}", pdf.display());
    }

    Ok(())
}
