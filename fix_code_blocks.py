#!/usr/bin/env python3
"""
Batch annotate unannotated code blocks in mdBook markdown files.

This script scans all .md files and adds appropriate annotations:
- `text` for output blocks, directory trees, ASCII diagrams
- `ignore` for incomplete Rust code snippets
- Preserves existing annotations

Usage:
    python fix_code_blocks.py [--dry-run]
"""

import os
import re
import sys
from pathlib import Path

# Patterns to detect different block types
OUTPUT_PATTERNS = [
    r'^\d+',                    # Starts with numbers (like "1", "10", etc.)
    r'^[A-Z][a-z]+:',           # Output lines like "Name:", "Error:"
    r'^hi from',                # Thread output
    r'^[├└│]',                  # Directory tree characters
    r'^examples/',              # Directory paths
    r'^my_project/',            # Project directories
    r'^[🦀📝📦✨]',              # Emoji starts (output examples)
    r'^Index \d+:',             # Index output
    r'^Count after',            # Counter output
    r'^inner:',                 # Variable output
    r'^outer:',                 # Variable output
    r'^\s*[_~^\\]',             # ASCII art patterns
    r'^thread.*panicked',       # Panic output
    r'^error\[E\d+\]',          # Compiler errors (output)
    r'^\s+-->',                 # Rust error pointer
]

# Patterns for incomplete Rust code that needs `ignore`
INCOMPLETE_RUST_PATTERNS = [
    r'^fn \w+.*\{$',            # Function without body end
    r'^\s*//.*→',               # Comments showing transformations
    r'^if let PATTERN',         # Syntax placeholder
    r'^while let PATTERN',      # Syntax placeholder
    r'EXPRESSION',              # Placeholder text
    r'____',                    # Fill-in-the-blank
    r'^&i32\s+//',              # Type annotation comments
    r'^fn \w+\(.+\)\s*$',       # Function signature without body
    r'tokio::',                 # Needs tokio crate
    r'anyhow::',                # Needs anyhow crate
    r'thiserror::',             # Needs thiserror crate
    r'^use (tokio|anyhow|thiserror)',  # External crate imports
    r'#\[tokio::main\]',        # Tokio async main
    r'some_value',              # Undefined variable placeholder
    r'some_option',             # Undefined variable placeholder
]

def is_output_block(content: str) -> bool:
    """Check if content looks like program output or non-code."""
    lines = content.strip().split('\n')
    if not lines:
        return False
    
    first_line = lines[0].strip()
    
    # Check if matches output patterns
    for pattern in OUTPUT_PATTERNS:
        if re.match(pattern, first_line):
            return True
    
    # Check for ASCII art (lots of special chars)
    special_chars = set('┌┐└┘├┤│─═║╔╗╚╝╠╣▶◀►◄→←↑↓∿')
    first_line_chars = set(first_line)
    if len(first_line_chars & special_chars) >= 2:
        return True
    
    return False

def needs_ignore(content: str) -> bool:
    """Check if Rust code is incomplete and needs ignore."""
    for pattern in INCOMPLETE_RUST_PATTERNS:
        if re.search(pattern, content, re.MULTILINE):
            return True
    
    # Check for function signatures without proper bodies
    lines = content.strip().split('\n')
    for line in lines:
        # Standalone function signature
        if re.match(r'^fn \w+\([^)]*\)(\s*->\s*[^{]+)?$', line.strip()):
            return True
    
    return False

def process_file(filepath: Path, dry_run: bool = False) -> dict:
    """Process a single markdown file and fix code blocks."""
    stats = {'fixed': 0, 'skipped': 0, 'file': str(filepath)}
    
    with open(filepath, 'r', encoding='utf-8') as f:
        content = f.read()
    
    original_content = content
    
    # Pattern to match code blocks
    # Matches: ```lang or just ```
    code_block_pattern = r'(```)(rust|[a-z]*)?(\n)(.*?)(```)'
    
    def replace_block(match):
        nonlocal stats
        
        prefix = match.group(1)  # ```
        lang = match.group(2) or ''  # rust, bash, etc. or empty
        newline = match.group(3)
        block_content = match.group(4)
        suffix = match.group(5)  # ```
        
        # Skip if already has annotation
        if ',' in lang:
            stats['skipped'] += 1
            return match.group(0)
        
        # Determine annotation
        if lang == '' or lang == 'rust':
            if is_output_block(block_content):
                new_lang = 'text'
                stats['fixed'] += 1
            elif lang == 'rust' and needs_ignore(block_content):
                new_lang = 'rust,ignore'
                stats['fixed'] += 1
            elif lang == '':
                # Unlabeled block - check if it looks like output
                if is_output_block(block_content):
                    new_lang = 'text'
                    stats['fixed'] += 1
                else:
                    # Might be Rust or other - mark as text to be safe
                    new_lang = 'text'
                    stats['fixed'] += 1
            else:
                stats['skipped'] += 1
                return match.group(0)
        else:
            stats['skipped'] += 1
            return match.group(0)
        
        return f'{prefix}{new_lang}{newline}{block_content}{suffix}'
    
    # Apply replacements
    new_content = re.sub(code_block_pattern, replace_block, content, flags=re.DOTALL)
    
    if new_content != original_content:
        if not dry_run:
            with open(filepath, 'w', encoding='utf-8') as f:
                f.write(new_content)
        print(f"{'[DRY RUN] ' if dry_run else ''}Fixed {stats['fixed']} blocks in {filepath}")
    
    return stats

def main():
    dry_run = '--dry-run' in sys.argv
    
    # Find all markdown files
    docs_dir = Path(__file__).parent / 'docs' / 'src'
    
    if not docs_dir.exists():
        # Try alternate path
        docs_dir = Path('docs/src')
    
    if not docs_dir.exists():
        print(f"Error: Could not find docs/src directory")
        sys.exit(1)
    
    print(f"Scanning {docs_dir}...")
    if dry_run:
        print("DRY RUN MODE - no files will be modified")
    print()
    
    total_fixed = 0
    total_skipped = 0
    files_modified = 0
    
    for md_file in docs_dir.rglob('*.md'):
        stats = process_file(md_file, dry_run)
        total_fixed += stats['fixed']
        total_skipped += stats['skipped']
        if stats['fixed'] > 0:
            files_modified += 1
    
    print()
    print("=" * 50)
    print(f"Summary:")
    print(f"  Files modified: {files_modified}")
    print(f"  Blocks fixed: {total_fixed}")
    print(f"  Blocks skipped (already annotated): {total_skipped}")
    
    if dry_run:
        print()
        print("Run without --dry-run to apply changes")

if __name__ == '__main__':
    main()
