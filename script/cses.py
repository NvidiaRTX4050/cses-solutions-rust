import os
import re
import requests
from bs4 import BeautifulSoup
from weasyprint import HTML
from latex2svg import latex2svg

def convert_latex_to_svg_html(text):
    # Convert inline $...$ and display $$...$$ math
    text = re.sub(r'\$(.+?)\$', lambda m: latex_to_svg_html(m.group(1)), text)
    text = re.sub(r'\$\$(.+?)\$\$', lambda m: latex_to_svg_html(m.group(1)), text)
    return text

def latex_to_svg_html(latex_code):
    try:
        svg_output = latex2svg(latex_code)['svg']
        return svg_output
    except Exception as e:
        print(f"❌ Error converting {latex_code}: {e}")
        return latex_code  # fallback

# Step 1: Get problem IDs
print("[1] Fetching all problem IDs from CSES...")
url = "https://cses.fi/problemset/"
response = requests.get(url)
soup = BeautifulSoup(response.text, "html.parser")

problem_links = soup.select('a[href^="/problemset/task/"]')

problem_ids = []
titles_list = []

for link in problem_links:
    href = link['href']
    pid = href.split('/')[3]
    if pid.isdigit():
        problem_ids.append(int(pid))
        titles_list.append(link.text.strip())

problem_ids = sorted(list(set(problem_ids)))

print(f"✅ Found {len(problem_ids)} problems")

# Step 2: Download problems and build HTML
toc_html = "<h1>Table of Contents</h1><ul>"
problem_pages_html = ""

for idx, (pid, title) in enumerate(zip(problem_ids, titles_list), start=1):
    anchor_name = f"problem{idx}"
    toc_html += f'<li><a href="#{anchor_name}">{idx}. {title}</a></li>'

    task_url = f"https://cses.fi/problemset/task/{pid}/"
    print(f"  ({idx}/{len(problem_ids)}) Fetching: {task_url}")
    
    task_resp = requests.get(task_url)
    task_soup = BeautifulSoup(task_resp.text, 'html.parser')

    title_tag = task_soup.find('h1')
    content_div = task_soup.find('div', class_='content')

    if not title_tag or not content_div:
        print(f"    ❌ Skipping problem ID {pid} (structure not found)")
        continue

    content_html = str(content_div)

    # Render math
    content_html = convert_latex_to_svg_html(content_html)

    # Format each problem
    page_html = f"""
    <a name="{anchor_name}"></a>
    <h1>Problem {idx}: {title}</h1>
    <div class="problem">{content_html}</div>

    <p class="solution-space">(Write your solution here...)</p>

    <!-- Blank back page -->
    <div class="blank-page"></div>
    <div style="page-break-after: always;"></div>
    """
    problem_pages_html += page_html

toc_html += "</ul>"
all_html = toc_html + problem_pages_html

# Step 3: Generate final HTML with styles
final_html = f"""
<html>
<head>
    <meta charset="utf-8">
    <style>
        @page {{
            size: A4;
            margin: 1in;
            @bottom-center {{
                content: "Page " counter(page);
                font-size: 8pt;
                color: #555;
            }}
            @top-center {{
                content: "CSES Problem Set";
                font-size: 9pt;
                color: #000;
            }}
        }}
        body {{
            font-family: sans-serif;
            font-size: 9pt;
            color: #000;
        }}
        h1 {{
            color: #333;
            font-size: 12pt;
            margin-bottom: 10px;
        }}
        ul {{
            font-size: 9pt;
        }}
        .problem {{
            font-size: 8pt;
            max-height: 45vh;
            overflow: hidden;
        }}
        .solution-space {{
            margin-top: 180px;
            height: 50vh;
            border-top: 1px dashed #aaa;
        }}
        .blank-page {{
            height: 100vh;
        }}
        svg {{
            vertical-align: middle;
            height: 1em;
        }}
        a {{
            text-decoration: none;
            color: blue;
        }}
    </style>
</head>
<body>
{all_html}
</body>
</html>
"""

with open("cses_problems.html", "w", encoding="utf-8") as f:
    f.write(final_html)

HTML('cses_problems.html').write_pdf('cses_problems.pdf')

print("✅ Done! Your math-rendered, paginated PDF is saved as: cses_problems.pdf")
