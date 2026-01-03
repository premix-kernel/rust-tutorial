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
    // 3. Dynamic Footer
    const contentWrapper = document.querySelector('main');
    if (contentWrapper) {
        const currentUrl = encodeURIComponent(window.location.href);
        const title = encodeURIComponent(document.title);

        const footer = document.createElement('footer');
        footer.innerHTML = `
            <div style="margin-top: 50px; border-top: 1px solid var(--table-border-color); padding-top: 20px; text-align: center; color: var(--fg); opacity: 0.8;">
                <div style="margin-bottom: 15px;">
                    <span style="font-size: 0.9em; margin-right: 10px; font-weight: bold;">Share this page:</span>
                    <a href="https://twitter.com/intent/tweet?url=${currentUrl}&text=${title}" target="_blank" style="text-decoration: none; margin-right: 10px; color: var(--fg);">𝕏 (Twitter)</a>
                    <a href="https://www.facebook.com/sharer/sharer.php?u=${currentUrl}" target="_blank" style="text-decoration: none; margin-right: 10px; color: var(--fg);">📘 Facebook</a>
                    <a href="https://www.linkedin.com/shareArticle?mini=true&url=${currentUrl}&title=${title}" target="_blank" style="text-decoration: none; color: var(--fg);">💼 LinkedIn</a>
                </div>
                <p>Created with ❤️ by <strong>Rust Tutorial Team</strong></p>
                <p style="font-size: 0.9em;">
                    <a href="https://github.com/premix-kernel/rust-tutorial" target="_blank" style="text-decoration: none;">GitHub Repository</a>
                    &nbsp;•&nbsp;
                    <a href="https://github.com/premix-kernel/rust-tutorial/issues" target="_blank" style="text-decoration: none;">Report Issue</a>
                </p>
            </div>
        `;
        contentWrapper.appendChild(footer);
    } else {
        console.error("Rust Tutorial: Could not find 'main' element to append footer.");
    }
});
