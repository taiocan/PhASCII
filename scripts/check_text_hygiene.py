#!/usr/bin/env python3
"""Check governance and Markdown files for text hygiene issues."""

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

CHECKED_FILES = [
    "AGENTS.md",
    "SPEC.md",
    "PLAN.md",
    "DECISIONS.md",
    "README.md",
    "QUALITY_RUBRIC.md",
    "BENCHMARK_PLAN.md",
]


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


def add_issue(
    issues: list[Issue],
    path: Path,
    message: str,
    line: int | None = None,
    byte_offset: int | None = None,
    code_point: str | None = None,
) -> None:
    issues.append(
        Issue(
            path=path,
            message=message,
            line=line,
            byte_offset=byte_offset,
            code_point=code_point,
        )
    )


def scan_file(path: Path) -> list[Issue]:
    data = path.read_bytes()
    issues: list[Issue] = []

    line_start = 0
    for line_no, chunk in enumerate(data.split(b"\n"), start=1):
        cursor = 0
        while True:
            cr_index = chunk.find(b"\r", cursor)
            if cr_index == -1:
                break
            add_issue(
                issues,
                path,
                "contains CR byte; file must be LF-only",
                line=line_no,
                byte_offset=line_start + cr_index,
            )
            cursor = cr_index + 1

        cursor = 0
        while True:
            nul_index = chunk.find(b"\x00", cursor)
            if nul_index == -1:
                break
            add_issue(
                issues,
                path,
                "contains NUL byte",
                line=line_no,
                byte_offset=line_start + nul_index,
            )
            cursor = nul_index + 1

        line_start += len(chunk) + 1

    try:
        text = data.decode("utf-8")
    except UnicodeDecodeError as exc:
        add_issue(
            issues,
            path,
            "is not valid UTF-8",
            byte_offset=exc.start,
        )
        return issues

    limit = 180 if path.name == "AGENTS.md" else 220
    is_agents = path.name == "AGENTS.md"

    byte_offset = 0
    for line_no, line in enumerate(text.split("\n"), start=1):
        if len(line) > limit:
            add_issue(
                issues,
                path,
                f"line length {len(line)} exceeds limit {limit}",
                line=line_no,
            )

        for char_index, ch in enumerate(line):
            code_point = ord(ch)
            if is_agents and code_point > 127:
                add_issue(
                    issues,
                    path,
                    "contains non-ASCII character; AGENTS.md must be ASCII-only",
                    line=line_no,
                    byte_offset=byte_offset + len(line[:char_index].encode("utf-8")),
                    code_point=f"U+{code_point:04X}",
                )
            if code_point in BIDI_CONTROLS:
                add_issue(
                    issues,
                    path,
                    f"contains bidi control character {BIDI_CONTROLS[code_point]}",
                    line=line_no,
                    byte_offset=byte_offset + len(line[:char_index].encode("utf-8")),
                    code_point=f"U+{code_point:04X}",
                )

        byte_offset += len(line.encode("utf-8")) + 1

    return issues


def main() -> int:
    issues: list[Issue] = []

    for name in CHECKED_FILES:
        path = Path(name)
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
