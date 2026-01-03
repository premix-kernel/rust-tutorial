#!/usr/bin/env python3
"""
Fix the incorrectly converted closing code block markers.
The previous script incorrectly changed closing ``` to ```text.
This script finds and fixes those issues.
"""

import os
import re
from pathlib import Path

def fix_file(filepath: Path) -> int:
    """Fix incorrectly formatted code blocks in a file."""
    with open(filepath, 'r', encoding='utf-8') as f:
        content = f.read()
    
    original = content
    fixes = 0
    
    # Pattern: ```text followed by blank line or --- (should be just ```)
    # This catches cases like:
    # }
    # ```text
    # 
    # ---
    
    # Fix pattern 1: }```text followed by newline and newline or ---
    pattern1 = r'(\n}\n)(```text)(\n\n---|\n\n###|\n\n##|\n\n\*\*|\n\nOutput|\n\n>|\n$)'
    content = re.sub(pattern1, r'\1```\3', content)
    
    # Fix pattern 2: Any closing block```text that's immediately followed by content that suggests it's a block end
    pattern2 = r"(```text)\n(\n---|\n\*\*|\n###|\n##|\n\n---|\nOutput:|\n\n\*\*|\n>\s)"
    content = re.sub(pattern2, r'```\n\2', content)
    
    # Fix pattern 3: Rust block that ends with ```text instead of ```
    # Look for closing of a should_panic or other Rust block
    pattern3 = r'(\n}\n)(```text)(\n)'
    content = re.sub(pattern3, r'\1```\3', content)
    
    # Count fixes
    if content != original:
        fixes = original.count('```text') - content.count('```text')
        with open(filepath, 'w', encoding='utf-8') as f:
            f.write(content)
        if fixes > 0:
            print(f"Fixed {fixes} blocks in {filepath}")
    
    return max(0, fixes)

def main():
    docs_dir = Path('docs/src')
    
    if not docs_dir.exists():
        docs_dir = Path(__file__).parent / 'docs' / 'src'
    
    if not docs_dir.exists():
        print("Error: Could not find docs/src")
        return
    
    total_fixes = 0
    for md_file in docs_dir.rglob('*.md'):
        fixes = fix_file(md_file)
        total_fixes += fixes
    
    print(f"\nTotal blocks fixed: {total_fixes}")

if __name__ == '__main__':
    main()
