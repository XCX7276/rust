from playwright.sync_api import sync_playwright
import pathlib

here = pathlib.Path(__file__).parent.resolve()
footer = '''<div style="width:100%;font-family:'Noto Sans CJK SC';font-size:7.5pt;color:#6b7390;
padding:0 17mm;display:flex;justify-content:space-between;-webkit-print-color-adjust:exact;">
<span>ТЗ · пиксельная action-RPG с генерируемым миром</span>
<span><span class="pageNumber"></span> / <span class="totalPages"></span></span></div>'''

with sync_playwright() as p:
    b = p.chromium.launch()
    pg = b.new_page()
    pg.goto((here / 'TZ.html').as_uri())
    pg.wait_for_timeout(500)
    pg.pdf(path=str(here / '../ParodiaR_TZ.pdf'), format='A4', print_background=True,
           display_header_footer=False, header_template='<span></span>', footer_template=footer,
           prefer_css_page_size=True)
    b.close()
print('pdf ok')
