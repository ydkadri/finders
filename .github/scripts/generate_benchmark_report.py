#!/usr/bin/env python3
"""
Generate custom HTML benchmark reports with historical trends.

Reads benchmarks/history.json and generates:
- Summary table with all scenarios
- Historical trends with color-coded changes
- Links to detailed Criterion reports
"""

import json
import sys
from pathlib import Path
from typing import List, Dict, Any


def load_history(history_file: Path) -> List[Dict[str, Any]]:
    """Load benchmark history from JSON file."""
    if not history_file.exists():
        return []

    with open(history_file) as f:
        return json.load(f)


def calculate_change(current: float, previous: float) -> Dict[str, Any]:
    """Calculate percentage change and determine status."""
    if previous == 0:
        return {"percent": 0, "status": "neutral"}

    change_percent = ((current - previous) / previous) * 100

    # For benchmark times, lower is better
    # Green: >5% faster, Amber: within 5%, Red: >5% slower
    if change_percent <= -5:
        status = "better"  # Faster
    elif change_percent >= 5:
        status = "worse"   # Slower
    else:
        status = "neutral" # Similar

    return {
        "percent": change_percent,
        "status": status,
        "symbol": "↓" if change_percent < 0 else ("↑" if change_percent > 0 else "→")
    }


def format_ms(value: int) -> str:
    """Format milliseconds nicely."""
    return f"{value}ms"


def generate_summary_table(history: List[Dict[str, Any]]) -> str:
    """Generate HTML summary table for latest results."""
    if not history:
        return "<p>No benchmark data available yet.</p>"

    latest = history[-1]
    previous = history[-2] if len(history) > 1 else None

    rows = []
    for result in latest["results"]:
        scenario = result["scenario"]
        size = result["size"]
        pattern = result["pattern"]

        # Get values
        finder = result.get("finder")
        find_grep = result.get("find_grep")
        ripgrep = result.get("ripgrep")

        # Calculate changes if we have previous data
        changes = {}
        if previous:
            prev_result = next((r for r in previous["results"] if r["scenario"] == scenario), None)
            if prev_result:
                if finder is not None and prev_result.get("finder"):
                    changes["finder"] = calculate_change(finder, prev_result["finder"])
                if find_grep is not None and prev_result.get("find_grep"):
                    changes["find_grep"] = calculate_change(find_grep, prev_result["find_grep"])
                if ripgrep is not None and prev_result.get("ripgrep"):
                    changes["ripgrep"] = calculate_change(ripgrep, prev_result["ripgrep"])

        # Format cells with change indicators
        def format_cell(value, tool):
            if value is None:
                return "N/A"

            change_html = ""
            if tool in changes:
                change = changes[tool]
                color_class = f"change-{change['status']}"
                change_html = f' <span class="{color_class}" title="{change["percent"]:.1f}%">{change["symbol"]}</span>'

            return f"{format_ms(value)}{change_html}"

        pattern_label = "Common pattern" if pattern == "common" else "Rare pattern"

        rows.append(f"""
        <tr>
            <td>{pattern_label}</td>
            <td>{size}</td>
            <td class="result-cell">{format_cell(finder, "finder")}</td>
            <td class="result-cell">{format_cell(find_grep, "find_grep")}</td>
            <td class="result-cell">{format_cell(ripgrep, "ripgrep")}</td>
        </tr>
        """)

    return f"""
    <table class="summary-table">
        <thead>
            <tr>
                <th>Scenario</th>
                <th>Repository Size</th>
                <th>finder</th>
                <th>find+grep</th>
                <th>ripgrep</th>
            </tr>
        </thead>
        <tbody>
            {''.join(rows)}
        </tbody>
    </table>
    """


def generate_history_section(history: List[Dict[str, Any]]) -> str:
    """Generate history section showing trends over time."""
    if len(history) < 2:
        return """
        <div class="info-box">
            <p>Historical trends will appear after multiple benchmark runs.</p>
        </div>
        """

    # Show last 5 runs
    recent = history[-5:]

    # Build table with sparkline-style trends
    rows = []

    # Get unique scenarios from latest run
    latest = history[-1]
    for result in latest["results"]:
        scenario = result["scenario"]
        pattern_label = "Common" if result["pattern"] == "common" else "Rare"
        scenario_label = f"{pattern_label} - {result['size']}"

        # Collect finder times across history for this scenario
        finder_trend = []
        for entry in recent:
            scenario_result = next((r for r in entry["results"] if r["scenario"] == scenario), None)
            if scenario_result and scenario_result.get("finder"):
                finder_trend.append(scenario_result["finder"])

        # Calculate overall trend
        if len(finder_trend) >= 2:
            first = finder_trend[0]
            last = finder_trend[-1]
            change = calculate_change(last, first)
            trend_class = f"trend-{change['status']}"
            trend_text = f"{change['symbol']} {abs(change['percent']):.1f}%"
        else:
            trend_class = "trend-neutral"
            trend_text = "—"

        # Create mini bar chart
        if finder_trend:
            max_val = max(finder_trend)
            bars = ' '.join([
                f'<span class="bar" style="height: {(val/max_val)*100}%" title="{val}ms"></span>'
                for val in finder_trend
            ])
        else:
            bars = "—"

        rows.append(f"""
        <tr>
            <td>{scenario_label}</td>
            <td class="sparkline">{bars}</td>
            <td class="{trend_class}">{trend_text}</td>
            <td>{format_ms(finder_trend[-1]) if finder_trend else 'N/A'}</td>
        </tr>
        """)

    return f"""
    <h2>📈 Performance Trends (finder)</h2>
    <p>Showing last {len(recent)} benchmark runs</p>
    <table class="history-table">
        <thead>
            <tr>
                <th>Scenario</th>
                <th>Trend</th>
                <th>Change</th>
                <th>Latest</th>
            </tr>
        </thead>
        <tbody>
            {''.join(rows)}
        </tbody>
    </table>
    """


def generate_html(history: List[Dict[str, Any]]) -> str:
    """Generate complete HTML report."""
    latest = history[-1] if history else None
    version = latest["version"] if latest else "Unknown"
    date = latest["date"] if latest else "Unknown"

    summary_table = generate_summary_table(history)
    history_section = generate_history_section(history)

    return f"""<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>FindeRS Benchmark Results</title>
    <style>
        * {{
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }}

        body {{
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif;
            line-height: 1.6;
            color: #333;
            background: #f5f5f5;
        }}

        .container {{
            max-width: 1200px;
            margin: 0 auto;
            padding: 2rem;
            background: white;
            min-height: 100vh;
        }}

        header {{
            border-bottom: 3px solid #0066cc;
            padding-bottom: 1rem;
            margin-bottom: 2rem;
        }}

        h1 {{
            font-size: 2.5rem;
            font-weight: 300;
            color: #0066cc;
            margin-bottom: 0.5rem;
        }}

        .subtitle {{
            color: #666;
            font-size: 1rem;
        }}

        h2 {{
            font-size: 1.8rem;
            font-weight: 400;
            color: #333;
            margin: 2rem 0 1rem 0;
            padding-bottom: 0.5rem;
            border-bottom: 1px solid #ddd;
        }}

        .info-box {{
            background: #e6f2ff;
            border-left: 4px solid #0066cc;
            padding: 1rem;
            margin: 1rem 0;
            border-radius: 4px;
        }}

        .info-box p {{
            margin: 0;
        }}

        table {{
            width: 100%;
            border-collapse: collapse;
            margin: 1.5rem 0;
            background: white;
            box-shadow: 0 1px 3px rgba(0,0,0,0.1);
        }}

        th, td {{
            padding: 0.75rem 1rem;
            text-align: left;
            border-bottom: 1px solid #ddd;
        }}

        th {{
            background: #f8f9fa;
            font-weight: 600;
            color: #495057;
            text-transform: uppercase;
            font-size: 0.875rem;
            letter-spacing: 0.5px;
        }}

        tr:hover {{
            background: #f8f9fa;
        }}

        .result-cell {{
            font-family: "SF Mono", Monaco, "Courier New", monospace;
            font-size: 0.95rem;
        }}

        .change-better {{
            color: #28a745;
            font-weight: bold;
        }}

        .change-worse {{
            color: #dc3545;
            font-weight: bold;
        }}

        .change-neutral {{
            color: #6c757d;
        }}

        .trend-better {{
            color: #28a745;
            font-weight: 600;
        }}

        .trend-worse {{
            color: #dc3545;
            font-weight: 600;
        }}

        .trend-neutral {{
            color: #6c757d;
        }}

        .sparkline {{
            width: 200px;
        }}

        .sparkline .bar {{
            display: inline-block;
            width: 8px;
            margin: 0 1px;
            background: #0066cc;
            vertical-align: bottom;
            min-height: 2px;
        }}

        .links {{
            margin: 2rem 0;
            padding: 1rem;
            background: #f8f9fa;
            border-radius: 4px;
        }}

        .links a {{
            color: #0066cc;
            text-decoration: none;
            font-weight: 500;
        }}

        .links a:hover {{
            text-decoration: underline;
        }}

        .about {{
            margin-top: 3rem;
            padding-top: 2rem;
            border-top: 1px solid #ddd;
            color: #666;
            font-size: 0.9rem;
        }}

        .about h3 {{
            font-size: 1.2rem;
            color: #333;
            margin-bottom: 0.5rem;
        }}

        code {{
            background: #f4f4f4;
            padding: 0.2rem 0.4rem;
            border-radius: 3px;
            font-family: "SF Mono", Monaco, "Courier New", monospace;
            font-size: 0.9em;
        }}

        footer {{
            margin-top: 3rem;
            padding-top: 2rem;
            border-top: 1px solid #ddd;
            text-align: center;
            color: #999;
            font-size: 0.875rem;
        }}
    </style>
</head>
<body>
    <div class="container">
        <header>
            <h1>🔍 FindeRS Benchmark Results</h1>
            <p class="subtitle">Version {version} • {date}</p>
        </header>

        <section>
            <h2>📊 Latest Results</h2>
            {summary_table}

            <div class="info-box">
                <p>
                    <strong>Legend:</strong>
                    <span class="change-better">↓ Better (faster)</span> •
                    <span class="change-neutral">→ Similar (±5%)</span> •
                    <span class="change-worse">↑ Worse (slower)</span>
                </p>
            </div>
        </section>

        <section>
            {history_section}
        </section>

        <div class="links">
            <p><strong>Detailed Reports:</strong> <a href="./latest/report/index.html">View Criterion benchmark reports →</a></p>
        </div>

        <section class="about">
            <h3>About These Benchmarks</h3>
            <p>
                Benchmarks compare <code>finder</code> against <code>find+grep</code> and <code>ripgrep</code>
                across different repository sizes and search patterns. All tools search for patterns in
                <code>*.rs</code> files.
            </p>
            <ul style="margin: 1rem 0; padding-left: 2rem;">
                <li><strong>Common pattern:</strong> Found in ~50% of files</li>
                <li><strong>Rare pattern:</strong> Found in 1 file</li>
                <li><strong>Small:</strong> ~100 files</li>
                <li><strong>Medium:</strong> ~1,000 files</li>
                <li><strong>Large:</strong> ~5,000 files</li>
            </ul>
        </section>

        <footer>
            <p>Generated automatically by comparison benchmarks workflow •
            <a href="https://github.com/ydkadri/finders">View repository →</a></p>
        </footer>
    </div>
</body>
</html>
"""


def main():
    """Main entry point."""
    if len(sys.argv) != 3:
        print("Usage: generate_benchmark_report.py <history.json> <output.html>")
        sys.exit(1)

    history_file = Path(sys.argv[1])
    output_file = Path(sys.argv[2])

    # Load history
    history = load_history(history_file)

    # Generate HTML
    html = generate_html(history)

    # Write output
    output_file.parent.mkdir(parents=True, exist_ok=True)
    with open(output_file, 'w') as f:
        f.write(html)

    print(f"✓ Generated benchmark report: {output_file}")
    print(f"  Processed {len(history)} historical entries")


if __name__ == "__main__":
    main()
