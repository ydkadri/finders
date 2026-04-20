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
from jinja2 import Environment, FileSystemLoader


def load_history(history_file: Path) -> List[Dict[str, Any]]:
    """Load benchmark history from JSON file."""
    if not history_file.exists():
        return []

    with open(history_file) as f:
        return json.load(f)


def calculate_change(current: float, previous: float) -> Dict[str, Any]:
    """Calculate percentage change and determine status."""
    if previous == 0:
        return {"percent": 0, "status": "neutral", "symbol": "→"}

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


def format_cell(value, change):
    """Format a table cell with value and optional change indicator."""
    if value is None:
        return "N/A"

    result = format_ms(value)
    if change:
        result += f' <span class="change-{change["status"]}" title="{change["percent"]:.1f}%">{change["symbol"]}</span>'

    return result


def prepare_summary_data(history: List[Dict[str, Any]]) -> List[Dict[str, Any]]:
    """Prepare data for summary table."""
    if not history:
        return []

    latest = history[-1]
    previous = history[-2] if len(history) > 1 else None

    results = []
    for result in latest["results"]:
        scenario = result["scenario"]
        pattern_label = "Common pattern" if result["pattern"] == "common" else "Rare pattern"

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

        results.append({
            "pattern_label": pattern_label,
            "size": result["size"],
            "finder_display": format_cell(finder, changes.get("finder")),
            "find_grep_display": format_cell(find_grep, changes.get("find_grep")),
            "ripgrep_display": format_cell(ripgrep, changes.get("ripgrep")),
        })

    return results


def prepare_trend_data(history: List[Dict[str, Any]]) -> List[Dict[str, Any]]:
    """Prepare data for trend section."""
    if len(history) < 2:
        return []

    # Show last 5 runs
    recent = history[-5:]
    trends = []

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
            bars = ''.join([
                f'<span class="bar" style="height: {(val/max_val)*100}%" title="{val}ms"></span>'
                for val in finder_trend
            ])
        else:
            bars = "—"

        trends.append({
            "scenario_label": scenario_label,
            "sparkline": bars,
            "trend_class": trend_class,
            "trend_text": trend_text,
            "latest": format_ms(finder_trend[-1]) if finder_trend else "N/A",
        })

    return trends


def generate_html(history: List[Dict[str, Any]], template_dir: Path, output_dir: Path) -> str:
    """Generate complete HTML report using Jinja2 template."""
    latest = history[-1] if history else None
    version = latest["version"] if latest else "Unknown"
    date = latest["date"] if latest else "Unknown"

    # Prepare data for template
    results = prepare_summary_data(history)
    trends = prepare_trend_data(history)

    # Set up Jinja2 environment
    env = Environment(loader=FileSystemLoader(template_dir))
    template = env.get_template('benchmark_report.html')

    # Render template
    html = template.render(
        version=version,
        date=date,
        results=results,
        history_count=len(history),
        runs_shown=min(5, len(history)),
        trends=trends,
    )

    # Copy CSS file to output directory
    css_src = template_dir / 'benchmark_report.css'
    css_dst = output_dir / 'benchmark_report.css'
    if css_src.exists():
        css_dst.write_text(css_src.read_text())

    return html


def main():
    """Main entry point."""
    if len(sys.argv) != 3:
        print("Usage: generate_benchmark_report.py <history.json> <output.html>")
        sys.exit(1)

    history_file = Path(sys.argv[1])
    output_file = Path(sys.argv[2])

    # Load history
    history = load_history(history_file)

    # Template directory is relative to this script
    script_dir = Path(__file__).parent
    template_dir = script_dir.parent / 'templates'
    output_dir = output_file.parent

    # Generate HTML
    html = generate_html(history, template_dir, output_dir)

    # Write output
    output_file.parent.mkdir(parents=True, exist_ok=True)
    with open(output_file, 'w') as f:
        f.write(html)

    print(f"✓ Generated benchmark report: {output_file}")
    print(f"  Processed {len(history)} historical entries")
    print(f"  CSS copied to: {output_dir / 'benchmark_report.css'}")


if __name__ == "__main__":
    main()

