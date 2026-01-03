/* =========================================
   mdBook Custom Logic
   ========================================= */

document.addEventListener('DOMContentLoaded', function () {
    // 1. Process Blockquotes for Callouts
    const blockquotes = document.querySelectorAll('.markdown-section blockquote');

    blockquotes.forEach(bq => {
        const text = bq.textContent.trim();

        // Check for markers and apply classes
        if (text.includes('📌') || text.toLowerCase().includes('note:')) {
            bq.classList.add('note');
            // Optional: Remove the marker from text if it looks ugly
        } else if (text.includes('💡') || text.toLowerCase().includes('tip:') || text.toLowerCase().includes('เคล็ดลับ:')) {
            bq.classList.add('tip');
        } else if (text.includes('⚠️') || text.toLowerCase().includes('warning:') || text.toLowerCase().includes('คำเตือน:')) {
            bq.classList.add('warning');
        } else if (text.includes('🎯') || text.toLowerCase().includes('exercise:') || text.toLowerCase().includes('ลองทำดู:')) {
            bq.classList.add('exercise');
        }
    });

    // 2. Add fade-in animation to main content
    const content = document.querySelector('.content');
    if (content) {
        content.style.opacity = '0';
        content.style.transition = 'opacity 0.5s ease-in-out';
        setTimeout(() => {
            content.style.opacity = '1';
        }, 50);
    }

    // 3. Dynamic Footer
    const pageContent = document.querySelector('.page-content');
    if (pageContent) {
        const footer = document.createElement('footer');
        footer.innerHTML = `
            <div style="margin-top: 50px; border-top: 1px solid var(--table-border-color); padding-top: 20px; text-align: center; color: var(--fg); opacity: 0.8;">
                <p>Created with ❤️ by <strong>Rust Tutorial Team</strong></p>
                <p style="font-size: 0.9em;">
                    <a href="https://github.com/premix-kernel/rust-tutorial" target="_blank" style="text-decoration: none;">GitHub Repository</a>
                    &nbsp;•&nbsp;
                    <a href="https://github.com/premix-kernel/rust-tutorial/issues" target="_blank" style="text-decoration: none;">Report Issue</a>
                </p>
            </div>
        `;
        pageContent.appendChild(footer);
    }
});
