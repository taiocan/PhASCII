#!/usr/bin/env python3
"""Check repository text hygiene for governance and Markdown files."""

from __future__ import annotations

import sys
from dataclasses import dataclass
from pathlib import Path


BIDI_CONTROLS = {
    0x202A: "LEFT-TO-RIGHT EMBEDDING",
    0x202B: "RIGHT-TO-LEFT EMBEDDING",
    0x202C: "POP DIRECTIONAL FORMATTING",
    0x202D: "LEFT-TO-RIGHT OVERRIDE",
    0x202E: "RIGHT-TO-LEFT OVERRIDE",
    0x2066: "LEFT-TO-RIGHT ISOLATE",
    0x2067: "RIGHT-TO-LEFT ISOLATE",
    0x2068: "FIRST STRONG ISOLATE",
    0x2069: "POP DIRECTIONAL ISOLATE",
    0x200E: "LEFT-TO-RIGHT MARK",
    0x200F: "RIGHT-TO-LEFT MARK",
}

DEFAULT_FILES = [
    "AGENTS.md",
    "SPEC.md",
    "PLAN.md",
    "DECISIONS.md",
    "README.md",
    "QUALITY_RUBRIC.md",
    "BENCHMARK_PLAN.md",
]

MAX_LINE_LENGTHS = {
    "AGENTS.md": 180,
}


@dataclass(frozen=True)
class Issue:
    path: Path
    message: str
    line: int | None = None
    byte_offset: int | None = None
    code_point: str | None = None

    def format(self) -> str:
        parts = [str(self.path)]
        if self.line is not None:
            parts.append(f"line {self.line}")
        if self.byte_offset is not None:
            parts.append(f"byte {self.byte_offset}")
        if self.code_point is not None:
            parts.append(self.code_point)
        parts.append(self.message)
        return ": ".join(parts)


def report_issue(issues: list[Issue], *args, **kwargs) -> None:
    issues.append(Issue(*args, **kwargs))


def scan_file(path: Path) -> list[Issue]:
    issues: list[Issue] = []
    data = path.read_bytes()

    # Raw byte checks first so LF/CR/NUL issues are always reported.
    line_start = 0
    for line_no, line_bytes in enumerate(data.split(b"\n"), start=1):
        rel_cr = line_bytes.find(b"\r")
        if rel_cr != -1:
            report_issue(
                issues,
                path=path,
                line=line_no,
                byte_offset=line_start + rel_cr,
                message="contains CR byte; file must be LF-only",
            )

        rel_nul = line_bytes.find(b"\x00")
        if rel_nul != -1:
            report_issue(
                issues,
                path=path,
                line=line_no,
                byte_offset=line_start + rel_nul,
                message="contains NUL byte",
            )

        line_start += len(line_bytes) + 1

    try:
        text = data.decode("utf-8")
    except UnicodeDecodeError as exc:
        report_issue(
            issues,
            path=path,
            byte_offset=exc.start,
            message="is not valid UTF-8",
        )
        return issues

    limit = MAX_LINE_LENGTHS.get(path.name, 220)
    for line_no, line in enumerate(text.splitlines(), start=1):
        if len(line) > limit:
            report_issue(
                issues,
                path=path,
                line=line_no,
                message=f"line length {len(line)} exceeds limit {limit}",
            )

        if path.name == "AGENTS.md":
            for char_index, ch in enumerate(line):
                if ord(ch) > 127:
                    byte_offset = len(line[:char_index].encode("utf-8"))
                    report_issue(
                        issues,
                        path=path,
                        line=line_no,
                        byte_offset=byte_offset,
                        code_point=f"U+{ord(ch):04X}",
                        message="contains non-ASCII character; AGENTS.md must be ASCII-only",
                    )
                    break

        for char_index, ch in enumerate(line):
            code_point = ord(ch)
            if code_point in BIDI_CONTROLS:
                byte_offset = len(line[:char_index].encode("utf-8"))
                report_issue(
                    issues,
                    path=path,
                    line=line_no,
                    byte_offset=byte_offset,
                    code_point=f"U+{code_point:04X}",
                    message=f"contains bidi control character {BIDI_CONTROLS[code_point]}",
                )

    return issues


def main() -> int:
    paths = [Path(arg) for arg in sys.argv[1:]] if len(sys.argv) > 1 else [Path(name) for name in DEFAULT_FILES]

    issues: list[Issue] = []
    for path in paths:
        if not path.exists():
            print(f"warning: skipping missing file {path}", file=sys.stderr)
            continue
        issues.extend(scan_file(path))

    for issue in issues:
        print(issue.format(), file=sys.stderr)

    if issues:
        print(f"Text hygiene check failed with {len(issues)} issue(s).", file=sys.stderr)
        return 1

    print("Text hygiene check passed.", file=sys.stderr)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
