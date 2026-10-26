"""TZ.md + style.css -> TZ.html (затем render_tz.py делает PDF).

Запуск:  python3 build_tz.py && python3 render_tz.py
Нужны пакеты: markdown, pygments (и playwright для render.py).
"""
import datetime
import pathlib
import re

import markdown
from pygments.formatters import HtmlFormatter

HERE = pathlib.Path(__file__).parent
TODAY = datetime.date.today().strftime('%d.%m.%Y')

src = (HERE.parent / 'TZ.md').read_text(encoding='utf-8')
md = markdown.Markdown(
    extensions=['tables', 'fenced_code', 'codehilite', 'md_in_html', 'attr_list'],
    extension_configs={'codehilite': {'guess_lang': False, 'css_class': 'codehilite'}},
)
body = md.convert(src)

# Нумерация разделов + оглавление
toc = []


def h2(m):
    toc.append((m.group(1), m.group(2)))
    return (f'<h2 id="{m.group(1)}"><span class="num">{len(toc):02d}</span>'
            f'<span class="t">{m.group(2)}</span></h2>')


body = re.sub(r'<h2 id="([^"]+)">(.*?)</h2>', h2, body)

# Пиксельный кристалл (тот же, что в ТЗ) — на обложку
crystal = [
    "......##......", ".....#aa#.....", "....#abba#....", "...#abbbba#...",
    "..#abbcbbba#..", ".#abbcccbbba#.", "#abbccxccbbba#", "#abbcx.xcbbba#",
    ".#abbx.xbbba#.", "..#abbxbbba#..", "...#abbbba#...", "....#abba#....",
    ".....#aa#.....", "......##......",
]
colors = {'#': '#1c2a44', 'a': '#2f6f8f', 'b': '#56d4e0', 'c': '#b4f1f5', 'x': '#f0b54a'}
px = 14
rects = [f'<rect x="{x*px}" y="{y*px}" width="{px}" height="{px}" fill="{colors[ch]}"/>'
         for y, row in enumerate(crystal) for x, ch in enumerate(row) if ch in colors]
for sx, sy, c in [(-3, 2, '#56d4e0'), (16, 4, '#b4f1f5'), (-2, 11, '#f0b54a'), (17, 10, '#56d4e0'), (15, 0, '#2f6f8f')]:
    rects.append(f'<rect x="{sx*px}" y="{sy*px}" width="{px}" height="{px}" fill="{c}"/>')
crystal_svg = (f'<svg class="crystal" viewBox="{-4*px} {-1*px} {23*px} {16*px}" '
               f'xmlns="http://www.w3.org/2000/svg" shape-rendering="crispEdges">{"".join(rects)}</svg>')

toc_html = '\n'.join(f'<li><a href="#{hid}"><span class="num">{i:02d}</span><span class="t">{t}</span></a></li>'
                     for i, (hid, t) in enumerate(toc, 1))

extra = '''@page { @bottom-left { content: "ParodiaR · техническое задание"; } }
h2#vision, h2#requirements, h2#stack, h2#method, h2#tree, h2#architecture, h2#plugin, h2#code, h2#stages, h2#decisions { break-before: page; margin-top: 0; }
figure { margin: 0 0 5mm; break-inside: avoid; }
figure img { width: 100%; display: block; image-rendering: pixelated; border: 1px solid var(--border); border-radius: 4px; }
figcaption { font-size: 8.6pt; color: var(--muted); margin-top: 4px; line-height: 1.4; }
figcaption b { color: var(--text); }
.figrow { display: grid; grid-template-columns: 1fr 1fr; gap: 12px; margin: 0 0 5mm; align-items: start; }
.figrow figure { margin: 0; }
.figtext ul { margin-top: 0; }
figure.atlas { width: 62%; }
.diagram svg { width: 100%; height: auto; }
h3 .tag { font-size: 9pt; padding: 2px 9px; }
.stage.milestone .dur { color: var(--gold); }
'''
css = (HERE / 'style.css').read_text(encoding='utf-8')
pyg_css = HtmlFormatter(style='one-dark').get_style_defs('.codehilite')

page = f'''<!doctype html>
<html lang="ru"><head><meta charset="utf-8"><title>ParodiaR — техническое задание</title>
<style>{pyg_css}</style><style>{css}</style><style>{extra}</style></head>
<body>
<section class="cover">
  <div class="cover-top"><span class="pill">Техническое задание · Rust + Bevy</span><span class="pill ghost">обновлён {TODAY}</span></div>
  <div class="cover-art">{crystal_svg}</div>
  <h1>ParodiaR<br><span>техническое задание</span></h1>
  <p class="cover-sub">Воксельная RPG-survival на Rust и Bevy: требования, стек, методологии, дерево проекта, архитектура, правила кода и этапы с проверкой «готово, когда».</p>
  <div class="refs">
    <div><b>Требования</b><span>что и какое</span></div>
    <div><b>Стек</b><span>методологии</span></div>
    <div><b>Дерево</b><span>архитектура</span></div>
    <div><b>Правила</b><span>кода и git</span></div>
    <div><b>Этапы</b><span>0 → 18</span></div>
  </div>
  <div class="cover-foot">Ведёт Claude вместе с Даней · исходник: docs/TZ.md</div>
</section>
<section class="toc"><h2 class="toc-h">Содержание</h2><ol>{toc_html}</ol></section>
<main>
{body}
</main>
</body></html>'''

(HERE / 'TZ.html').write_text(page, encoding='utf-8')
print('ok:', len(toc), 'разделов')
