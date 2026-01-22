#!/usr/bin/env python3
"""Generate offline HTML docs into docs/.

Inputs:
- docs/README.md
- docs/REFERENCES.md

Both inputs are bilingual and must contain blocks:
- <!-- BEGIN:FR --> ... <!-- END:FR -->
- <!-- BEGIN:EN --> ... <!-- END:EN -->

Outputs:
- docs/README.html
- docs/REFERENCES.html
- docs/icon.png (copied from resources/icon.png)

The generated HTML is self-contained (relative links only) and works without
any subfolder inside docs/.
"""

from __future__ import annotations

import re
from pathlib import Path

import markdown

ROOT = Path(__file__).resolve().parent.parent
DOCS_DIR = ROOT / "docs"


def _extract_block(md_text: str, lang: str) -> str:
    match = re.search(
        rf"<!--\s*BEGIN:{lang}\s*-->(.*?)<!--\s*END:{lang}\s*-->",
        md_text,
        flags=re.S,
    )
    if not match:
        raise RuntimeError(f"Missing bilingual block for {lang}")
    return match.group(1).strip() + "\n"


def _render(md_text: str) -> tuple[str, str]:
    extensions = [
        "fenced_code",
        "tables",
        "toc",
        "sane_lists",
    ]
    md = markdown.Markdown(
        extensions=extensions,
        extension_configs={"toc": {"permalink": True}},
    )
    html = md.convert(md_text)
    toc = md.toc or ""
    return html, toc


def _write_page(*, title: str, current: str, out_file: Path, md_file: Path) -> None:
    md_text = md_file.read_text(encoding="utf-8")
    fr_md = _extract_block(md_text, "FR")
    en_md = _extract_block(md_text, "EN")

    fr_html, toc_fr = _render(fr_md)
    en_html, toc_en = _render(en_md)

    nav_pages = (
        "          <div class=\"nav-title\">Pages</div>\n"
        f"          <a href=\"README.html\"{' aria-current=\"page\"' if current == 'README.html' else ''}>README</a>\n"
        f"<a href=\"REFERENCES.html\"{' aria-current=\"page\"' if current == 'REFERENCES.html' else ''}>References</a>\n"
    )

    html = f"""<!doctype html>
<html lang="fr">
<head>
  <meta charset="utf-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1" />
  <title>{title}</title>
  <link rel="stylesheet" href="style.css" />
</head>
<body>
  <input class="docs-toggle" type="radio" name="lang" id="lang-fr" checked />
  <input class="docs-toggle" type="radio" name="lang" id="lang-en" />

  <input class="docs-toggle" type="radio" name="theme" id="theme-auto" checked />
  <input class="docs-toggle" type="radio" name="theme" id="theme-light" />
  <input class="docs-toggle" type="radio" name="theme" id="theme-dark" />

  <div class="app">
    <div class="layout">
      <aside class="sidebar" aria-label="Documentation navigation">
        <div class="brand">
          <img src="icon.png" alt="Battery Manager" />
          <div>
            <div class="title">Battery Manager</div>
            <div class="subtitle">Docs • menu • FR/EN • thème</div>
          </div>
        </div>

        <div class="controls">
          <div class="control-group">
            <div class="label">Langue / Language</div>
            <div class="pills" role="group" aria-label="Language">
              <label class="pill" for="lang-fr">FR</label>
              <label class="pill" for="lang-en">EN</label>
            </div>
          </div>

          <div class="control-group">
            <div class="label">Thème / Theme</div>
            <div class="pills" role="group" aria-label="Theme">
              <label class="pill" for="theme-auto">Auto</label>
              <label class="pill" for="theme-light">Clair</label>
              <label class="pill" for="theme-dark">Sombre</label>
            </div>
          </div>
        </div>

        <nav class="nav" aria-label="Pages">
{nav_pages}
          <div class="nav-title">Sommaire</div>
          <div class="toc-fr">{toc_fr}</div>
          <div class="toc-en">{toc_en}</div>
        </nav>
      </aside>

      <main class="content">
        <div class="lang lang-fr">{fr_html}</div>
        <div class="lang lang-en">{en_html}</div>
      </main>
    </div>
  </div>
</body>
</html>
"""

    out_file.write_text(html, encoding="utf-8")


def main() -> None:
    DOCS_DIR.mkdir(parents=True, exist_ok=True)

    # Keep icon in docs/ (no subfolder)
    (DOCS_DIR / "icon.png").write_bytes((ROOT / "resources" / "icon.png").read_bytes())

    _write_page(
        title="Battery Manager — README",
        current="README.html",
        out_file=DOCS_DIR / "README.html",
        md_file=DOCS_DIR / "README.md",
    )
    _write_page(
        title="Battery Manager — References",
        current="REFERENCES.html",
        out_file=DOCS_DIR / "REFERENCES.html",
        md_file=DOCS_DIR / "REFERENCES.md",
    )


if __name__ == "__main__":
    main()
